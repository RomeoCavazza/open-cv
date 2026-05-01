//! `GenerateApplicationUseCase` — le cœur métier de l'app.
//!
//! Pipeline en 7 étapes :
//!
//!   1. RETRIEVE  — embedding de l'offre, top-K chunks via pgvector
//!   2. RERANK    — LLM filtre les top-K à top-N pertinents
//!   3. PLAN      — LLM produit un plan (sections à mettre en avant, angle
//!                  de la lettre, mots-clés à intégrer)
//!   4. PARALLEL  — 3 générations LLM en parallèle :
//!                    • Restitution (analyse de l'offre)
//!                    • Resume       (CV adapté)
//!                    • CoverLetter  (lettre adaptée)
//!   5. VALIDATE  — schéma JSON, longueurs raisonnables, anti-hallucination
//!   6. PERSIST   — UPDATE instances + miroir fichier `data/instances/<slug>/`
//!   7. DONE      — événement final
//!
//! Chaque étape émet un événement sur le `EventBus` pour le streaming SSE.

use std::sync::Arc;

use chrono::Utc;
use domain::{
    Chunk, CoverLetter, Instance, InstanceId, InstanceStatus, Offre,
    OffreId, ProfilId, Restitution, Resume, Slug,
};
use ports::{
    ChunkRepo, EmbedMode, Embedder, ExtractionRequest, InstanceRepo, LlmClient,
    OffreRepo, ProfilRepo,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{info, instrument, warn};

use crate::events::{EventBus, GenerationStep};
use crate::AppError;

// ─────────────────────────────────────────────────────────────────
// Inputs / outputs
// ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct GenerateInput {
    pub offre_id: OffreId,
    pub profil_id: ProfilId,
    pub livrables: Livrables,
}

#[derive(Debug, Clone, Copy)]
pub struct Livrables {
    pub restitution: bool,
    pub resume: bool,
    pub cover_letter: bool,
}

impl Default for Livrables {
    fn default() -> Self {
        Self {
            restitution: true,
            resume: true,
            cover_letter: true,
        }
    }
}

impl Livrables {
    pub fn aucun(&self) -> bool {
        !self.restitution && !self.resume && !self.cover_letter
    }
}

#[derive(Debug, Clone)]
pub struct GenerateOutput {
    pub instance_id: InstanceId,
    pub slug: Slug,
    pub restitution: Option<Restitution>,
    pub resume: Option<Resume>,
    pub cover_letter: Option<CoverLetter>,
}

// ─────────────────────────────────────────────────────────────────
// Erreurs spécifiques au use case
// ─────────────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum GenerateError {
    #[error("offre introuvable : {0}")]
    OffreIntrouvable(OffreId),

    #[error("profil introuvable : {0}")]
    ProfilIntrouvable(ProfilId),

    #[error("aucun livrable demandé")]
    AucunLivrable,

    #[error("RAG : aucun chunk pertinent trouvé pour ce profil")]
    AucunChunkPertinent,

    #[error("génération invalide : {0}")]
    Invalide(String),

    #[error(transparent)]
    App(#[from] AppError),
}

// ─────────────────────────────────────────────────────────────────
// Sous-types pour rerank et plan
// ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct RerankResponse {
    /// Indices (0-based) des chunks à conserver, par ordre de pertinence
    /// décroissante.
    indices_retenus: Vec<usize>,
    /// Justification courte du choix.
    raisonnement: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct CandidaturePlan {
    /// Angle stratégique de la candidature, en 1-2 phrases.
    angle: String,
    /// Forces à mettre en avant.
    forces_a_souligner: Vec<String>,
    /// Mots-clés de l'offre à intégrer dans le CV/lettre.
    mots_cles_critiques: Vec<String>,
    /// Si pertinent : faiblesses à adresser dans la lettre.
    faiblesses_a_adresser: Vec<String>,
}

// ─────────────────────────────────────────────────────────────────
// Le use case
// ─────────────────────────────────────────────────────────────────

