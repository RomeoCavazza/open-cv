use application::generate::GenerateApplicationUseCase;
use application::intake::IntakeOffreUseCase;
use ports::{InstanceRepo, LlmClient, OffreRepo};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Semaphore;

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
    pub offre_repo: Arc<dyn OffreRepo>,
    pub instance_repo: Arc<dyn InstanceRepo>,
    pub profil_repo: Arc<dyn ports::ProfilRepo>,
    pub generate_uc: Arc<GenerateApplicationUseCase>,
    pub intake_uc: Arc<IntakeOffreUseCase>,
    pub chunk_repo: Arc<dyn ports::ChunkRepo>,
    pub annexe_repo: Arc<dyn ports::AnnexeRepo>,
    pub message_repo: Arc<dyn ports::MessageRepo>,
    pub snapshot_repo: Arc<dyn ports::SnapshotRepo>,
    pub embedder: Arc<dyn ports::Embedder>,
    pub llm_registry: Arc<HashMap<String, Arc<dyn LlmClient>>>,
    pub generation_slots: Arc<Semaphore>,
    pub generation_queue: Arc<Semaphore>,
}
