use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Invite {
    pub id: Uuid,
    pub code: String,
    pub server_id: Uuid,
    pub created_by: Uuid,
    pub max_uses: Option<i32>,
    pub uses: i32,
    pub expires_at: Option<DateTime<Utc>>,
    pub revoked: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateInvitePayload {
    pub max_uses: Option<i32>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JoinServerWithCodePayload {
    pub code: String,
}

#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)]
pub struct InviteResponse {
    pub code: String,
    pub server_id: Uuid,
    pub max_uses: Option<i32>,
    pub uses: i32,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl From<Invite> for InviteResponse {
    fn from(invite: Invite) -> Self {
        Self {
            code: invite.code,
            server_id: invite.server_id,
            max_uses: invite.max_uses,
            uses: invite.uses,
            expires_at: invite.expires_at,
            created_at: invite.created_at,
        }
    }
}
