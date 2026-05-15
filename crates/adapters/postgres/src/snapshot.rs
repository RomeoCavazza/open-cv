use crate::helpers::map_sqlx;
use async_trait::async_trait;
use domain::{InstanceId, InstanceSnapshot};
use ports::{RepoError, SnapshotRepo};
use sqlx::{PgPool, Row};

pub struct SnapshotRepoPg {
    pool: PgPool,
}

impl SnapshotRepoPg {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SnapshotRepo for SnapshotRepoPg {
    async fn save(&self, snapshot: &InstanceSnapshot) -> Result<(), RepoError> {
        sqlx::query(
            r#"
            INSERT INTO instance_snapshots (
                id, instance_id, version, resume_json, cover_letter_json, restitution, content_hash, trigger_message, created_at
            )
            VALUES (
                $1, $2, $3, $4, $5, $6,
                encode(
                    digest(
                        jsonb_build_object(
                            'resume_json', COALESCE($4::jsonb, 'null'::jsonb),
                            'cover_letter_json', COALESCE($5::jsonb, 'null'::jsonb),
                            'restitution', COALESCE($6::jsonb, 'null'::jsonb)
                        )::text,
                        'sha256'
                    ),
                    'hex'
                ),
                $7, $8
            )
            ON CONFLICT (instance_id, content_hash) DO NOTHING
            "#,
        )
        .bind(snapshot.id)
        .bind(snapshot.instance_id.as_uuid())
        .bind(snapshot.version)
        .bind(serde_json::to_value(&snapshot.resume_json).unwrap_or(serde_json::Value::Null))
        .bind(serde_json::to_value(&snapshot.cover_letter_json).unwrap_or(serde_json::Value::Null))
        .bind(serde_json::to_value(&snapshot.restitution).unwrap_or(serde_json::Value::Null))
        .bind(&snapshot.trigger_message)
        .bind(snapshot.created_at)
        .execute(&self.pool)
        .await
        .map_err(map_sqlx)?;

        Ok(())
    }

    async fn get_latest(
        &self,
        instance_id: InstanceId,
    ) -> Result<Option<InstanceSnapshot>, RepoError> {
        let row = sqlx::query(
            r#"
            SELECT id, instance_id, version, resume_json, cover_letter_json, restitution, content_hash, trigger_message, created_at
            FROM instance_snapshots
            WHERE instance_id = $1
            ORDER BY version DESC
            LIMIT 1
            "#,
        )
        .bind(instance_id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx)?;

        row.map(|r| {
            Ok(InstanceSnapshot {
                id: r.get("id"),
                instance_id: InstanceId::from_uuid(r.get("instance_id")),
                version: r.get("version"),
                resume_json: serde_json::from_value(r.get("resume_json")).unwrap_or_default(),
                cover_letter_json: serde_json::from_value(r.get("cover_letter_json"))
                    .unwrap_or_default(),
                restitution: serde_json::from_value(r.get("restitution")).unwrap_or_default(),
                content_hash: r.get("content_hash"),
                trigger_message: r.get("trigger_message"),
                created_at: r.get("created_at"),
            })
        })
        .transpose()
    }

    async fn list_by_instance(
        &self,
        instance_id: InstanceId,
    ) -> Result<Vec<InstanceSnapshot>, RepoError> {
        let rows = sqlx::query(
            r#"
            SELECT id, instance_id, version, resume_json, cover_letter_json, restitution, content_hash, trigger_message, created_at
            FROM instance_snapshots
            WHERE instance_id = $1
            ORDER BY version DESC
            "#,
        )
        .bind(instance_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx)?;

        let mut snapshots = Vec::new();
        for r in rows {
            snapshots.push(InstanceSnapshot {
                id: r.get("id"),
                instance_id: InstanceId::from_uuid(r.get("instance_id")),
                version: r.get("version"),
                resume_json: serde_json::from_value(r.get("resume_json")).unwrap_or_default(),
                cover_letter_json: serde_json::from_value(r.get("cover_letter_json"))
                    .unwrap_or_default(),
                restitution: serde_json::from_value(r.get("restitution")).unwrap_or_default(),
                content_hash: r.get("content_hash"),
                trigger_message: r.get("trigger_message"),
                created_at: r.get("created_at"),
            });
        }

        Ok(snapshots)
    }

    async fn get_by_version(
        &self,
        instance_id: InstanceId,
        version: i32,
    ) -> Result<Option<InstanceSnapshot>, RepoError> {
        let row = sqlx::query(
            r#"
            SELECT id, instance_id, version, resume_json, cover_letter_json, restitution, content_hash, trigger_message, created_at
            FROM instance_snapshots
            WHERE instance_id = $1 AND version = $2
            "#,
        )
        .bind(instance_id.as_uuid())
        .bind(version)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx)?;

        row.map(|r| {
            Ok(InstanceSnapshot {
                id: r.get("id"),
                instance_id: InstanceId::from_uuid(r.get("instance_id")),
                version: r.get("version"),
                resume_json: serde_json::from_value(r.get("resume_json")).unwrap_or_default(),
                cover_letter_json: serde_json::from_value(r.get("cover_letter_json"))
                    .unwrap_or_default(),
                restitution: serde_json::from_value(r.get("restitution")).unwrap_or_default(),
                content_hash: r.get("content_hash"),
                trigger_message: r.get("trigger_message"),
                created_at: r.get("created_at"),
            })
        })
        .transpose()
    }

    async fn count_by_instance(&self, instance_id: InstanceId) -> Result<i32, RepoError> {
        let count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM instance_snapshots WHERE instance_id = $1")
                .bind(instance_id.as_uuid())
                .fetch_one(&self.pool)
                .await
                .map_err(map_sqlx)?;

        Ok(count as i32)
    }
}