pub struct GenerateApplicationUseCase {
    pub offres: Arc<dyn OffreRepo>,
    pub profils: Arc<dyn ProfilRepo>,
    pub chunks: Arc<dyn ChunkRepo>,
    pub instances: Arc<dyn InstanceRepo>,
    pub llm: Arc<dyn LlmClient>,
    pub embedder: Arc<dyn Embedder>,
    pub events: Arc<EventBus>,
}

impl GenerateApplicationUseCase {
    pub fn new(
        offres: Arc<dyn OffreRepo>,
        profils: Arc<dyn ProfilRepo>,
        chunks: Arc<dyn ChunkRepo>,
        instances: Arc<dyn InstanceRepo>,
        llm: Arc<dyn LlmClient>,
        embedder: Arc<dyn Embedder>,
        events: Arc<EventBus>,
    ) -> Self {
        Self {
            offres,
            profils,
            chunks,
            instances,
            llm,
            embedder,
            events,
        }
    }
    /// Exécute le pipeline complet. La fonction est `instrument` pour que tous
    /// les logs internes soient enrichis avec `instance_id` automatiquement.
    #[instrument(skip(self), fields(instance_id))]
    pub async fn execute(
        &self,
        input: GenerateInput,
    ) -> Result<GenerateOutput, GenerateError> {
        if input.livrables.aucun() {
            return Err(GenerateError::AucunLivrable);
        }

        // Création de l'instance "draft" en base, pour avoir un ID stable
        // dès le début (utile pour le SSE).
        let offre = self
            .offres
            .get_by_id(input.offre_id)
            .await
            .map_err(|e| AppError::Repo(e))?
            .ok_or(GenerateError::OffreIntrouvable(input.offre_id))?;

        let profil = self
            .profils
            .get_by_id(input.profil_id)
            .await
            .map_err(|e| AppError::Repo(e))?
            .ok_or(GenerateError::ProfilIntrouvable(input.profil_id))?;

        let instance_id = InstanceId::new();
        let slug = build_slug(&offre, instance_id);

        // Tracing field : ajoute instance_id au span courant pour tous les logs.
        tracing::Span::current()
            .record("instance_id", &tracing::field::display(&instance_id));

        info!("démarrage génération pour offre={} profil={}", offre.entreprise, profil.label);

        // Étape 1 : RETRIEVE
        self.events.started(instance_id, GenerationStep::Retrieve);
        let candidates = self.retrieve_chunks(&offre, profil.id).await?;
        self.events.done(
            instance_id,
            GenerationStep::Retrieve,
            Some(format!("{} chunks candidats", candidates.len())),
        );

        if candidates.is_empty() {
            return Err(GenerateError::AucunChunkPertinent);
        }

        // Étape 2 : RERANK
        self.events.started(instance_id, GenerationStep::Rerank);
        let retained = self.rerank(&offre, &candidates).await?;
        self.events.done(
            instance_id,
            GenerationStep::Rerank,
            Some(format!("{} chunks retenus", retained.len())),
        );

        // Étape 3 : PLAN
        self.events.started(instance_id, GenerationStep::Plan);
        let plan = self.plan(&offre, &retained).await?;
        self.events.done(
            instance_id,
            GenerationStep::Plan,
            Some(plan.angle.clone()),
        );

        // Étape 4 : 3 générations en parallèle.
        // tokio::join! attend les 3, peu importe l'ordre de terminaison.
        let (restitution_res, resume_res, cover_letter_res) = tokio::join!(
            self.maybe_generate_restitution(input.livrables, &offre, instance_id),
            self.maybe_generate_resume(
                input.livrables,
                &offre,
                &profil,
                &retained,
                &plan,
                instance_id,
            ),
            self.maybe_generate_cover_letter(
                input.livrables,
                &offre,
                &profil,
                &retained,
                &plan,
                instance_id,
            ),
        );

        let restitution = restitution_res?;
        let resume = resume_res?;
        let cover_letter = cover_letter_res?;

        // Étape 5 : VALIDATE
        self.events.started(instance_id, GenerationStep::Validate);
        validate_outputs(&offre, restitution.as_ref(), resume.as_ref(), cover_letter.as_ref())?;
        self.events.done(instance_id, GenerationStep::Validate, None);

        // Étape 6 : PERSIST
        self.events.started(instance_id, GenerationStep::Persist);
        let now = Utc::now();
        let instance = Instance {
            id: instance_id,
            slug: slug.clone(),
            offre_id: offre.id,
            profil_id: profil.id,
            status: InstanceStatus::Ready,
            resume_json: resume
                .as_ref()
                .map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null)),
            cover_letter_json: cover_letter
                .as_ref()
                .map(|cl| serde_json::to_value(cl).unwrap_or(serde_json::Value::Null)),
            notes: serde_json::json!({
                "restitution": restitution.as_ref(),
                "plan": plan,
            }),
            created_at: now,
            updated_at: now,
            sent_at: None,
        };
        self.instances.upsert(&instance).await.map_err(AppError::Repo)?;
        self.events.done(instance_id, GenerationStep::Persist, None);

        self.events.done(instance_id, GenerationStep::Done, None);

        Ok(GenerateOutput {
            instance_id,
            slug,
            restitution,
            resume,
            cover_letter,
        })
    }

    // ─────────────────────────────────────────────────────────────
    // Étape 1 — RETRIEVE
    // ─────────────────────────────────────────────────────────────
    async fn retrieve_chunks(
        &self,
        offre: &Offre,
        profil_id: ProfilId,
    ) -> Result<Vec<(Chunk, f32)>, GenerateError> {
        let query_text = build_query_text(offre);

        let mut embeddings = self
            .embedder
            .embed(&[&query_text], EmbedMode::Query)
            .await
            .map_err(|e| AppError::Other(e.to_string()))?;

        let query_embedding = embeddings
            .pop()
            .ok_or_else(|| AppError::Other("embedder a renvoyé 0 vecteurs".into()))?;

        let candidates = self
            .chunks
            .top_k_by_embedding(profil_id, &query_embedding, 12)
            .await
            .map_err(AppError::Repo)?;

        Ok(candidates)
    }

    // ─────────────────────────────────────────────────────────────
    // Étape 2 — RERANK
    // ─────────────────────────────────────────────────────────────
    async fn rerank(
        &self,
        offre: &Offre,
        candidates: &[(Chunk, f32)],
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

        let req = ExtractionRequest {
            system: Some(
                "Tu es un assistant qui sélectionne les expériences/projets/compétences \
                 d'un candidat les plus pertinents pour une offre donnée."
                    .into(),
            ),
            instruction: format!(
                "Voici une offre. Voici {} chunks candidats du profil. \
                 Renvoie les indices (max 6) des chunks réellement pertinents \
                 pour cette offre, par ordre de priorité décroissante.",
                candidates.len()
            ),
            input: format!(
                "## OFFRE\nEntreprise: {}\nIntitulé: {}\nMissions: {}\nStack: {}\nExigences: {}\n\n## CHUNKS CANDIDATS\n{}",
                offre.entreprise,
                offre.intitule,
                offre.structured.missions.join(" ; "),
                offre.structured.stack.join(", "),
                offre.structured.exigences.join(" ; "),
                listing
            ),
            schema_name: "RerankResponse".into(),
            schema_description: "Sélection des chunks pertinents avec justification".into(),
            json_schema: serde_json::to_value(&schemars::schema_for!(RerankResponse)).unwrap(),
            model: None,
            max_tokens: Some(1024),
        };

        let response_json = self
            .llm
            .extract(req)
            .await
            .map_err(|e| AppError::Other(e.to_string()))?;

        let response: RerankResponse = serde_json::from_value(response_json)
            .map_err(|e| AppError::Other(e.to_string()))?;

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

    // ─────────────────────────────────────────────────────────────
    // Étape 3 — PLAN
    // ─────────────────────────────────────────────────────────────
    async fn plan(
        &self,
        offre: &Offre,
        retained: &[Chunk],
    ) -> Result<CandidaturePlan, GenerateError> {
        let chunks_listing = retained
            .iter()
            .map(|c| format!("- ({}) {} — {}", c.kind.as_str(), c.titre, truncate(&c.content, 200)))
            .collect::<Vec<_>>()
            .join("\n");

        let req = ExtractionRequest {
            system: Some(
                "Tu es un coach RH qui prépare la stratégie d'une candidature. \
                 Tu dois identifier l'angle le plus efficace, les forces à \
                 souligner, et les éventuelles faiblesses à adresser."
                    .into(),
            ),
            instruction:
                "Produis un plan de candidature pour cette offre, à partir des \
                 chunks de profil retenus."
                    .into(),
            input: format!(
                "## OFFRE\n{}\n## ENTREPRISE: {}\n## INTITULÉ: {}\n\n## CHUNKS RETENUS\n{}",
                offre.structured.resume_court,
                offre.entreprise,
                offre.intitule,
                chunks_listing,
            ),
            schema_name: "CandidaturePlan".into(),
            schema_description: "Stratégie de la candidature".into(),
            json_schema: serde_json::to_value(&schemars::schema_for!(CandidaturePlan)).unwrap(),
            model: None,
            max_tokens: Some(1024),
        };

        let response_json = self
            .llm
            .extract(req)
            .await
            .map_err(|e| GenerateError::App(AppError::Other(e.to_string())))?;

        serde_json::from_value(response_json)
            .map_err(|e| GenerateError::App(AppError::Other(e.to_string())))
    }

    // ─────────────────────────────────────────────────────────────
    // Étape 4a — RESTITUTION (parallèle)
    // ─────────────────────────────────────────────────────────────
    async fn maybe_generate_restitution(
        &self,
        livrables: Livrables,
        offre: &Offre,
        instance_id: InstanceId,
    ) -> Result<Option<Restitution>, GenerateError> {
        if !livrables.restitution {
            return Ok(None);
        }
        self.events.started(instance_id, GenerationStep::Restitution);

        let req = ExtractionRequest {
            system: Some(
                "Tu produis des fiches d'analyse d'offres d'emploi pour aider \
                 un candidat junior à décider et à se préparer."
                    .into(),
            ),
            instruction:
                "Analyse cette offre. Produis une restitution structurée : \
                 synthèse, fit (avec score 0-100, justifié), contenu explicite, \
                 signaux implicites, points d'attention, questions d'entretien."
                    .into(),
            input: format!(
                "Entreprise: {}\nIntitulé: {}\nLocalisation: {}\nContrat: {}\n\nTexte brut:\n{}",
                offre.entreprise,
                offre.intitule,
                offre.localisation.as_deref().unwrap_or("?"),
                offre.contrat.as_deref().unwrap_or("?"),
                truncate(&offre.raw_text, 4000),
            ),
            schema_name: "Restitution".into(),
            schema_description: "Fiche d'analyse structurée d'une offre".into(),
            json_schema: serde_json::to_value(&schemars::schema_for!(Restitution)).unwrap(),
            model: None,
            max_tokens: Some(2048),
        };

        let response_json = self
            .llm
            .extract(req)
            .await
            .map_err(|e| {
                self.events
                    .failed(instance_id, GenerationStep::Restitution, e.to_string());
                AppError::Other(e.to_string())
            })?;

        let restitution: Restitution = serde_json::from_value(response_json)
            .map_err(|e| {
                self.events
                    .failed(instance_id, GenerationStep::Restitution, e.to_string());
                AppError::Other(e.to_string())
            })?;

        self.events
            .done(instance_id, GenerationStep::Restitution, None);
        Ok(Some(restitution))
    }

    // ─────────────────────────────────────────────────────────────
    // Étape 4b — RESUME (parallèle)
    // ─────────────────────────────────────────────────────────────
    async fn maybe_generate_resume(
        &self,
        livrables: Livrables,
        offre: &Offre,
        profil: &domain::Profil,
        retained: &[Chunk],
        plan: &CandidaturePlan,
        instance_id: InstanceId,
    ) -> Result<Option<Resume>, GenerateError> {
        if !livrables.resume {
            return Ok(None);
        }
        self.events.started(instance_id, GenerationStep::Resume);

        let req = ExtractionRequest {
            system: Some(
                "Tu produis des CV en français adaptés à une offre. \
                 La structure du CV est fixe ; seul le contenu est adapté. \
                 Tu n'inventes JAMAIS d'expérience, de stack ou de chiffre. \
                 Tu reformules ce qui existe dans le profil pour le rendre \
                 le plus pertinent possible vis-à-vis de l'offre."
                    .into(),
            ),
            instruction:
                "Génère un CV adapté à cette offre, en respectant le schéma fourni. \
                 Mets en avant les expériences/projets/compétences les plus pertinents."
                    .into(),
            input: build_generation_input(offre, profil, retained, plan),
            schema_name: "Resume".into(),
            schema_description: "CV structuré, contenu adapté à l'offre".into(),
            json_schema: serde_json::to_value(&schemars::schema_for!(Resume)).unwrap(),
            model: None,
            max_tokens: Some(3000),
        };

        let response_json = self.llm.extract(req).await.map_err(|e| {
            self.events
                .failed(instance_id, GenerationStep::Resume, e.to_string());
            AppError::Other(e.to_string())
        })?;

        let resume: Resume = serde_json::from_value(response_json).map_err(|e| {
            self.events
                .failed(instance_id, GenerationStep::Resume, e.to_string());
            AppError::Other(e.to_string())
        })?;

        self.events.done(instance_id, GenerationStep::Resume, None);
        Ok(Some(resume))
    }

    // ─────────────────────────────────────────────────────────────
    // Étape 4c — COVER LETTER (parallèle)
    // ─────────────────────────────────────────────────────────────
    async fn maybe_generate_cover_letter(
        &self,
        livrables: Livrables,
        offre: &Offre,
        profil: &domain::Profil,
        retained: &[Chunk],
        plan: &CandidaturePlan,
        instance_id: InstanceId,
    ) -> Result<Option<CoverLetter>, GenerateError> {
        if !livrables.cover_letter {
            return Ok(None);
        }
        self.events.started(instance_id, GenerationStep::CoverLetter);

        let req = ExtractionRequest {
            system: Some(
                "Tu rédiges des lettres de motivation en français. \
                 Tu suis la structure : salutation, accroche, projets, vous, \
                 pourquoi, clôture. Tu n'inventes rien. Tu es concret, sobre, \
                 sans formules grandiloquentes ni emphase artificielle."
                    .into(),
            ),
            instruction:
                "Rédige une lettre de motivation pour cette offre, en respectant \
                 le schéma fourni. Chaque paragraphe est typé."
                    .into(),
            input: build_generation_input(offre, profil, retained, plan),
            schema_name: "CoverLetter".into(),
            schema_description: "Lettre structurée par paragraphes typés".into(),
            json_schema: serde_json::to_value(&schemars::schema_for!(CoverLetter)).unwrap(),
            model: None,
            max_tokens: Some(2500),
        };

        let response_json = self.llm.extract(req).await.map_err(|e| {
            self.events
                .failed(instance_id, GenerationStep::CoverLetter, e.to_string());
            AppError::Other(e.to_string())
        })?;

        let cover_letter: CoverLetter = serde_json::from_value(response_json).map_err(|e| {
            self.events
                .failed(instance_id, GenerationStep::CoverLetter, e.to_string());
            AppError::Other(e.to_string())
        })?;

        self.events
            .done(instance_id, GenerationStep::CoverLetter, None);
        Ok(Some(cover_letter))
    }
}

