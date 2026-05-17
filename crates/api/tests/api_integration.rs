mod common;

use api::create_app;
use api::state::AppState;
use axum_test::TestServer;
use common::{MockEmbedder, MockLlm, MockRepos, MockScraper};
use domain::{Profil, ProfilContent, ProfilId, ProfileSection};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Semaphore;

async fn setup_test_server() -> (TestServer, Arc<MockRepos>) {
    let mock_repos = Arc::new(MockRepos::new());
    let mock_llm = Arc::new(MockLlm);
    let mock_embedder = Arc::new(MockEmbedder);
    let mock_scraper = Arc::new(MockScraper);
    let event_bus = Arc::new(application::events::EventBus::new());

    let generate_uc = Arc::new(application::generate::GenerateApplicationUseCase::new(
        mock_repos.clone(),
        mock_repos.clone(),
        mock_repos.clone(),
        mock_repos.clone(),
        mock_llm.clone(),
        mock_embedder.clone(),
        event_bus.clone(),
    ));

    let intake_uc = Arc::new(application::intake::IntakeOffreUseCase::new(
        mock_repos.clone(),
        mock_repos.clone(),
        mock_repos.clone(),
        mock_llm.clone(),
        mock_scraper,
    ));

    let mut llm_registry = HashMap::new();
    llm_registry.insert("mock".to_string(), mock_llm as Arc<dyn ports::LlmClient>);

    let state = AppState {
        pool: sqlx::PgPool::connect_lazy("postgres://localhost/unused").unwrap(),
        offre_repo: mock_repos.clone(),
        instance_repo: mock_repos.clone(),
        profil_repo: mock_repos.clone(),
        generate_uc,
        intake_uc,
        chunk_repo: mock_repos.clone(),
        annexe_repo: mock_repos.clone(),
        message_repo: mock_repos.clone(),
        snapshot_repo: mock_repos.clone(),
        embedder: mock_embedder.clone(),
        llm_registry: Arc::new(llm_registry),
        generation_slots: Arc::new(Semaphore::new(2)),
        generation_queue: Arc::new(Semaphore::new(8)),
    };

    let app = create_app(state);
    // Dans axum-test 15.x, TestServer::new prend le Router directement.
    let server = TestServer::new(app).expect("Failed to create test server");
    (server, mock_repos)
}

#[tokio::test]
async fn test_get_profile_404_when_empty() {
    let (server, _) = setup_test_server().await;
    let response = server.get("/api/profile/active").await;
    response.assert_status_not_found();
}

#[tokio::test]
async fn test_get_profile_200_when_seeded() {
    let (server, repos) = setup_test_server().await;

    let profil = Profil {
        id: ProfilId::new(),
        label: "Test Profile".to_string(),
        is_active: true,
        content: ProfilContent {
            profile: ProfileSection {
                firstname: "John".to_string(),
                lastname: "Doe".to_string(),
                ..Default::default()
            },
            ..Default::default()
        },
        created_at: chrono::Utc::now(),
        profile_photo: None,
        calendar_pdf: None,
        resume_template: None,
        cover_letter_template: None,
        notes: domain::JsonValue::Object(Default::default()),
    };
    repos
        .profils
        .lock()
        .unwrap()
        .insert(profil.id, profil.clone());

    let response = server.get("/api/profile/active").await;
    response.assert_status_success();

    let body = response.json::<Profil>();
    assert_eq!(body.content.profile.firstname, "John");
}

#[tokio::test]
async fn test_put_profile_200_updates_content() {
    let (server, repos) = setup_test_server().await;

    let id = ProfilId::new();
    let profil = Profil {
        id,
        label: "Initial".to_string(),
        is_active: true,
        content: ProfilContent::default(),
        created_at: chrono::Utc::now(),
        profile_photo: None,
        calendar_pdf: None,
        resume_template: None,
        cover_letter_template: None,
        notes: domain::JsonValue::Object(Default::default()),
    };
    repos.profils.lock().unwrap().insert(id, profil);

    let new_content = ProfilContent {
        profile: ProfileSection {
            firstname: "Updated".to_string(),
            ..Default::default()
        },
        ..Default::default()
    };

    let response = server.put("/api/profile/active").json(&new_content).await;
    response.assert_status_success();

    // Verify in mock repo
    let updated = repos.profils.lock().unwrap().get(&id).unwrap().clone();
    assert_eq!(updated.content.profile.firstname, "Updated");
}

