use async_trait::async_trait;
use domain::Profil;
use ports::{ProfilRepo, RepoError};
use sqlx::PgPool;

use crate::helpers::{build_profil, map_sqlx, ProfilRow};

pub struct ProfilRepoPg {
    pool: PgPool,
}

impl ProfilRepoPg {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProfilRepo for ProfilRepoPg {
    async fn get_active(&self) -> Result<Option<Profil>, RepoError> {
        let row = sqlx::query!(
            r#"
            SELECT id, label, content, is_active, profile_photo, calendar_pdf, resume_template, cover_letter_template, notes, created_at
            FROM profils
            WHERE is_active = true
            LIMIT 1
            "#
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx)?;

        Ok(row.map(|r| {
            build_profil(ProfilRow {
                id: r.id,
                label: r.label,
                content: r.content,
                is_active: r.is_active,
                profile_photo: r.profile_photo,
                calendar_pdf: r.calendar_pdf,
                resume_template: r.resume_template,
                cover_letter_template: r.cover_letter_template,
                notes: r.notes,
                created_at: r.created_at,
            })
        }))
    }

    async fn get_by_id(&self, id: domain::ProfilId) -> Result<Option<Profil>, RepoError> {
        let row = sqlx::query!(
            r#"
            SELECT id, label, content, is_active, profile_photo, calendar_pdf, resume_template, cover_letter_template, notes, created_at
            FROM profils
            WHERE id = $1
            "#,
            id.as_uuid()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx)?;

        Ok(row.map(|r| {
            build_profil(ProfilRow {
                id: r.id,
                label: r.label,
                content: r.content,
                is_active: r.is_active,
                profile_photo: r.profile_photo,
                calendar_pdf: r.calendar_pdf,
                resume_template: r.resume_template,
                cover_letter_template: r.cover_letter_template,
                notes: r.notes,
                created_at: r.created_at,
            })
        }))
    }

    async fn list_all(&self) -> Result<Vec<Profil>, RepoError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, label, content, is_active, profile_photo, calendar_pdf, resume_template, cover_letter_template, notes, created_at
            FROM profils
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx)?;

        Ok(rows
            .into_iter()
            .map(|r| {
                build_profil(ProfilRow {
                    id: r.id,
                    label: r.label,
                    content: r.content,
                    is_active: r.is_active,
                    profile_photo: r.profile_photo,
                    calendar_pdf: r.calendar_pdf,
                    resume_template: r.resume_template,
                    cover_letter_template: r.cover_letter_template,
                    notes: r.notes,
                    created_at: r.created_at,
                })
            })
            .collect())
    }

    async fn upsert(&self, profil: &Profil) -> Result<(), RepoError> {
        sqlx::query!(
            r#"
            INSERT INTO profils (id, label, content, is_active, profile_photo, calendar_pdf, resume_template, cover_letter_template, notes, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (id) DO UPDATE SET
                label = EXCLUDED.label,
                content = EXCLUDED.content,
                is_active = EXCLUDED.is_active,
                profile_photo = COALESCE(EXCLUDED.profile_photo, profils.profile_photo),
                calendar_pdf = COALESCE(EXCLUDED.calendar_pdf, profils.calendar_pdf),
                resume_template = COALESCE(EXCLUDED.resume_template, profils.resume_template),
                cover_letter_template = COALESCE(EXCLUDED.cover_letter_template, profils.cover_letter_template),
                notes = EXCLUDED.notes
            "#,
            profil.id.as_uuid(),
            profil.label,
            serde_json::to_value(&profil.content).expect("ProfilContent is always serializable"),
            profil.is_active,
            profil.profile_photo,
            profil.calendar_pdf,
            profil.resume_template,
            profil.cover_letter_template,
            profil.notes,
            profil.created_at
        )
        .execute(&self.pool)
        .await
        .map_err(map_sqlx)?;
        Ok(())
    }
}
