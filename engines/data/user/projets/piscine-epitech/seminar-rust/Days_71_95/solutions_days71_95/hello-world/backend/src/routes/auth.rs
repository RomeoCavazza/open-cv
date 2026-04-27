use crate::{handlers::auth, AppState};
use axum::{routing::post, Router};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/auth/signup", post(auth::signup))
        .route("/auth/login", post(auth::login))
    // logout est dans routes_protected (nécessite auth)
}
