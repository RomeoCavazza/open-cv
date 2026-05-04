//! `IntakeOffreUseCase` — ingestion d'une offre avec extraction LLM.
//!
//! Sous-étapes :
//!   1. Normalisation de l'input (URL → scrape, texte → raw_text direct)
//!   2. Dédup par content hash
//!   3. Extraction LLM → `OffreStructured` (intitulé, entreprise, stack, missions, etc.)
//!   4. Persist de l'offre en base
//!   5. Création d'une instance brouillon associée

use std::sync::Arc;
use once_cell::sync::Lazy;

use chrono::Utc;
use domain::{Instance, InstanceId, InstanceStatus, Offre, OffreId, OffreStructured, Slug};
use ports::{ExtractionRequest, InstanceRepo, LlmClient, OffreRepo, ProfilRepo, Scraper};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use tracing::{info, warn};

use crate::AppError;

// ─────────────────────────────────────────────────────────────────
// Inputs / outputs
// ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct IntakeInput {
    /// Texte brut OU URL unique.
    pub raw_input: String,
    /// Profil à associer à l'instance draft.
    pub profil_id: domain::ProfilId,
}

#[derive(Debug, Clone, Serialize)]
pub struct IntakeOutput {
    pub offre_slug: String,
    pub instance_id: domain::InstanceId,
    pub instance_slug: String,
    /// `true` si l'offre existait déjà (dédup).
    pub was_duplicate: bool,
}

// ─────────────────────────────────────────────────────────────────
// Schéma LLM pour l'extraction à l'intake
// ─────────────────────────────────────────────────────────────────

/// Métadonnées extraites par le LLM depuis le raw_text d'une offre.
/// Combine les champs "d'en-tête" (intitulé, entreprise, etc.) et le
/// structured (stack, missions, exigences).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct OffreExtraction {
    /// Intitulé du poste tel qu'il apparaît dans l'offre.
    intitule: String,
    /// Nom de l'entreprise qui recrute.
    entreprise: String,
    /// Localisation du poste (ville, région).
    localisation: Option<String>,
    /// Type de contrat (alternance, stage, CDI, etc.).
    contrat: Option<String>,
    /// Résumé en 1-2 phrases de l'offre.
    resume_court: String,
    /// Stack technique mentionnée (langages, frameworks, outils).
    stack: Vec<String>,
    /// Missions principales décrites dans l'offre.
    missions: Vec<String>,
    /// Compétences exigées (hard skills).
    exigences: Vec<String>,
    /// Soft skills mentionnés.
    soft_skills: Vec<String>,
    /// Niveau d'études requis (Bac+5, Bac+3, etc.).
    niveau_etudes: Option<String>,
    /// Type de contrat extrait (alternance, stage, CDI...).
    type_contrat: Option<String>,
    /// Mots-clés à intégrer dans le CV/lettre.
    mots_cles: Vec<String>,
}

static EXTRACTION_SCHEMA: Lazy<serde_json::Value> =
    Lazy::new(|| serde_json::to_value(schemars::schema_for!(OffreExtraction)).unwrap());

impl OffreExtraction {
    fn into_parts(
        self,
    ) -> (
        String,
        String,
        Option<String>,
        Option<String>,
        OffreStructured,
    ) {
        (
            self.intitule,
            self.entreprise,
            self.localisation,
            self.contrat,
            OffreStructured {
                resume_court: self.resume_court,
                stack: self.stack,
                missions: self.missions,
                exigences: self.exigences,
                soft_skills: self.soft_skills,
                niveau_etudes: self.niveau_etudes,
                type_contrat: self.type_contrat,
                mots_cles: self.mots_cles,
            },
        )
    }
}

// ─────────────────────────────────────────────────────────────────
// Le use case
// ─────────────────────────────────────────────────────────────────

pub struct IntakeOffreUseCase {
    pub offres: Arc<dyn OffreRepo>,
    pub instances: Arc<dyn InstanceRepo>,
    pub profils: Arc<dyn ProfilRepo>,
    pub llm: Arc<dyn LlmClient>,
    pub scraper: Arc<dyn Scraper>,
}

