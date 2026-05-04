//! Adapter Postgres — implémente les traits de `ports::repos`.
//!
//! Phase 0 : seulement `OffreRepoPg` et `InstanceRepoPg`. Les autres
//! viendront en Phase 1-2.

use async_trait::async_trait;
use domain::{
    Annexe, AnnexeId, Instance, InstanceId, InstanceStatus, Message, MessageRole, Offre, OffreId,
    OffreStructured, ProfilId, Slug,
};
use ports::{AnnexeRepo, InstanceRepo, MessageRepo, OffreRepo, RepoError};
use sqlx::PgPool;

pub struct OffreRepoPg {
    pool: PgPool,
}

impl OffreRepoPg {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OffreRepo for OffreRepoPg {
    async fn get_by_id(&self, id: OffreId) -> Result<Option<Offre>, RepoError> {
        let row = sqlx::query!(
            r#"
            SELECT id, slug, source_url, source_host, source_hash,
                   entreprise, intitule, localisation, contrat,
                   raw_text, structured, scraped_at, last_seen_at, closed_at, categorie
            FROM offres
            WHERE id = $1
            "#,
            id.as_uuid()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx)?;

        row.map(|r| {
            Ok(Offre {
                id: OffreId::from_uuid(r.id),
                slug: Slug::parse(r.slug).map_err(|e| RepoError::Other(e.to_string()))?,
                source_url: r.source_url,
                source_host: r.source_host,
                source_hash: r.source_hash,
                entreprise: r.entreprise,
                intitule: r.intitule,
                localisation: r.localisation,
                contrat: r.contrat,
                raw_text: r.raw_text,
                structured: serde_json::from_value::<OffreStructured>(r.structured)
                    .map_err(|e| RepoError::Other(e.to_string()))?,
                scraped_at: r.scraped_at,
                last_seen_at: r.last_seen_at,
                closed_at: r.closed_at,
                categorie: r.categorie,
            })
        })
        .transpose()
    }

    async fn get_by_slug(&self, slug: &Slug) -> Result<Option<Offre>, RepoError> {
        let row = sqlx::query!(
            r#"
            SELECT id, slug, source_url, source_host, source_hash,
                   entreprise, intitule, localisation, contrat,
                   raw_text, structured, scraped_at, last_seen_at, closed_at, categorie
            FROM offres
            WHERE slug = $1
            "#,
            slug.as_str()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx)?;

        row.map(|r| {
            Ok(Offre {
                id: OffreId::from_uuid(r.id),
                slug: Slug::parse(r.slug).map_err(|e| RepoError::Other(e.to_string()))?,
                source_url: r.source_url,
                source_host: r.source_host,
                source_hash: r.source_hash,
                entreprise: r.entreprise,
                intitule: r.intitule,
                localisation: r.localisation,
                contrat: r.contrat,
                raw_text: r.raw_text,
                structured: serde_json::from_value::<OffreStructured>(r.structured)
                    .map_err(|e| RepoError::Other(e.to_string()))?,
                scraped_at: r.scraped_at,
                last_seen_at: r.last_seen_at,
                closed_at: r.closed_at,
                categorie: r.categorie,
            })
        })
        .transpose()
    }

    async fn list_all(&self) -> Result<Vec<Offre>, RepoError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, slug, source_url, source_host, source_hash,
                   entreprise, intitule, localisation, contrat,
                   raw_text, structured, scraped_at, last_seen_at, closed_at, categorie
            FROM offres
            ORDER BY scraped_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx)?;

