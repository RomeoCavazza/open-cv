use axum::{extract::State, Json};

use crate::ctx::Ctx;
use crate::models::{UpdateMePayload, UserResponse};
use crate::Error;
use crate::{AppState, Result};

pub async fn me(State(state): State<AppState>, ctx: Ctx) -> Result<Json<UserResponse>> {
    let user = state
        .user_repo
        .find_by_id(ctx.user_id())
        .await?
        .ok_or(Error::UserNotFound)?;

    Ok(Json(user.into()))
}

pub async fn update_me(
    State(state): State<AppState>,
    ctx: Ctx,
    Json(payload): Json<UpdateMePayload>,
) -> Result<Json<UserResponse>> {
    let user = state
        .user_repo
        .update_profile(ctx.user_id(), payload)
        .await?
        .ok_or(Error::UserNotFound)?;

    Ok(Json(user.into()))
}
