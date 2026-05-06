//! Instance : 1 candidature = (offre + profil + CV généré + lettre générée).

use crate::ids::{InstanceId, OffreId, ProfilId, Slug};
use crate::json::JsonValue;
use crate::{CoverLetter, Restitution, Resume};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstanceStatus {
    Draft,
    Generating,
    Ready,
    Sent,
    Archived,
    Failed,
}

impl InstanceStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Generating => "generating",
            Self::Ready => "ready",
            Self::Sent => "sent",
            Self::Archived => "archived",
            Self::Failed => "failed",
        }
    }

    /// Une instance ne peut transitionner que dans certaines directions.
    /// On encode la machine à états ici, dans le domaine.
    pub fn can_transition_to(&self, next: Self) -> bool {
        use InstanceStatus::*;
        match (self, next) {
            (Draft, Generating) => true,
            (Generating, Ready) => true,
            (Generating, Failed) => true,
            (Ready, Sent) => true,
            (Ready, Archived) => true,
            (Sent, Archived) => true,
            (Failed, Generating) => true, // retry
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    pub id: InstanceId,
    pub slug: Slug,
    pub offre_id: OffreId,
    pub profil_id: ProfilId,
    pub status: InstanceStatus,
    pub restitution: Option<Restitution>,
    pub resume_json: Option<Resume>,
    pub cover_letter_json: Option<CoverLetter>,
    pub notes: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub sent_at: Option<DateTime<Utc>>,
}

#[cfg(test)]
mod tests {
    use super::InstanceStatus::*;

    #[test]
    fn transitions_legales() {
        assert!(Draft.can_transition_to(Generating));
        assert!(Generating.can_transition_to(Ready));
        assert!(Ready.can_transition_to(Sent));
        assert!(Failed.can_transition_to(Generating)); // retry possible
    }

    #[test]
    fn transitions_illegales() {
        assert!(!Draft.can_transition_to(Sent)); // skip pas autorisé
        assert!(!Sent.can_transition_to(Draft)); // pas de retour arrière
        assert!(!Archived.can_transition_to(Ready));
    }
}
