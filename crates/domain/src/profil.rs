//! Le profil candidat (toi). Versionné pour pouvoir rejouer.

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;

use crate::ids::{AnnexeId, ProfilId};

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
    pub resume_template: Option<Json>,
    pub cover_letter_template: Option<Json>,
    pub notes: Json,
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
    pub firstname: String,
    pub lastname: String,
    pub title: String,
    pub offer_type: String,
    pub pitch: String,
    pub location: String,
    pub phone: String,
    pub email: String,
    pub linkedin: String,
    pub website: String,
    pub github: String,
    pub image: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct ApprenticeshipSection {
    pub duration: String,
    pub rhythm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct ExperienceEntry {
    pub role: String,
    pub company: String,
    pub period: String,
    pub description: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct EducationEntry {
    pub school: String,
    pub degree: String,
    pub period: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct SkillCategoryEntry {
    pub category: String,
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct LanguageEntry {
    pub name: String,
    pub level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct DocumentSection {
    pub resume_template: Option<Json>,
    pub cover_letter_template: Option<Json>,
    pub apprenticeship_calendar: Option<Json>,
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
