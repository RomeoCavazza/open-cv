use async_trait::async_trait;
use domain::{Instance, InstanceId, Slug};
use ports::{InstanceRepo, RepoError};
use sqlx::PgPool;

use crate::helpers::{build_instance, map_sqlx, InstanceRow};

pub struct InstanceRepoPg {
    pool: PgPool,
}

impl InstanceRepoPg {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl InstanceRepo for InstanceRepoPg {
    async fn get_by_id(&self, id: InstanceId) -> Result<Option<Instance>, RepoError> {
        let row = sqlx::query(
            r#"
            SELECT id, slug, offre_id, profil_id, status::text,
                   restitution, resume_json, cover_letter_json, notes,
                   created_at, updated_at, sent_at
            FROM instances
            WHERE id = $1
            "#,
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx)?;

        row.map(|r| {
            use sqlx::Row;
            build_instance(InstanceRow {
                id: r.get("id"),
                slug: r.get::<String, _>("slug"),
                offre_id: r.get("offre_id"),
                profil_id: r.get("profil_id"),
                status: r.get("status"),
                restitution: r.get("restitution"),
                resume_json: r.get("resume_json"),
                cover_letter_json: r.get("cover_letter_json"),
                notes: r.get("notes"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
                sent_at: r.get("sent_at"),
            })
        })
        .transpose()
    }

    async fn get_by_slug(&self, slug: &Slug) -> Result<Option<Instance>, RepoError> {
        let row = sqlx::query(
            r#"
            SELECT id, slug, offre_id, profil_id, status::text,
                   restitution, resume_json, cover_letter_json, notes,
                   created_at, updated_at, sent_at
            FROM instances
            WHERE slug = $1
            "#,
        )
        .bind(slug.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx)?;

        row.map(|r| {
            use sqlx::Row;
            build_instance(InstanceRow {
                id: r.get("id"),
                slug: r.get::<String, _>("slug"),
                offre_id: r.get("offre_id"),
                profil_id: r.get("profil_id"),
                status: r.get("status"),
                restitution: r.get("restitution"),
                resume_json: r.get("resume_json"),
                cover_letter_json: r.get("cover_letter_json"),
                notes: r.get("notes"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
                sent_at: r.get("sent_at"),
            })
        })
        .transpose()
    }

    async fn list_recent(&self, limit: u32) -> Result<Vec<Instance>, RepoError> {
        let rows = sqlx::query(
            r#"
            SELECT id, slug, offre_id, profil_id, status::text,
                   restitution, resume_json, cover_letter_json, notes,
                   created_at, updated_at, sent_at
            FROM instances
            ORDER BY created_at DESC
            LIMIT $1
            "#,
        )
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx)?;

        rows.into_iter()
            .map(|r| {
                use sqlx::Row;
                build_instance(InstanceRow {
                    id: r.get("id"),
                    slug: r.get::<String, _>("slug"),
                    offre_id: r.get("offre_id"),
                    profil_id: r.get("profil_id"),
                    status: r.get("status"),
                    restitution: r.get("restitution"),
                    resume_json: r.get("resume_json"),
                    cover_letter_json: r.get("cover_letter_json"),
                    notes: r.get("notes"),
                    created_at: r.get("created_at"),
                    updated_at: r.get("updated_at"),
                    sent_at: r.get("sent_at"),
                })
            })
            .collect()
    }

    async fn upsert(&self, instance: &Instance) -> Result<(), RepoError> {
        tracing::info!(
            "DB: Upserting instance {} (slug: {}). Notes size: {} bytes, History entries: {}",
            instance.id,
            instance.slug.as_str(),
            instance.notes.to_string().len(),
            instance
                .notes
                .get("chat_history")
                .and_then(|h| h.as_array())
                .map(|a| a.len())
                .unwrap_or(0)
        );
        sqlx::query(
            r#"
            INSERT INTO instances (
                id, slug, offre_id, profil_id, status,
                restitution, resume_json, cover_letter_json, notes,
                created_at, updated_at, sent_at
            )
            VALUES ($1, $2, $3, $4, $5::instance_status, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT (id) DO UPDATE SET
                status            = EXCLUDED.status,
                restitution       = EXCLUDED.restitution,
                resume_json       = EXCLUDED.resume_json,
                cover_letter_json = EXCLUDED.cover_letter_json,
                notes             = EXCLUDED.notes,
                updated_at        = EXCLUDED.updated_at,
                sent_at           = EXCLUDED.sent_at
            "#,
        )
        .bind(instance.id.as_uuid())
        .bind(instance.slug.as_str())
        .bind(instance.offre_id.as_uuid())
        .bind(instance.profil_id.as_uuid())
        .bind(instance.status.as_str())
        .bind(serde_json::to_value(&instance.restitution).unwrap_or(serde_json::Value::Null))
        .bind(serde_json::to_value(&instance.resume_json).unwrap_or(serde_json::Value::Null))
        .bind(serde_json::to_value(&instance.cover_letter_json).unwrap_or(serde_json::Value::Null))
        .bind(instance.notes.clone())
        .bind(instance.created_at)
        .bind(instance.updated_at)
        .bind(instance.sent_at)
        .execute(&self.pool)
        .await
        .map_err(map_sqlx)?;

        Ok(())
    }

    async fn get_by_offre_id(
        &self,
        offre_id: domain::OffreId,
    ) -> Result<Option<Instance>, RepoError> {
        let row = sqlx::query(
            r#"
            SELECT id, slug, offre_id, profil_id, status::text,
                   restitution, resume_json, cover_letter_json, notes,
                   created_at, updated_at, sent_at
            FROM instances
            WHERE offre_id = $1
            ORDER BY
                CASE
                    WHEN restitution IS NOT NULL
                      OR resume_json IS NOT NULL
                      OR cover_letter_json IS NOT NULL
                    THEN 0
                    ELSE 1
                END,
                created_at DESC
            LIMIT 1
            "#,
        )
        .bind(offre_id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx)?;

        row.map(|r| {
            use sqlx::Row;
            build_instance(InstanceRow {
                id: r.get("id"),
                slug: r.get::<String, _>("slug"),
                offre_id: r.get("offre_id"),
                profil_id: r.get("profil_id"),
                status: r.get("status"),
                restitution: r.get("restitution"),
                resume_json: r.get("resume_json"),
                cover_letter_json: r.get("cover_letter_json"),
                notes: r.get("notes"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
                sent_at: r.get("sent_at"),
            })
        })
        .transpose()
    }
}
