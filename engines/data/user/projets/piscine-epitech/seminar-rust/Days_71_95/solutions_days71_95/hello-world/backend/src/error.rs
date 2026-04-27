use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize, Error)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    #[error("Missing authorization header")]
    AuthFailNoAuthHeader,
    #[error("Invalid token")]
    AuthFailInvalidToken,
    #[error("Token expired")]
    AuthFailTokenExpired,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Email already exists")]
    EmailAlreadyExists,
    #[error("User not found")]
    UserNotFound,
    #[error("Server not found")]
    ServerNotFound,
    #[error("Server access forbidden")]
    ServerForbidden,
    #[error("Owner cannot leave server")]
    ServerOwnerCannotLeave,
    #[error("Already a member")]
    ServerAlreadyMember,
    #[error("Channel not found")]
    ChannelNotFound,
    #[error("Channel access forbidden")]
    ChannelForbidden,
    #[error("Message not found")]
    MessageNotFound,
    #[error("Message access forbidden")]
    MessageForbidden,
    #[error("Database error: {message}")]
    DatabaseError { message: String },
    #[error("Internal error: {message}")]
    InternalError { message: String },
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Error::DatabaseError {
            message: err.to_string(),
        }
    }
}

impl From<bcrypt::BcryptError> for Error {
    fn from(err: bcrypt::BcryptError) -> Self {
        Error::InternalError {
            message: format!("Password error: {}", err),
        }
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        Error::InternalError {
            message: format!("JWT error: {}", err),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, client_error) = self.client_status_and_error();

        // Inclure le message détaillé pour les erreurs de base de données en développement
        let mut body = serde_json::json!({
            "error": client_error,
        });

        // Ajouter le message détaillé pour DatabaseError et InternalError
        match &self {
            Self::DatabaseError { message } => {
                body["details"] = serde_json::json!(message);
            }
            Self::InternalError { message } => {
                body["details"] = serde_json::json!(message);
            }
            _ => {}
        }

        (status, Json(body)).into_response()
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, &'static str) {
        match self {
            Self::AuthFailNoAuthHeader => {
                (StatusCode::UNAUTHORIZED, "Missing authorization header")
            }
            Self::AuthFailInvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
            Self::AuthFailTokenExpired => (StatusCode::UNAUTHORIZED, "Token expired"),
            Self::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid email or password"),
            Self::EmailAlreadyExists => (StatusCode::CONFLICT, "Email already exists"),
            Self::UserNotFound => (StatusCode::NOT_FOUND, "User not found"),
            Self::ServerNotFound => (StatusCode::NOT_FOUND, "Server not found"),
            Self::ServerForbidden => (StatusCode::FORBIDDEN, "Server access forbidden"),
            Self::ServerOwnerCannotLeave => (StatusCode::BAD_REQUEST, "Owner cannot leave server"),
            Self::ServerAlreadyMember => (StatusCode::CONFLICT, "Already a member"),
            Self::ChannelNotFound => (StatusCode::NOT_FOUND, "Channel not found"),
            Self::ChannelForbidden => (StatusCode::FORBIDDEN, "Channel access forbidden"),
            Self::MessageNotFound => (StatusCode::NOT_FOUND, "Message not found"),
            Self::MessageForbidden => (StatusCode::FORBIDDEN, "Message access forbidden"),
            Self::DatabaseError { .. } => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            Self::InternalError { .. } => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error"),
        }
    }
}
