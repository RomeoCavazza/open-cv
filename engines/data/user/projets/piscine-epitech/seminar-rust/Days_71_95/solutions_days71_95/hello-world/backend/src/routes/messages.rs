use axum::{
    routing::{post, put},
    Router,
};

use crate::handlers::messages;
use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/channels/{channel_id}/messages",
            post(messages::create_message).get(messages::list_messages),
        )
        .route(
            "/messages/{id}",
            put(messages::update_message).delete(messages::delete_message),
        )
}
