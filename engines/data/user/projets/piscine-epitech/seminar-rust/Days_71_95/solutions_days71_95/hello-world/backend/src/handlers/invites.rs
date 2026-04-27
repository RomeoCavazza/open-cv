use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::ctx::Ctx;
use crate::error::Result;
use crate::models::{CreateInvitePayload, Invite, JoinServerWithCodePayload};
use crate::services;
use crate::AppState;

pub async fn create_invite(
    State(state): State<AppState>,
    ctx: Ctx,
    Path(server_id): Path<Uuid>,
    Json(payload): Json<CreateInvitePayload>,
) -> Result<Json<Invite>> {
    let invite = services::create_invite(
        &state.invite_repo,
        &state.server_repo,
        server_id,
        payload,
        ctx.user_id(),
    )
    .await?;
    Ok(Json(invite))
}

pub async fn list_invites(
    State(state): State<AppState>,
    ctx: Ctx,
    Path(server_id): Path<Uuid>,
) -> Result<Json<Vec<Invite>>> {
    let invites = services::list_invites(
        &state.invite_repo,
        &state.server_repo,
        server_id,
        ctx.user_id(),
    )
    .await?;
    Ok(Json(invites))
}

pub async fn get_invite(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<Json<Invite>> {
    let invite = services::get_invite_by_code(&state.invite_repo, &code).await?;
    Ok(Json(invite))
}

pub async fn accept_invite(
    State(state): State<AppState>,
    ctx: Ctx,
    Path(code): Path<String>,
) -> Result<Json<serde_json::Value>> {
    let payload = JoinServerWithCodePayload { code };
    let server_id = services::join_server_with_code(
        &state.invite_repo,
        &state.server_repo,
        payload,
        ctx.user_id(),
    )
    .await?;
    Ok(Json(serde_json::json!({ "server_id": server_id })))
}

pub async fn join_server_with_code(
    State(state): State<AppState>,
    ctx: Ctx,
    Json(payload): Json<JoinServerWithCodePayload>,
) -> Result<Json<serde_json::Value>> {
    let server_id = services::join_server_with_code(
        &state.invite_repo,
        &state.server_repo,
        payload,
        ctx.user_id(),
    )
    .await?;
    Ok(Json(serde_json::json!({ "server_id": server_id })))
}
