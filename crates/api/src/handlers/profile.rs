use crate::errors::ApiError;
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use domain::{AnnexeId, Profil};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

mod mapping;

use mapping::{
    active_profile_content, active_profile_cover_letter_template,
    active_profile_resume_template_or_content, apply_persisted_markers, apply_profile_update,
    build_annexe_from_request, build_annexe_metadata, build_download_response,
    resolve_active_profile,
};

pub async fn get_active_profile_handler(
    State(state): State<AppState>,
) -> Result<Json<Profil>, ApiError> {
    let mut profil = resolve_active_profile(state.generate_uc.profils.as_ref()).await?;

    apply_persisted_markers(&mut profil);

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
    let profil = resolve_active_profile(state.generate_uc.profils.as_ref()).await?;

    Ok(Json(active_profile_content(profil)))
}

pub async fn get_active_profile_resume_template_handler(
    State(state): State<AppState>,
) -> Result<Json<JsonValue>, ApiError> {
    let profil = resolve_active_profile(state.generate_uc.profils.as_ref()).await?;

    Ok(Json(active_profile_resume_template_or_content(profil)))
}

pub async fn get_active_profile_cover_letter_template_handler(
    State(state): State<AppState>,
) -> Result<Json<JsonValue>, ApiError> {
    let profil = resolve_active_profile(state.generate_uc.profils.as_ref()).await?;

    Ok(Json(active_profile_cover_letter_template(profil)?))
}

pub async fn update_active_profile_handler(
    State(state): State<AppState>,
    Json(new_content): Json<JsonValue>,
) -> Result<(), ApiError> {
    tracing::info!("Début de la mise à jour du profil actif");
    let mut profil = resolve_active_profile(state.generate_uc.profils.as_ref()).await?;

    apply_profile_update(&mut profil, new_content)?;

    state
        .generate_uc
        .profils
        .upsert(&profil)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnexeMetadata {
    pub id: AnnexeId,
    pub label: String,
    pub filename: String,
    pub content_type: String,
}

pub async fn list_annexes_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<AnnexeMetadata>>, ApiError> {
    let profil = resolve_active_profile(state.profil_repo.as_ref()).await?;

    let annexes = state
        .annexe_repo
        .list_by_profil_id(profil.id)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    tracing::info!(
        "{} annexes trouvées pour le profil {}",
        annexes.len(),
        profil.id
    );

    let metadata = annexes.into_iter().map(build_annexe_metadata).collect();

    Ok(Json(metadata))
}

#[derive(Debug, Deserialize)]
pub struct UploadAnnexeRequest {
    pub label: String,
    pub filename: String,
    pub content_type: String,
    pub data_url: String,
}

pub async fn upload_annexe_handler(
    State(state): State<AppState>,
    Json(req): Json<UploadAnnexeRequest>,
) -> Result<Json<AnnexeId>, ApiError> {
    tracing::info!(
        "Upload d'une nouvelle annexe : {} ({})",
        req.label,
        req.filename
    );
    tracing::debug!("Taille de la data URL : {} chars", req.data_url.len());
    let profil = resolve_active_profile(state.profil_repo.as_ref()).await?;

    let annexe = build_annexe_from_request(profil.id, req).map_err(ApiError::BadRequest)?;

    state
        .annexe_repo
        .upsert(&annexe)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(annexe.id))
}

pub async fn download_annexe_handler(
    State(state): State<AppState>,
    Path(annexe_id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let id = AnnexeId::from_uuid(annexe_id);
    let annexe = state
        .annexe_repo
        .get_by_id(id)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound("Annexe introuvable".to_string()))?;

    Ok(build_download_response(annexe))
}

pub async fn delete_annexe_handler(
    State(state): State<AppState>,
    Path(annexe_id): Path<uuid::Uuid>,
) -> Result<(), ApiError> {
    let id = AnnexeId::from_uuid(annexe_id);
    state
        .annexe_repo
        .delete(id)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(())
}

pub async fn get_active_profile_photo_handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    let profil = resolve_active_profile(state.profil_repo.as_ref()).await?;
    match profil.profile_photo {
        Some(photo) => Ok((
            [("content-type", "image/jpeg")], // On assume JPEG pour le seed
            photo,
        )),
        None => Err(ApiError::NotFound("Photo non disponible".to_string())),
    }
}

pub async fn get_active_profile_calendar_handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    let profil = resolve_active_profile(state.profil_repo.as_ref()).await?;
    match profil.calendar_pdf {
        Some(pdf) => Ok((
            [("content-type", "application/pdf")],
            pdf,
        )),
        None => Err(ApiError::NotFound("Calendrier non disponible".to_string())),
    }
}
