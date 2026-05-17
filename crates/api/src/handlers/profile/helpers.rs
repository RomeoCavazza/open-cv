use crate::errors::ApiError;
use crate::handlers::profile::AnnexeMetadata;
use axum::http::header::{CONTENT_DISPOSITION, CONTENT_TYPE};
use base64::{engine::general_purpose, Engine as _};
use domain::{Annexe, AnnexeId, Profil, ProfilId};
use ports::ProfilRepo;
use serde_json::Value as JsonValue;

use super::UploadAnnexeRequest;

pub(super) async fn resolve_active_profile(
    profil_repo: &dyn ProfilRepo,
) -> Result<Profil, ApiError> {
    profil_repo
        .get_active()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound("Aucun profil actif trouvé".to_string()))
}

pub(super) fn active_profile_content(profil: Profil) -> JsonValue {
    profil.content
}

pub(super) fn active_profile_resume_template_or_content(profil: Profil) -> JsonValue {
    if let Some(template) = profil
        .content
        .get("documents")
        .and_then(|docs| docs.get("resume_template"))
    {
        return template.clone();
    }

    profil.content
}

pub(super) fn active_profile_cover_letter_template(profil: Profil) -> Result<JsonValue, ApiError> {
    profil
        .content
        .get("documents")
        .and_then(|docs| docs.get("cover_letter_template"))
        .cloned()
        .ok_or_else(|| ApiError::NotFound("Aucun modèle de lettre trouvé".to_string()))
}

pub(super) fn apply_persisted_markers(profil: &mut Profil) {
    if profil.profile_photo.is_some() {
        if let Some(profile) = profil
            .content
            .get_mut("profile")
            .and_then(|p| p.as_object_mut())
        {
            profile.insert(
                "image".to_string(),
                JsonValue::String("persisted:bytea".to_string()),
            );
        }
    }

    if profil.calendar_pdf.is_some() {
        if let Some(documents) = profil
            .content
            .get_mut("documents")
            .and_then(|d| d.as_object_mut())
        {
            documents.insert(
                "apprenticeship_calendar".to_string(),
                JsonValue::String("persisted:bytea".to_string()),
            );
        }
    }
}

pub(super) fn apply_profile_update(profil: &mut Profil, new_content: JsonValue) {
    merge_profile_content(&mut profil.content, new_content);
    extract_profile_photo(profil);
    extract_calendar_pdf(profil);
    apply_persisted_markers(profil);
}

pub(super) fn decode_data_url(payload: &str) -> Result<Vec<u8>, String> {
    general_purpose::STANDARD
        .decode(payload)
        .map_err(|e| e.to_string())
}

pub(super) fn build_annexe_metadata(annexe: Annexe) -> AnnexeMetadata {
    AnnexeMetadata {
        id: annexe.id,
        label: annexe.label,
        filename: annexe.filename,
        content_type: annexe.content_type,
    }
}

pub(super) fn build_annexe_from_request(
    profil_id: ProfilId,
    req: UploadAnnexeRequest,
) -> Result<Annexe, String> {
    let b64_data = req
        .data_url
        .split(',')
        .nth(1)
        .ok_or_else(|| "Format de donnée invalide".to_string())?;

    let content = decode_data_url(b64_data).map_err(|e| format!("Base64 invalide : {}", e))?;

    Ok(Annexe {
        id: AnnexeId::new(),
        profil_id,
        label: req.label,
        filename: req.filename,
        content_type: req.content_type,
        content,
        created_at: chrono::Utc::now(),
    })
}