// ─────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────

fn build_query_text(offre: &Offre) -> String {
    format!(
        "{} chez {}. Stack: {}. Missions: {}. Exigences: {}.",
        offre.intitule,
        offre.entreprise,
        offre.structured.stack.join(", "),
        offre.structured.missions.join(" ; "),
        offre.structured.exigences.join(" ; "),
    )
}

fn build_generation_input(
    offre: &Offre,
    profil: &domain::Profil,
    retained: &[Chunk],
    plan: &CandidaturePlan,
) -> String {
    let chunks_listing = retained
        .iter()
        .map(|c| format!("### {} — {}\n{}", c.kind.as_str(), c.titre, c.content))
        .collect::<Vec<_>>()
        .join("\n\n");

    format!(
        "## OFFRE\nEntreprise: {}\nIntitulé: {}\nLocalisation: {}\n\n## RÉSUMÉ DE L'OFFRE\n{}\n\n## STACK\n{}\n\n## MISSIONS\n{}\n\n## EXIGENCES\n{}\n\n## PLAN STRATÉGIQUE\nAngle: {}\nForces à souligner: {}\nMots-clés critiques: {}\n\n## PROFIL CANDIDAT\n{}\n\n## CHUNKS PERTINENTS DU PROFIL\n{}",
        offre.entreprise,
        offre.intitule,
        offre.localisation.as_deref().unwrap_or("non précisé"),
        offre.structured.resume_court,
        offre.structured.stack.join(", "),
        offre.structured.missions.join(" ; "),
        offre.structured.exigences.join(" ; "),
        plan.angle,
        plan.forces_a_souligner.join(" ; "),
        plan.mots_cles_critiques.join(", "),
        serde_json::to_string_pretty(&profil.content).unwrap_or_default(),
        chunks_listing,
    )
}

