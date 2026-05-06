//! Le profil candidat (toi). Versionné pour pouvoir rejouer.

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ids::{AnnexeId, ProfilId};
use crate::json::JsonValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profil {
    pub id: ProfilId,
    pub label: String,
    pub content: ProfilContent,
    pub is_active: bool,
    #[serde(skip)]
    pub profile_photo: Option<Vec<u8>>,
    #[serde(skip)]
    pub calendar_pdf: Option<Vec<u8>>,
    pub resume_template: Option<JsonValue>,
    pub cover_letter_template: Option<JsonValue>,
    pub notes: JsonValue,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct ProfilContent {
    #[serde(default)]
    pub profile: ProfileSection,
    #[serde(default)]
    pub apprenticeship: ApprenticeshipSection,
    #[serde(default)]
    pub experiences: Vec<ExperienceEntry>,
    #[serde(default)]
    pub projects: Vec<ExperienceEntry>,
    #[serde(default)]
    pub education: Vec<EducationEntry>,
    #[serde(default)]
    pub skills: Vec<SkillCategoryEntry>,
    #[serde(default)]
    pub languages: Vec<LanguageEntry>,
    #[serde(default)]
    pub documents: DocumentSection,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct ProfileSection {
    #[serde(default)]
    pub firstname: String,
    #[serde(default)]
    pub lastname: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub offer_type: String,
    #[serde(default)]
    pub pitch: String,
    #[serde(default)]
    pub location: String,
    #[serde(default)]
    pub phone: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub linkedin: String,
    #[serde(default)]
    pub website: String,
    #[serde(default)]
    pub github: String,
    #[serde(default)]
    pub image: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct ApprenticeshipSection {
    #[serde(default)]
    pub duration: String,
    #[serde(default)]
    pub rhythm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct ExperienceEntry {
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub company: String,
    #[serde(default)]
    pub period: String,
    #[serde(default)]
    pub description: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct EducationEntry {
    #[serde(default)]
    pub school: String,
    #[serde(default)]
    pub degree: String,
    #[serde(default)]
    pub period: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct SkillCategoryEntry {
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct LanguageEntry {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct DocumentSection {
    #[serde(default)]
    pub resume_template: Option<JsonValue>,
    #[serde(default)]
    pub cover_letter_template: Option<JsonValue>,
    #[serde(default)]
    pub apprenticeship_calendar: Option<JsonValue>,
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