pub(super) fn build_download_response(annexe: Annexe) -> ([(&'static str, String); 2], Vec<u8>) {
    let headers = [
        (CONTENT_TYPE.as_str(), annexe.content_type),
        (
            CONTENT_DISPOSITION.as_str(),
            format!("inline; filename=\"{}\"", annexe.filename),
        ),
    ];

    (headers, annexe.content)
}

fn merge_profile_content(existing: &mut JsonValue, new_content: JsonValue) {
    if let (Some(old_obj), Some(new_obj)) = (existing.as_object_mut(), new_content.as_object()) {
        for (key, value) in new_obj {
            if key == "documents" {
                if let (Some(old_docs), Some(new_docs)) = (
                    old_obj.get_mut("documents").and_then(|docs| docs.as_object_mut()),
                    value.as_object(),
                ) {
                    for (doc_key, doc_value) in new_docs {
                        if !doc_value.is_null() {
                            old_docs.insert(doc_key.clone(), doc_value.clone());
                        }
                    }
                } else {
                    old_obj.insert(key.clone(), value.clone());
                }
            } else {
                old_obj.insert(key.clone(), value.clone());
            }
        }
    } else {
        *existing = new_content;
    }
}

fn extract_profile_photo(profil: &mut Profil) {
    if let Some(profile) = profil.content.get("profile").and_then(|p| p.as_object()) {
        if let Some(image) = profile.get("image").and_then(|value| value.as_str()) {
            if let Some(payload) = image.strip_prefix("data:").and_then(|_| image.split(',').nth(1)) {
                if let Ok(bytes) = decode_data_url(payload) {
                    profil.profile_photo = Some(bytes);
                }
            }
        }
    }
}

fn extract_calendar_pdf(profil: &mut Profil) {
    if let Some(documents) = profil.content.get("documents").and_then(|d| d.as_object()) {
        if let Some(calendar) = documents.get("apprenticeship_calendar") {
            if let Some(data_url) = calendar.get("data_url").and_then(|value| value.as_str()) {
                if let Some(payload) = data_url.strip_prefix("data:").and_then(|_| data_url.split(',').nth(1)) {
                    if let Ok(bytes) = decode_data_url(payload) {
                        profil.calendar_pdf = Some(bytes);
                    }
                }
            } else if let Some(data_url) = calendar.as_str() {
                if let Some(payload) = data_url.strip_prefix("data:").and_then(|_| data_url.split(',').nth(1)) {
                    if let Ok(bytes) = decode_data_url(payload) {
                        profil.calendar_pdf = Some(bytes);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use domain::{Annexe, AnnexeId, Profil, ProfilId};
    use serde_json::json;

    fn build_profile(with_resume_template: bool) -> Profil {
        let documents = if with_resume_template {
            json!({"resume_template": {"foo": "bar"}})
        } else {
            json!({})
        };

        Profil {
            id: ProfilId::new(),
            label: "Test".into(),
            content: json!({
                "profile": {},
                "documents": documents,
                "title": "Original"
            }),
            is_active: true,
            profile_photo: None,
            calendar_pdf: None,
            resume_template: None,
            cover_letter_template: None,
            notes: json!({}),
            created_at: Utc::now(),
        }
    }

    #[test]
    fn decode_data_url_decodes_payload() {
        let bytes = decode_data_url("aGVsbG8=").expect("base64 should decode");
        assert_eq!(bytes, b"hello");
    }

    #[test]
    fn active_profile_content_returns_raw_content() {
        let profil = build_profile(true);
        assert_eq!(active_profile_content(profil)["title"], json!("Original"));
    }

    #[test]
    fn active_profile_resume_template_prefers_template() {
        let profil = build_profile(true);
        assert_eq!(active_profile_resume_template_or_content(profil), json!({"foo": "bar"}));
    }

    #[test]
    fn active_profile_resume_template_falls_back_to_content() {
        let profil = build_profile(false);
        assert_eq!(
            active_profile_resume_template_or_content(profil),
            json!({
                "profile": {},
                "documents": {},
                "title": "Original"
            })
        );
    }

    #[test]
    fn active_profile_cover_letter_template_errors_when_missing() {
        let mut profil = build_profile(true);
        profil.content["documents"] = json!({});

        let error = active_profile_cover_letter_template(profil).expect_err("missing template");
        assert!(matches!(error, ApiError::NotFound(_)));
    }

    #[test]
    fn apply_persisted_markers_marks_binary_fields() {
        let mut profil = build_profile(true);
        profil.profile_photo = Some(vec![1, 2, 3]);
        profil.calendar_pdf = Some(vec![4, 5, 6]);

        apply_persisted_markers(&mut profil);

        assert_eq!(profil.content["profile"]["image"], json!("persisted:bytea"));
        assert_eq!(
            profil.content["documents"]["apprenticeship_calendar"],
            json!("persisted:bytea")
        );
    }

    #[test]
    fn apply_profile_update_merges_and_extracts_media() {
        let mut profil = build_profile(true);
        let new_content = json!({
            "profile": {
                "image": "data:image/png;base64,aGVsbG8="
            },
            "documents": {
                "apprenticeship_calendar": {
                    "data_url": "data:application/pdf;base64,d29ybGQ="
                }
            },
            "title": "Updated"
        });

        apply_profile_update(&mut profil, new_content);

        assert_eq!(profil.content["title"], json!("Updated"));
        assert_eq!(profil.content["documents"]["resume_template"], json!({"foo": "bar"}));
        assert_eq!(profil.content["profile"]["image"], json!("persisted:bytea"));
        assert_eq!(profil.content["documents"]["apprenticeship_calendar"], json!("persisted:bytea"));
        assert_eq!(profil.profile_photo, Some(b"hello".to_vec()));
        assert_eq!(profil.calendar_pdf, Some(b"world".to_vec()));
    }

    #[test]
    fn build_annexe_from_request_decodes_payload() {
        let req = UploadAnnexeRequest {
            label: "CV".into(),
            filename: "cv.pdf".into(),
            content_type: "application/pdf".into(),
            data_url: "data:application/pdf;base64,aGVsbG8=".into(),
        };

        let annexe = build_annexe_from_request(ProfilId::new(), req).expect("annexe should build");

        assert_eq!(annexe.content, b"hello");
        assert_eq!(annexe.filename, "cv.pdf");
        assert_eq!(annexe.label, "CV");
    }

    #[test]
    fn build_download_response_sets_inline_headers() {
        let annexe = Annexe {
            id: AnnexeId::new(),
            profil_id: ProfilId::new(),
            label: "CV".into(),
            filename: "cv.pdf".into(),
            content_type: "application/pdf".into(),
            content: b"hello".to_vec(),
            created_at: Utc::now(),
        };

        let (headers, body) = build_download_response(annexe);

        assert_eq!(headers[0].0, CONTENT_TYPE.as_str());
        assert_eq!(headers[0].1, "application/pdf");
        assert_eq!(headers[1].0, CONTENT_DISPOSITION.as_str());
        assert!(headers[1].1.contains("cv.pdf"));
        assert_eq!(body, b"hello");
    }
}
