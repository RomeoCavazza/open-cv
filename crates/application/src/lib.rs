//! Use cases — l'orchestration métier qui compose domaine + ports.

use std::sync::Arc;

use domain::{Instance, Offre, Slug};
use ports::{InstanceRepo, OffreRepo, RepoError};
use thiserror::Error;

pub mod events;
pub mod generate;

#[cfg(any(test, feature = "test-mocks"))]
pub mod test_mocks;

pub use events::{EventBus, GenerationEvent, GenerationStep, StepStatus};
pub use generate::{
    GenerateApplicationUseCase, GenerateError, GenerateInput, GenerateOutput, Livrables,
};

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Repo(#[from] RepoError),

    #[error("ressource introuvable")]
    NotFound,

    #[error("autre : {0}")]
    Other(String),
}

// ─────────────────────────────────────────────────────────────────
// Use cases simples (lecture)
// ─────────────────────────────────────────────────────────────────

pub struct ListOffresUseCase {
    offres: Arc<dyn OffreRepo>,
}

impl ListOffresUseCase {
    pub fn new(offres: Arc<dyn OffreRepo>) -> Self {
        Self { offres }
    }

    pub async fn execute(&self, limit: u32) -> Result<Vec<Offre>, AppError> {
        let limit = limit.clamp(1, 100);
        Ok(self.offres.list_recent(limit).await?)
    }
}

pub struct GetInstanceBySlugUseCase {
    instances: Arc<dyn InstanceRepo>,
}

impl GetInstanceBySlugUseCase {
    pub fn new(instances: Arc<dyn InstanceRepo>) -> Self {
        Self { instances }
    }

    pub async fn execute(&self, slug: &Slug) -> Result<Instance, AppError> {
        self.instances
            .get_by_slug(slug)
            .await?
            .ok_or(AppError::NotFound)
    }
}
