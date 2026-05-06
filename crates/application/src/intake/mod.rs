//! Intake — Orchestration de l'ingestion d'une offre.

pub mod deduplicator;
pub mod extractor;
pub mod resolver;
#[cfg(test)]
mod tests;

use chrono::Utc;
use domain::{Instance, InstanceId, InstanceStatus, JsonValue, Offre, OffreId, Slug};
use ports::{InstanceRepo, LlmClient, OffreRepo, ProfilRepo};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{error, info};

use self::deduplicator::Deduplicator;
use self::extractor::StructuredExtractor;
use self::resolver::ContentResolver;
use crate::AppError;

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
    #[allow(clippy::too_many_arguments)]
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
        let (raw_text, source_url) = self.resolve_content(&input.raw_input).await?;
        let host = self.deduplicator.resolve_host(&source_url);
        let hash = self.deduplicator.compute_hash(&raw_text);

        if let Some(existing) = self.find_existing_offre(&source_url, &host, &hash).await? {
            return self.handle_existing_offre(existing, input.profil_id).await;
        }

        info!("début extraction structurée via LLM");
        let (intitule, entreprise, localisation, contrat, structured) =
            self.extractor.extract(&raw_text, llm_override).await;
        info!(entreprise = %entreprise, intitule = %intitule, "extraction terminée");

        let offre_id = OffreId::new();
        let slug = extractor::build_offre_slug(&entreprise, &intitule);
        let offre = self.build_offre(
            offre_id,
            slug.clone(),
            source_url,
            host,
            hash,
            entreprise,
            intitule,
            localisation,
            contrat,
            raw_text,
            structured,
        );

        self.persist_offre(&offre).await?;
        info!("création instance draft");
        let instance = self
            .create_draft_instance(offre_id, slug.clone(), input.profil_id)
            .await
            .map_err(|e| {
                error!(error = %e, "échec création instance");
                e
            })?;

        Ok(IntakeOutput {
            offre_slug: slug.to_string(),
            instance_id: instance.id,
            instance_slug: instance.slug.to_string(),
            was_duplicate: false,
        })
    }

    async fn resolve_content(&self, raw_input: &str) -> Result<(String, String), AppError> {
        info!(input_len = raw_input.len(), "début résolution contenu");
        let (raw_text, source_url) = self.resolver.resolve(raw_input).await.map_err(|e| {
            error!(error = %e, "échec résolution contenu");
            e
        })?;
        info!(source_url = %source_url, text_len = raw_text.len(), "résolution réussie");
        Ok((raw_text, source_url))
    }

    async fn find_existing_offre(
        &self,
        source_url: &str,
        host: &str,
        hash: &[u8],
    ) -> Result<Option<Offre>, AppError> {
        if let Some(existing) = self
            .deduplicator
            .find_by_url(source_url)
            .await
            .map_err(AppError::Repo)?
        {
            return Ok(Some(existing));
        }

        self.deduplicator
            .find_existing(host, hash)
            .await
            .map_err(AppError::Repo)
    }

    async fn handle_existing_offre(
        &self,
        existing: Offre,
        profil_id: domain::ProfilId,
    ) -> Result<IntakeOutput, AppError> {
        info!(slug = %existing.slug, "offre existante trouvée (déduplication)");
        if let Some(instance) = self
            .instances
            .get_by_offre_and_profil(existing.id, profil_id)
            .await
            .map_err(AppError::Repo)?
        {
            info!(instance_slug = %instance.slug, "instance existante trouvée pour cette offre et ce profil");
            return Ok(IntakeOutput {
                offre_slug: existing.slug.to_string(),
                instance_id: instance.id,
                instance_slug: instance.slug.to_string(),
                was_duplicate: true,
            });
        }

        info!("pas d'instance pour cette offre/profil, création d'une nouvelle instance draft");
        let instance = self
            .create_draft_instance(existing.id, existing.slug.clone(), profil_id)
            .await?;
        Ok(IntakeOutput {
            offre_slug: existing.slug.to_string(),
            instance_id: instance.id,
            instance_slug: instance.slug.to_string(),
            was_duplicate: false,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn build_offre(
        &self,
        offre_id: OffreId,
        slug: Slug,
        source_url: String,
        source_host: String,
        source_hash: Vec<u8>,
        entreprise: String,
        intitule: String,
        localisation: Option<String>,
        contrat: Option<String>,
        raw_text: String,
        structured: domain::OffreStructured,
    ) -> Offre {
        Offre {
            id: offre_id,
            slug,
            source_url,
            source_host,
            source_hash,
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
        }
    }

    async fn persist_offre(&self, offre: &Offre) -> Result<(), AppError> {
        self.offres.upsert(offre).await.map_err(|e| {
            error!(error = %e, "échec upsert offre");
            AppError::Repo(e)
        })?;
        info!(slug = %offre.slug, "nouvelle offre ingérée");
        Ok(())
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
            notes: JsonValue::Null,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            sent_at: None,
        };

        self.instances
            .upsert(&instance)
            .await
            .map_err(AppError::Repo)?;
        Ok(instance)
    }
}
