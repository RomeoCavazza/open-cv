use application::generate::GenerateApplicationUseCase;
use ports::{InstanceRepo, OffreRepo};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub offre_repo: Arc<dyn OffreRepo>,
    pub instance_repo: Arc<dyn InstanceRepo>,
    pub generate_uc: Arc<GenerateApplicationUseCase>,
}
