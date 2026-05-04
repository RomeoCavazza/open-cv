use crate::errors::ApiError;
use crate::state::AppState;
use axum::{extract::State, response::Json};

pub async fn chat_handler(
    State(state): State<AppState>,
    Json(req): Json<application::chat::ChatRequest>,
) -> Result<Json<application::chat::ChatResponse>, ApiError> {
    let usecase =
        application::chat::ChatWithApplicationUseCase::new(application::chat::ChatDependencies {
            offre_repo: state.offre_repo.clone(),
            instance_repo: state.instance_repo.clone(),
            profil_repo: state.profil_repo.clone(),
            annexe_repo: state.annexe_repo.clone(),
            chunk_repo: state.chunk_repo.clone(),
            message_repo: state.message_repo.clone(),
            embedder: state.embedder.clone(),
            llm_registry: state.llm_registry.as_ref().clone(),
        });

    let res = usecase
        .execute(req)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(res))
}
