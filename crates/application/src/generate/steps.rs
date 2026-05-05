use super::helpers::{build_generation_input, build_query_text, truncate};
use super::{
    CandidaturePlan, GenerateApplicationUseCase, GenerateError, Livrables, RerankResponse,
};
use crate::events::GenerationStep;
use crate::prompts;
use crate::AppError;
use domain::{Chunk, CoverLetter, InstanceId, Offre, ProfilId, Restitution, Resume};
use ports::{EmbedMode, ExtractionRequest, LlmClient};
use once_cell::sync::Lazy;
use std::sync::Arc;
use tracing::warn;

static RERANK_SCHEMA: Lazy<serde_json::Value> =
    Lazy::new(|| serde_json::to_value(schemars::schema_for!(RerankResponse)).unwrap());
static PLAN_SCHEMA: Lazy<serde_json::Value> =
    Lazy::new(|| serde_json::to_value(schemars::schema_for!(CandidaturePlan)).unwrap());
static RESTITUTION_SCHEMA: Lazy<serde_json::Value> =
    Lazy::new(|| serde_json::to_value(schemars::schema_for!(Restitution)).unwrap());
static RESUME_SCHEMA: Lazy<serde_json::Value> =
    Lazy::new(|| serde_json::to_value(schemars::schema_for!(Resume)).unwrap());
static COVER_LETTER_SCHEMA: Lazy<serde_json::Value> =
    Lazy::new(|| serde_json::to_value(schemars::schema_for!(CoverLetter)).unwrap());

pub async fn retrieve_chunks(
    this: &GenerateApplicationUseCase,
    offre: &Offre,
    profil_id: ProfilId,
) -> Result<Vec<(Chunk, f32)>, GenerateError> {
    let query_text = build_query_text(offre);

    let mut embeddings = this
        .embedder
        .embed(&[&query_text], EmbedMode::Query)
        .await
        .map_err(|e| AppError::Other(e.to_string()))?;

    let query_embedding = embeddings
        .pop()
        .ok_or_else(|| AppError::Other("embedder a renvoyé 0 vecteurs".into()))?;

    let candidates = this
        .chunks
        .top_k_by_embedding(profil_id, &query_embedding, 8)
        .await
        .map_err(AppError::Repo)?;

    Ok(candidates)
}