impl IntakeOffreUseCase {
    pub fn new(
        offres: Arc<dyn OffreRepo>,
        instances: Arc<dyn InstanceRepo>,
        profils: Arc<dyn ProfilRepo>,
        llm: Arc<dyn LlmClient>,
        scraper: Arc<dyn Scraper>,
    ) -> Self {
        Self {
            offres,
            instances,
            profils,
            llm,
            scraper,
        }
    }

    /// Ingère une offre (URL ou texte brut). Retourne le slug de l'offre.
    pub async fn execute(
        &self,
        input: IntakeInput,
        llm_override: Option<Arc<dyn LlmClient>>,
    ) -> Result<IntakeOutput, AppError> {
        let llm = llm_override.unwrap_or_else(|| self.llm.clone());
        let raw = input.raw_input.trim().to_string();

        if raw.is_empty() {
            return Err(AppError::Other("input vide".into()));
        }

        // ── Étape 1 : Résolution du contenu ──
        let (raw_text_uncleaned, source_url) = if looks_like_url(&raw) {
            info!(url = %raw, "scraping de l'URL");
            let result = self
                .scraper
                .scrape(&raw)
                .await
                .map_err(|e| AppError::Other(format!("erreur de scraping : {e}")))?;
            (result.raw_text, raw.clone())
        } else {
            // Texte brut collé : tout le bloc = une offre
            if raw.len() < 200 {
                return Err(AppError::Other(
                    "texte trop court pour être une offre (<200 chars)".into(),
                ));
            }
            (raw, "manual".to_string())
        };

        // ── Étape 1b : Nettoyage du bruit (Fix 2) ──
        let raw_text = clean_raw_text(&raw_text_uncleaned);

        // ── Étape 1c : Détection de qualité (Fix 3) ──
        let business_keywords = [
            "mission",
            "profil",
            "compétence",
            "expérience",
            "formation",
            "responsabilité",
            "stack",
            "technologie",
        ];
        let has_business_signal = business_keywords
            .iter()
            .any(|kw| raw_text.to_lowercase().contains(kw));

        if raw_text.len() < 500 && !has_business_signal {
            warn!(
                "contenu rejeté car trop pauvre ou manque de signal métier ({} chars)",
                raw_text.len()
            );
            return Err(AppError::Other(
                "Le contenu fourni semble être principalement de la navigation ou du bruit. \
                 Veuillez fournir le texte complet de l'offre directement (missions, profil recherché, etc.).".into()
            ));
        }

        // ── Étape 2 : Dédup par hash ──
        let host = if source_url.starts_with("http") {
            url::Url::parse(&source_url)
                .ok()
                .and_then(|u| u.host_str().map(|h| h.to_string()))
                .unwrap_or_else(|| "external".to_string())
        } else {
            "manual".to_string()
        };

        let mut hasher = sha2::Sha256::new();
        hasher.update(raw_text.as_bytes());
        let source_hash = hasher.finalize().to_vec();

        let existing = self
            .offres
            .find_by_content_hash(&host, &source_hash)
            .await
            .map_err(AppError::Repo)?;

        if let Some(existing_offre) = existing {
            // Offre existe déjà — réutiliser l'instance liée à cette offre,
            // même si son slug ne correspond pas exactement au slug d'offre.
            if let Some(instance) = self
                .instances
                .get_by_offre_id(existing_offre.id)
                .await
                .map_err(AppError::Repo)?
            {
                info!(slug = %existing_offre.slug, instance_slug = %instance.slug, "offre déjà ingérée avec instance");
                return Ok(IntakeOutput {
                    offre_slug: existing_offre.slug.to_string(),
                    instance_id: instance.id,
                    instance_slug: instance.slug.to_string(),
                    was_duplicate: true,
                });
            }

            // Offre existe mais pas d'instance — créer l'instance
            let instance = self
                .create_draft_instance(
                    existing_offre.id,
                    existing_offre.slug.clone(),
                    input.profil_id,
                )
                .await?;

            return Ok(IntakeOutput {
                offre_slug: existing_offre.slug.to_string(),
                instance_id: instance.id,
                instance_slug: instance.slug.to_string(),
                was_duplicate: true,
            });
        }

        // ── Étape 3 : Extraction LLM pour OffreStructured ──
        let (intitule, entreprise, localisation, contrat, structured) =
            self.extract_structured(&raw_text, llm.clone()).await;

        // ── Étape 4 : Création de l'offre ──
        let offre_id = OffreId::new();
        let slug = build_offre_slug(&entreprise, &intitule);

        let offre = Offre {
            id: offre_id,
            slug: slug.clone(),
            source_url,
            source_host: host,
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
        };

        self.offres.upsert(&offre).await.map_err(AppError::Repo)?;

        info!(slug = %offre.slug, "offre ingérée");

        // ── Étape 5 : Instance draft ──
        let instance = self
            .create_draft_instance(offre_id, slug.clone(), input.profil_id)
            .await?;

        Ok(IntakeOutput {
            offre_slug: slug.to_string(),
            instance_id: instance.id,
            instance_slug: instance.slug.to_string(),
            was_duplicate: false,
        })
    }