        rows.into_iter()
            .map(|r| {
                Ok(Offre {
                    id: OffreId::from_uuid(r.id),
                    slug: Slug::parse(r.slug).map_err(|e| RepoError::Other(e.to_string()))?,
                    source_url: r.source_url,
                    source_host: r.source_host,
                    source_hash: r.source_hash,
                    entreprise: r.entreprise,
                    intitule: r.intitule,
                    localisation: r.localisation,
                    contrat: r.contrat,
                    raw_text: r.raw_text,
                    structured: serde_json::from_value::<OffreStructured>(r.structured)
                        .map_err(|e| RepoError::Other(e.to_string()))?,
                    scraped_at: r.scraped_at,
                    last_seen_at: r.last_seen_at,
                    closed_at: r.closed_at,
                    categorie: r.categorie,
                })
            })
            .collect()
    }

    async fn list_recent(&self, limit: u32) -> Result<Vec<Offre>, RepoError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, slug, source_url, source_host, source_hash,
                   entreprise, intitule, localisation, contrat,
                   raw_text, structured, scraped_at, last_seen_at, closed_at, categorie
            FROM offres
            ORDER BY scraped_at DESC
            LIMIT $1
            "#,
            limit as i64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx)?;

        rows.into_iter()
            .map(|r| {
                Ok(Offre {
                    id: OffreId::from_uuid(r.id),
                    slug: Slug::parse(r.slug).map_err(|e| RepoError::Other(e.to_string()))?,
                    source_url: r.source_url,
                    source_host: r.source_host,
                    source_hash: r.source_hash,
                    entreprise: r.entreprise,
                    intitule: r.intitule,
                    localisation: r.localisation,
                    contrat: r.contrat,
                    raw_text: r.raw_text,
                    structured: serde_json::from_value::<OffreStructured>(r.structured)
                        .map_err(|e| RepoError::Other(e.to_string()))?,
                    scraped_at: r.scraped_at,
                    last_seen_at: r.last_seen_at,
                    closed_at: r.closed_at,
                    categorie: r.categorie,
                })
            })
            .collect()
    }

    async fn upsert(&self, offre: &Offre) -> Result<(), RepoError> {
        let structured =
            serde_json::to_value(&offre.structured).map_err(|e| RepoError::Other(e.to_string()))?;

        sqlx::query!(
            r#"
            INSERT INTO offres (
                id, slug, source_url, source_host, source_hash,
                entreprise, intitule, localisation, contrat,
                raw_text, structured, scraped_at, last_seen_at, closed_at, categorie
            )
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15)
            ON CONFLICT (id) DO UPDATE SET
                last_seen_at = EXCLUDED.last_seen_at,
                closed_at    = EXCLUDED.closed_at,
                structured   = EXCLUDED.structured,
                categorie    = EXCLUDED.categorie
            "#,
            offre.id.as_uuid(),
            offre.slug.as_str(),
            offre.source_url,
            offre.source_host,
            offre.source_hash,
            offre.entreprise,
            offre.intitule,
            offre.localisation,
            offre.contrat,
            offre.raw_text,
            structured,
            offre.scraped_at,
            offre.last_seen_at,
            offre.closed_at,
            offre.categorie,
        )
        .execute(&self.pool)
        .await
        .map_err(map_sqlx)?;

        Ok(())
    }

    async fn count(&self) -> Result<u64, RepoError> {
        let row = sqlx::query!("SELECT COUNT(*) AS c FROM offres")
            .fetch_one(&self.pool)
            .await
            .map_err(map_sqlx)?;

        Ok(row.c.unwrap_or(0) as u64)
    }

    async fn find_by_content_hash(
        &self,
        source_host: &str,
        hash: &[u8],
    ) -> Result<Option<Offre>, RepoError> {
        let row = sqlx::query!(
            r#"
            SELECT id, slug, source_url, source_host, source_hash,
                   entreprise, intitule, localisation, contrat,
                   raw_text, structured, scraped_at, last_seen_at, closed_at, categorie
            FROM offres
            WHERE source_host = $1 AND source_hash = $2
            "#,
            source_host,
            hash
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx)?;

        row.map(|r| {
            Ok(Offre {
                id: OffreId::from_uuid(r.id),
                slug: Slug::parse(r.slug).map_err(|e| RepoError::Other(e.to_string()))?,
                source_url: r.source_url,
                source_host: r.source_host,
                source_hash: r.source_hash,
                entreprise: r.entreprise,
                intitule: r.intitule,
                localisation: r.localisation,
                contrat: r.contrat,
                raw_text: r.raw_text,
                structured: serde_json::from_value::<OffreStructured>(r.structured)
                    .map_err(|e| RepoError::Other(e.to_string()))?,
                scraped_at: r.scraped_at,
                last_seen_at: r.last_seen_at,
                closed_at: r.closed_at,
                categorie: r.categorie,
            })
        })
        .transpose()
    }
}

