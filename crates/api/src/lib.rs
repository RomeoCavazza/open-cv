use axum::{
    extract::State,
    routing::{get, post, put},
    Router,
};
use axum::response::IntoResponse;
use axum::http::StatusCode;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

pub mod errors;
pub(crate) mod handlers;
pub mod state;

use crate::state::AppState;
use crate::handlers::{
    chat::chat_handler,
    ingest::ingest_handler,
    instance::{get_instance_by_offre_slug, get_instance_by_slug},
    offre::{get_offre_by_slug, list_offres},
    profile::{
        get_active_profile_cover_letter_template_handler, get_active_profile_handler,
        get_active_profile_resume_handler, get_active_profile_resume_template_handler,
        list_annexes_handler, list_profiles_handler, update_active_profile_handler,
    },
};

pub fn create_app(state: AppState) -> Router {
    let web_dir = std::env::var("WEB_DIR").unwrap_or_else(|_| "web".to_string());
    
    Router::new()
        .route("/health", get(health))
        .route("/api/offres", get(list_offres))
        .route("/api/offres/:slug", get(get_offre_by_slug))
        .route("/api/offres/:slug/instance", get(get_instance_by_offre_slug))
        .route("/api/instances/:slug", get(get_instance_by_slug))
        .route("/api/profile/active", get(get_active_profile_handler))
        .route("/api/profile/active", put(update_active_profile_handler))
        .route("/api/profile/active/resume", get(get_active_profile_resume_handler))
        .route("/api/profile/active/resume/template", get(get_active_profile_resume_template_handler))
        .route("/api/profile/active/cover-letter/template", get(get_active_profile_cover_letter_template_handler))
        .route("/api/profile/active/calendar", get(get_active_profile_calendar_handler))
        .route("/api/profile/active/photo", get(get_active_profile_photo_handler))
        .route("/api/profiles", get(list_profiles_handler))
        .route("/api/profile/active/annexes", get(list_annexes_handler))
        .route("/api/chat", post(chat_handler))
        .route("/api/ingest", post(ingest_handler))
        .nest_service("/assets", tower_http::services::ServeDir::new(format!("{}/assets", web_dir)))
        .fallback(get(get_index))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
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

async fn get_active_profile_calendar_handler(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let row = sqlx::query!("SELECT calendar_pdf FROM profils WHERE is_active = true LIMIT 1")
        .fetch_optional(&state.pool)
        .await;

    match row {
        Ok(Some(r)) => {
            if let Some(bytes) = r.calendar_pdf {
                ([(axum::http::header::CONTENT_TYPE, "application/pdf")], bytes).into_response()
            } else {
                (StatusCode::NOT_FOUND, "Aucun calendrier configuré").into_response()
            }
        }
        _ => (StatusCode::NOT_FOUND, "Profil introuvable").into_response(),
    }
}

async fn get_active_profile_photo_handler(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let row = sqlx::query!("SELECT profile_photo FROM profils WHERE is_active = true LIMIT 1")
        .fetch_optional(&state.pool)
        .await;

    match row {
        Ok(Some(r)) => {
            if let Some(bytes) = r.profile_photo {
                ([(axum::http::header::CONTENT_TYPE, "image/jpeg")], bytes).into_response()
            } else {
                (StatusCode::NOT_FOUND, "Aucune photo configurée").into_response()
            }
        }
        _ => (StatusCode::NOT_FOUND, "Profil introuvable").into_response(),
    }
}
