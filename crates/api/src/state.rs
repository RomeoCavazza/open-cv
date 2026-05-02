use application::generate::GenerateApplicationUseCase;
use application::intake::IntakeOffreUseCase;
use ports::{InstanceRepo, LlmClient, OffreRepo};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub offre_repo: Arc<dyn OffreRepo>,
    pub instance_repo: Arc<dyn InstanceRepo>,
    pub generate_uc: Arc<GenerateApplicationUseCase>,
    pub intake_uc: Arc<IntakeOffreUseCase>,
    pub llm_registry: Arc<HashMap<String, Arc<dyn LlmClient>>>,
}
