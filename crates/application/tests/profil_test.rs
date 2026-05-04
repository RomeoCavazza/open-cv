use domain::{Profil, ProfilId};
use chrono::Utc;
use serde_json::json;

#[test]
fn test_profil_serialization_regression() {
    let profil = Profil {
        id: ProfilId::new(),
        label: "Test Profil".into(),
        content: domain::ProfilContent {
            profile: domain::ProfileSection {
                firstname: "John".into(),
                lastname: "Doe".into(),
                image: "data:image/png;base64,...".into(),
                ..Default::default()
            },
            documents: domain::DocumentSection {
                resume_template: Some(json!({"foo": "bar"})),
                ..Default::default()
            },
            ..Default::default()
        },
        is_active: true,
        profile_photo: None,
        calendar_pdf: None,
        resume_template: None,
        cover_letter_template: None,
        notes: json!({}),
        created_at: Utc::now(),
    };

    let serialized = serde_json::to_string(&profil).unwrap();
    let deserialized: Profil = serde_json::from_str(&serialized).unwrap();

    assert_eq!(profil.id, deserialized.id);
    assert_eq!(profil.label, deserialized.label);
    assert_eq!(profil.content.profile.firstname, "John");
}
