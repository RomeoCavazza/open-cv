use crate::errors::ApiError;
use crate::state::AppState;
use axum::{extract::State, Json};
use domain::Profil;
use serde_json::Value as JsonValue;

pub async fn get_active_profile_handler(
    State(state): State<AppState>,
) -> Result<Json<Profil>, ApiError> {
    let profil = state
        .generate_uc
        .profils
        .get_active()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound("Aucun profil actif trouvé".to_string()))?;

    Ok(Json(profil))
}

pub async fn list_profiles_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<domain::Profil>>, ApiError> {
    let profils = state
        .generate_uc
        .profils
        .list_all()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(profils))
}

pub async fn get_active_profile_resume_handler(
    State(state): State<AppState>,
) -> Result<Json<JsonValue>, ApiError> {
    let profil = state
        .generate_uc
        .profils
        .get_active()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound("Aucun profil actif trouvé".to_string()))?;

    Ok(Json(profil.content))
}

pub async fn get_active_profile_resume_template_handler(
    State(state): State<AppState>,
) -> Result<Json<JsonValue>, ApiError> {
    let profil = state
        .generate_uc
        .profils
        .get_active()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound("Aucun profil actif trouvé".to_string()))?;

    if let Some(template) = profil
        .content
        .get("documents")
        .and_then(|docs| docs.get("resume_template"))
    {
        return Ok(Json(template.clone()));
    }

    Ok(Json(profil.content))
}

pub async fn get_active_profile_cover_letter_template_handler(
    State(state): State<AppState>,
) -> Result<Json<JsonValue>, ApiError> {
    let profil = state
        .generate_uc
        .profils
        .get_active()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound("Aucun profil actif trouvé".to_string()))?;

    let template = profil
        .content
        .get("documents")
        .and_then(|docs| docs.get("cover_letter_template"))
        .cloned()
        .ok_or_else(|| ApiError::NotFound("Aucun modèle de lettre trouvé".to_string()))?;

    Ok(Json(template))
}

pub async fn update_active_profile_handler(
    State(state): State<AppState>,
    Json(content): Json<JsonValue>,
) -> Result<(), ApiError> {
    let mut profil = state
        .generate_uc
        .profils
        .get_active()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound("Aucun profil actif trouvé".to_string()))?;

    profil.content = content;

    state
        .generate_uc
        .profils
        .upsert(&profil)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(())
}
