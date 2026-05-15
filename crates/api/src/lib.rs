use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

pub mod errors;
pub(crate) mod handlers;
pub mod state;

use crate::handlers::{
    chat::{
        chat_handler, chat_stream_handler, list_snapshots_handler, restore_snapshot_handler,
        undo_handler,
    },
    ingest::ingest_handler,
    instance::{
        generate_instance, get_instance_by_offre_slug, get_instance_by_slug,
        get_instance_cover_letter, get_instance_resume,
    },
    offre::{get_offre_by_slug, list_offres},
    profile::{
        delete_annexe_handler, download_annexe_handler, get_active_profile_calendar_handler,
        get_active_profile_cover_letter_template_handler, get_active_profile_handler,
        get_active_profile_photo_handler, get_active_profile_resume_handler,
        get_active_profile_resume_template_handler, list_annexes_handler, list_profiles_handler,
        update_active_profile_handler, upload_annexe_handler,
    },
};
use crate::state::AppState;

pub fn create_app(state: AppState) -> Router {
    let web_dir = std::env::var("WEB_DIR").unwrap_or_else(|_| "web".to_string());

    Router::new()
        .route("/health", get(health))
        .route("/api/offres", get(list_offres))
        .route("/api/offres/:slug", get(get_offre_by_slug))
        .route(
            "/api/offres/:slug/instance",
            get(get_instance_by_offre_slug),
        )
        .route("/api/instances/:slug", get(get_instance_by_slug))
        .route("/api/instances/:slug/resume", get(get_instance_resume))
        .route(
            "/api/instances/:slug/cover-letter",
            get(get_instance_cover_letter),
        )
        .route("/api/instances/:slug/generate", post(generate_instance))
        .route("/api/profile/active", get(get_active_profile_handler))
        .route("/api/profile/active", put(update_active_profile_handler))
        .route(
            "/api/profile/active/resume",
            get(get_active_profile_resume_handler),
        )
        .route(
            "/api/profile/active/resume/template",
            get(get_active_profile_resume_template_handler),
        )
        .route(
            "/api/profile/active/cover-letter/template",
            get(get_active_profile_cover_letter_template_handler),
        )
        .route(
            "/api/profile/active/calendar",
            get(get_active_profile_calendar_handler),
        )
        .route(
            "/api/profile/active/photo",
            get(get_active_profile_photo_handler),
        )
        .route("/api/profiles", get(list_profiles_handler))
        .route("/api/profile/active/annexes", get(list_annexes_handler))
        .route("/api/profile/active/annexes", post(upload_annexe_handler))
        .route(
            "/api/profile/active/annexes/:id",
            get(download_annexe_handler),
        )
        .route(
            "/api/profile/active/annexes/:id",
            delete(delete_annexe_handler),
        )
        .route("/api/chat", post(chat_handler))
        .route("/api/chat/stream", post(chat_stream_handler))
        .route("/api/chat/undo", post(undo_handler))
        .route("/api/instances/:id/snapshots", get(list_snapshots_handler))
        .route("/api/instances/:id/restore", post(restore_snapshot_handler))
        .route("/api/ingest", post(ingest_handler))
        .nest_service(
            "/assets",
            tower_http::services::ServeDir::new(format!("{}/assets", web_dir)),
        )
        .nest_service(
            "/restitution",
            tower_http::services::ServeDir::new(format!("{}/restitution", web_dir)),
        )
        .nest_service(
            "/resume",
            tower_http::services::ServeDir::new(format!("{}/resume", web_dir)),
        )
        .nest_service(
            "/cover-letter",
            tower_http::services::ServeDir::new(format!("{}/cover-letter", web_dir)),
        )
        .fallback(get(get_index))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .layer(axum::extract::DefaultBodyLimit::max(10 * 1024 * 1024))
}

async fn health() -> impl IntoResponse {
    StatusCode::OK
}

async fn get_index() -> impl IntoResponse {
    let index_path = std::env::var("WEB_DIR").unwrap_or_else(|_| "web".to_string()) + "/index.html";
    match tokio::fs::read_to_string(index_path).await {
        Ok(html) => (StatusCode::OK, axum::response::Html(html)).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "index.html non trouvé").into_response(),
    }
}
