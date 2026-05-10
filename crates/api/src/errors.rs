use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
    NotFound(String),
    TooManyRequests(String),
    Internal(String),
}

impl From<application::AppError> for ApiError {
    fn from(e: application::AppError) -> Self {
        match e {
            application::AppError::NotFound => Self::NotFound("ressource introuvable".into()),
            application::AppError::Validation(m) => Self::BadRequest(m),
            other => Self::Internal(other.to_string()),
        }
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(e: serde_json::Error) -> Self {
        Self::Internal(e.to_string())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::BadRequest(m) => (StatusCode::BAD_REQUEST, m),
            Self::NotFound(m) => (StatusCode::NOT_FOUND, m),
            Self::TooManyRequests(m) => (StatusCode::TOO_MANY_REQUESTS, m),
            Self::Internal(m) => (StatusCode::INTERNAL_SERVER_ERROR, m),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}
