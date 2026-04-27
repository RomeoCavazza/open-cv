use sqlx::PgPool;
use uuid::Uuid;

use crate::models::Invite;

#[derive(Clone)]
pub struct InviteRepository {
    pool: PgPool,
}

impl InviteRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    pub async fn create(
        &self,
        server_id: Uuid,
        created_by: Uuid,
        code: &str,
        max_uses: Option<i32>,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> sqlx::Result<Invite> {
        sqlx::query_as::<_, Invite>(
            r#"
            INSERT INTO invites (code, server_id, created_by, max_uses, expires_at, uses, revoked, created_at)
            VALUES ($1, $2, $3, $4, $5, 0, FALSE, NOW())
            RETURNING id, code, server_id, created_by, max_uses, uses, expires_at, revoked, created_at
            "#,
        )
        .bind(code)
        .bind(server_id)
        .bind(created_by)
        .bind(max_uses)
        .bind(expires_at)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn find_by_code(&self, code: &str) -> sqlx::Result<Option<Invite>> {
        sqlx::query_as::<_, Invite>(
            r#"
            SELECT id, code, server_id, created_by, max_uses, uses, expires_at, revoked, created_at
            FROM invites
            WHERE code = $1
            "#,
        )
        .bind(code)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn list_by_server(&self, server_id: Uuid) -> sqlx::Result<Vec<Invite>> {
        sqlx::query_as::<_, Invite>(
            "SELECT id, code, server_id, created_by, max_uses, uses, expires_at, revoked, created_at FROM invites WHERE server_id = $1 ORDER BY created_at DESC",
        )
        .bind(server_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn increment_uses(&self, invite_id: Uuid) -> sqlx::Result<()> {
        sqlx::query("UPDATE invites SET uses = uses + 1 WHERE id = $1")
            .bind(invite_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn increment_use_if_valid(&self, invite_id: Uuid) -> sqlx::Result<bool> {
        let res = sqlx::query(
            r#"
        UPDATE invites
        SET uses = uses + 1
        WHERE id = $1
          AND revoked = FALSE
          AND (expires_at IS NULL OR expires_at > NOW())
          AND (max_uses IS NULL OR uses < max_uses)
        "#,
        )
        .bind(invite_id)
        .execute(&self.pool)
        .await?;

        Ok(res.rows_affected() == 1)
    }

    pub async fn revoke(&self, invite_id: Uuid) -> sqlx::Result<()> {
        sqlx::query("UPDATE invites SET revoked = TRUE WHERE id = $1")
            .bind(invite_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
