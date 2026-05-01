use std::sync::Arc;
use application::generate::GenerateApplicationUseCase;
use ports::{InstanceRepo, OffreRepo};

#[derive(Clone)]
pub struct AppState {
    pub offre_repo: Arc<dyn OffreRepo>,
    pub instance_repo: Arc<dyn InstanceRepo>,
    pub generate_uc: Arc<GenerateApplicationUseCase>,
}
