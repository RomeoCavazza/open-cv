use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization};

use crate::models::{AuthResponse, LoginPayload, SignupPayload};
use crate::services::{self, auth::AuthError, verify_token};
use crate::AppState;

/// Convertit AuthError en réponse HTTP
impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match &self {
            AuthError::EmailExists => (StatusCode::CONFLICT, "Email already exists"),
            AuthError::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, "Invalid email or password")
            }
            AuthError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AuthError::PasswordHash(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Password error"),
            AuthError::Jwt(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Token error"),
        };

        let mut body = serde_json::json!({ "error": message });

        // Ajouter les détails pour les erreurs de base de données
        if let AuthError::Database(err) = &self {
            body["details"] = serde_json::json!(err.to_string());
        }

        (status, Json(body)).into_response()
    }
}

/// POST /auth/signup - Créer un compte
pub async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<SignupPayload>,
) -> Result<Json<AuthResponse>, AuthError> {
    let response = services::signup(&state.db, payload, &state.jwt_secret).await?;
    Ok(Json(response))
}

/// POST /auth/login - Se connecter
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<AuthResponse>, AuthError> {
    let response = services::login(&state.db, payload, &state.jwt_secret).await?;
    Ok(Json(response))
}

/// POST /auth/logout - Se déconnecter
pub async fn logout(
    State(state): State<AppState>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<StatusCode, AuthError> {
    let claims =
        verify_token(auth.token(), &state.jwt_secret).map_err(|_| AuthError::InvalidCredentials)?;

    services::logout(&state.db, claims.sub).await?;
    Ok(StatusCode::NO_CONTENT)
}
