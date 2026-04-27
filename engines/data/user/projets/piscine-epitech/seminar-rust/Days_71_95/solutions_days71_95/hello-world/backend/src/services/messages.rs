use chrono::Utc;
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::models::{ChannelMessage, CreateMessagePayload, MessageWithUser, UpdateMessagePayload};
use crate::repositories::{ChannelRepository, MessageRepository, ServerRepository, UserRepository};
use crate::services::{channels, servers};

pub async fn create_message(
    server_repo: &ServerRepository,
    channel_repo: &ChannelRepository,
    user_repo: &UserRepository,
    message_repo: &MessageRepository,
    channel_id: Uuid,
    user_id: Uuid,
    payload: CreateMessagePayload,
) -> Result<MessageWithUser> {
    let channel = channels::get_channel(server_repo, channel_repo, channel_id, user_id).await?;

    servers::get_member(server_repo, channel.server_id, user_id)
        .await?
        .ok_or(Error::MessageForbidden)?;

    let username = user_repo
        .get_username(user_id)
        .await?
        .ok_or(Error::UserNotFound)?;

    let message_id = Uuid::new_v4();
    let now = Utc::now();
    let content = payload.content.clone();

    let message = ChannelMessage {
        id: None,
        message_id,
        server_id: channel.server_id,
        channel_id,
        author_id: user_id,
        content: content.clone(),
        created_at: now,
        edited_at: None,
        deleted_at: None,
        deleted_by: None,
    };

    message_repo
        .create(&message)
        .await
        .map_err(|e| Error::DatabaseError {
            message: format!("MongoDB insert failed: {}", e),
        })?;

    Ok(MessageWithUser {
        id: message_id,
        server_id: channel.server_id,
        channel_id,
        author_id: user_id,
        username,
        content,
        created_at: now,
        edited_at: None,
    })
}

#[allow(clippy::too_many_arguments)]
pub async fn list_messages(
    server_repo: &ServerRepository,
    channel_repo: &ChannelRepository,
    user_repo: &UserRepository,
    message_repo: &MessageRepository,
    channel_id: Uuid,
    user_id: Uuid,
    limit: i64,
    before: Option<Uuid>,
) -> Result<Vec<MessageWithUser>> {
    let channel = channels::get_channel(server_repo, channel_repo, channel_id, user_id).await?;

    servers::get_member(server_repo, channel.server_id, user_id)
        .await?
        .ok_or(Error::MessageForbidden)?;

    let messages = message_repo
        .list_by_channel(channel_id, limit, before)
        .await
        .map_err(|e| Error::DatabaseError {
            message: format!("MongoDB query failed: {}", e),
        })?;

    if messages.is_empty() {
        return Ok(vec![]);
    }

    let author_ids: Vec<Uuid> = messages.iter().map(|m| m.author_id).collect();
    let usernames = user_repo.get_usernames_batch(&author_ids).await?;

    let mut result: Vec<MessageWithUser> = messages
        .into_iter()
        .map(|m| MessageWithUser {
            id: m.message_id,
            server_id: m.server_id,
            channel_id: m.channel_id,
            author_id: m.author_id,
            username: usernames
                .get(&m.author_id)
                .cloned()
                .unwrap_or_else(|| "Unknown".to_string()),
            content: m.content,
            created_at: m.created_at,
            edited_at: m.edited_at,
        })
        .collect();

    result.reverse();
    Ok(result)
}

pub async fn delete_message(
    message_repo: &MessageRepository,
    message_id: Uuid,
    user_id: Uuid,
) -> Result<()> {
    let message = message_repo
        .find_by_id(message_id)
        .await
        .map_err(|e| Error::DatabaseError {
            message: format!("MongoDB query failed: {}", e),
        })?
        .ok_or(Error::MessageNotFound)?;

    if message.author_id != user_id {
        return Err(Error::MessageForbidden);
    }

    message_repo
        .soft_delete(message_id, user_id)
        .await
        .map_err(|e| Error::DatabaseError {
            message: format!("MongoDB update failed: {}", e),
        })?;

    Ok(())
}

pub async fn update_message(
    message_repo: &MessageRepository,
    message_id: Uuid,
    user_id: Uuid,
    payload: UpdateMessagePayload,
) -> Result<MessageWithUser> {
    let message = message_repo
        .find_by_id(message_id)
        .await
        .map_err(|e| Error::DatabaseError {
            message: format!("MongoDB query failed: {}", e),
        })?
        .ok_or(Error::MessageNotFound)?;

    if message.author_id != user_id {
        return Err(Error::MessageForbidden);
    }

    if message.deleted_at.is_some() {
        return Err(Error::MessageNotFound);
    }

    message_repo
        .update_content(message_id, &payload.content)
        .await
        .map_err(|e| Error::DatabaseError {
            message: format!("MongoDB update failed: {}", e),
        })?;

    Ok(MessageWithUser {
        id: message.message_id,
        server_id: message.server_id,
        channel_id: message.channel_id,
        author_id: message.author_id,
        username: String::new(),
        content: payload.content,
        created_at: message.created_at,
        edited_at: Some(Utc::now()),
    })
}
