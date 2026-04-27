use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Default)]
#[sqlx(type_name = "member_role", rename_all = "lowercase")]
pub enum MemberRole {
    #[default]
    Owner,
    Admin,
    Member,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Server {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct ServerMember {
    pub server_id: Uuid,
    pub user_id: Uuid,
    pub role: MemberRole,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ServerMemberWithUser {
    pub server_id: Uuid,
    pub user_id: Uuid,
    pub role: MemberRole,
    pub joined_at: DateTime<Utc>,
    pub username: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateServerPayload {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateServerPayload {
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMemberRolePayload {
    pub role: MemberRole,
}

#[derive(Debug, Deserialize)]
pub struct TransferOwnershipPayload {
    pub new_owner_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct BanMemberPayload {
    pub reason: Option<String>,
    /// ISO 8601 string or null. If provided, ban is temporary until this timestamp.
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct ServerBan {
    pub server_id: Uuid,
    pub user_id: Uuid,
    pub banned_by: Uuid,
    pub reason: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub banned_at: DateTime<Utc>,
}
