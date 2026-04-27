use axum::extract::Path;
use axum::extract::State;
use axum::Json;

use uuid::Uuid;

use crate::models::PublicUserResponse;
use crate::AppState;
use crate::Error;
use crate::Result;

pub async fn get_public_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<PublicUserResponse>> {
    let user = state
        .user_repo
        .find_by_id(user_id)
        .await?
        .ok_or(Error::UserNotFound)?;
    Ok(Json(user.into()))
}