// ─────────────────────────────────────────────────────────────────
// InstanceRepoPg — minimal pour Phase 0
// ─────────────────────────────────────────────────────────────────

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
            Ok(Instance {
                id: InstanceId::from_uuid(r.get("id")),
                slug: Slug::parse(r.get::<String, _>("slug"))
                    .map_err(|e| RepoError::Other(e.to_string()))?,
                offre_id: domain::OffreId::from_uuid(r.get("offre_id")),
                profil_id: domain::ProfilId::from_uuid(r.get("profil_id")),
                status: parse_status(r.get("status"))?,
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
            Ok(Instance {
                id: InstanceId::from_uuid(r.get("id")),
                slug: Slug::parse(r.get::<String, _>("slug"))
                    .map_err(|e| RepoError::Other(e.to_string()))?,
                offre_id: domain::OffreId::from_uuid(r.get("offre_id")),
                profil_id: domain::ProfilId::from_uuid(r.get("profil_id")),
                status: parse_status(r.get("status"))?,
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
                Ok(Instance {
                    id: InstanceId::from_uuid(r.get("id")),
                    slug: Slug::parse(r.get::<String, _>("slug"))
                        .map_err(|e| RepoError::Other(e.to_string()))?,
                    offre_id: domain::OffreId::from_uuid(r.get("offre_id")),
                    profil_id: domain::ProfilId::from_uuid(r.get("profil_id")),
                    status: parse_status(r.get("status"))?,
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
        .bind(instance.restitution.clone())
        .bind(instance.resume_json.clone())
        .bind(instance.cover_letter_json.clone())
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
            Ok(Instance {
                id: InstanceId::from_uuid(r.get("id")),
                slug: Slug::parse(r.get::<String, _>("slug"))
                    .map_err(|e| RepoError::Other(e.to_string()))?,
                offre_id: domain::OffreId::from_uuid(r.get("offre_id")),
                profil_id: domain::ProfilId::from_uuid(r.get("profil_id")),
                status: parse_status(r.get("status"))?,
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

// ─────────────────────────────────────────────────────────────────
// AnnexeRepoPg
// ─────────────────────────────────────────────────────────────────

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

        Ok(row.map(|r| Annexe {
            id: AnnexeId::from_uuid(r.id),
            profil_id: ProfilId::from_uuid(r.profil_id),
            label: r.label,
            filename: r.filename,
            content_type: r.content_type,
            content: r.content,
            created_at: r.created_at,
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
            .map(|r| Annexe {
                id: AnnexeId::from_uuid(r.id),
                profil_id: ProfilId::from_uuid(r.profil_id),
                label: r.label,
                filename: r.filename,
                content_type: r.content_type,
                content: r.content,
                created_at: r.created_at,
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

// ─────────────────────────────────────────────────────────────────
// MessageRepoPg — Pour le chat V1
// ─────────────────────────────────────────────────────────────────

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
            .map(|r| {
                Ok(Message {
                    id: r.id,
                    instance_id: InstanceId::from_uuid(r.instance_id),
                    role: parse_role(&r.role)?,
                    content: r.content,
                    created_at: r.created_at,
                })
            })
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
            .map(|r| {
                Ok(Message {
                    id: r.id,
                    instance_id: InstanceId::from_uuid(r.instance_id),
                    role: parse_role(&r.role)?,
                    content: r.content,
                    created_at: r.created_at,
                })
            })
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

// ─────────────────────────────────────────────────────────────────
// ProfilRepo
// ─────────────────────────────────────────────────────────────────

pub struct ProfilRepoPg {
    pool: PgPool,
}

impl ProfilRepoPg {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ports::ProfilRepo for ProfilRepoPg {
    async fn get_active(&self) -> Result<Option<domain::Profil>, ports::RepoError> {
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

        Ok(row.map(|r| domain::Profil {
            id: domain::ProfilId::from_uuid(r.id),
            label: r.label,
            content: r.content,
            is_active: r.is_active,
            profile_photo: r.profile_photo,
            calendar_pdf: r.calendar_pdf,
            resume_template: r.resume_template,
            cover_letter_template: r.cover_letter_template,
            notes: r.notes,
            created_at: r.created_at,
        }))
    }

    async fn get_by_id(
        &self,
        id: domain::ProfilId,
    ) -> Result<Option<domain::Profil>, ports::RepoError> {
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

        Ok(row.map(|r| domain::Profil {
            id: domain::ProfilId::from_uuid(r.id),
            label: r.label,
            content: r.content,
            is_active: r.is_active,
            profile_photo: r.profile_photo,
            calendar_pdf: r.calendar_pdf,
            resume_template: r.resume_template,
            cover_letter_template: r.cover_letter_template,
            notes: r.notes,
            created_at: r.created_at,
        }))
    }

    async fn list_all(&self) -> Result<Vec<domain::Profil>, ports::RepoError> {
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
            .map(|r| domain::Profil {
                id: domain::ProfilId::from_uuid(r.id),
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
            .collect())
    }

    async fn upsert(&self, profil: &domain::Profil) -> Result<(), ports::RepoError> {
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
            profil.content,
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

// ─────────────────────────────────────────────────────────────────
// ChunkRepo
// ─────────────────────────────────────────────────────────────────

pub struct ChunkRepoPg {
    pool: PgPool,
}

impl ChunkRepoPg {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ports::ChunkRepo for ChunkRepoPg {
    async fn top_k_by_embedding(
        &self,
        profil_id: domain::ProfilId,
        embedding: &[f32],
        limit: u32,
    ) -> Result<Vec<(domain::Chunk, f32)>, ports::RepoError> {
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
                let id: uuid::Uuid = r.get("id");
                let profil_id: uuid::Uuid = r.get("profil_id");
                let kind_str: String = r.get("kind");
                let titre: String = r.get("titre");
                let content: String = r.get("content");
                let metadata: serde_json::Value = r.get("metadata");
                let emb_str: String = r.get("embedding");
                let distance: f64 = r.get("distance");
                let created_at: chrono::DateTime<chrono::Utc> = r.get("created_at");

                let emb: Vec<f32> = emb_str
                    .trim_start_matches('[')
                    .trim_end_matches(']')
                    .split(',')
                    .map(|s| s.parse().unwrap_or(0.0))
                    .collect();

                let chunk = domain::Chunk {
                    id: domain::ChunkId::from_uuid(id),
                    profil_id: domain::ProfilId::from_uuid(profil_id),
                    kind: match kind_str.as_str() {
                        "experience" => domain::ChunkKind::Experience,
                        "projet" => domain::ChunkKind::Projet,
                        "formation" => domain::ChunkKind::Formation,
                        "competence" => domain::ChunkKind::Competence,
                        "phrase_lettre" => domain::ChunkKind::PhraseLettre,
                        _ => domain::ChunkKind::Experience,
                    },
                    titre,
                    content,
                    metadata,
                    embedding: emb,
                    created_at,
                };
                Ok((chunk, distance as f32))
            })
            .collect()
    }

    async fn upsert(&self, chunk: &domain::Chunk) -> Result<(), ports::RepoError> {
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
        .bind(&chunk.metadata)
        .bind(embedding_str)
        .bind(chunk.created_at)
        .execute(&self.pool)
        .await
        .map_err(map_sqlx)?;
        Ok(())
    }
}

fn parse_status(s: &str) -> Result<InstanceStatus, RepoError> {
    match s {
        "draft" => Ok(InstanceStatus::Draft),
        "generating" => Ok(InstanceStatus::Generating),
        "ready" => Ok(InstanceStatus::Ready),
        "sent" => Ok(InstanceStatus::Sent),
        "archived" => Ok(InstanceStatus::Archived),
        "failed" => Ok(InstanceStatus::Failed),
        _ => Err(RepoError::Other(format!("statut inconnu : {s}"))),
    }
}

fn parse_role(s: &str) -> Result<MessageRole, RepoError> {
    match s {
        "user" => Ok(MessageRole::User),
        "assistant" => Ok(MessageRole::Assistant),
        "system" => Ok(MessageRole::System),
        _ => Err(RepoError::Other(format!("rôle inconnu : {s}"))),
    }
}

fn map_sqlx(e: sqlx::Error) -> RepoError {
    match &e {
        sqlx::Error::Database(db) if db.is_unique_violation() => {
            RepoError::UniqueViolation(db.message().to_string())
        }
        _ => RepoError::Sql(e.to_string()),
    }
}

/// Crée le pool Postgres et exécute `MIGRATE` au démarrage.
pub async fn connect(database_url: &str) -> Result<PgPool, sqlx::Error> {
    use sqlx::postgres::PgPoolOptions;
    PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(database_url)
        .await
}
