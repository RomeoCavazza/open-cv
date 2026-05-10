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

const MAX_EXTRACTED_OFFRES_PER_INPUT: usize = 5;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntakeExecutionResult {
    pub outputs: Vec<IntakeOutput>,
    pub ignored_count: usize,
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
    ) -> Result<IntakeExecutionResult, AppError> {
        let (raw_text, source_url) = self.resolve_content(&input.raw_input).await?;
        let host = self.deduplicator.resolve_host(&source_url);

        if let Some(existing) = self.find_existing_offre_by_url(&source_url).await? {
            let out = self
                .handle_existing_offre(existing, input.profil_id)
                .await?;
            return Ok(IntakeExecutionResult {
                outputs: vec![out],
                ignored_count: 0,
            });
        }

        info!("début extraction structurée via LLM");
        let extractions = self.extractor.extract(&raw_text, llm_override).await;
        let (extractions, dropped_count) = cap_items(extractions, MAX_EXTRACTED_OFFRES_PER_INPUT);
        if dropped_count > 0 {
            info!(
                kept = extractions.len(),
                dropped = dropped_count,
                max = MAX_EXTRACTED_OFFRES_PER_INPUT,
                "extraction limitée au maximum autorisé"
            );
        }
        info!(count = extractions.len(), "extraction terminée");

        let mut outputs = Vec::new();

        for (intitule, entreprise, localisation, contrat, structured) in extractions {
            let hash = self.deduplicator.compute_hash(&offer_fingerprint(
                &raw_text,
                &intitule,
                &entreprise,
            ));
            if let Some(existing) = self.find_existing_offre_by_hash(&host, &hash).await? {
                let out = self
                    .handle_existing_offre(existing, input.profil_id)
                    .await?;
                outputs.push(out);
                continue;
            }

            let offre_id = OffreId::new();
            let slug = extractor::build_offre_slug(&entreprise, &intitule);
            if let Some(existing) = self.find_existing_offre_by_slug(&slug).await? {
                let out = self
                    .handle_existing_offre(existing, input.profil_id)
                    .await?;
                outputs.push(out);
                continue;
            }
            let offre = self.build_offre(
                offre_id,
                slug.clone(),
                source_url.clone(), // On réutilise la même URL source pour le lot
                host.clone(),
                hash.clone(),
                entreprise,
                intitule,
                localisation,
                contrat,
                raw_text.clone(),
                structured,
            );

            self.persist_offre(&offre).await?;
            info!("création instance draft pour {}", slug);
            let instance = self
                .create_draft_instance(offre_id, slug.clone(), input.profil_id)
                .await?;

            outputs.push(IntakeOutput {
                offre_slug: slug.to_string(),
                instance_id: instance.id,
                instance_slug: instance.slug.to_string(),
                was_duplicate: false,
            });
        }

        if outputs.is_empty() {
            return Err(AppError::Validation(
                "Aucune offre n'a pu être extraite".into(),
            ));
        }

        Ok(IntakeExecutionResult {
            outputs,
            ignored_count: dropped_count,
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

    async fn find_existing_offre_by_url(
        &self,
        source_url: &str,
    ) -> Result<Option<Offre>, AppError> {
        self.deduplicator
            .find_by_url(source_url)
            .await
            .map_err(AppError::Repo)
    }

    async fn find_existing_offre_by_hash(
        &self,
        host: &str,
        hash: &[u8],
    ) -> Result<Option<Offre>, AppError> {
        self.deduplicator
            .find_existing(host, hash)
            .await
            .map_err(AppError::Repo)
    }

    async fn find_existing_offre_by_slug(&self, slug: &Slug) -> Result<Option<Offre>, AppError> {
        self.offres.get_by_slug(slug).await.map_err(AppError::Repo)
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

fn offer_fingerprint(raw_text: &str, intitule: &str, entreprise: &str) -> String {
    // Hash par offre (et non seulement par texte global) pour éviter les collisions
    // lorsqu'un même prompt produit plusieurs offres distinctes.
    format!(
        "{}\n::title::{}\n::company::{}",
        raw_text.trim(),
        intitule.trim().to_lowercase(),
        entreprise.trim().to_lowercase()
    )
}

fn cap_items<T>(mut items: Vec<T>, max: usize) -> (Vec<T>, usize) {
    if items.len() <= max {
        return (items, 0);
    }
    let dropped = items.len() - max;
    items.truncate(max);
    (items, dropped)
}

#[cfg(test)]
mod fingerprint_tests {
    use super::{cap_items, offer_fingerprint};

    #[test]
    fn fingerprint_differs_for_distinct_titles() {
        let raw = "Crée 2 candidatures en alternance.";
        let fp_1 = offer_fingerprint(raw, "Data Scientist", "Non spécifié");
        let fp_2 = offer_fingerprint(raw, "Data Engineer", "Non spécifié");
        assert_ne!(fp_1, fp_2);
    }

    #[test]
    fn cap_items_enforces_limit_and_reports_dropped() {
        let (kept, dropped) = cap_items(vec![1, 2, 3, 4, 5, 6], 5);
        assert_eq!(kept, vec![1, 2, 3, 4, 5]);
        assert_eq!(dropped, 1);
    }
}
