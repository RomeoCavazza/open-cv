use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::ctx::Ctx;
use crate::error::Result;
use crate::models::{Channel, CreateChannelPayload, UpdateChannelPayload};
use crate::services;
use crate::AppState;

pub async fn create_channel(
    State(state): State<AppState>,
    ctx: Ctx,
    Path(server_id): Path<Uuid>,
    Json(payload): Json<CreateChannelPayload>,
) -> Result<Json<Channel>> {
    let channel = services::create_channel(
        &state.server_repo,
        &state.channel_repo,
        server_id,
        ctx.user_id(),
        payload,
    )
    .await?;
    Ok(Json(channel))
}

pub async fn list_channels(
    State(state): State<AppState>,
    ctx: Ctx,
    Path(server_id): Path<Uuid>,
) -> Result<Json<Vec<Channel>>> {
    let channels = services::list_channels(
        &state.server_repo,
        &state.channel_repo,
        server_id,
        ctx.user_id(),
    )
    .await?;
    Ok(Json(channels))
}

pub async fn get_channel(
    State(state): State<AppState>,
    ctx: Ctx,
    Path(id): Path<Uuid>,
) -> Result<Json<Channel>> {
    let channel =
        services::get_channel(&state.server_repo, &state.channel_repo, id, ctx.user_id()).await?;
    Ok(Json(channel))
}

pub async fn update_channel(
    State(state): State<AppState>,
    ctx: Ctx,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateChannelPayload>,
) -> Result<Json<Channel>> {
    let channel = services::update_channel(
        &state.server_repo,
        &state.channel_repo,
        id,
        ctx.user_id(),
        payload,
    )
    .await?;
    Ok(Json(channel))
}

pub async fn delete_channel(
    State(state): State<AppState>,
    ctx: Ctx,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    services::delete_channel(&state.server_repo, &state.channel_repo, id, ctx.user_id()).await?;
    Ok(StatusCode::NO_CONTENT)
}
