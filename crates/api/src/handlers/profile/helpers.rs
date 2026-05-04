use base64::{engine::general_purpose, Engine as _};
use domain::Profil;
use serde_json::Value as JsonValue;

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
    use domain::ProfilId;
    use serde_json::json;

    fn build_profile() -> Profil {
        Profil {
            id: ProfilId::new(),
            label: "Test".into(),
            content: json!({
                "profile": {},
                "documents": {"resume_template": {"foo": "bar"}},
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
    fn apply_persisted_markers_marks_binary_fields() {
        let mut profil = build_profile();
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
        let mut profil = build_profile();
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
}