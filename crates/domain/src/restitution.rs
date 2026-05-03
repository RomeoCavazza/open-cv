//! Restitution — fiche d'analyse structurée d'une offre.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Restitution {
    pub synthese: String,
    pub entreprise: String,
    pub poste: String,
    pub profil_recherche: String,
    pub fit_score: u8,
    pub fit_justification: String,
    pub forces: Vec<String>,
    pub faiblesses: Vec<String>,
    pub missions: Vec<String>,
    pub stack_technique: Vec<String>,
    pub exigences: Vec<String>,
    pub contrat: Option<String>,
    pub localisation: Option<String>,
    pub remote: Option<String>,
    pub points_attention: Vec<String>,
    pub questions_entretien: Vec<String>,
}

impl Restitution {
    pub fn est_pertinente(&self) -> bool {
        self.fit_score >= 50
    }
}
