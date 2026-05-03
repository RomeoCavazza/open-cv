//! Restitution — fiche d'analyse structurée d'une offre.
//!
//! C'est le 3ème livrable de chaque candidature, à côté du CV et de la lettre.
//! But : (a) aperçu rapide quand 50 offres défilent, (b) contexte mâché pour
//! les générations CV/lettre, (c) base pour la préparation d'entretien.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Restitution {
    /// Synthèse en 2-3 phrases : qui, quoi, où, pourquoi intéressant.
    #[serde(default)]
    pub synthese: String,

    /// Résumé structuré de l'entreprise (secteur, taille, enjeux).
    #[serde(default)]
    pub entreprise_resume: String,

    /// Résumé structuré du poste (contexte, équipe, objectifs).
    #[serde(default)]
    pub poste_resume: String,

    /// Résumé du profil recherché (diplôme, mindset, expériences clés).
    #[serde(default)]
    pub profil_recherche: String,

    /// Fit auto-évalué entre l'offre et le profil.
    #[serde(default)]
    pub fit: FitAnalysis,

    /// Ce que l'offre dit explicitement.
    #[serde(default)]
    pub explicite: ExplicitContent,

    /// Ce que le LLM a inféré (signaux faibles).
    #[serde(default)]
    pub implicite: ImplicitSignals,

    /// Points d'attention pour la candidature.
    #[serde(default)]
    pub points_a_traiter: Vec<PointAttention>,

    /// Questions à creuser en entretien.
    #[serde(default)]
    pub questions_entretien: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct FitAnalysis {
    /// Score 0-100. Le LLM doit être sévère : 80+ = très bon match, 60 = OK,
    /// < 50 = pas pour toi.
    #[serde(default)]
    pub score: u8,
    #[serde(default)]
    pub justification: String,
    #[serde(default)]
    pub forces: Vec<String>,
    #[serde(default)]
    pub faiblesses: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct ExplicitContent {
    #[serde(default)]
    pub missions: Vec<String>,
    #[serde(default)]
    pub stack_technique: Vec<String>,
    /// Diplôme, années d'expérience, certifs requis.
    #[serde(default)]
    pub exigences_dures: Vec<String>,
    #[serde(default)]
    pub soft_skills: Vec<String>,
    #[serde(default)]
    pub conditions: Conditions,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Conditions {
    #[serde(default)]
    pub contrat: Option<String>,
    #[serde(default)]
    pub duree: Option<String>,
    #[serde(default)]
    pub remuneration: Option<String>,
    #[serde(default)]
    pub localisation: Option<String>,
    #[serde(default)]
    pub remote: Option<String>,
    #[serde(default)]
    pub date_debut: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct ImplicitSignals {
    /// "junior accompagné", "senior autonome", etc.
    #[serde(default)]
    pub niveau_autonomie: String,
    /// "greenfield", "mature", "legacy à moderniser".
    #[serde(default)]
    pub maturite_equipe: String,
    /// "corporate", "startup", "labo R&D", "industrie classique".
    #[serde(default)]
    pub culture: String,
    /// Stack non dite mais probable.
    #[serde(default)]
    pub stack_implicite: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PointAttention {
    pub categorie: PointCategorie,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PointCategorie {
    /// "À mettre en avant absolument."
    Forcer,
    /// "À traiter dans la lettre, point faible perçu."
    Adresser,
    /// "Ambigu, à éclaircir en entretien."
    Questionner,
    /// "Red flag potentiel."
    Vigilance,
}

impl Restitution {
    /// Vrai si le score laisse penser que ça vaut le coup de candidater.
    pub fn est_pertinente(&self) -> bool {
        self.fit.score >= 50
    }
}
