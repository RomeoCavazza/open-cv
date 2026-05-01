use axum::{extract::State, Json};
use crate::state::AppState;
use crate::errors::ApiError;
use domain::Profil;
use serde_json::Value as JsonValue;

pub async fn get_active_profile_handler(
    State(state): State<AppState>,
) -> Result<Json<Profil>, ApiError> {
    let profil = state.generate_uc.profils.get_active().await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound("Aucun profil actif trouvé".to_string()))?;

    Ok(Json(profil))
}

pub async fn get_active_profile_resume_handler(
    State(state): State<AppState>,
) -> Result<Json<JsonValue>, ApiError> {
    let profil = state.generate_uc.profils.get_active().await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound("Aucun profil actif trouvé".to_string()))?;

    Ok(Json(profil.content))
}

pub async fn update_active_profile_handler(
    State(state): State<AppState>,
    Json(content): Json<JsonValue>,
) -> Result<(), ApiError> {
    let mut profil = state.generate_uc.profils.get_active().await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound("Aucun profil actif trouvé".to_string()))?;

    profil.content = content;
    
    state.generate_uc.profils.upsert(&profil).await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(())
}
