//! Chunks : expériences, projets, compétences découpés pour le RAG.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;

use crate::ids::{ChunkId, ProfilId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChunkKind {
    Experience,
    Projet,
    Formation,
    Competence,
    PhraseLettre,
}

impl ChunkKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Experience => "experience",
            Self::Projet => "projet",
            Self::Formation => "formation",
            Self::Competence => "competence",
            Self::PhraseLettre => "phrase_lettre",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub id: ChunkId,
    pub profil_id: ProfilId,
    pub kind: ChunkKind,
    pub titre: String,
    pub content: String,
    pub metadata: Json,
    /// Embedding (dimension dépend du modèle ; ex 1024 pour voyage-3).
    pub embedding: Vec<f32>,
    pub created_at: DateTime<Utc>,
}