    /// Extraction LLM de l'offre structurée. Fallback silencieux si le LLM échoue.
    async fn extract_structured(
        &self,
        raw_text: &str,
        llm: Arc<dyn LlmClient>,
    ) -> (
        String,
        String,
        Option<String>,
        Option<String>,
        OffreStructured,
    ) {
        let req = ExtractionRequest {
            system: Some(
                "Tu extrais les métadonnées structurées d'une offre d'emploi. \
                 Tu dois identifier l'intitulé du poste, l'entreprise, la localisation, \
                 le type de contrat, et une analyse détaillée : stack technique, missions, \
                 exigences, soft skills, mots-clés. Sois exhaustif et précis."
                    .into(),
            ),
            instruction: "Extrais toutes les informations structurées de cette offre d'emploi. \
                 Le résumé doit capturer l'essentiel en 2-3 phrases. \
                 La stack doit lister TOUS les outils/langages/frameworks mentionnés. \
                 Les missions doivent être des phrases complètes."
                .into(),
            input: vec![ports::MessageContent::Text(raw_text.to_string())],
            schema_name: "OffreExtraction".into(),
            schema_description: "Métadonnées structurées extraites d'une offre d'emploi".into(),
            json_schema: EXTRACTION_SCHEMA.clone(),
            model: None,
            max_tokens: Some(4000),
        };

        match llm.extract(req).await {
            Ok(json) => match serde_json::from_value::<OffreExtraction>(json) {
                Ok(extraction) => {
                    info!("extraction LLM réussie (provider: {})", llm.name());
                    extraction.into_parts()
                }
                Err(e) => {
                    warn!("déserialisation extraction échouée : {e}, fallback placeholder");
                    fallback_extraction(raw_text)
                }
            },
            Err(e) => {
                warn!(
                    "extraction LLM échouée (provider: {}) : {e}, fallback placeholder",
                    llm.name()
                );
                fallback_extraction(raw_text)
            }
        }
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

        self.instances
            .upsert(&instance)
            .await
            .map_err(AppError::Repo)?;

        Ok(instance)
    }
}

// ─────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────

fn clean_raw_text(input: &str) -> String {
    let nav_keywords = [
        "Our Teams",
        "Our Culture",
        "How We Hire",
        "Recruitment Process",
        "FAQ",
        "See All Jobs",
        "Cookie",
        "Imagine New Horizons",
        "Mentions légales",
        "Careers",
        "Politique de confidentialité",
    ];

    let lines: Vec<&str> = input
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                return true;
            }
            // Vire les lignes qui sont uniquement des liens markdown
            if is_just_markdown_link(trimmed) {
                return false;
            }
            // Vire les lignes contenant des keywords de nav
            if nav_keywords.iter().any(|kw| trimmed.contains(kw)) {
                return false;
            }
            true
        })
        .collect();

    lines.join("\n")
}

