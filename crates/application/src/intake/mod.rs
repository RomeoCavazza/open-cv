//! Intake — Orchestration de l'ingestion d'une offre.

pub mod resolver;
pub mod deduplicator;
pub mod extractor;
#[cfg(test)]
mod tests;

use std::sync::Arc;
use chrono::Utc;
use domain::{Instance, InstanceId, InstanceStatus, Offre, OffreId, Slug};
use ports::{InstanceRepo, LlmClient, OffreRepo, ProfilRepo};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::AppError;
use self::resolver::ContentResolver;
use self::deduplicator::Deduplicator;
use self::extractor::StructuredExtractor;

#[derive(Debug, Clone)]
pub struct IntakeInput {
    pub raw_input: String,
    pub profil_id: domain::ProfilId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntakeOutput {
    pub offre_slug: String,
    pub instance_id: domain::InstanceId,
    pub instance_slug: String,
    pub was_duplicate: bool,
}

pub struct IntakeOffreUseCase {
    pub offres: Arc<dyn OffreRepo>,
    pub instances: Arc<dyn InstanceRepo>,
    pub profils: Arc<dyn ProfilRepo>,
    pub resolver: ContentResolver,
    pub deduplicator: Deduplicator,
    pub extractor: StructuredExtractor,
}

impl IntakeOffreUseCase {
    pub fn new(
        offres: Arc<dyn OffreRepo>,
        instances: Arc<dyn InstanceRepo>,
        profils: Arc<dyn ProfilRepo>,
        llm: Arc<dyn LlmClient>,
        scraper: Arc<dyn ports::Scraper>,
    ) -> Self {
        Self {
            offres: offres.clone(),
            instances,
            profils,
            resolver: ContentResolver::new(scraper),
            deduplicator: Deduplicator::new(offres),
            extractor: StructuredExtractor::new(llm),
        }
    }

    pub async fn execute(
        &self,
        input: IntakeInput,
        llm_override: Option<Arc<dyn LlmClient>>,
    ) -> Result<IntakeOutput, AppError> {
        // 1. Résolution & Nettoyage
        let (raw_text, source_url) = self.resolver.resolve(&input.raw_input).await?;

        // 2. Déduplication
        let host = self.deduplicator.resolve_host(&source_url);
        let hash = self.deduplicator.compute_hash(&raw_text);

        if let Some(existing_offre) = self.deduplicator.find_existing(&host, &hash).await.map_err(AppError::Repo)? {
            if let Some(instance) = self.instances.get_by_offre_id(existing_offre.id).await.map_err(AppError::Repo)? {
                return Ok(IntakeOutput {
                    offre_slug: existing_offre.slug.to_string(),
                    instance_id: instance.id,
                    instance_slug: instance.slug.to_string(),
                    was_duplicate: true,
                });
            }

            let instance = self.create_draft_instance(existing_offre.id, existing_offre.slug.clone(), input.profil_id).await?;
            return Ok(IntakeOutput {
                offre_slug: existing_offre.slug.to_string(),
                instance_id: instance.id,
                instance_slug: instance.slug.to_string(),
                was_duplicate: true,
            });
        }

        // 3. Extraction
        let (intitule, entreprise, localisation, contrat, structured) = self.extractor.extract(&raw_text, llm_override).await;

        // 4. Persistance Offre
        let offre_id = OffreId::new();
        let slug = extractor::build_offre_slug(&entreprise, &intitule);

        let offre = Offre {
            id: offre_id,
            slug: slug.clone(),
            source_url,
            source_host: host,
            source_hash: hash,
            entreprise,
            intitule,
            localisation,
            contrat,
            raw_text,
            structured,
            scraped_at: Utc::now(),
            last_seen_at: Utc::now(),
            closed_at: None,
            categorie: None,
        };

        self.offres.upsert(&offre).await.map_err(AppError::Repo)?;
        info!(slug = %offre.slug, "nouvelle offre ingérée");

        // 5. Instance Draft
        let instance = self.create_draft_instance(offre_id, slug.clone(), input.profil_id).await?;

        Ok(IntakeOutput {
            offre_slug: slug.to_string(),
            instance_id: instance.id,
            instance_slug: instance.slug.to_string(),
            was_duplicate: false,
        })
    }

    async fn create_draft_instance(
        &self,
        offre_id: OffreId,
        slug: Slug,
        profil_id: domain::ProfilId,
    ) -> Result<Instance, AppError> {
        let instance = Instance {
            id: InstanceId::new(),
            slug,
            offre_id,
            profil_id,
            status: InstanceStatus::Draft,
            restitution: None,
            resume_json: None,
            cover_letter_json: None,
            notes: serde_json::Value::Null,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            sent_at: None,
        };

        self.instances.upsert(&instance).await.map_err(AppError::Repo)?;
        Ok(instance)
    }
}
