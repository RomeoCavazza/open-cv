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

    // LLM Client (Anthropic)
    let anthropic_key =
        std::env::var("ANTHROPIC_API_KEY").context("ANTHROPIC_API_KEY non défini")?;
    let claude_client = Arc::new(adapter_llm_claude::ClaudeClient::new(anthropic_key));

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
        claude_client.clone(),
        embedder.clone(),
        event_bus.clone(),
    ));

    let state = AppState {
        offre_repo,
        instance_repo,
        generate_uc,
    };

    let web_dir = std::env::var("WEB_DIR").unwrap_or_else(|_| "web".to_string());

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/offres", get(list_offres))
        .route("/api/offres/:slug", get(get_offre_by_slug))
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
        .fallback_service(ServeDir::new(web_dir))
        .layer(TraceLayer::new_for_http())
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
) -> Result<StatusCode, ApiError> {
    let slug = domain::Slug::parse(slug).map_err(|e| ApiError::BadRequest(e.to_string()))?;

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
        .execute(input)
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

async fn list_offres(
    State(state): State<AppState>,
    Query(q): Query<ListOffresQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let usecase = application::ListOffresUseCase::new(state.offre_repo.clone());
    let offres = usecase.execute(q.limit).await?;

    // Format compatible avec l'ancien liste.json pour le frontend
    let entries: Vec<serde_json::Value> = offres
        .iter()
        .map(|o| {
            serde_json::json!({
                "title": o.intitule,
                "url": o.source_url,
                "job_id": o.slug.as_str(),
                "category": o.categorie.as_deref().unwrap_or("Autres"),
                "status": "draft",
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
