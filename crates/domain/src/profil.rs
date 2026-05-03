//! Le profil candidat (toi). Versionné pour pouvoir rejouer.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;

use crate::ids::ProfilId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profil {
    pub id: ProfilId,
    /// Label humain : "v2026-04", "data-focus", etc.
    pub label: String,
    /// Profil structuré complet (libre, sera défini quand tu importeras
    /// `data/user/profile.md`).
    pub content: Json,
    pub is_active: bool,
    pub resume_template: Option<Json>,
    pub cover_letter_template: Option<Json>,
    pub calendar_pdf: Option<Vec<u8>>,
    pub notes: Json,
    pub created_at: DateTime<Utc>,
}
