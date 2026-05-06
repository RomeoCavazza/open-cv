use async_trait::async_trait;
use domain::{Offre, OffreId, Slug};
use ports::{OffreRepo, RepoError};
use sqlx::PgPool;

use crate::helpers::{build_offre, map_sqlx, OffreRow};

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
            build_offre(OffreRow {
                id: r.id,
                slug: r.slug,
                source_url: r.source_url,
                source_host: r.source_host,
                source_hash: r.source_hash,
                entreprise: r.entreprise,
                intitule: r.intitule,
                localisation: r.localisation,
                contrat: r.contrat,
                raw_text: r.raw_text,
                structured: r.structured,
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
            build_offre(OffreRow {
                id: r.id,
                slug: r.slug,
                source_url: r.source_url,
                source_host: r.source_host,
                source_hash: r.source_hash,
                entreprise: r.entreprise,
                intitule: r.intitule,
                localisation: r.localisation,
                contrat: r.contrat,
                raw_text: r.raw_text,
                structured: r.structured,
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
                build_offre(OffreRow {
                    id: r.id,
                    slug: r.slug,
                    source_url: r.source_url,
                    source_host: r.source_host,
                    source_hash: r.source_hash,
                    entreprise: r.entreprise,
                    intitule: r.intitule,
                    localisation: r.localisation,
                    contrat: r.contrat,
                    raw_text: r.raw_text,
                    structured: r.structured,
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
                build_offre(OffreRow {
                    id: r.id,
                    slug: r.slug,
                    source_url: r.source_url,
                    source_host: r.source_host,
                    source_hash: r.source_hash,
                    entreprise: r.entreprise,
                    intitule: r.intitule,
                    localisation: r.localisation,
                    contrat: r.contrat,
                    raw_text: r.raw_text,
                    structured: r.structured,
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

    async fn find_by_url(&self, url: &str) -> Result<Option<Offre>, RepoError> {
        let row = sqlx::query!(
            r#"
            SELECT id, slug, source_url, source_host, source_hash,
                   entreprise, intitule, localisation, contrat,
                   raw_text, structured, scraped_at, last_seen_at, closed_at, categorie
            FROM offres
            WHERE source_url = $1
            "#,
            url
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx)?;

        row.map(|r| {
            build_offre(OffreRow {
                id: r.id,
                slug: r.slug,
                source_url: r.source_url,
                source_host: r.source_host,
                source_hash: r.source_hash,
                entreprise: r.entreprise,
                intitule: r.intitule,
                localisation: r.localisation,
                contrat: r.contrat,
                raw_text: r.raw_text,
                structured: r.structured,
                scraped_at: r.scraped_at,
                last_seen_at: r.last_seen_at,
                closed_at: r.closed_at,
                categorie: r.categorie,
            })
        })
        .transpose()
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
            build_offre(OffreRow {
                id: r.id,
                slug: r.slug,
                source_url: r.source_url,
                source_host: r.source_host,
                source_hash: r.source_hash,
                entreprise: r.entreprise,
                intitule: r.intitule,
                localisation: r.localisation,
                contrat: r.contrat,
                raw_text: r.raw_text,
                structured: r.structured,
                scraped_at: r.scraped_at,
                last_seen_at: r.last_seen_at,
                closed_at: r.closed_at,
                categorie: r.categorie,
            })
        })
        .transpose()
    }
}
