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
                id, instance_id, version, resume_json, cover_letter_json, restitution, trigger_message, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
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
            SELECT id, instance_id, version, resume_json, cover_letter_json, restitution, trigger_message, created_at
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
