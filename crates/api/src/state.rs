use application::generate::GenerateApplicationUseCase;
use application::intake::IntakeOffreUseCase;
use ports::{InstanceRepo, LlmClient, OffreRepo};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
    pub offre_repo: Arc<dyn OffreRepo>,
    pub instance_repo: Arc<dyn InstanceRepo>,
    pub profil_repo: Arc<dyn ports::ProfilRepo>,
    pub generate_uc: Arc<GenerateApplicationUseCase>,
    pub intake_uc: Arc<IntakeOffreUseCase>,
    pub chunk_repo: Arc<dyn ports::ChunkRepo>,
    pub embedder: Arc<dyn ports::Embedder>,
    pub llm_registry: Arc<HashMap<String, Arc<dyn LlmClient>>>,
}
