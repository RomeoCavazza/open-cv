use crate::errors::ApiError;
use crate::state::AppState;
use axum::{extract::State, response::Json};

pub async fn chat_handler(
    State(state): State<AppState>,
    Json(req): Json<application::chat::ChatRequest>,
) -> Result<Json<application::chat::ChatResponse>, ApiError> {
    let usecase =
        application::chat::ChatWithApplicationUseCase::new(
            state.offre_repo.clone(),
            state.instance_repo.clone(),
            state.profil_repo.clone(),
            state.annexe_repo.clone(),
            state.chunk_repo.clone(),
            state.message_repo.clone(),
            state.embedder.clone(),
            state.llm_registry.as_ref().clone(),
        );

    let res = usecase
        .execute(req)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(res))
}
