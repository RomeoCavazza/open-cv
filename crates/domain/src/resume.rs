//! Resume (CV) — structure stable, contenu adapté.
//!
//! Le contrat est : la STRUCTURE ne change jamais, seul le CONTENU est adapté
//! à l'offre par le LLM. Ça garantit que le renderer HTML reste stable et
//! qu'on n'a pas de surprises de mise en page.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Resume {
    pub identite: Identite,
    pub accroche: Accroche,
    pub contact: Contact,
    pub competences: Vec<GroupeCompetences>,
    pub experiences: Vec<Experience>,
    pub formations: Vec<Formation>,
    pub projets: Vec<Projet>,
    pub langues: Vec<Langue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Identite {
    pub nom_complet: String,
    pub photo_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Accroche {
    /// Ex: "ALTERNANCE — DÉVELOPPEUR IA"
    pub titre: String,
    /// 3-4 lignes adaptées à l'offre.
    pub paragraphe: String,
    /// "24 mois — à partir de septembre 2026"
    pub duree: String,
    /// "6 semaines en entreprise / 2 semaines en cours"
    pub rythme: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Contact {
    pub localisation: String,
    pub telephone: Option<String>,
    pub email: String,
    pub site_web: Option<String>,
    pub linkedin: Option<String>,
    pub github: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct GroupeCompetences {
    /// "Programmation", "MLOps", "Algorithmie", etc.
    pub categorie: String,
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Experience {
    pub poste: String,
    pub entreprise: String,
    pub localisation: Option<String>,
    /// "Janvier-Juin 2026" ou "2025 — Présent".
    pub periode: String,
    /// 2-4 bullets adaptés à l'offre. Pas de phrases à rallonge.
    pub bullets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Formation {
    pub etablissement: String,
    pub localisation: Option<String>,
    pub periode: String,
    pub diplome: String,
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Projet {
    pub nom: String,
    /// "(2025 — Présent)" ou similaire.
    pub periode: String,
    pub bullets: Vec<String>,
    pub lien: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Langue {
    pub langue: String,
    /// "natif", "C1", "B2", etc.
    pub niveau: String,
}
