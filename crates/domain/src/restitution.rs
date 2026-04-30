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
    pub synthese: String,

    /// Fit auto-évalué entre l'offre et le profil.
    pub fit: FitAnalysis,

    /// Ce que l'offre dit explicitement.
    pub explicite: ExplicitContent,

    /// Ce que le LLM a inféré (signaux faibles).
    pub implicite: ImplicitSignals,

    /// Points d'attention pour la candidature.
    pub points_a_traiter: Vec<PointAttention>,

    /// Questions à creuser en entretien.
    pub questions_entretien: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FitAnalysis {
    /// Score 0-100. Le LLM doit être sévère : 80+ = très bon match, 60 = OK,
    /// < 50 = pas pour toi.
    pub score: u8,
    pub justification: String,
    pub forces: Vec<String>,
    pub faiblesses: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ExplicitContent {
    pub missions: Vec<String>,
    pub stack_technique: Vec<String>,
    /// Diplôme, années d'expérience, certifs requis.
    pub exigences_dures: Vec<String>,
    pub soft_skills: Vec<String>,
    pub conditions: Conditions,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Conditions {
    pub contrat: Option<String>,
    pub duree: Option<String>,
    pub remuneration: Option<String>,
    pub localisation: Option<String>,
    pub remote: Option<String>,
    pub date_debut: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ImplicitSignals {
    /// "junior accompagné", "senior autonome", etc.
    pub niveau_autonomie: String,
    /// "greenfield", "mature", "legacy à moderniser".
    pub maturite_equipe: String,
    /// "corporate", "startup", "labo R&D", "industrie classique".
    pub culture: String,
    /// Stack non dite mais probable.
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
