//! Extractor — Extraction structurée via LLM avec fallback robuste.

use std::sync::Arc;
use once_cell::sync::Lazy;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use domain::{OffreStructured, Slug};
use ports::{ExtractionRequest, LlmClient};
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct OffreExtraction {
    pub intitule: String,
    pub entreprise: String,
    pub localisation: Option<String>,
    pub contrat: Option<String>,
    pub resume_court: String,
    pub stack: Vec<String>,
    pub missions: Vec<String>,
    pub exigences: Vec<String>,
    pub soft_skills: Vec<String>,
    pub niveau_etudes: Option<String>,
    pub type_contrat: Option<String>,
    pub mots_cles: Vec<String>,
}

static EXTRACTION_SCHEMA: Lazy<serde_json::Value> =
    Lazy::new(|| serde_json::to_value(schemars::schema_for!(OffreExtraction)).expect("Schema is always serializable"));

pub struct StructuredExtractor {
    llm: Arc<dyn LlmClient>,
}

impl StructuredExtractor {
    pub fn new(llm: Arc<dyn LlmClient>) -> Self {
        Self { llm }
    }

    pub async fn extract(&self, text: &str, llm_override: Option<Arc<dyn LlmClient>>) -> (String, String, Option<String>, Option<String>, OffreStructured) {
        let llm = llm_override.unwrap_or_else(|| self.llm.clone());
        
        let req = ExtractionRequest {
            system: Some("Tu extrais les métadonnées structurées d'une offre d'emploi.".into()),
            instruction: "Extrais toutes les informations structurées de cette offre d'emploi.".into(),
            input: vec![ports::MessageContent::Text(text.to_string())],
            schema_name: "OffreExtraction".into(),
            schema_description: "Métadonnées structurées extraites d'une offre d'emploi".into(),
            json_schema: EXTRACTION_SCHEMA.clone(),
            model: None,
            max_tokens: Some(4000),
        };

        match llm.extract(req).await {
            Ok(json) => match serde_json::from_value::<OffreExtraction>(json) {
                Ok(ext) => {
                    info!("extraction LLM réussie");
                    (ext.intitule, ext.entreprise, ext.localisation, ext.contrat, OffreStructured {
                        resume_court: ext.resume_court,
                        stack: ext.stack,
                        missions: ext.missions,
                        exigences: ext.exigences,
                        soft_skills: ext.soft_skills,
                        niveau_etudes: ext.niveau_etudes,
                        type_contrat: ext.type_contrat,
                        mots_cles: ext.mots_cles,
                    })
                }
                Err(e) => {
                    warn!("déserialisation extraction échouée : {e}");
                    fallback_extraction(text)
                }
            },
            Err(e) => {
                warn!("extraction LLM échouée : {e}");
                fallback_extraction(text)
            }
        }
    }
}

pub(crate) fn fallback_extraction(text: &str) -> (String, String, Option<String>, Option<String>, OffreStructured) {
    let first_line = text.lines().next().unwrap_or("Offre importée").to_string();
    (
        first_line,
        "Non identifié".into(),
        None,
        None,
        OffreStructured {
            resume_court: "Extraction LLM échouée.".into(),
            ..Default::default()
        }
    )
}

pub fn build_offre_slug(entreprise: &str, intitule: &str) -> Slug {
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
