use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::ctx::Ctx;
use crate::error::Result;
use crate::models::{
    BanMemberPayload, CreateServerPayload, Server, ServerBan, ServerMember,
    TransferOwnershipPayload, UpdateMemberRolePayload, UpdateServerPayload,
};
use crate::services;
use crate::AppState;

pub async fn create_server(
    State(state): State<AppState>,
    ctx: Ctx,
    Json(payload): Json<CreateServerPayload>,
) -> Result<Json<Server>> {
    let server =
        services::create_server(&state.server_repo, &state.user_repo, ctx.user_id(), payload)
            .await?;
    Ok(Json(server))
}

pub async fn list_servers(State(state): State<AppState>, ctx: Ctx) -> Result<Json<Vec<Server>>> {
    let servers = services::list_user_servers(&state.server_repo, ctx.user_id()).await?;
    Ok(Json(servers))
}

pub async fn get_server(
    State(state): State<AppState>,
    ctx: Ctx,
    Path(id): Path<Uuid>,
) -> Result<Json<Server>> {
    let member = services::get_member(&state.server_repo, id, ctx.user_id()).await?;
    if member.is_none() {
        return Err(crate::Error::ServerForbidden);
    }

    let server = services::get_server(&state.server_repo, id).await?;
    Ok(Json(server))
}

pub async fn update_server(
    State(state): State<AppState>,
    ctx: Ctx,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateServerPayload>,
) -> Result<Json<Server>> {
    let server = services::update_server(&state.server_repo, id, ctx.user_id(), payload).await?;
    Ok(Json(server))
}

pub async fn delete_server(
    State(state): State<AppState>,
    ctx: Ctx,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    services::delete_server(&state.server_repo, id, ctx.user_id()).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn join_server(
    State(state): State<AppState>,
    ctx: Ctx,
    Path(id): Path<Uuid>,
) -> Result<Json<ServerMember>> {
    let member = services::join_server(&state.server_repo, id, ctx.user_id()).await?;
    Ok(Json(member))
}

pub async fn leave_server(
    State(state): State<AppState>,
    ctx: Ctx,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    services::leave_server(&state.server_repo, id, ctx.user_id()).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_members(
    State(state): State<AppState>,
    ctx: Ctx,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<crate::models::ServerMemberWithUser>>> {
    let member = services::get_member(&state.server_repo, id, ctx.user_id()).await?;
    if member.is_none() {
        return Err(crate::Error::ServerForbidden);
    }

    let members = services::list_members(&state.server_repo, id).await?;

    // Enrichir avec les infos utilisateur
    let mut members_with_user = Vec::new();
    for member in members {
        if let Ok(Some(user)) = state.user_repo.find_by_id(member.user_id).await {
            members_with_user.push(crate::models::ServerMemberWithUser {
                server_id: member.server_id,
                user_id: member.user_id,
                role: member.role,
                joined_at: member.joined_at,
                username: user.username,
                avatar_url: user.avatar_url,
            });
        }
    }

    Ok(Json(members_with_user))
}

pub async fn kick_member(
    State(state): State<AppState>,
    ctx: Ctx,
    Path((server_id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode> {
    services::kick_member(&state.server_repo, server_id, user_id, ctx.user_id()).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn ban_member(
    State(state): State<AppState>,
    ctx: Ctx,
    Path((server_id, user_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<BanMemberPayload>,
) -> Result<Json<ServerBan>> {
    let ban = services::ban_member(
        &state.server_repo,
        server_id,
        user_id,
        payload,
        ctx.user_id(),
    )
    .await?;
    Ok(Json(ban))
}

pub async fn unban_member(
    State(state): State<AppState>,
    ctx: Ctx,
    Path((server_id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode> {
    services::unban_member(&state.server_repo, server_id, user_id, ctx.user_id()).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_bans(
    State(state): State<AppState>,
    ctx: Ctx,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<ServerBan>>> {
    let bans = services::list_bans(&state.server_repo, id, ctx.user_id()).await?;
    Ok(Json(bans))
}

pub async fn update_member_role(
    State(state): State<AppState>,
    ctx: Ctx,
    Path((server_id, user_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateMemberRolePayload>,
) -> Result<Json<ServerMember>> {
    let member = services::update_member_role(
        &state.server_repo,
        server_id,
        user_id,
        payload.role,
        ctx.user_id(),
    )
    .await?;
    Ok(Json(member))
}

pub async fn transfer_ownership(
    State(state): State<AppState>,
    ctx: Ctx,
    Path(id): Path<Uuid>,
    Json(payload): Json<TransferOwnershipPayload>,
) -> Result<Json<Server>> {
    let server =
        services::transfer_ownership(&state.server_repo, id, payload, ctx.user_id()).await?;
    Ok(Json(server))
}
