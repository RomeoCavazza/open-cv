pub mod auth;
pub mod channels;
pub mod invites;
pub mod messages;
pub mod servers;

use axum::Router;

use crate::AppState;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/servers", servers::routes())
        .merge(channels::routes())
        .merge(messages::routes())
        .merge(invites::routes())
}
