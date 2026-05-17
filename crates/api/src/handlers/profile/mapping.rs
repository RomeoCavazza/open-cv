use crate::errors::ApiError;
use crate::handlers::profile::AnnexeMetadata;
use axum::http::header::{CONTENT_DISPOSITION, CONTENT_TYPE};
use base64::{engine::general_purpose, Engine as _};
use domain::{Annexe, AnnexeId, JsonValue as DomainJsonValue, Profil, ProfilId};
use ports::ProfilRepo;
use serde_json::Value as SerdeJsonValue;

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

pub(super) fn active_profile_content(profil: Profil) -> SerdeJsonValue {
    serde_json::to_value(profil.content).expect("ProfilContent is always serializable")
}

pub(super) fn active_profile_resume_template_or_content(profil: Profil) -> SerdeJsonValue {
    if let Some(template) = &profil.content.documents.resume_template {
        return serde_json::to_value(template).expect("JsonValue is always serializable");
    }

    serde_json::to_value(profil.content).expect("ProfilContent is always serializable")
}

pub(super) fn active_profile_cover_letter_template(
    profil: Profil,
) -> Result<SerdeJsonValue, ApiError> {
    profil
        .content
        .documents
        .cover_letter_template
        .clone()
        .map(|v| serde_json::to_value(v).expect("JsonValue is always serializable"))
        .ok_or_else(|| ApiError::NotFound("Aucun modèle de lettre trouvé".to_string()))
}

pub(super) fn apply_persisted_markers(profil: &mut Profil) {
    if profil.profile_photo.is_some() {
        profil.content.profile.image = "persisted:bytea".to_string();
    }

    if profil.calendar_pdf.is_some() {
        profil.content.documents.apprenticeship_calendar =
            Some(DomainJsonValue::String("persisted:bytea".to_string()));
    }
}

pub(super) fn apply_profile_update(
    profil: &mut Profil,
    new_content: SerdeJsonValue,
) -> Result<(), ApiError> {
    let mut content = serde_json::from_value::<domain::ProfilContent>(new_content)
        .map_err(|e| ApiError::BadRequest(format!("Invalid profile payload: {e}")))?;

    content.profile.location = normalize_location(&content.profile.location);

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

fn normalize_location(raw: &str) -> String {
    let normalized_space = raw.split_whitespace().collect::<Vec<_>>().join(" ");
    let trimmed = normalized_space.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    if let Some(arr) = extract_paris_arrondissement(trimmed) {
        return format!("Paris, {} arr.", format_arrondissement(arr));
    }

    trimmed.to_string()
}

fn extract_paris_arrondissement(input: &str) -> Option<u8> {
    let lower = input.to_lowercase();
    if !lower.contains("paris") {
        return None;
    }

    // Ex: 75011 => 11e
    if let Some(code) = lower
        .split(|c: char| !c.is_ascii_digit())
        .find(|chunk| chunk.len() == 5 && chunk.starts_with("75"))
    {
        if let Ok(postal) = code.parse::<u32>() {
            let district = (postal % 100) as u8;
            if (1..=20).contains(&district) {
                return Some(district);
            }
        }
    }

    // Ex: Paris 11e / Paris, 11e arrondissement / Paris XIe
    let token = lower
        .replace([',', '.'], " ")
        .split_whitespace()
        .map(str::to_string)
        .collect::<Vec<_>>();

    for part in token {
        if let Some(n) = parse_arrondissement_token(&part) {
            return Some(n);
        }
    }

    None
}

fn parse_arrondissement_token(token: &str) -> Option<u8> {
    if token.is_empty() {
        return None;
    }

    let digits = token
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>();
    if !digits.is_empty() {
        if let Ok(n) = digits.parse::<u8>() {
            if (1..=20).contains(&n) {
                return Some(n);
            }
        }
    }

    // XIe / xie / xiiie...
    let roman = token
        .chars()
        .take_while(|c| matches!(c.to_ascii_lowercase(), 'i' | 'v' | 'x' | 'l' | 'c' | 'd' | 'm'))
        .collect::<String>();
    if roman.is_empty() {
        return None;
    }
    let n = roman_to_int(&roman)?;
    if (1..=20).contains(&n) {
        return Some(n as u8);
    }
    None
}

fn roman_to_int(roman: &str) -> Option<u32> {
    fn value(c: char) -> Option<u32> {
        match c.to_ascii_uppercase() {
            'I' => Some(1),
            'V' => Some(5),
            'X' => Some(10),
            'L' => Some(50),
            'C' => Some(100),
            'D' => Some(500),
            'M' => Some(1000),
            _ => None,
        }
    }

    let chars = roman.chars().collect::<Vec<_>>();
    if chars.is_empty() {
        return None;
    }

    let mut total = 0u32;
    let mut i = 0usize;
    while i < chars.len() {
        let current = value(chars[i])?;
        let next = if i + 1 < chars.len() {
            value(chars[i + 1])?
        } else {
            0
        };

        if next > current {
            total = total.saturating_add(next.saturating_sub(current));
            i += 2;
        } else {
            total = total.saturating_add(current);
            i += 1;
        }
    }

    Some(total)
}

fn format_arrondissement(n: u8) -> String {
    if n == 1 {
        "1er".to_string()
    } else {
        format!("{n}e")
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
            content.documents.resume_template =
                Some(serde_json::from_value(json!({"foo": "bar"})).unwrap());
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
            notes: DomainJsonValue::Object(Default::default()),
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
            Some(DomainJsonValue::String("persisted:bytea".to_string()))
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
            Some(serde_json::from_value(json!({"foo": "bar"})).unwrap())
        );
        assert_eq!(profil.content.profile.image, "persisted:bytea");
        assert_eq!(
            profil.content.documents.apprenticeship_calendar,
            Some(DomainJsonValue::String("persisted:bytea".to_string()))
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

    #[test]
    fn apply_profile_update_normalizes_paris_location() {
        let mut profil = build_profile(true);
        let new_content = json!({
            "profile": {
                "firstname": "Updated",
                "lastname": "",
                "title": "",
                "offer_type": "",
                "pitch": "",
                "location": "paris 11e arrondissement",
                "phone": "",
                "email": "",
                "linkedin": "",
                "website": "",
                "github": "",
                "image": ""
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
                "resume_template": {"foo": "bar"}
            }
        });

        let _ = apply_profile_update(&mut profil, new_content);

        assert_eq!(profil.content.profile.location, "Paris, 11e arr.");
    }
}
