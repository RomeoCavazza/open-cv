//! Binaire serveur Axum.
//!
//! Phase 0 :
//!   GET /health           → 200 OK
//!   GET /api/offres       → liste des offres récentes (DB)
//!   GET /api/instances/:slug → instance par slug
//!   GET /                 → sert web/

use std::sync::Arc;

use anyhow::Context;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use ports::{InstanceRepo, OffreRepo};
use serde::Deserialize;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::info;

mod state;
mod errors;

use crate::errors::ApiError;
use crate::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();

    init_tracing();

    let database_url = std::env::var("DATABASE_URL")
        .context("DATABASE_URL non défini (vois .env.example)")?;

    info!("connexion à Postgres...");
    let pool = adapter_postgres::connect(&database_url)
        .await
        .context("connexion Postgres impossible")?;

    info!("application des migrations...");
    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .context("migrations échouées")?;

    let offre_repo: Arc<dyn OffreRepo> =
        Arc::new(adapter_postgres::OffreRepoPg::new(pool.clone()));
    let instance_repo: Arc<dyn InstanceRepo> =
        Arc::new(adapter_postgres::InstanceRepoPg::new(pool.clone()));

    let state = AppState {
        offre_repo,
        instance_repo,
    };

    let web_dir = std::env::var("WEB_DIR").unwrap_or_else(|_| "web".to_string());
    info!(web_dir = %web_dir, "front statique");

    // Routes API d'abord ; ServeDir en fallback pour le front statique.
    // Sans ça, `nest_service("/", ...)` capturerait /health et /api/*.
    let app = Router::new()
        .route("/health", get(health))
        .route("/api/offres", get(list_offres))
        .route("/api/instances/:slug", get(get_instance_by_slug))
        .with_state(state)
        .fallback_service(ServeDir::new(&web_dir))
        .layer(TraceLayer::new_for_http());

    let bind = std::env::var("BIND").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    info!("écoute sur http://{bind}");

    let listener = tokio::net::TcpListener::bind(&bind).await?;
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
    20
}

async fn list_offres(
    State(state): State<AppState>,
    Query(q): Query<ListOffresQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let usecase = application::ListOffresUseCase::new(state.offre_repo.clone());
    let offres = usecase.execute(q.limit).await?;

    Ok(Json(serde_json::json!({
        "count": offres.len(),
        "offres": offres,
    })))
}

async fn get_instance_by_slug(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let slug = domain::Slug::parse(slug)
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let usecase = application::GetInstanceBySlugUseCase::new(state.instance_repo.clone());
    let instance = usecase.execute(&slug).await?;

    Ok(Json(serde_json::to_value(&instance)?))
}
