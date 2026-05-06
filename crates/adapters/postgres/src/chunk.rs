use async_trait::async_trait;
use domain::{Chunk, ProfilId};
use ports::{ChunkRepo, RepoError};
use sqlx::PgPool;

use crate::helpers::{build_chunk, map_sqlx, ChunkRow};

pub struct ChunkRepoPg {
    pool: PgPool,
}

impl ChunkRepoPg {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ChunkRepo for ChunkRepoPg {
    async fn top_k_by_embedding(
        &self,
        profil_id: ProfilId,
        embedding: &[f32],
        limit: u32,
    ) -> Result<Vec<(Chunk, f32)>, RepoError> {
        let embedding_str = format!(
            "[{}]",
            embedding
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );

        let rows = sqlx::query(
            r#"
            SELECT id, profil_id, kind::text as kind, titre, content, metadata, embedding::text as embedding, created_at,
                   (embedding <=> $2::vector) as distance
            FROM chunks
            WHERE profil_id = $1
            ORDER BY distance
            LIMIT $3
            "#
        )
        .bind(profil_id.as_uuid())
        .bind(embedding_str)
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx)?;

        rows.into_iter()
            .map(|r| {
                use sqlx::Row;
                Ok(build_chunk(ChunkRow {
                    id: r.get("id"),
                    profil_id: r.get("profil_id"),
                    kind: r.get("kind"),
                    titre: r.get("titre"),
                    content: r.get("content"),
                    metadata: r.get("metadata"),
                    embedding: r.get("embedding"),
                    created_at: r.get("created_at"),
                    distance: r.get("distance"),
                }))
            })
            .collect()
    }

    async fn upsert(&self, chunk: &Chunk) -> Result<(), RepoError> {
        let embedding_str = format!(
            "[{}]",
            chunk
                .embedding
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );
        sqlx::query(
            r#"
            INSERT INTO chunks (id, profil_id, kind, titre, content, metadata, embedding, created_at)
            VALUES ($1, $2, $3::chunk_kind, $4, $5, $6, $7::vector, $8)
            ON CONFLICT (id) DO UPDATE SET
                titre = EXCLUDED.titre,
                content = EXCLUDED.content,
                metadata = EXCLUDED.metadata,
                embedding = EXCLUDED.embedding
            "#
        )
        .bind(chunk.id.as_uuid())
        .bind(chunk.profil_id.as_uuid())
        .bind(chunk.kind.as_str())
        .bind(&chunk.titre)
        .bind(&chunk.content)
        .bind(serde_json::to_value(&chunk.metadata).unwrap_or(serde_json::Value::Null))
        .bind(embedding_str)
        .bind(chunk.created_at)
        .execute(&self.pool)
        .await
        .map_err(map_sqlx)?;
        Ok(())
    }
}
