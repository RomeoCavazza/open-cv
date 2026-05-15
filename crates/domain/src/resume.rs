//! Resume (CV) — structure stable, contenu adapté.
//!
//! Le contrat est : la STRUCTURE ne change jamais, seul le CONTENU est adapté
//! à l'offre par le LLM. Ça garantit que le renderer HTML reste stable et
//! qu'on n'a pas de surprises de mise en page.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
    #[serde(default)]
    pub nom_complet: String,
    pub photo_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Accroche {
    #[serde(default)]
    pub titre: String,
    #[serde(default)]
    pub paragraphe: String,
    #[serde(default)]
    pub duree: String,
    #[serde(default)]
    pub rythme: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Contact {
    #[serde(default)]
    pub localisation: String,
    pub telephone: Option<String>,
    #[serde(default)]
    pub email: String,
    pub site_web: Option<String>,
    pub linkedin: Option<String>,
    pub github: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct GroupeCompetences {
    #[serde(default)]
    pub categorie: String,
    #[serde(default)]
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Experience {
    #[serde(default)]
    pub poste: String,
    #[serde(default)]
    pub entreprise: String,
    pub localisation: Option<String>,
    #[serde(default)]
    pub periode: String,
    #[serde(default)]
    pub bullets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Formation {
    #[serde(default)]
    pub etablissement: String,
    pub localisation: Option<String>,
    #[serde(default)]
    pub periode: String,
    #[serde(default)]
    pub diplome: String,
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Projet {
    #[serde(default)]
    pub nom: String,
    #[serde(default)]
    pub periode: String,
    #[serde(default)]
    pub bullets: Vec<String>,
    pub lien: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Langue {
    #[serde(default)]
    pub langue: String,
    #[serde(default)]
    pub niveau: String,
}
