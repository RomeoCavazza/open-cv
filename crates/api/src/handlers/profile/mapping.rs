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
    serde_json::to_value(profil.content).expect("ProfilContent is always serializable")
}

pub(super) fn active_profile_resume_template_or_content(profil: Profil) -> JsonValue {
    if let Some(template) = &profil.content.documents.resume_template {
        return template.clone();
    }

    serde_json::to_value(profil.content).expect("ProfilContent is always serializable")
}

pub(super) fn active_profile_cover_letter_template(profil: Profil) -> Result<JsonValue, ApiError> {
    profil
        .content
        .documents
        .cover_letter_template
        .clone()
        .ok_or_else(|| ApiError::NotFound("Aucun modèle de lettre trouvé".to_string()))
}

pub(super) fn apply_persisted_markers(profil: &mut Profil) {
    if profil.profile_photo.is_some() {
        profil.content.profile.image = "persisted:bytea".to_string();
    }

    if profil.calendar_pdf.is_some() {
        profil.content.documents.apprenticeship_calendar =
            Some(serde_json::json!("persisted:bytea"));
    }
}

pub(super) fn apply_profile_update(
    profil: &mut Profil,
    new_content: JsonValue,
) -> Result<(), ApiError> {
    let content = serde_json::from_value::<domain::ProfilContent>(new_content)
        .map_err(|e| ApiError::BadRequest(format!("Invalid profile payload: {e}")))?;

    profil.content = content;
    extract_profile_photo(profil);
    extract_calendar_pdf(profil);
    apply_persisted_markers(profil);

    Ok(())
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

fn extract_profile_photo(profil: &mut Profil) {
    let image = &profil.content.profile.image;
    if let Some(payload) = image
        .strip_prefix("data:")
        .and_then(|_| image.split(',').nth(1))
    {
        if let Ok(bytes) = decode_data_url(payload) {
            profil.profile_photo = Some(bytes);
        }
    }
}

fn extract_calendar_pdf(profil: &mut Profil) {
    if let Some(calendar) = &profil.content.documents.apprenticeship_calendar {
        if let Some(data_url) = calendar.get("data_url").and_then(|value| value.as_str()) {
            if let Some(payload) = data_url
                .strip_prefix("data:")
                .and_then(|_| data_url.split(',').nth(1))
            {
                if let Ok(bytes) = decode_data_url(payload) {
                    profil.calendar_pdf = Some(bytes);
                }
            }
        } else if let Some(data_url) = calendar.as_str() {
            if let Some(payload) = data_url
                .strip_prefix("data:")
                .and_then(|_| data_url.split(',').nth(1))
            {
                if let Ok(bytes) = decode_data_url(payload) {
                    profil.calendar_pdf = Some(bytes);
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
        let mut content = domain::ProfilContent::default();
        content.profile.firstname = "Original".into();
        if with_resume_template {
            content.documents.resume_template = Some(json!({"foo": "bar"}));
        }

        Profil {
            id: ProfilId::new(),
            label: "Test".into(),
            content,
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
        assert_eq!(
            active_profile_content(profil)["profile"]["firstname"],
            json!("Original")
        );
    }

    #[test]
    fn active_profile_resume_template_prefers_template() {
        let profil = build_profile(true);
        assert_eq!(
            active_profile_resume_template_or_content(profil),
            json!({"foo": "bar"})
        );
    }

    #[test]
    fn active_profile_resume_template_falls_back_to_content() {
        let profil = build_profile(false);
        let res = active_profile_resume_template_or_content(profil);
        assert_eq!(res["profile"]["firstname"], json!("Original"));
    }

    #[test]
    fn active_profile_cover_letter_template_errors_when_missing() {
        let profil = build_profile(true);
        let error = active_profile_cover_letter_template(profil).expect_err("missing template");
        assert!(matches!(error, ApiError::NotFound(_)));
    }

    #[test]
    fn apply_persisted_markers_marks_binary_fields() {
        let mut profil = build_profile(true);
        profil.profile_photo = Some(vec![1, 2, 3]);
        profil.calendar_pdf = Some(vec![4, 5, 6]);

        apply_persisted_markers(&mut profil);

        assert_eq!(profil.content.profile.image, "persisted:bytea");
        assert_eq!(
            profil.content.documents.apprenticeship_calendar,
            Some(json!("persisted:bytea"))
        );
    }

    #[test]
    fn apply_profile_update_merges_and_extracts_media() {
        let mut profil = build_profile(true);
        let new_content = json!({
            "profile": {
                "firstname": "Updated",
                "lastname": "",
                "title": "",
                "offer_type": "",
                "pitch": "",
                "location": "",
                "phone": "",
                "email": "",
                "linkedin": "",
                "website": "",
                "github": "",
                "image": "data:image/png;base64,aGVsbG8="
            },
            "apprenticeship": {
                "duration": "",
                "rhythm": ""
            },
            "experiences": [],
            "projects": [],
            "education": [],
            "skills": [],
            "languages": [],
            "documents": {
                "resume_template": {"foo": "bar"},
                "apprenticeship_calendar": {
                    "data_url": "data:application/pdf;base64,d29ybGQ="
                }
            }
        });

        let _ = apply_profile_update(&mut profil, new_content);

        assert_eq!(profil.content.profile.firstname, "Updated");
        assert_eq!(
            profil.content.documents.resume_template,
            Some(json!({"foo": "bar"}))
        );
        assert_eq!(profil.content.profile.image, "persisted:bytea");
        assert_eq!(
            profil.content.documents.apprenticeship_calendar,
            Some(json!("persisted:bytea"))
        );
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
