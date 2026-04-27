use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;

use crate::ctx::Ctx;
use crate::error::{Error, Result};
use crate::services::verify_token;
use crate::AppState;

pub async fn mw_require_auth(
    State(_state): State<AppState>,
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    let ctx = req
        .extensions()
        .get::<Result<Ctx>>()
        .ok_or(Error::AuthFailNoAuthHeader)?
        .clone();

    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let result_ctx = match auth_header {
        Some(header) if header.starts_with("Bearer ") => {
            let token = &header[7..];
            match verify_token(token, &state.jwt_secret) {
                Ok(claims) => match state.user_repo.find_by_id(claims.sub).await {
                    Ok(Some(_)) => Ok(Ctx::new(claims.sub)),
                    Ok(None) => Err(Error::AuthFailInvalidToken),
                    Err(e) => Err(Error::from(e)),
                },
                Err(_) => Err(Error::AuthFailInvalidToken),
            }
        }
        Some(_) => Err(Error::AuthFailInvalidToken),
        None => Err(Error::AuthFailNoAuthHeader),
    };

    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::AuthFailNoAuthHeader)?
            .clone()
    }
}
