use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::invites;
use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/servers/{id}/invites",
            post(invites::create_invite).get(invites::list_invites),
        )
        .route("/invites/join", post(invites::join_server_with_code))
        .route("/invites/{code}", get(invites::get_invite))
        .route("/invites/{code}/accept", post(invites::accept_invite))
}
