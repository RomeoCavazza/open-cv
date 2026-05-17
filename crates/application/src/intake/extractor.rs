//! Extractor — Extraction structurée via LLM avec fallback robuste.

use domain::{OffreStructured, Slug};
use ports::{ExtractionRequest, LlmClient, LlmClientExt};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, OnceLock};
use tracing::{info, warn};

/// Serde helper: deserializes `null` as `""` instead of failing.
mod null_as_empty {
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt = Option::<String>::deserialize(deserializer)?;
        Ok(opt.unwrap_or_default())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MultiOffreExtraction {
    /// Liste des offres identifiées dans le texte.
    /// Règle : Toujours renvoyer une liste, même s'il n'y a qu'une seule offre.
    #[serde(default)]
    pub offres: Vec<OffreExtraction>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct OffreExtraction {
    /// Nom du métier (ex: 'Développeur Java'). Retirer 'CV de' ou 'stp'.
    #[serde(default, deserialize_with = "null_as_empty::deserialize")]
    pub intitule: String,
    /// Nom de l'entreprise ou administration (ex: 'Direction de la sécurité sociale', 'Google').
    /// Pour le public, extraire l'entité la plus précise (Ministère, Direction ou Service).
    #[serde(default, deserialize_with = "null_as_empty::deserialize")]
    pub entreprise: String,
    pub localisation: Option<String>,
    pub contrat: Option<String>,
    #[serde(default, deserialize_with = "null_as_empty::deserialize")]
    pub resume_court: String,
    #[serde(default)]
    pub stack: Vec<String>,
    #[serde(default)]
    pub missions: Vec<String>,
    #[serde(default)]
    pub exigences: Vec<String>,
    #[serde(default)]
    pub soft_skills: Vec<String>,
    pub niveau_etudes: Option<String>,
    pub type_contrat: Option<String>,
    #[serde(default)]
    pub mots_cles: Vec<String>,
}

static EXTRACTION_SCHEMA: OnceLock<serde_json::Value> = OnceLock::new();

fn extraction_schema() -> &'static serde_json::Value {
    EXTRACTION_SCHEMA.get_or_init(|| {
        serde_json::to_value(schemars::schema_for!(MultiOffreExtraction))
            .expect("Schema is always serializable")
    })
}

pub struct StructuredExtractor {
    llm: Arc<dyn LlmClient>,
}

impl StructuredExtractor {
    pub fn new(llm: Arc<dyn LlmClient>) -> Self {
        Self { llm }
    }

    pub async fn extract(
        &self,
        text: &str,
        source_url: Option<&str>,
        llm_override: Option<Arc<dyn LlmClient>>,
    ) -> Vec<(
        String,
        String,
        Option<String>,
        Option<String>,
        OffreStructured,
    )> {
        let llm = llm_override.unwrap_or_else(|| self.llm.clone());

        let combined_text = if let Some(url) = source_url {
            format!("SOURCE URL: {}\n\n---\n\n{}", url, text)
        } else {
            text.to_string()
        };

        let req = ExtractionRequest {
            system: Some("Tu es un expert en recrutement. Ton rôle est d'extraire les données d'une ou plusieurs offres d'emploi.\n\
                Règles d'or :\n\
                0. FORMAT : Réponds UNIQUEMENT avec l'objet JSON de données.\n\
                1. ENTREPRISE (CRITIQUE) : Identifie l'employeur réel. RECHERCHE ACTIVE : Analyse l'URL source fournie au début du texte et TOUT le contenu. Si l'URL contient 'safran-group.com', l'entreprise est 'Safran'. Si elle contient 'totalenergies.com', c'est 'TotalEnergies'. Ne renvoie 'Non spécifié' que si aucun nom d'organisation n'est mentionné.\n\
                2. INTITULÉ : Sois exhaustif et précis (ex: 'Apprenti Data Analyst'). Retire les mentions inutiles comme 'H/F', 'STP', 'URGENT'.\n\
                3. CONTEXTE : Si le texte contient '__TARGET_TITLE__:', utilise cet intitulé exact.".into()),
            instruction: "Analyse le texte et l'URL fournis pour extraire la liste des offres d'emploi.".into(),
            input: vec![ports::MessageContent::Text(combined_text)],
            schema_name: "MultiOffreExtraction".into(),
            schema_description: "Liste d'offres extraites".into(),
            json_schema: extraction_schema().clone(),
            model: None,
            max_tokens: Some(4000),
        };

        match llm.extract_typed::<MultiOffreExtraction>(req).await {
            Ok(ext) => {
                info!(
                    "extraction LLM réussie : {} offre(s) trouvée(s)",
                    ext.offres.len()
                );
                ext.offres
                    .into_iter()
                    .map(|ext| {
                        (
                            ext.intitule,
                            ext.entreprise,
                            ext.localisation,
                            ext.contrat,
                            OffreStructured {
                                resume_court: ext.resume_court,
                                stack: ext.stack,
                                missions: ext.missions,
                                exigences: ext.exigences,
                                soft_skills: ext.soft_skills,
                                niveau_etudes: ext.niveau_etudes,
                                type_contrat: ext.type_contrat,
                                mots_cles: ext.mots_cles,
                            },
                        )
                    })
                    .collect()
            }
            Err(e) => {
                warn!("extraction LLM échouée : {e}");
                vec![fallback_extraction(text)]
            }
        }
    }
}

pub(crate) fn fallback_extraction(
    text: &str,
) -> (
    String,
    String,
    Option<String>,
    Option<String>,
    OffreStructured,
) {
    let first_line = text.lines().next().unwrap_or("Offre importée").to_string();
    (
        first_line,
        "Non spécifié".into(),
        None,
        None,
        OffreStructured {
            resume_court: "Extraction LLM échouée.".into(),
            ..Default::default()
        },
    )
}

pub fn build_offre_slug(entreprise: &str, intitule: &str) -> Slug {
    let raw = if entreprise == "Non spécifié" || entreprise.is_empty() {
        intitule.to_string()
    } else {
        format!("{}_{}", entreprise, intitule)
    };
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

    let slug_str: String = slug_str
        .split('_')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("_");

    let slug_str = if slug_str.len() > 80 {
        slug_str[..80].trim_end_matches('_').to_string()
    } else {
        slug_str
    };

    Slug::parse(&slug_str).unwrap_or_else(|_| Slug::new_v4())
}
