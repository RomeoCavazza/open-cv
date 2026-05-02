use crate::AppError;
use chrono::Utc;
use domain::{Chunk, ChunkId, ChunkKind, ProfilId};
use ports::{ChunkRepo, EmbedMode, Embedder, ProfilRepo};
use std::sync::Arc;

pub struct IndexProfilUseCase {
    profils: Arc<dyn ProfilRepo>,
    chunks: Arc<dyn ChunkRepo>,
    embedder: Arc<dyn Embedder>,
}

impl IndexProfilUseCase {
    pub fn new(
        profils: Arc<dyn ProfilRepo>,
        chunks: Arc<dyn ChunkRepo>,
        embedder: Arc<dyn Embedder>,
    ) -> Self {
        Self {
            profils,
            chunks,
            embedder,
        }
    }

    pub async fn execute(&self, profil_id: ProfilId) -> Result<(), AppError> {
        let profil = self
            .profils
            .get_by_id(profil_id)
            .await?
            .ok_or(AppError::NotFound)?;

        let mut chunks_to_index = Vec::new();

        // 2. Parser le pitch
        if let Some(pitch) = profil.content.get("pitch").and_then(|v| v.as_str()) {
            chunks_to_index.push((
                "Pitch personnel".to_string(),
                pitch.to_string(),
                ChunkKind::Experience,
            ));
        }

        // 3. Parser les expériences
        if let Some(exps) = profil.content.get("experiences").and_then(|v| v.as_array()) {
            for exp in exps {
                let role = exp
                    .get("role")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Inconnu");
                let company = exp
                    .get("company")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Inconnue");
                let desc = exp
                    .get("description")
                    .and_then(|v| v.as_array())
                    .map(|a| {
                        a.iter()
                            .map(|v| v.as_str().unwrap_or(""))
                            .collect::<Vec<_>>()
                            .join("\n")
                    })
                    .unwrap_or_default();

                chunks_to_index.push((
                    format!("Expérience : {} chez {}", role, company),
                    desc,
                    ChunkKind::Experience,
                ));
            }
        }

        // 4. Parser les projets
        if let Some(projs) = profil.content.get("projects").and_then(|v| v.as_array()) {
            for proj in projs {
                let title = proj
                    .get("role")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Projet");
                let name = proj.get("company").and_then(|v| v.as_str()).unwrap_or("");
                let desc = proj
                    .get("description")
                    .and_then(|v| v.as_array())
                    .map(|a| {
                        a.iter()
                            .map(|v| v.as_str().unwrap_or(""))
                            .collect::<Vec<_>>()
                            .join("\n")
                    })
                    .unwrap_or_default();

                chunks_to_index.push((
                    format!("Projet : {} ({})", title, name),
                    desc,
                    ChunkKind::Projet,
                ));
            }
        }

        println!(
            "✨ Génération des embeddings pour {} chunks...",
            chunks_to_index.len()
        );

        for (titre, content, kind) in chunks_to_index {
            if content.is_empty() {
                continue;
            }

            let embeddings = self
                .embedder
                .embed(&[&content], EmbedMode::Document)
                .await
                .map_err(|e| AppError::Other(e.to_string()))?;

            if let Some(embedding) = embeddings.into_iter().next() {
                let chunk = Chunk {
                    id: ChunkId::new(),
                    profil_id,
                    kind,
                    titre,
                    content,
                    metadata: serde_json::json!({}),
                    embedding,
                    created_at: Utc::now(),
                };
                self.chunks.upsert(&chunk).await?;
            }
        }

        Ok(())
    }
}
