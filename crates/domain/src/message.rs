use crate::ids::InstanceId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

impl MessageRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Assistant => "assistant",
            Self::System => "system",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub instance_id: InstanceId,
    pub role: MessageRole,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl Message {
    pub fn new(instance_id: InstanceId, role: MessageRole, content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            instance_id,
            role,
            content,
            created_at: Utc::now(),
        }
    }
}
