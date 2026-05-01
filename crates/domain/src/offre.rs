//! L'offre d'emploi canonique. La dédup se fait à l'intake via `source_hash`.

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ids::{OffreId, Slug};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Offre {
    pub id: OffreId,
    pub slug: Slug,
    pub source_url: String,
    pub source_host: String,
    pub source_hash: Vec<u8>,
    pub entreprise: String,
    pub intitule: String,
    pub localisation: Option<String>,
    pub contrat: Option<String>,
    pub raw_text: String,
    pub structured: OffreStructured,
    pub scraped_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub categorie: Option<String>,
}

/// Sortie structurée de l'extraction LLM. Sert à la fois pour persistence
/// (colonne `structured` JSONB) et comme schéma pour `LlmClient::extract`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct OffreStructured {
    /// Résumé en 1 phrase de l'offre.
    pub resume_court: String,
    /// Stack technique mentionnée (langages, frameworks, outils).
    pub stack: Vec<String>,
    /// Missions principales décrites dans l'offre.
    pub missions: Vec<String>,
    /// Compétences exigées (hard skills).
    pub exigences: Vec<String>,
    /// Soft skills mentionnés.
    pub soft_skills: Vec<String>,
    /// Niveau d'études requis (Bac+5, Bac+3, etc.).
    pub niveau_etudes: Option<String>,
    /// Type de contrat tel qu'extrait (alternance, stage, CDI...).
    pub type_contrat: Option<String>,
    /// Mots-clés à intégrer dans le CV/lettre.
    pub mots_cles: Vec<String>,
}

impl Offre {
    /// Vrai si l'offre est encore ouverte (jamais fermée).
    pub fn est_ouverte(&self) -> bool {
        self.closed_at.is_none()
    }

    /// Identifiant court pour les logs (8 premiers caractères de l'UUID).
    pub fn short_id(&self) -> String {
        self.id.to_string().chars().take(8).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn offre_de_test(closed: Option<DateTime<Utc>>) -> Offre {
        Offre {
            id: OffreId::new(),
            slug: Slug::parse("test_offre").unwrap(),
            source_url: "https://example.com/offre".into(),
            source_host: "example.com".into(),
            source_hash: vec![0u8; 32],
            entreprise: "Test SA".into(),
            intitule: "Alternance Dev".into(),
            localisation: Some("Paris".into()),
            contrat: Some("alternance".into()),
            raw_text: "blah".into(),
            structured: OffreStructured {
                resume_court: "Test".into(),
                stack: vec!["Rust".into()],
                missions: vec![],
                exigences: vec![],
                soft_skills: vec![],
                niveau_etudes: None,
                type_contrat: None,
                mots_cles: vec![],
            },
            scraped_at: Utc::now(),
            last_seen_at: Utc::now(),
            closed_at: closed,
            categorie: None,
        }
    }

    #[test]
    fn offre_ouverte_par_defaut() {
        assert!(offre_de_test(None).est_ouverte());
    }

    #[test]
    fn offre_fermee_si_closed_at_set() {
        assert!(!offre_de_test(Some(Utc::now())).est_ouverte());
    }
}
