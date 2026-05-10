//! Snapshot d'instance — capture de l'état des documents avant mutation.

use crate::ids::InstanceId;
use crate::{CoverLetter, Restitution, Resume};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceSnapshot {
    pub id: Uuid,
    pub instance_id: InstanceId,
    pub version: i32,
    pub resume_json: Option<Resume>,
    pub cover_letter_json: Option<CoverLetter>,
    pub restitution: Option<Restitution>,
    pub trigger_message: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl InstanceSnapshot {
    /// Crée un snapshot de l'état actuel d'une instance avant mutation.
    pub fn capture(
        instance: &crate::Instance,
        version: i32,
        trigger_message: impl Into<Option<String>>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            instance_id: instance.id,
            version,
            resume_json: instance.resume_json.clone(),
            cover_letter_json: instance.cover_letter_json.clone(),
            restitution: instance.restitution.clone(),
            trigger_message: trigger_message.into(),
            created_at: Utc::now(),
        }
    }
}
