//! Binaire serveur Axum.
//!
//! Phase 0 :
//!   GET /health           → 200 OK
//!   GET /api/offres       → liste des offres récentes (DB)
//!   GET /api/instances/:slug → instance par slug
//!   GET /                 → sert web/

use std::sync::Arc;

use anyhow::Context;
use async_trait::async_trait;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use ports::{InstanceRepo, OffreRepo};
use serde::Deserialize;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::info;

mod errors;
mod state;

use crate::errors::ApiError;
use crate::state::AppState;

mod handlers {
    pub mod ingest;
    pub mod profile;
}
use handlers::ingest::ingest_handler;
use handlers::profile::{
    get_active_profile_cover_letter_template_handler, get_active_profile_handler,
    get_active_profile_resume_handler, get_active_profile_resume_template_handler,
    list_profiles_handler, update_active_profile_handler,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();

    init_tracing();

    let database_url =
        std::env::var("DATABASE_URL").context("DATABASE_URL non défini (vois .env.example)")?;

    info!("connexion à Postgres...");
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
    let ollama_model = std::env::var("OLLAMA_MODEL").unwrap_or_else(|_| "llama3.1".into());
    info!("LLM: Ollama activé ({} @ {})", ollama_model, ollama_base);
    llm_map.insert(
        "ollama".to_string(),
        Arc::new(adapter_llm_ollama::OllamaClient::new(
            ollama_base,
            ollama_model,
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

    // Embedder (Mock pour débloquer la compilation sans clé OpenAI/Mistral)
    struct MockEmbedder;
    #[async_trait]
    impl ports::Embedder for MockEmbedder {
        async fn embed(
            &self,
            texts: &[&str],
            _mode: ports::EmbedMode,
        ) -> Result<Vec<Vec<f32>>, ports::EmbedError> {
            Ok(texts
                .iter()
                .map(|text| pseudo_embedding(text, 1024))
                .collect())
        }
        fn dimension(&self) -> usize {
            1024
        }
        fn name(&self) -> &'static str {
            "mock-embedder"
        }
    }

    fn pseudo_embedding(text: &str, dim: usize) -> Vec<f32> {
        use std::hash::{Hash, Hasher};

        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        text.hash(&mut hasher);
        let seed = hasher.finish();

        (0..dim)
            .map(|i| {
                let mixed = seed.wrapping_add(i as u64).wrapping_mul(2654435761);
                ((mixed % 1000) as f32 / 1000.0) - 0.5
            })
            .collect()
    }
    let embedder = Arc::new(MockEmbedder);

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
        generate_uc,
        intake_uc,
        chunk_repo: chunk_repo.clone(),
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
            "/api/profile/active/resume-template",
            get(get_active_profile_resume_template_handler),
        )
        .route(
            "/api/profile/active/cover-letter-template",
            get(get_active_profile_cover_letter_template_handler),
        )
        .route("/api/instances/:slug", get(get_instance_by_slug))
        .route("/api/instances/:slug/resume", get(get_instance_resume))
        .route(
            "/api/instances/:slug/cover-letter",
            get(get_instance_cover_letter),
        )
        .route(
            "/api/instances/:slug/generate",
            axum::routing::post(generate_instance),
        )
        .route("/", get(get_index))
        .route("/applications", get(get_index))
        .route("/applications/:slug", get(get_index))
        .route("/applications/:slug/:tab", get(get_index))
        .route("/profil", get(get_index))
        .fallback_service(ServeDir::new(web_dir))
        .layer(TraceLayer::new_for_http())
        .layer(axum::extract::DefaultBodyLimit::max(20 * 1024 * 1024))
        .with_state(state);

    let bind = std::env::var("BIND").unwrap_or_else(|_| "127.0.0.1:8000".to_string());
    info!("écoute sur http://{}", bind);
    let listener = tokio::net::TcpListener::bind(bind).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn generate_instance(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Query(q): Query<serde_json::Value>, // Temporary way to get query params
) -> Result<StatusCode, ApiError> {
    let slug = domain::Slug::parse(slug).map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let llm_provider = q
        .get("llm_provider")
        .and_then(|v| v.as_str())
        .and_then(|p| state.llm_registry.get(p))
        .cloned();

    // 1. Récupérer l'instance pour avoir l'offre_id et profil_id
    let instance = state
        .instance_repo
        .get_by_slug(&slug)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound(format!("Instance {} inconnue", slug)))?;

    // 2. Lancer la génération
    let input = application::generate::GenerateInput {
        offre_id: instance.offre_id,
        profil_id: instance.profil_id,
        existing_instance: Some(instance),
        livrables: application::generate::Livrables::default(),
    };

    state
        .generate_uc
        .execute(input, llm_provider)
        .await
        .map_err(|e| match e {
            application::generate::GenerateError::AucunChunkPertinent => ApiError::BadRequest(
                "Aucun chunk de profil disponible pour cette génération. Il faut d'abord indexer le profil dans la base."
                    .to_string(),
            ),
            application::generate::GenerateError::AucunLivrable
            | application::generate::GenerateError::Invalide(_) => {
                ApiError::BadRequest(e.to_string())
            }
            _ => ApiError::Internal(e.to_string()),
        })?;

    Ok(StatusCode::ACCEPTED)
}

fn init_tracing() {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(fmt::layer().compact())
        .init();
}

// ─────────────────────────────────────────────────────────────────
// Handlers
// ─────────────────────────────────────────────────────────────────

async fn health() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}

