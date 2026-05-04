//! Binaire serveur Axum.

use std::sync::Arc;

use anyhow::Context;
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use ports::{InstanceRepo, OffreRepo};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::info;

mod errors;
mod state;

use crate::state::AppState;

mod handlers;

use handlers::chat::chat_handler;
use handlers::ingest::ingest_handler;
use handlers::instance::{
    generate_instance, get_instance_by_offre_slug, get_instance_by_slug, get_instance_cover_letter,
    get_instance_resume,
};
use handlers::offre::{get_offre_by_slug, list_offres};
use handlers::profile::{
    delete_annexe_handler, download_annexe_handler,
    get_active_profile_cover_letter_template_handler, get_active_profile_handler,
    get_active_profile_resume_handler, get_active_profile_resume_template_handler,
    list_annexes_handler, list_profiles_handler, update_active_profile_handler,
    upload_annexe_handler,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();

    init_tracing();

    let database_url =
        std::env::var("DATABASE_URL").context("DATABASE_URL non défini (vois .env.example)")?;

    info!("connexion à Postgres...");
    let pool = adapter_postgres::connect(&database_url)
        .await
        .context("connexion Postgres impossible")?;

    info!("application des migrations...");
    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .context("migrations échouées")?;

    let offre_repo: Arc<dyn OffreRepo> = Arc::new(adapter_postgres::OffreRepoPg::new(pool.clone()));
    let instance_repo: Arc<dyn InstanceRepo> =
        Arc::new(adapter_postgres::InstanceRepoPg::new(pool.clone()));

    let profil_repo: Arc<dyn ports::ProfilRepo> =
        Arc::new(adapter_postgres::ProfilRepoPg::new(pool.clone()));
    let chunk_repo: Arc<dyn ports::ChunkRepo> =
        Arc::new(adapter_postgres::ChunkRepoPg::new(pool.clone()));
    let annexe_repo: Arc<dyn ports::AnnexeRepo> =
        Arc::new(adapter_postgres::AnnexeRepoPg::new(pool.clone()));
    let message_repo: Arc<dyn ports::MessageRepo> =
        Arc::new(adapter_postgres::MessageRepoPg::new(pool.clone()));

    // LLM Registry (Multiple providers)
    let mut llm_map: std::collections::HashMap<String, Arc<dyn ports::LlmClient>> =
        std::collections::HashMap::new();

    // Anthropic
    if let Ok(key) = std::env::var("ANTHROPIC_API_KEY") {
        if !key.is_empty() {
            info!("LLM: Anthropic (Claude) activé");
            llm_map.insert(
                "claude".to_string(),
                Arc::new(adapter_llm_claude::ClaudeClient::new(key)),
            );
        }
    }

    // OpenAI
    if let Ok(key) = std::env::var("OPENAI_API_KEY") {
        if !key.is_empty() {
            info!("LLM: OpenAI activé");
            llm_map.insert(
                "openai".to_string(),
                Arc::new(adapter_llm_openai::OpenAiClient::new(key)),
            );
        }
    }

    // Ollama
    let ollama_base =
        std::env::var("OLLAMA_BASE_URL").unwrap_or_else(|_| "http://localhost:11434".into());
    let ollama_model = std::env::var("OLLAMA_MODEL").unwrap_or_else(|_| "qwen2.5:7b".into());
    info!("LLM: Ollama activé ({} @ {})", ollama_model, ollama_base);
    llm_map.insert(
        "ollama".to_string(),
        Arc::new(adapter_llm_ollama::OllamaClient::new(
            ollama_base,
            ollama_model,
            4096, // Qwen 2.5:7b default
        )),
    );

    let llm_registry = Arc::new(llm_map);

    // Default LLM for UseCases (if none specified)
    let default_llm = llm_registry
        .get("ollama")
        .or_else(|| llm_registry.get("claude"))
        .or_else(|| llm_registry.get("openai"))
        .cloned()
        .context("Aucun provider LLM disponible")?;

    // Embedder (Utilise Ollama avec mxbai-embed-large en local)
    let embed_base =
        std::env::var("OLLAMA_BASE_URL").unwrap_or_else(|_| "http://localhost:11434".into());
    let embed_model =
        std::env::var("OLLAMA_EMBED_MODEL").unwrap_or_else(|_| "mxbai-embed-large".into());
    info!("Embedder: Ollama activé ({} @ {})", embed_model, embed_base);

    let embedder: Arc<dyn ports::Embedder> = Arc::new(adapter_llm_ollama::OllamaClient::new(
        embed_base,
        embed_model,
        1024, // mxbai-embed-large dimension
    ));

    // Event Bus (Simple in-memory pour l'instant)
    let event_bus = Arc::new(application::events::EventBus::new());

    // Generate Use Case
    let generate_uc = Arc::new(application::generate::GenerateApplicationUseCase::new(
        offre_repo.clone(),
        profil_repo.clone(),
        chunk_repo.clone(),
        instance_repo.clone(),
        default_llm.clone(),
        embedder.clone(),
        event_bus.clone(),
    ));

    // Intake Use Case
    let scraper: Arc<dyn ports::Scraper> = Arc::new(adapter_scraper_http::HttpScraper::new());
    let intake_uc = Arc::new(application::intake::IntakeOffreUseCase::new(
        offre_repo.clone(),
        instance_repo.clone(),
        profil_repo.clone(),
        default_llm.clone(),
        scraper,
    ));

    let state = AppState {
        pool: pool.clone(),
        offre_repo,
        instance_repo,
        profil_repo: profil_repo.clone(),
        generate_uc,
        intake_uc,
        chunk_repo: chunk_repo.clone(),
        annexe_repo: annexe_repo.clone(),
        message_repo: message_repo.clone(),
        embedder: embedder.clone(),
        llm_registry,
    };

    let web_dir = std::env::var("WEB_DIR").unwrap_or_else(|_| "web".to_string());

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/offres", get(list_offres))
        .route("/api/offres/:slug", get(get_offre_by_slug))
        .route(
            "/api/offres/:slug/instance",
            get(get_instance_by_offre_slug),
        )
        .route("/api/chat", post(chat_handler))
        .route("/api/ingest", post(ingest_handler))
        .route("/api/profiles", get(list_profiles_handler))
        .route(
            "/api/profile/active",
            get(get_active_profile_handler).put(update_active_profile_handler),
        )
        .route(
            "/api/profile/active/resume",
            get(get_active_profile_resume_handler),
        )
        .route(
            "/api/profile/active/calendar",
            get(get_active_profile_calendar_handler),
        )
        .route(
            "/api/profile/active/photo",
            get(get_active_profile_photo_handler),
        )
        .route(
            "/api/profile/active/resume-template",
            get(get_active_profile_resume_template_handler),
        )
        .route(
            "/api/profile/active/cover-letter-template",
            get(get_active_profile_cover_letter_template_handler),
        )
        .route(
            "/api/profile/active/annexes",
            get(list_annexes_handler).post(upload_annexe_handler),
        )
        .route(
            "/api/profile/active/annexes/:id",
            get(download_annexe_handler).delete(delete_annexe_handler),
        )
        .route("/api/instances/:slug", get(get_instance_by_slug))
        .route("/api/instances/:slug/resume", get(get_instance_resume))
        .route(
            "/api/instances/:slug/cover-letter",
            get(get_instance_cover_letter),
        )
        .route(
            "/api/instances/:slug/generate",
            post(generate_instance),
        )
        .route("/", get(get_index))
        .route("/applications", get(get_index))
        .route("/applications/:slug", get(get_index))
        .route("/applications/:slug/:tab", get(get_index))
        .route("/profil", get(get_index))
        .fallback_service(ServeDir::new(web_dir))
        .layer(TraceLayer::new_for_http())
        .layer(axum::extract::DefaultBodyLimit::max(50 * 1024 * 1024))
        .with_state(state);

    let bind = std::env::var("BIND").unwrap_or_else(|_| "127.0.0.1:8000".to_string());
    info!("écoute sur http://{}", bind);
    let listener = tokio::net::TcpListener::bind(bind).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn init_tracing() {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(fmt::layer().compact())
        .init();
}

// ─────────────────────────────────────────────────────────────────
// Static Handlers
// ─────────────────────────────────────────────────────────────────

async fn health() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}

