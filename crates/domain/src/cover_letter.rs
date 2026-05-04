//! Cover Letter — lettre de motivation typée par paragraphe.
//!
//! Pourquoi typer chaque paragraphe : ça permet plus tard au chat IA de dire
//! "régénère seulement le paragraphe Vous" ou "raccourcis l'accroche" sans
//! toucher au reste. C'est aussi l'invariant qui garantit qu'une lettre a
//! toujours sa structure attendue (pas d'absence de signature, pas de
//! "Cordialement" qui apparaît au milieu).

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::resume::{Contact, Identite};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct CoverLetter {
    pub expediteur: Expediteur,
    pub destinataire: Destinataire,
    pub objet: Objet,
    /// Paragraphes dans l'ordre de rendu, avec leur rôle typé.
    pub paragraphes: Vec<Paragraphe>,
    pub signature: Signature,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Expediteur {
    pub identite: Identite,
    pub contact: Contact,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Destinataire {
    pub entreprise: String,
    /// Format libre : "25 avril 2026". L'extraction LLM la formate en français.
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Objet {
    /// "ALTERNANCE", "STAGE", "CDI"... — la catégorie en MAJ pour le rendu.
    pub categorie: String,
    /// "ALTERNANCE — INGÉNIEUR IA"
    pub libelle: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Paragraphe {
    pub role: ParagrapheRole,
    pub contenu: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum ParagrapheRole {
    #[default]
    /// "Madame, Monsieur,"
    Salutation,
    /// "Actuellement étudiant en Master IA à EPITECH..."
    Accroche,
    /// "Mes projets récents, notamment..." — démontre les compétences.
    Projets,
    /// "Au-delà de la couche algorithmique..." — adresse l'offre/l'entreprise.
    Vous,
    /// "Travailler pour ArianeGroup..." — pourquoi cette boîte précise.
    Pourquoi,
    /// "Je reste à votre entière disposition..." — clôture.
    Cloture,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Signature {
    /// "Cordialement,"
    pub formule_politesse: String,
    pub nom: String,
}

impl CoverLetter {
    /// Récupère le contenu d'un paragraphe donné, s'il existe.
    pub fn paragraphe(&self, role: ParagrapheRole) -> Option<&str> {
        self.paragraphes
            .iter()
            .find(|p| p.role == role)
            .map(|p| p.contenu.as_str())
    }

    /// Vrai si la lettre a tous les paragraphes essentiels.
    pub fn est_complete(&self) -> bool {
        use ParagrapheRole::*;
        let roles_requis = [Salutation, Accroche, Cloture];
        roles_requis
            .iter()
            .all(|r| self.paragraphes.iter().any(|p| p.role == *r))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lettre_avec(roles: &[ParagrapheRole]) -> CoverLetter {
        CoverLetter {
            expediteur: Expediteur {
                identite: Identite {
                    nom_complet: "Test".into(),
                    photo_url: None,
                },
                contact: Contact {
                    localisation: "Paris".into(),
                    telephone: None,
                    email: "t@t.t".into(),
                    site_web: None,
                    linkedin: None,
                    github: None,
                },
            },
            destinataire: Destinataire {
                entreprise: "Test SA".into(),
                date: "2026-04-25".into(),
            },
            objet: Objet {
                categorie: "ALTERNANCE".into(),
                libelle: "ALTERNANCE - DEV".into(),
            },
            paragraphes: roles
                .iter()
                .map(|r| Paragraphe {
                    role: *r,
                    contenu: "blah".into(),
                })
                .collect(),
            signature: Signature {
                formule_politesse: "Cordialement,".into(),
                nom: "Test".into(),
            },
        }
    }

    #[test]
    fn complete_avec_essentiels() {
        use ParagrapheRole::*;
        let l = lettre_avec(&[Salutation, Accroche, Pourquoi, Cloture]);
        assert!(l.est_complete());
    }

    #[test]
    fn incomplete_sans_cloture() {
        use ParagrapheRole::*;
        let l = lettre_avec(&[Salutation, Accroche]);
        assert!(!l.est_complete());
    }

    #[test]
    fn lookup_par_role() {
        use ParagrapheRole::*;
        let l = lettre_avec(&[Salutation, Accroche]);
        assert!(l.paragraphe(Salutation).is_some());
        assert!(l.paragraphe(Cloture).is_none());
    }
}
