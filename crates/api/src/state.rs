use std::sync::Arc;

use ports::{InstanceRepo, OffreRepo};

#[derive(Clone)]
pub struct AppState {
    pub offre_repo: Arc<dyn OffreRepo>,
    pub instance_repo: Arc<dyn InstanceRepo>,
}