fn is_just_markdown_link(line: &str) -> bool {
    let trimmed = line.trim_start_matches(|c: char| c == '-' || c == '*' || c.is_whitespace());
    trimmed.starts_with('[') && trimmed.contains("](")
}

fn looks_like_url(s: &str) -> bool {
    let trimmed = s.trim();
    trimmed.starts_with("http://") || trimmed.starts_with("https://")
}

/// Fallback si le LLM échoue : on met des placeholders mais on sauvegarde quand même.
fn fallback_extraction(
    raw_text: &str,
) -> (
    String,
    String,
    Option<String>,
    Option<String>,
    OffreStructured,
) {
    let lines: Vec<&str> = raw_text
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .take(10)
        .collect();

    let mut intitule = "Offre importée".to_string();
    let mut entreprise = "Non identifié".to_string();

    if !lines.is_empty() {
        intitule = lines[0].to_string();
        if lines.len() > 1 {
            // Souvent le nom de l'entreprise est sur la 2ème ligne ou contient "chez", "at"
            for line in lines.iter().skip(1).take(3) {
                if line.to_lowercase().contains("chez") || line.to_lowercase().contains("at ") {
                    entreprise = line
                        .replace("chez", "")
                        .replace("Chez", "")
                        .replace("at", "")
                        .replace("At", "")
                        .trim()
                        .to_string();
                    break;
                }
            }
            if entreprise == "Non identifié" {
                entreprise = lines[1].to_string();
            }
        }
    }

    if intitule.len() > 120 {
        intitule = format!("{}…", &intitule[..117]);
    }
    if entreprise.len() > 100 {
        entreprise = format!("{}…", &entreprise[..97]);
    }

    (
        intitule,
        entreprise,
        None,
        None,
        OffreStructured {
            resume_court: "Extraction LLM échouée. Données brutes conservées.".into(),
            stack: vec![],
            missions: vec![],
            exigences: vec![],
            soft_skills: vec![],
            niveau_etudes: None,
            type_contrat: None,
            mots_cles: vec![],
        },
    )
}

fn build_offre_slug(entreprise: &str, intitule: &str) -> Slug {
    let raw = format!("{}_{}", entreprise, intitule);
    let slug_str: String = raw
        .to_lowercase()
        .chars()
        .map(|c| match c {
            'a'..='z' | '0'..='9' => c,
            'é' | 'è' | 'ê' | 'ë' => 'e',
            'à' | 'â' | 'ä' => 'a',
            'ù' | 'û' | 'ü' => 'u',
            'ô' | 'ö' => 'o',
            'î' | 'ï' => 'i',
            'ç' => 'c',
            _ => '_',
        })
        .collect::<String>();

    // Collapse multiple underscores and trim
    let slug_str: String = slug_str
        .split('_')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("_");

    // Tronquer à 80 chars max pour le slug
    let slug_str = if slug_str.len() > 80 {
        slug_str[..80].trim_end_matches('_').to_string()
    } else {
        slug_str
    };

    Slug::parse(&slug_str).unwrap_or_else(|_| {
        Slug::new_v4()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_detection() {
        assert!(looks_like_url("https://example.com/job"));
        assert!(looks_like_url("http://foo.bar"));
        assert!(!looks_like_url("Ceci est du texte brut"));
        assert!(!looks_like_url(""));
    }

    #[test]
    fn slug_generation() {
        let slug = build_offre_slug("Safran", "Alternance Data Analyst (Roche la Molière)");
        assert!(slug.as_str().contains("safran"));
        assert!(slug.as_str().contains("data"));
    }

    #[test]
    fn fallback_gives_first_line() {
        let (intitule, entreprise, _, _, structured) =
            fallback_extraction("Mon offre cool\nAutre ligne\nEncore");
        assert_eq!(intitule, "Mon offre cool");
        assert_eq!(entreprise, "Autre ligne");
        assert!(structured.resume_court.contains("échouée"));
    }
}
