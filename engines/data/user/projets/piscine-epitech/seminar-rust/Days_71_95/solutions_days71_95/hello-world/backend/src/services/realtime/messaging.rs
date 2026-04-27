//! Gestion des messages temps réel via WebSocket

use uuid::Uuid;

use crate::error::{Error, Result};
use crate::models::CreateMessagePayload;
use crate::services::{channels, messages, servers};
use crate::web::ws::protocol::ServerEvent;
use crate::AppState;

/// Traite l'envoi d'un message via WebSocket
/// Crée le message en DB et le broadcast aux abonnés du channel
pub async fn handle_send_message(
    state: &AppState,
    user_id: Uuid,
    channel_id: Uuid,
    content: String,
) -> Result<()> {
    // Validation basique
    if content.trim().is_empty() {
        return Err(Error::InternalError {
            message: "Message content cannot be empty".to_string(),
        });
    }

    if content.len() > 2000 {
        return Err(Error::InternalError {
            message: "Message too long (max 2000 chars)".to_string(),
        });
    }

    // Vérifier les permissions (même logique que le service HTTP)
    let channel =
        channels::get_channel(&state.server_repo, &state.channel_repo, channel_id, user_id).await?;

    servers::get_member(&state.server_repo, channel.server_id, user_id)
        .await?
        .ok_or(Error::MessageForbidden)?;

    // Récupérer le username (sera inclus dans le message créé)
    let _username = state
        .user_repo
        .get_username(user_id)
        .await?
        .ok_or(Error::UserNotFound)?;

    // Créer le message (même logique que le service HTTP)
    let payload = CreateMessagePayload {
        content: content.clone(),
    };

    let message_with_user = messages::create_message(
        &state.server_repo,
        &state.channel_repo,
        &state.user_repo,
        &state.message_repo,
        channel_id,
        user_id,
        payload,
    )
    .await?;

    // Broadcast via WebSocket aux abonnés du channel
    let event = ServerEvent::MessageCreate {
        id: message_with_user.id,
        channel_id: message_with_user.channel_id,
        server_id: message_with_user.server_id,
        author_id: message_with_user.author_id,
        username: message_with_user.username,
        content: message_with_user.content,
        created_at: message_with_user.created_at,
        edited_at: message_with_user.edited_at,
    };

    state
        .ws_hub
        .broadcast_to_channel_with_metrics(channel_id, &event, Some(&state.ws_metrics))
        .await;

    Ok(())
}