#[tokio::test]
async fn test_put_profile_400_on_malformed_payload() {
    let (server, repos) = setup_test_server().await;

    let profil = Profil {
        id: ProfilId::new(),
        label: "Test".to_string(),
        is_active: true,
        content: ProfilContent::default(),
        created_at: chrono::Utc::now(),
        profile_photo: None,
        calendar_pdf: None,
        resume_template: None,
        cover_letter_template: None,
        notes: domain::JsonValue::Object(Default::default()),
    };
    repos.profils.lock().unwrap().insert(profil.id, profil);

    // Malformed JSON (firstname should be a string, not a number)
    let malformed = serde_json::json!({
        "profile": {
            "firstname": 12345
        }
    });

    let response = server.put("/api/profile/active").json(&malformed).await;
    response.assert_status_bad_request();

    let text = response.text();
    assert!(text.contains("Invalid profile payload"));
}

#[tokio::test]
async fn test_post_profile_active_405() {
    let (server, _) = setup_test_server().await;
    let response = server
        .post("/api/profile/active")
        .json(&serde_json::json!({}))
        .await;
    response.assert_status(axum::http::StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn test_post_ingest_200_with_restitution() {
    let (server, repos) = setup_test_server().await;

    let profil = Profil {
        id: ProfilId::new(),
        label: "Test".to_string(),
        is_active: true,
        content: ProfilContent::default(),
        created_at: chrono::Utc::now(),
        profile_photo: None,
        calendar_pdf: None,
        resume_template: None,
        cover_letter_template: None,
        notes: domain::JsonValue::Object(Default::default()),
    };
    repos.profils.lock().unwrap().insert(profil.id, profil);

    let payload = serde_json::json!({
        "input": "http://job-offer.com",
        "config": {
            "restitution": true,
            "resume": false,
            "cover": false
        }
    });

    let response = server.post("/api/ingest").json(&payload).await;
    println!("RESPONSE: {}", response.text());
    response.assert_status_success();
}

#[tokio::test]
async fn test_post_ingest_400_with_legacy_analysis_field() {
    let (server, _) = setup_test_server().await;

    let payload = serde_json::json!({
        "input": "http://job-offer.com",
        "config": {
            "analysis": true,
            "resume": false,
            "cover": false
        }
    });

    let response = server.post("/api/ingest").json(&payload).await;
    assert!(response.status_code() == 400 || response.status_code() == 422);
}

#[tokio::test]
async fn test_post_ingest_200_with_more_than_5_urls_truncates_input() {
    let (server, repos) = setup_test_server().await;

    let profil = Profil {
        id: ProfilId::new(),
        label: "Test".to_string(),
        is_active: true,
        content: ProfilContent::default(),
        created_at: chrono::Utc::now(),
        profile_photo: None,
        calendar_pdf: None,
        resume_template: None,
        cover_letter_template: None,
        notes: domain::JsonValue::Object(Default::default()),
    };
    repos.profils.lock().unwrap().insert(profil.id, profil);

    let input = (1..=6)
        .map(|i| format!("https://example.com/job-{i}"))
        .collect::<Vec<_>>()
        .join("\n");
    let payload = serde_json::json!({ "input": input });

    let response = server.post("/api/ingest").json(&payload).await;
    response.assert_status_success();
    let body = response.json::<serde_json::Value>();
    assert_eq!(body["queue_limit"].as_u64(), Some(5));
    assert_eq!(body["rejected_count"].as_u64(), Some(1));
    assert_eq!(body["accepted_count"].as_u64(), Some(5));
    assert_eq!(body["items"].as_array().map(|v| v.len()), Some(5));
}

#[tokio::test]
async fn test_get_annexes_200() {
    let (server, repos) = setup_test_server().await;

    // Seed active profile
    let profil = Profil {
        id: ProfilId::new(),
        label: "Test".to_string(),
        is_active: true,
        content: ProfilContent::default(),
        created_at: chrono::Utc::now(),
        profile_photo: None,
        calendar_pdf: None,
        resume_template: None,
        cover_letter_template: None,
        notes: domain::JsonValue::Object(Default::default()),
    };
    repos.profils.lock().unwrap().insert(profil.id, profil);

    let response = server.get("/api/profile/active/annexes").await;
    response.assert_status_success();
}
