//! Adapter Postgres — implémente les traits de `ports::repos`.
//!
//! Phase 0 : seulement `OffreRepoPg` et `InstanceRepoPg`. Les autres
//! viendront en Phase 1-2.

use async_trait::async_trait;
use domain::{
    Instance, InstanceId, InstanceStatus, Offre, OffreId, OffreStructured, Slug,
};
use ports::{InstanceRepo, OffreRepo, RepoError};
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
                   raw_text, structured, scraped_at, last_seen_at, closed_at
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
            })
        })
        .transpose()
    }

    async fn get_by_slug(&self, slug: &Slug) -> Result<Option<Offre>, RepoError> {
        let row = sqlx::query!(
            r#"
            SELECT id, slug, source_url, source_host, source_hash,
                   entreprise, intitule, localisation, contrat,
                   raw_text, structured, scraped_at, last_seen_at, closed_at
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
            })
        })
        .transpose()
    }

    async fn list_recent(&self, limit: u32) -> Result<Vec<Offre>, RepoError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, slug, source_url, source_host, source_hash,
                   entreprise, intitule, localisation, contrat,
                   raw_text, structured, scraped_at, last_seen_at, closed_at
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
                    slug: Slug::parse(r.slug)
                        .map_err(|e| RepoError::Other(e.to_string()))?,
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
                raw_text, structured, scraped_at, last_seen_at, closed_at
            )
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14)
            ON CONFLICT (id) DO UPDATE SET
                last_seen_at = EXCLUDED.last_seen_at,
                closed_at    = EXCLUDED.closed_at,
                structured   = EXCLUDED.structured
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
                   raw_text, structured, scraped_at, last_seen_at, closed_at
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
        let row = sqlx::query!(
            r#"
            SELECT id, slug, offre_id, profil_id, status::text AS "status!",
                   resume_json, cover_letter_json, notes,
                   created_at, updated_at, sent_at
            FROM instances
            WHERE id = $1
            "#,
            id.as_uuid()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx)?;

        row.map(|r| {
            Ok(Instance {
                id: InstanceId::from_uuid(r.id),
                slug: Slug::parse(r.slug).map_err(|e| RepoError::Other(e.to_string()))?,
                offre_id: domain::OffreId::from_uuid(r.offre_id),
                profil_id: domain::ProfilId::from_uuid(r.profil_id),
                status: parse_status(&r.status)?,
                resume_json: r.resume_json,
                cover_letter_json: r.cover_letter_json,
                notes: r.notes,
                created_at: r.created_at,
                updated_at: r.updated_at,
                sent_at: r.sent_at,
            })
        })
        .transpose()
    }

    async fn get_by_slug(&self, slug: &Slug) -> Result<Option<Instance>, RepoError> {
        let row = sqlx::query!(
            r#"
            SELECT id, slug, offre_id, profil_id, status::text AS "status!",
                   resume_json, cover_letter_json, notes,
                   created_at, updated_at, sent_at
            FROM instances
            WHERE slug = $1
            "#,
            slug.as_str()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx)?;

        row.map(|r| {
            Ok(Instance {
                id: InstanceId::from_uuid(r.id),
                slug: Slug::parse(r.slug).map_err(|e| RepoError::Other(e.to_string()))?,
                offre_id: domain::OffreId::from_uuid(r.offre_id),
                profil_id: domain::ProfilId::from_uuid(r.profil_id),
                status: parse_status(&r.status)?,
                resume_json: r.resume_json,
                cover_letter_json: r.cover_letter_json,
                notes: r.notes,
                created_at: r.created_at,
                updated_at: r.updated_at,
                sent_at: r.sent_at,
            })
        })
        .transpose()
    }

    async fn list_recent(&self, limit: u32) -> Result<Vec<Instance>, RepoError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, slug, offre_id, profil_id, status::text AS "status!",
                   resume_json, cover_letter_json, notes,
                   created_at, updated_at, sent_at
            FROM instances
            ORDER BY created_at DESC
            LIMIT $1
            "#,
            limit as i64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx)?;

        rows.into_iter()
            .map(|r| {
                Ok(Instance {
                    id: InstanceId::from_uuid(r.id),
                    slug: Slug::parse(r.slug)
                        .map_err(|e| RepoError::Other(e.to_string()))?,
                    offre_id: domain::OffreId::from_uuid(r.offre_id),
                    profil_id: domain::ProfilId::from_uuid(r.profil_id),
                    status: parse_status(&r.status)?,
                    resume_json: r.resume_json,
                    cover_letter_json: r.cover_letter_json,
                    notes: r.notes,
                    created_at: r.created_at,
                    updated_at: r.updated_at,
                    sent_at: r.sent_at,
                })
            })
            .collect()
    }

    async fn upsert(&self, _instance: &Instance) -> Result<(), RepoError> {
        // Phase 1 : implémentation complète. Pour l'instant on lit seulement.
        Err(RepoError::Other(
            "InstanceRepoPg::upsert pas encore implémenté (Phase 1)".into(),
        ))
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