fn build_slug(offre: &Offre, instance_id: InstanceId) -> Slug {
    // Format : <offre_slug>__<short_instance_id>
    // Garantit l'unicité même si on génère plusieurs instances pour la même offre.
    let short = instance_id.to_string().chars().take(8).collect::<String>();
    let combined = format!("{}__{}", offre.slug.as_str(), short);
    Slug::parse(combined).unwrap_or_else(|_| {
        // Fallback en cas de slug invalide (ne devrait jamais arriver)
        Slug::parse(format!("instance_{}", short)).expect("short id is always valid")
    })
}

fn truncate(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max_chars).collect();
        format!("{truncated}…")
    }
}

// ─────────────────────────────────────────────────────────────────
// Étape 5 — VALIDATE
// ─────────────────────────────────────────────────────────────────

fn validate_outputs(
    offre: &Offre,
    restitution: Option<&Restitution>,
    resume: Option<&Resume>,
    cover_letter: Option<&CoverLetter>,
) -> Result<(), GenerateError> {
    // Restitution : score doit être ≤ 100.
    if let Some(r) = restitution {
        if r.fit.score > 100 {
            return Err(GenerateError::Invalide(format!(
                "score de fit > 100 : {}",
                r.fit.score
            )));
        }
    }

    // Resume : doit avoir au moins 1 expérience et 1 formation.
    if let Some(r) = resume {
        if r.experiences.is_empty() {
            return Err(GenerateError::Invalide("CV sans expérience".into()));
        }
        if r.formations.is_empty() {
            return Err(GenerateError::Invalide("CV sans formation".into()));
        }
    }

    // Cover Letter : doit être complète (salutation + accroche + clôture)
    // et mentionner l'entreprise au moins une fois (anti-hallucination basique).
    if let Some(cl) = cover_letter {
        if !cl.est_complete() {
            return Err(GenerateError::Invalide(
                "lettre incomplète (manque salutation/accroche/clôture)".into(),
            ));
        }

        let texte_complet: String = cl
            .paragraphes
            .iter()
            .map(|p| p.contenu.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        let entreprise_lower = offre.entreprise.to_lowercase();
        if !texte_complet.to_lowercase().contains(&entreprise_lower) {
            return Err(GenerateError::Invalide(format!(
                "lettre ne mentionne jamais l'entreprise '{}'",
                offre.entreprise
            )));
        }
    }

    Ok(())
}

// ─────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_court_inchange() {
        assert_eq!(truncate("hello", 10), "hello");
    }

    #[test]
    fn truncate_long_coupe() {
        let s = "a".repeat(100);
        let out = truncate(&s, 10);
        assert_eq!(out.chars().count(), 11); // 10 + ellipsis
        assert!(out.ends_with('…'));
    }

    #[test]
    fn livrables_par_defaut_tous_actifs() {
        let l = Livrables::default();
        assert!(l.restitution && l.resume && l.cover_letter);
        assert!(!l.aucun());
    }

    #[test]
    fn livrables_aucun_si_tout_off() {
        let l = Livrables {
            restitution: false,
            resume: false,
            cover_letter: false,
        };
        assert!(l.aucun());
    }
}