#[derive(Debug, Deserialize)]
struct ListOffresQuery {
    #[serde(default = "default_limit")]
    limit: u32,
}

fn default_limit() -> u32 {
    50
}

fn infer_business_category(slug: &str, title: &str) -> &'static str {
    let haystack = format!("{} {}", slug.to_lowercase(), title.to_lowercase());

    if [
        "data",
        " ai",
        "ia",
        "intelligence artificielle",
        "llm",
        "langchain",
        "gallica",
        "automation",
        "scientist",
        "machine learning",
    ]
    .iter()
    .any(|needle| haystack.contains(needle))
    {
        return "Data Engineering & Data Science";
    }

    if [
        "developpeur",
        "développeur",
        "software",
        "java",
        "api",
        "logiciel",
        "full stack",
        "full-stack",
        "embarqu",
        "engineering",
    ]
    .iter()
    .any(|needle| haystack.contains(needle))
    {
        return "Ingénierie Logicielle Spécialisée (Embarqué, C++, Simulations, Systèmes)";
    }

    if [
        "pilotage",
        "projet",
        "transformation",
        "strategie",
        "stratégie",
    ]
    .iter()
    .any(|needle| haystack.contains(needle))
    {
        return "Pilotage de Projet, Stratégie IT & Transformation Numérique";
    }

    "Autres"
}

fn public_offer_category(slug: &str, title: &str, raw: Option<&str>) -> String {
    let category = raw.unwrap_or("").trim();
    if category.is_empty()
        || category.eq_ignore_ascii_case("inbox")
        || category.eq_ignore_ascii_case("legacy restored")
    {
        infer_business_category(slug, title).to_string()
    } else {
        category.to_string()
    }
}

async fn list_offres(
    State(state): State<AppState>,
    Query(q): Query<ListOffresQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let rows = sqlx::query!(
        r#"
        SELECT o.id, o.slug, o.intitule, o.source_url, o.entreprise, o.categorie,
               EXISTS(SELECT 1 FROM instances i WHERE i.offre_id = o.id) as "has_instance!"
        FROM offres o
        ORDER BY o.scraped_at DESC
        LIMIT $1
        "#,
        q.limit as i64
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;

    let entries: Vec<serde_json::Value> = rows
        .iter()
        .map(|r| {
            serde_json::json!({
                "title": r.intitule,
                "url": r.source_url,
                "job_id": r.slug,
                "entreprise": r.entreprise,
                "category": public_offer_category(&r.slug, &r.intitule, r.categorie.as_deref()),
                "status": if r.has_instance { "ready" } else { "draft" },
            })
        })
        .collect();

    Ok(Json(serde_json::json!({
        "entries": entries,
    })))
}

async fn get_offre_by_slug(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<domain::Offre>, ApiError> {
    let slug = domain::Slug::parse(slug).map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let usecase = application::GetOffreBySlugUseCase::new(state.offre_repo.clone());
    let offre = usecase.execute(&slug).await?;

    Ok(Json(offre))
}

async fn get_instance_by_slug(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let slug = domain::Slug::parse(slug).map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let usecase = application::GetInstanceBySlugUseCase::new(state.instance_repo.clone());
    let instance = usecase.execute(&slug).await?;

    Ok(Json(serde_json::to_value(&instance)?))
}

async fn get_instance_by_offre_slug(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let slug = domain::Slug::parse(slug).map_err(|e| ApiError::BadRequest(e.to_string()))?;

    // 1. Trouver l'offre
    let offre = state
        .offre_repo
        .get_by_slug(&slug)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound(format!("Offre {} inconnue", slug)))?;

    // 2. Trouver l'instance via le repo
    let instance = state
        .instance_repo
        .get_by_offre_id(offre.id)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound(format!("Pas d'instance pour l'offre {}", slug)))?;

    Ok(Json(serde_json::to_value(&instance)?))
}

async fn get_instance_resume(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let slug = domain::Slug::parse(slug).map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let usecase = application::GetInstanceBySlugUseCase::new(state.instance_repo.clone());
    let instance = usecase.execute(&slug).await?;

    match instance.resume_json {
        Some(json) => Ok(Json(json)),
        None => Err(ApiError::NotFound(format!("Pas de CV pour {}", slug))),
    }
}

async fn get_instance_cover_letter(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let slug = domain::Slug::parse(slug).map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let usecase = application::GetInstanceBySlugUseCase::new(state.instance_repo.clone());
    let instance = usecase.execute(&slug).await?;

    match instance.cover_letter_json {
        Some(json) => Ok(Json(json)),
        None => Err(ApiError::NotFound(format!("Pas de lettre pour {}", slug))),
    }
}
async fn get_index() -> impl IntoResponse {
    let web_dir = std::env::var("WEB_DIR").unwrap_or_else(|_| "web".to_string());
    match tokio::fs::read_to_string(format!("{}/index.html", web_dir)).await {
        Ok(html) => (StatusCode::OK, axum::response::Html(html)).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "index.html non trouvé").into_response(),
    }
}
async fn chat_handler(
    State(state): State<AppState>,
    Json(req): Json<application::chat::ChatRequest>,
) -> Result<Json<application::chat::ChatResponse>, ApiError> {
    let usecase = application::chat::ChatWithApplicationUseCase::new(
        state.instance_repo.clone(),
        state.chunk_repo.clone(),
        state.embedder.clone(),
        state.llm_registry.as_ref().clone(),
    );

    let res = usecase
        .execute(req)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(res))
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
