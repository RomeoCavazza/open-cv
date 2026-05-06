//! `GenerateApplicationUseCase` — Orchestrateur du pipeline.

use chrono::Utc;
use domain::{Instance, InstanceId, JsonValue, Offre, OffreId, ProfilId};
use ports::{ChunkRepo, Embedder, InstanceRepo, LlmClient, OffreRepo, ProfilRepo};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;

use crate::events::{EventBus, GenerationStep};
use crate::AppError;

pub mod helpers;
mod steps;

#[cfg(test)]
mod tests;

use self::helpers::{build_slug, validate_outputs};

// ─────────────────────────────────────────────────────────────────
// Inputs / outputs
// ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct GenerateInput {
    pub offre_id: OffreId,
    pub profil_id: ProfilId,
    pub existing_instance: Option<Instance>,
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

// ─────────────────────────────────────────────────────────────────
// Erreurs spécifiques
// ─────────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
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
pub struct RerankResponse {
    pub indices_retenus: Vec<usize>,
    pub raisonnement: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CandidaturePlan {
    pub angle: String,
    pub forces_a_souligner: Vec<String>,
    pub mots_cles_critiques: Vec<String>,
    pub faiblesses_a_adresser: Vec<String>,
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
    #[allow(clippy::too_many_arguments)]
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

    pub async fn execute(
        &self,
        input: GenerateInput,
        llm_override: Option<Arc<dyn LlmClient>>,
    ) -> Result<domain::Instance, GenerateError> {
        let llm = llm_override.unwrap_or_else(|| self.llm.clone());
        info!(offre_id = %input.offre_id, "génération de l'application...");
        if input.livrables.aucun() {
            return Err(GenerateError::AucunLivrable);
        }

        let offre = self
            .offres
            .get_by_id(input.offre_id)
            .await
            .map_err(AppError::Repo)?
            .ok_or(GenerateError::OffreIntrouvable(input.offre_id))?;

        let profil = self
            .profils
            .get_by_id(input.profil_id)
            .await
            .map_err(AppError::Repo)?
            .ok_or(GenerateError::ProfilIntrouvable(input.profil_id))?;

        let existing_instance = input.existing_instance.clone();
        let now = Utc::now();
        let (instance_id, slug, created_at, existing_notes) = match existing_instance {
            Some(ref instance) => (
                instance.id,
                instance.slug.clone(),
                instance.created_at,
                Some(instance.notes.clone()),
            ),
            None => {
                let instance_id = InstanceId::new();
                (instance_id, build_slug(&offre, instance_id), now, None)
            }
        };

        tracing::Span::current().record("instance_id", tracing::field::display(&instance_id));

        info!(
            "démarrage génération pour offre={} profil={}",
            offre.entreprise, profil.label
        );

        let old = existing_instance.as_ref();
        let rest = if input.livrables.restitution {
            None
        } else {
            old.and_then(|i| i.restitution.clone())
        };
        let resu = if input.livrables.resume {
            None
        } else {
            old.and_then(|i| i.resume_json.clone())
        };
        let cove = if input.livrables.cover_letter {
            None
        } else {
            old.and_then(|i| i.cover_letter_json.clone())
        };

        self.instances
            .upsert(&Instance {
                id: instance_id,
                slug: slug.clone(),
                offre_id: offre.id,
                profil_id: profil.id,
                status: domain::InstanceStatus::Generating,
                restitution: rest,
                resume_json: resu,
                cover_letter_json: cove,
                notes: existing_notes.unwrap_or_else(|| JsonValue::Object(Default::default())),
                created_at,
                updated_at: Utc::now(),
                sent_at: old.and_then(|i| i.sent_at),
            })
            .await
            .map_err(AppError::Repo)?;

        let pipeline_result: Result<Instance, GenerateError> = async {
            // Étape 1 : RETRIEVE
            self.events.started(instance_id, GenerationStep::Retrieve);
            let candidates = self.retrieve_chunks(&offre, profil.id).await?;
            self.events.done(
                instance_id,
                GenerationStep::Retrieve,
                Some(format!("{} chunks candidats", candidates.len())),
            );

            if candidates.is_empty() {
                info!(
                    "RAG : aucun chunk trouvé, la génération continuera sans contexte granulaire"
                );
            }

            // Étape 2 : RERANK
            self.events.started(instance_id, GenerationStep::Rerank);
            let retained = self.rerank(&offre, &candidates, llm.clone()).await?;
            self.events.done(
                instance_id,
                GenerationStep::Rerank,
                Some(format!("{} chunks retenus", retained.len())),
            );

            // Étape 3 : PLAN
            self.events.started(instance_id, GenerationStep::Plan);
            let plan = self.plan(&offre, &retained, llm.clone()).await?;
            self.events
                .done(instance_id, GenerationStep::Plan, Some(plan.angle.clone()));

            // Étape 4 : Génération séquentielle
            let restitution = self
                .maybe_generate_restitution(input.livrables, &offre, instance_id, llm.clone())
                .await?;
            let resume = self
                .maybe_generate_resume(
                    input.livrables,
                    &offre,
                    &profil,
                    &retained,
                    &plan,
                    instance_id,
                    llm.clone(),
                )
                .await?;
            let cover_letter = self
                .maybe_generate_cover_letter(
                    input.livrables,
                    &offre,
                    &profil,
                    &retained,
                    &plan,
                    instance_id,
                    llm.clone(),
                )
                .await?;

            // Étape 5 : VALIDATE
            self.events.started(instance_id, GenerationStep::Validate);
            validate_outputs(
                &offre,
                restitution.as_ref(),
                resume.as_ref(),
                cover_letter.as_ref(),
            )?;
            self.events
                .done(instance_id, GenerationStep::Validate, None);

            let mut instance = self
                .instances
                .get_by_id(instance_id)
                .await
                .map_err(AppError::Repo)?
                .ok_or_else(|| AppError::Other("Instance introuvable après génération".into()))?;

            merge_generated_outputs(&mut instance, restitution, resume, cover_letter);
            instance.status = domain::InstanceStatus::Ready;
            instance.updated_at = Utc::now();

            self.events.started(instance_id, GenerationStep::Persist);
            self.instances
                .upsert(&instance)
                .await
                .map_err(AppError::Repo)?;
            self.events.done(instance_id, GenerationStep::Persist, None);
            self.events.done(instance_id, GenerationStep::Done, None);

            Ok(instance)
        }
        .await;

        match pipeline_result {
            Ok(instance) => Ok(instance),
            Err(error) => {
                if let Ok(Some(mut instance)) = self.instances.get_by_id(instance_id).await {
                    instance.status = domain::InstanceStatus::Failed;
                    instance.updated_at = Utc::now();
                    let _ = self.instances.upsert(&instance).await;
                }
                Err(error)
            }
        }
    }

    async fn retrieve_chunks(
        &self,
        offre: &Offre,
        profil_id: ProfilId,
    ) -> Result<Vec<(domain::Chunk, f32)>, GenerateError> {
        steps::retrieve_chunks(self, offre, profil_id).await
    }

    async fn rerank(
        &self,
        offre: &Offre,
        candidates: &[(domain::Chunk, f32)],
        llm: Arc<dyn LlmClient>,
    ) -> Result<Vec<domain::Chunk>, GenerateError> {
        steps::rerank(self, offre, candidates, llm).await
    }

    async fn plan(
        &self,
        offre: &Offre,
        retained: &[domain::Chunk],
        llm: Arc<dyn LlmClient>,
    ) -> Result<CandidaturePlan, GenerateError> {
        steps::plan(self, offre, retained, llm).await
    }

    async fn maybe_generate_restitution(
        &self,
        livrables: Livrables,
        offre: &Offre,
        instance_id: domain::InstanceId,
        llm: Arc<dyn LlmClient>,
    ) -> Result<Option<domain::Restitution>, GenerateError> {
        steps::maybe_generate_restitution(self, livrables, offre, instance_id, llm).await
    }

    #[allow(clippy::too_many_arguments)]
    async fn maybe_generate_resume(
        &self,
        livrables: Livrables,
        offre: &Offre,
        profil: &domain::Profil,
        retained: &[domain::Chunk],
        plan: &CandidaturePlan,
        instance_id: domain::InstanceId,
        llm: Arc<dyn LlmClient>,
    ) -> Result<Option<domain::Resume>, GenerateError> {
        steps::maybe_generate_resume(
            self,
            livrables,
            offre,
            profil,
            retained,
            plan,
            instance_id,
            llm,
        )
        .await
    }

    #[allow(clippy::too_many_arguments)]
    async fn maybe_generate_cover_letter(
        &self,
        livrables: Livrables,
        offre: &Offre,
        profil: &domain::Profil,
        retained: &[domain::Chunk],
        plan: &CandidaturePlan,
        instance_id: domain::InstanceId,
        llm: Arc<dyn LlmClient>,
    ) -> Result<Option<domain::CoverLetter>, GenerateError> {
        steps::maybe_generate_cover_letter(
            self,
            livrables,
            offre,
            profil,
            retained,
            plan,
            instance_id,
            llm,
        )
        .await
    }
}

fn merge_generated_outputs(
    instance: &mut Instance,
    restitution: Option<domain::Restitution>,
    resume: Option<domain::Resume>,
    cover_letter: Option<domain::CoverLetter>,
) {
    if let Some(restitution) = restitution {
        instance.restitution = Some(restitution);
    }
    if let Some(resume) = resume {
        instance.resume_json = Some(resume);
    }
    if let Some(cover_letter) = cover_letter {
        instance.cover_letter_json = Some(cover_letter);
    }
}
