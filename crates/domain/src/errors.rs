//! Erreurs du domaine — purement métier, sans détail d'infra.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("transition d'état illégale : {from:?} -> {to:?}")]
    InvalidTransition {
        from: crate::InstanceStatus,
        to: crate::InstanceStatus,
    },

    #[error("offre fermée : impossible de générer une nouvelle instance")]
    OffreFermee,

    #[error("aucun profil actif")]
    AucunProfilActif,

    #[error("slug invalide : {0}")]
    Slug(#[from] crate::SlugError),
}
