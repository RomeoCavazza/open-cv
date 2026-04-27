use sqlx::PgPool;
use uuid::Uuid;

use crate::models::Channel;

#[derive(Clone)]
pub struct ChannelRepository {
    pool: PgPool,
}

impl ChannelRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        channel_id: Uuid,
        server_id: Uuid,
        name: &str,
        position: i32,
    ) -> sqlx::Result<Channel> {
        sqlx::query_as::<_, Channel>(
            r#"
            INSERT INTO channels (id, server_id, name, position, created_at, updated_at)
            VALUES ($1, $2, $3, $4, NOW(), NOW())
            RETURNING id, server_id, name, position, created_at, updated_at
            "#,
        )
        .bind(channel_id)
        .bind(server_id)
        .bind(name)
        .bind(position)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn find_by_id(&self, channel_id: Uuid) -> sqlx::Result<Option<Channel>> {
        sqlx::query_as::<_, Channel>(
            "SELECT id, server_id, name, position, created_at, updated_at FROM channels WHERE id = $1",
        )
        .bind(channel_id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn list_by_server(&self, server_id: Uuid) -> sqlx::Result<Vec<Channel>> {
        sqlx::query_as::<_, Channel>(
            "SELECT id, server_id, name, position, created_at, updated_at FROM channels WHERE server_id = $1 ORDER BY position",
        )
        .bind(server_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_max_position(&self, server_id: Uuid) -> sqlx::Result<Option<i32>> {
        let result: Option<Option<i32>> = sqlx::query_scalar::<_, Option<i32>>(
            "SELECT MAX(position) FROM channels WHERE server_id = $1",
        )
        .bind(server_id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(result.flatten())
    }

    pub async fn update(
        &self,
        channel_id: Uuid,
        name: Option<String>,
        position: Option<i32>,
    ) -> sqlx::Result<Option<Channel>> {
        sqlx::query_as::<_, Channel>(
            r#"
            UPDATE channels
            SET name = COALESCE($1, name), position = COALESCE($2, position), updated_at = NOW()
            WHERE id = $3
            RETURNING id, server_id, name, position, created_at, updated_at
            "#,
        )
        .bind(name)
        .bind(position)
        .bind(channel_id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn delete(&self, channel_id: Uuid) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM channels WHERE id = $1")
            .bind(channel_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
