use crate::handlers::servers;
use crate::AppState;
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(servers::create_server).get(servers::list_servers))
        // ON CHANGE /:id PAR /{id}
        .route(
            "/{id}",
            get(servers::get_server)
                .put(servers::update_server)
                .delete(servers::delete_server),
        )
        // ON CHANGE AUSSI TOUTES LES SUIVANTES
        .route("/{id}/join", post(servers::join_server))
        .route("/{id}/leave", delete(servers::leave_server))
        .route("/{id}/members", get(servers::list_members))
        .route(
            "/{id}/members/{userId}",
            put(servers::update_member_role).delete(servers::kick_member),
        )
        .route(
            "/{id}/members/{userId}/ban",
            post(servers::ban_member).delete(servers::unban_member),
        )
        .route("/{id}/bans", get(servers::list_bans))
        .route("/{id}/transfer", put(servers::transfer_ownership))
}
