use async_trait::async_trait;
use domain::{Message, InstanceId, ProfilId};
use ports::{MessageRepo, RepoError};
use sqlx::PgPool;

use crate::helpers::{build_message, map_sqlx};

pub struct MessageRepoPg {
    pool: PgPool,
}

impl MessageRepoPg {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MessageRepo for MessageRepoPg {
    async fn list_by_instance_id(
        &self,
        instance_id: InstanceId,
    ) -> Result<Vec<Message>, RepoError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, instance_id, role::text as "role!", content, created_at
            FROM messages
            WHERE instance_id = $1
            ORDER BY created_at ASC
            "#,
            instance_id.as_uuid()
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx)?;

        rows.into_iter()
            .map(|r| build_message(r.id, r.instance_id, r.role, r.content, r.created_at))
            .collect()
    }

    async fn list_by_profil_id(&self, profil_id: ProfilId) -> Result<Vec<Message>, RepoError> {
        let rows = sqlx::query!(
            r#"
            SELECT m.id, m.instance_id, m.role::text as "role!", m.content, m.created_at
            FROM messages m
            JOIN instances i ON m.instance_id = i.id
            WHERE i.profil_id = $1
            ORDER BY m.created_at ASC
            "#,
            profil_id.as_uuid()
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx)?;

        rows.into_iter()
            .map(|r| build_message(r.id, r.instance_id, r.role, r.content, r.created_at))
            .collect()
    }

    async fn push(&self, message: &Message) -> Result<(), RepoError> {
        sqlx::query(
            r#"
            INSERT INTO messages (id, instance_id, role, content, created_at)
            VALUES ($1, $2, $3::message_role, $4, $5)
            "#,
        )
        .bind(message.id)
        .bind(message.instance_id.as_uuid())
        .bind(message.role.as_str())
        .bind(&message.content)
        .bind(message.created_at)
        .execute(&self.pool)
        .await
        .map_err(map_sqlx)?;
        Ok(())
    }

    async fn delete_all_for_instance(&self, instance_id: InstanceId) -> Result<(), RepoError> {
        sqlx::query!(
            "DELETE FROM messages WHERE instance_id = $1",
            instance_id.as_uuid()
        )
        .execute(&self.pool)
        .await
        .map_err(map_sqlx)?;
        Ok(())
    }
}