async fn get_index() -> impl IntoResponse {
    let web_dir = std::env::var("WEB_DIR").unwrap_or_else(|_| "web".to_string());
    match tokio::fs::read_to_string(format!("{}/index.html", web_dir)).await {
        Ok(html) => (StatusCode::OK, axum::response::Html(html)).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "index.html non trouvé").into_response(),
    }
}

async fn get_active_profile_calendar_handler(
    State(state): State<AppState>,
) -> impl axum::response::IntoResponse {
    let row = sqlx::query!("SELECT calendar_pdf FROM profils WHERE is_active = true LIMIT 1")
        .fetch_optional(&state.pool)
        .await;

    match row {
        Ok(Some(r)) => {
            if let Some(bytes) = r.calendar_pdf {
                (
                    [(axum::http::header::CONTENT_TYPE, "application/pdf")],
                    bytes,
                )
                    .into_response()
            } else {
                (
                    axum::http::StatusCode::NOT_FOUND,
                    "Aucun calendrier configuré",
                )
                    .into_response()
            }
        }
        _ => (axum::http::StatusCode::NOT_FOUND, "Profil introuvable").into_response(),
    }
}

async fn get_active_profile_photo_handler(
    State(state): State<AppState>,
) -> impl axum::response::IntoResponse {
    let row = sqlx::query!("SELECT profile_photo FROM profils WHERE is_active = true LIMIT 1")
        .fetch_optional(&state.pool)
        .await;

    match row {
        Ok(Some(r)) => {
            if let Some(bytes) = r.profile_photo {
                ([(axum::http::header::CONTENT_TYPE, "image/jpeg")], bytes).into_response()
            } else {
                (axum::http::StatusCode::NOT_FOUND, "Aucune photo configurée").into_response()
            }
        }
        _ => (axum::http::StatusCode::NOT_FOUND, "Profil introuvable").into_response(),
    }
}
