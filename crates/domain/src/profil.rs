//! Le profil candidat (toi). Versionné pour pouvoir rejouer.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;

use crate::ids::{ProfilId, AnnexeId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profil {
    pub id: ProfilId,
    pub label: String,
    pub content: Json,
    pub is_active: bool,
    #[serde(skip)]
    pub profile_photo: Option<Vec<u8>>,
    #[serde(skip)]
    pub calendar_pdf: Option<Vec<u8>>,
    pub resume_template: Option<Json>,
    pub cover_letter_template: Option<Json>,
    pub notes: Json,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annexe {
    pub id: AnnexeId,
    pub profil_id: ProfilId,
    pub label: String,
    pub filename: String,
    pub content_type: String,
    #[serde(skip)]
    pub content: Vec<u8>,
    pub created_at: DateTime<Utc>,
}
