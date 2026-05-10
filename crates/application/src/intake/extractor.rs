//! Extractor — Extraction structurée via LLM avec fallback robuste.

use domain::{OffreStructured, Slug};
use ports::{ExtractionRequest, LlmClient, LlmClientExt};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, OnceLock};
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MultiOffreExtraction {
    /// Liste des offres identifiées dans le texte.
    /// Règle : Toujours renvoyer une liste, même s'il n'y a qu'une seule offre.
    pub offres: Vec<OffreExtraction>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct OffreExtraction {
    /// Nom du métier (ex: 'Développeur Java'). Retirer 'CV de' ou 'stp'.
    pub intitule: String,
    /// Nom de l'entreprise ou administration (ex: 'Direction de la sécurité sociale', 'Google').
    /// Pour le public, extraire l'entité la plus précise (Ministère, Direction ou Service).
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
        llm_override: Option<Arc<dyn LlmClient>>,
    ) -> Vec<(
        String,
        String,
        Option<String>,
        Option<String>,
        OffreStructured,
    )> {
        let llm = llm_override.unwrap_or_else(|| self.llm.clone());

        let req = ExtractionRequest {
            system: Some("Tu es un expert en recrutement. Ton rôle est d'extraire les données d'une ou plusieurs offres d'emploi.\n\
                Règles d'or :\n\
                0. FORMAT : Réponds UNIQUEMENT avec l'objet JSON de données. Ne renvoie JAMAIS le schéma JSON lui-même.\n\
                1. ENTREPRISE : Identifie l'employeur réel (ex: 'Direction de la sécurité sociale'). Ne confonds pas la plateforme de diffusion avec l'employeur. Si absent: 'Non spécifié'.\n\
                2. INTITULÉ : Sois exhaustif et précis (ex: 'Apprenti - Pilotage stratégique SI'). Retire 'H/F', 'STP', 'CV de', 'URGENT'.\n\
                3. CONTEXTE : Si le texte contient '__TARGET_TITLE__:', utilise cette info en priorité absolue. Ne recopie JAMAIS le label '__TARGET_TITLE__:', extrais uniquement l'intitulé.".into()),
            instruction: "Analyse le texte et extrais-en la liste des offres d'emploi (même s'il n'y en a qu'une seule).".into(),
            input: vec![ports::MessageContent::Text(text.to_string())],
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
