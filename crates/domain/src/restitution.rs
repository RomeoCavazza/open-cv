//! Restitution — fiche d'analyse structurée d'une offre.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Restitution {
    #[serde(default, deserialize_with = "null_as_empty::deserialize")]
    pub synthese: String,
    #[serde(default, deserialize_with = "null_as_empty::deserialize")]
    pub entreprise: String,
    #[serde(default, deserialize_with = "null_as_empty::deserialize")]
    pub poste: String,
    #[serde(default, deserialize_with = "null_as_empty::deserialize")]
    pub profil_recherche: String,
    pub fit_score: u8,
    #[serde(default, deserialize_with = "null_as_empty::deserialize")]
    pub fit_justification: String,
    #[serde(default)]
    pub forces: Vec<String>,
    #[serde(default)]
    pub faiblesses: Vec<String>,
    #[serde(default)]
    pub missions: Vec<String>,
    #[serde(default)]
    pub stack_technique: Vec<String>,
    #[serde(default)]
    pub exigences: Vec<String>,
    pub contrat: Option<String>,
    pub localisation: Option<String>,
    pub remote: Option<String>,
    #[serde(default)]
    pub points_attention: Vec<String>,
    #[serde(default)]
    pub questions_entretien: Vec<String>,
}

impl Restitution {
    pub fn est_pertinente(&self) -> bool {
        self.fit_score >= 50
    }
}
