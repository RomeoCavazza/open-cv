use crate::errors::ApiError;
use crate::state::AppState;
use axum::{extract::{State, Path}, Json, response::IntoResponse};
use axum::http::header::{CONTENT_TYPE, CONTENT_DISPOSITION};
use domain::{Annexe, AnnexeId, Profil, ProfilId};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

pub async fn get_active_profile_handler(
    State(state): State<AppState>,
) -> Result<Json<Profil>, ApiError> {
    let mut profil = state
        .generate_uc
        .profils
        .get_active()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound("Aucun profil actif trouvé".to_string()))?;

    // INJECTION DES MARQUEURS BINAIRES
    // On informe le frontend que ces données existent en DB (BYTEA)
    if profil.profile_photo.is_some() {
        if let Some(p) = profil.content.get_mut("profile").and_then(|p| p.as_object_mut()) {
            p.insert("image".to_string(), serde_json::Value::String("persisted:bytea".to_string()));
        }
    }
    
    if profil.calendar_pdf.is_some() {
        if let Some(d) = profil.content.get_mut("documents").and_then(|d| d.as_object_mut()) {
            d.insert("apprenticeship_calendar".to_string(), serde_json::Value::String("persisted:bytea".to_string()));
        }
    }

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
    Json(new_content): Json<JsonValue>,
) -> Result<(), ApiError> {
    tracing::info!("Début de la mise à jour du profil actif");
    let mut profil = state
        .generate_uc
        .profils
        .get_active()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound("Aucun profil actif trouvé".to_string()))?;

    // FUSION INTELLIGENTE : On garde l'ancien contenu et on écrase seulement ce qui arrive
    if let (Some(old_obj), Some(new_obj)) =
        (profil.content.as_object_mut(), new_content.as_object())
    {
        for (k, v) in new_obj {
            // On n'écrase pas avec du null/vide si c'est sensible (documents)
            if k == "documents" {
                if let (Some(old_docs), Some(new_docs)) = (
                    old_obj.get_mut("documents").and_then(|d| d.as_object_mut()),
                    v.as_object(),
                ) {
                    for (dk, dv) in new_docs {
                        if !dv.is_null() {
                            old_docs.insert(dk.clone(), dv.clone());
                        }
                    }
                } else {
                    old_obj.insert(k.clone(), v.clone());
                }
            } else {
                old_obj.insert(k.clone(), v.clone());
            }
        }
    } else {
        profil.content = new_content;
    }

    // EXTRACTION DES BINAIRES (Photo, Calendrier)
    // On extrait depuis le JSON pour peupler les colonnes BYTEA dédiées
    if let Some(profile) = profil.content.get("profile").and_then(|p| p.as_object()) {
        if let Some(img_base64) = profile.get("image").and_then(|i| i.as_str()) {
            if img_base64.starts_with("data:") {
                if let Some(b64_part) = img_base64.split(',').nth(1) {
                    use base64::{engine::general_purpose, Engine as _};
                    if let Ok(bytes) = general_purpose::STANDARD.decode(b64_part) {
                        profil.profile_photo = Some(bytes);
                    }
                }
            }
        }
    }

    if let Some(docs) = profil.content.get("documents").and_then(|d| d.as_object()) {
        if let Some(cal_data) = docs.get("apprenticeship_calendar") {
            // Si c'est un objet avec data_url (nouveau format frontend)
            if let Some(data_url) = cal_data.get("data_url").and_then(|u| u.as_str()) {
                if data_url.starts_with("data:") {
                    if let Some(b64_part) = data_url.split(',').nth(1) {
                        use base64::{engine::general_purpose, Engine as _};
                        if let Ok(bytes) = general_purpose::STANDARD.decode(b64_part) {
                            profil.calendar_pdf = Some(bytes);
                        }
                    }
                }
            } 
            // Si c'est directement une string data_url (ancien format)
            else if let Some(data_url) = cal_data.as_str() {
                if data_url.starts_with("data:") {
                    if let Some(b64_part) = data_url.split(',').nth(1) {
                        use base64::{engine::general_purpose, Engine as _};
                        if let Ok(bytes) = general_purpose::STANDARD.decode(b64_part) {
                            profil.calendar_pdf = Some(bytes);
                        }
                    }
                }
            }
        }
    }

    // Nettoyage optionnel du JSON pour ne pas stocker le binaire en double (gain de place)
    // On garde le champ mais on vide la data lourde si on a réussi l'extraction
    if profil.profile_photo.is_some() {
        if let Some(p) = profil.content.get_mut("profile").and_then(|p| p.as_object_mut()) {
            p.insert("image".to_string(), serde_json::Value::String("persisted:bytea".to_string()));
        }
    }

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
    let profil = state
        .profil_repo
        .get_active()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound("Profil actif introuvable".to_string()))?;

    let annexes = state
        .annexe_repo
        .list_by_profil_id(profil.id)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    tracing::info!("{} annexes trouvées pour le profil {}", annexes.len(), profil.id);

    let metadata = annexes
        .into_iter()
        .map(|a| AnnexeMetadata {
            id: a.id,
            label: a.label,
            filename: a.filename,
            content_type: a.content_type,
        })
        .collect();

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
    tracing::info!("Upload d'une nouvelle annexe : {} ({})", req.label, req.filename);
    tracing::debug!("Taille de la data URL : {} chars", req.data_url.len());
    let profil = state
        .profil_repo
        .get_active()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound("Profil actif introuvable".to_string()))?;

    let b64_data = req
        .data_url
        .split(',')
        .nth(1)
        .ok_or_else(|| ApiError::BadRequest("Format de donnée invalide".to_string()))?;

    use base64::{engine::general_purpose, Engine as _};
    let content = general_purpose::STANDARD
        .decode(b64_data)
        .map_err(|e| ApiError::BadRequest(format!("Base64 invalide : {}", e)))?;

    let annexe = Annexe {
        id: AnnexeId::new(),
        profil_id: profil.id,
        label: req.label,
        filename: req.filename,
        content_type: req.content_type,
        content,
        created_at: chrono::Utc::now(),
    };

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

    let headers = [
        (CONTENT_TYPE, annexe.content_type),
        (
            CONTENT_DISPOSITION,
            format!("inline; filename=\"{}\"", annexe.filename),
        ),
    ];

    Ok((headers, annexe.content))
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