pub async fn rerank(
    _this: &GenerateApplicationUseCase,
    offre: &Offre,
    candidates: &[(Chunk, f32)],
    llm: Arc<dyn LlmClient>,
) -> Result<Vec<Chunk>, GenerateError> {
    let listing = candidates
        .iter()
        .enumerate()
        .map(|(i, (c, score))| {
            format!(
                "[{i}] ({}, score={:.2}) {} — {}",
                c.kind.as_str(),
                score,
                c.titre,
                truncate(&c.content, 300)
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    let req = ports::ExtractionRequest {
        system: Some(prompts::generate::RERANK_SYSTEM.into()),
        instruction: prompts::generate::rerank_instruction(candidates.len()),
        input: vec![ports::MessageContent::Text(format!(
            "## OFFRE\nEntreprise: {}\nIntitulé: {}\nMissions: {}\nStack: {}\nExigences: {}\n\n## CHUNKS CANDIDATS\n{}",
            offre.entreprise,
            offre.intitule,
            offre.structured.missions.join(" ; "),
            offre.structured.stack.join(", "),
            offre.structured.exigences.join(" ; "),
            listing
        ))],
        schema_name: "RerankResponse".into(),
        schema_description: "Sélection des chunks pertinents avec justification".into(),
        json_schema: RERANK_SCHEMA.clone(),
        model: None,
        max_tokens: Some(1024),
    };

    let response_json = llm
        .extract(req)
        .await
        .map_err(|e| AppError::Other(e.to_string()))?;

    let response: RerankResponse =
        serde_json::from_value(response_json).map_err(|e| AppError::Other(e.to_string()))?;

    let retained: Vec<Chunk> = response
        .indices_retenus
        .into_iter()
        .filter_map(|i| candidates.get(i).map(|(c, _)| c.clone()))
        .take(6)
        .collect();

    if retained.is_empty() {
        warn!("rerank a retenu 0 chunks — fallback sur les top-3 par score");
        return Ok(candidates.iter().take(3).map(|(c, _)| c.clone()).collect());
    }

    Ok(retained)
}

pub async fn plan(
    _this: &GenerateApplicationUseCase,
    offre: &Offre,
    retained: &[Chunk],
    llm: Arc<dyn LlmClient>,
) -> Result<CandidaturePlan, GenerateError> {
    let chunks_listing = retained
        .iter()
        .map(|c| {
            format!(
                "- ({}) {} — {}",
                c.kind.as_str(),
                c.titre,
                truncate(&c.content, 200)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let req = ports::ExtractionRequest {
        system: Some(prompts::generate::PLAN_SYSTEM.into()),
        instruction: prompts::generate::PLAN_INSTRUCTION.into(),
        input: vec![ports::MessageContent::Text(format!(
            "## OFFRE\n{}\n## ENTREPRISE: {}\n## INTITULÉ: {}\n\n## CHUNKS RETENUS\n{}",
            offre.structured.resume_court, offre.entreprise, offre.intitule, chunks_listing,
        ))],
        schema_name: "CandidaturePlan".into(),
        schema_description: "Stratégie de la candidature".into(),
        json_schema: PLAN_SCHEMA.clone(),
        model: None,
        max_tokens: Some(1024),
    };

    let response_json = llm
        .extract(req)
        .await
        .map_err(|e| GenerateError::App(AppError::Other(e.to_string())))?;

    serde_json::from_value(response_json)
        .map_err(|e| GenerateError::App(AppError::Other(e.to_string())))
}

pub async fn maybe_generate_restitution(
    this: &GenerateApplicationUseCase,
    livrables: Livrables,
    offre: &Offre,
    instance_id: InstanceId,
    llm: Arc<dyn LlmClient>,
) -> Result<Option<Restitution>, GenerateError> {
    if !livrables.restitution {
        return Ok(None);
    }
    this.events
        .started(instance_id, GenerationStep::Restitution);

    let req = ports::ExtractionRequest {
        system: Some(prompts::generate::RESTITUTION_SYSTEM.into()),
        instruction: prompts::generate::RESTITUTION_INSTRUCTION.into(),
        input: vec![ports::MessageContent::Text(format!(
            "Entreprise: {}\nIntitulé: {}\nLocalisation: {}\nContrat: {}\n\nTexte brut de l'offre:\n{}",
            offre.entreprise,
            offre.intitule,
            offre.localisation.as_deref().unwrap_or("?"),
            offre.contrat.as_deref().unwrap_or("?"),
            truncate(&offre.raw_text, 12000),
        ))],
        schema_name: "Restitution".into(),
        schema_description: "Fiche d'analyse haute-fidélité d'une offre".into(),
        json_schema: RESTITUTION_SCHEMA.clone(),
        model: None,
        max_tokens: Some(4000),
    };

    let response_json = llm.extract(req).await.map_err(|e| {
        this.events
            .failed(instance_id, GenerationStep::Restitution, e.to_string());
        AppError::Other(e.to_string())
    })?;

    let restitution: Restitution = serde_json::from_value(response_json).map_err(|e| {
        this.events
            .failed(instance_id, GenerationStep::Restitution, e.to_string());
        AppError::Other(e.to_string())
    })?;

    this.events
        .done(instance_id, GenerationStep::Restitution, None);
    Ok(Some(restitution))
}

pub async fn maybe_generate_resume(
    this: &GenerateApplicationUseCase,
    livrables: Livrables,
    offre: &Offre,
    profil: &domain::Profil,
    retained: &[Chunk],
    plan: &CandidaturePlan,
    instance_id: InstanceId,
    llm: Arc<dyn LlmClient>,
) -> Result<Option<Resume>, GenerateError> {
    if !livrables.resume {
        return Ok(None);
    }
    this.events.started(instance_id, GenerationStep::Resume);

    let req = ExtractionRequest {
        system: Some(prompts::generate::RESUME_SYSTEM.into()),
        instruction: prompts::generate::RESUME_INSTRUCTION.into(),
        input: vec![ports::MessageContent::Text(build_generation_input(
            offre, profil, retained, plan,
        ))],
        schema_name: "Resume".into(),
        schema_description: "CV structuré, contenu adapté à l'offre".into(),
        json_schema: RESUME_SCHEMA.clone(),
        model: None,
        max_tokens: Some(3000),
    };

    let response_json = llm.extract(req).await.map_err(|e| {
        this.events
            .failed(instance_id, GenerationStep::Resume, e.to_string());
        AppError::Other(e.to_string())
    })?;

    let resume: Resume = serde_json::from_value(response_json).map_err(|e| {
        this.events
            .failed(instance_id, GenerationStep::Resume, e.to_string());
        AppError::Other(e.to_string())
    })?;

    this.events.done(instance_id, GenerationStep::Resume, None);
    Ok(Some(resume))
}

pub async fn maybe_generate_cover_letter(
    this: &GenerateApplicationUseCase,
    livrables: Livrables,
    offre: &Offre,
    profil: &domain::Profil,
    retained: &[Chunk],
    plan: &CandidaturePlan,
    instance_id: InstanceId,
    llm: Arc<dyn LlmClient>,
) -> Result<Option<CoverLetter>, GenerateError> {
    if !livrables.cover_letter {
        return Ok(None);
    }
    this.events
        .started(instance_id, GenerationStep::CoverLetter);

    let req = ExtractionRequest {
        system: Some(prompts::generate::COVER_LETTER_SYSTEM.into()),
        instruction: prompts::generate::COVER_LETTER_INSTRUCTION.into(),
        input: vec![ports::MessageContent::Text(build_generation_input(
            offre, profil, retained, plan,
        ))],
        schema_name: "CoverLetter".into(),
        schema_description: "Lettre structurée par paragraphes typés".into(),
        json_schema: COVER_LETTER_SCHEMA.clone(),
        model: None,
        max_tokens: Some(2500),
    };

    let response_json = llm.extract(req).await.map_err(|e| {
        this.events
            .failed(instance_id, GenerationStep::CoverLetter, e.to_string());
        AppError::Other(e.to_string())
    })?;

    let cover_letter: CoverLetter = serde_json::from_value(response_json).map_err(|e| {
        this.events
            .failed(instance_id, GenerationStep::CoverLetter, e.to_string());
        AppError::Other(e.to_string())
    })?;

    this.events
        .done(instance_id, GenerationStep::CoverLetter, None);
    Ok(Some(cover_letter))
}
