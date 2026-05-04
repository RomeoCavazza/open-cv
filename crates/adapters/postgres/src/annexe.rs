use domain::{Annexe, AnnexeId, ProfilId};
use ports::{AnnexeRepo, RepoError};
use sqlx::PgPool;

use crate::helpers::{build_annexe, map_sqlx};

pub struct AnnexeRepoPg {
    pool: PgPool,
}

impl AnnexeRepoPg {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl AnnexeRepo for AnnexeRepoPg {
    async fn get_by_id(&self, id: AnnexeId) -> Result<Option<Annexe>, RepoError> {
        let row = sqlx::query!(
            r#"
            SELECT id, profil_id, label, filename, content_type, content, created_at
            FROM annexes
            WHERE id = $1
            "#,
            id.as_uuid()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx)?;

        Ok(row.map(|r| {
            build_annexe(
                r.id,
                r.profil_id,
                r.label,
                r.filename,
                r.content_type,
                r.content,
                r.created_at,
            )
        }))
    }

    async fn list_by_profil_id(&self, profil_id: ProfilId) -> Result<Vec<Annexe>, RepoError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, profil_id, label, filename, content_type, content, created_at
            FROM annexes
            WHERE profil_id = $1
            ORDER BY created_at DESC
            "#,
            profil_id.as_uuid()
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx)?;

        Ok(rows
            .into_iter()
            .map(|r| {
                build_annexe(
                    r.id,
                    r.profil_id,
                    r.label,
                    r.filename,
                    r.content_type,
                    r.content,
                    r.created_at,
                )
            })
            .collect())
    }

    async fn upsert(&self, annexe: &Annexe) -> Result<(), RepoError> {
        sqlx::query!(
            r#"
            INSERT INTO annexes (id, profil_id, label, filename, content_type, content, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE SET
                label = EXCLUDED.label,
                filename = EXCLUDED.filename,
                content_type = EXCLUDED.content_type,
                content = EXCLUDED.content
            "#,
            annexe.id.as_uuid(),
            annexe.profil_id.as_uuid(),
            annexe.label,
            annexe.filename,
            annexe.content_type,
            annexe.content,
            annexe.created_at
        )
        .execute(&self.pool)
        .await
        .map_err(map_sqlx)?;
        Ok(())
    }

    async fn delete(&self, id: AnnexeId) -> Result<(), RepoError> {
        sqlx::query!("DELETE FROM annexes WHERE id = $1", id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(map_sqlx)?;
        Ok(())
    }
}
