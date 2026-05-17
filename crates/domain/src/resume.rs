//! Resume (CV) — structure stable, contenu adapté.
//!
//! Le contrat est : la STRUCTURE ne change jamais, seul le CONTENU est adapté
//! à l'offre par le LLM. Ça garantit que le renderer HTML reste stable et
//! qu'on n'a pas de surprises de mise en page.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Serde helper: deserializes `null` as `""` instead of failing.
/// Required because small LLMs (qwen2.5:7b) sometimes emit `"field": null`
/// for String fields. `#[serde(default)]` only handles *missing* keys, not
/// explicit nulls.
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
pub struct Resume {
    #[serde(default)]
    pub identite: Identite,
    #[serde(default)]
    pub accroche: Accroche,
    #[serde(default)]
    pub contact: Contact,
    #[serde(default)]
    pub competences: Vec<GroupeCompetences>,
    #[serde(default)]
    pub experiences: Vec<Experience>,
    #[serde(default)]
    pub formations: Vec<Formation>,
    #[serde(default)]
    pub projets: Vec<Projet>,
    #[serde(default)]
    pub langues: Vec<Langue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Identite {
    #[serde(default, deserialize_with = "null_as_empty::deserialize")]
    pub nom_complet: String,
    pub photo_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Accroche {
    #[serde(default, deserialize_with = "null_as_empty::deserialize")]
    pub titre: String,
    #[serde(default, deserialize_with = "null_as_empty::deserialize")]
    pub paragraphe: String,
    #[serde(default, deserialize_with = "null_as_empty::deserialize")]
    pub duree: String,
    #[serde(default, deserialize_with = "null_as_empty::deserialize")]
    pub rythme: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Contact {
    #[serde(default, deserialize_with = "null_as_empty::deserialize")]
    pub localisation: String,
    pub telephone: Option<String>,
    #[serde(default, deserialize_with = "null_as_empty::deserialize")]
    pub email: String,
    pub site_web: Option<String>,
    pub linkedin: Option<String>,
    pub github: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct GroupeCompetences {
    #[serde(default, deserialize_with = "null_as_empty::deserialize")]
    pub categorie: String,
    #[serde(default)]
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Experience {
    #[serde(default, deserialize_with = "null_as_empty::deserialize")]
    pub poste: String,
    #[serde(default, deserialize_with = "null_as_empty::deserialize")]
    pub entreprise: String,
    pub localisation: Option<String>,
    #[serde(default, deserialize_with = "null_as_empty::deserialize")]
    pub periode: String,
    #[serde(default)]
    pub bullets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Formation {
    #[serde(default, deserialize_with = "null_as_empty::deserialize")]
    pub etablissement: String,
    pub localisation: Option<String>,
    #[serde(default, deserialize_with = "null_as_empty::deserialize")]
    pub periode: String,
    #[serde(default, deserialize_with = "null_as_empty::deserialize")]
    pub diplome: String,
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Projet {
    #[serde(default, deserialize_with = "null_as_empty::deserialize")]
    pub nom: String,
    #[serde(default, deserialize_with = "null_as_empty::deserialize")]
    pub periode: String,
    #[serde(default)]
    pub bullets: Vec<String>,
    pub lien: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Langue {
    #[serde(default, deserialize_with = "null_as_empty::deserialize")]
    pub langue: String,
    #[serde(default, deserialize_with = "null_as_empty::deserialize")]
    pub niveau: String,
}
