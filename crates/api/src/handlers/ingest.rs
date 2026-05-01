use crate::errors::ApiError;
use crate::state::AppState;
use anyhow::Result;
use axum::{extract::State, Json};
use chrono::Utc;
use domain::{Offre, OffreId, OffreStructured, Slug};
use ports::Scraper;
use serde::Deserialize;
use sha2::Digest;
use uuid::Uuid;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct IngestConfig {
    pub resume: bool,
    pub cover: bool,
    pub analysis: bool,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct IngestRequest {
    pub input: String,
    pub config: Option<IngestConfig>,
    pub profil_id: Option<Uuid>,
}

pub async fn ingest_handler(
    State(state): State<AppState>,
    Json(payload): Json<IngestRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let lines: Vec<&str> = payload
        .input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();

    let mut results = Vec::new();

    // On utilise un client HTTP simple pour scrapper si c'est une URL
    let scraper = adapter_scraper_http::HttpScraper::new();

    // Récupération du profil à utiliser
    let profil = if let Some(pid) = payload.profil_id {
        state
            .generate_uc
            .profils
            .get_by_id(domain::ProfilId::from_uuid(pid))
            .await
            .map_err(|e| ApiError::Internal(e.to_string()))?
            .ok_or_else(|| ApiError::BadRequest(format!("Profil {} introuvable", pid)))?
    } else {
        state
            .generate_uc
            .profils
            .get_active()
            .await
            .map_err(|e| ApiError::Internal(e.to_string()))?
            .ok_or_else(|| ApiError::BadRequest("Aucun profil actif trouvé".to_string()))?
    };

    for item in lines {
        let (intitule, entreprise, content, url) = if item.starts_with("http") {
            // C'est une URL
            match scraper.scrape(item).await {
                Ok(res) => (
                    format!("Job from {}", item),
                    "Scrapped Corp".to_string(),
                    res.raw_text,
                    item.to_string(),
                ),
                Err(e) => {
                    return Err(ApiError::BadRequest(format!(
                        "Erreur de scraping pour {}: {}",
                        item, e
                    )));
                }
            }
        } else {
            // Texte brut
            (
                "Manual Job Entry".to_string(),
                "Unknown".to_string(),
                item.to_string(),
                "manual".to_string(),
            )
        };

        // Création de l'offre
        let id = OffreId::new();
        let slug_str = format!("job_{}", Uuid::new_v4().to_string()[..8].to_string());
        let slug = Slug::parse(&slug_str)
            .unwrap_or_else(|_| Slug::parse("job_default").expect("job_default is a valid slug"));

        let host = if url.starts_with("http") {
            url::Url::parse(&url)
                .ok()
                .and_then(|u| u.host_str().map(|h| h.to_string()))
                .unwrap_or_else(|| "external".to_string())
        } else {
            "manual".to_string()
        };

        let mut hasher = sha2::Sha256::new();
        hasher.update(content.as_bytes());
        let source_hash = hasher.finalize().to_vec();

        let existing_offre = state
            .offre_repo
            .find_by_content_hash(&host, &source_hash)
            .await
            .map_err(|e| ApiError::Internal(e.to_string()))?;

        let (offre_id, instance_slug) = if let Some(existing) = existing_offre {
            if state
                .instance_repo
                .get_by_slug(&existing.slug)
                .await
                .map_err(|e| ApiError::Internal(e.to_string()))?
                .is_some()
            {
                results.push(existing.slug.to_string());
                continue;
            }

            (existing.id, existing.slug)
        } else {
            let offre = Offre {
                id,
                slug: slug.clone(),
                source_url: url.clone(),
                source_host: host,
                source_hash,
                entreprise,
                intitule,
                localisation: None,
                contrat: None,
                raw_text: content,
                structured: OffreStructured {
                    resume_court: "Ingested via dashboard".into(),
                    stack: vec![],
                    missions: vec![],
                    exigences: vec![],
                    soft_skills: vec![],
                    niveau_etudes: None,
                    type_contrat: None,
                    mots_cles: vec![],
                },
                scraped_at: Utc::now(),
                last_seen_at: Utc::now(),
                closed_at: None,
                categorie: Some("Inbox".to_string()),
            };

            state
                .offre_repo
                .upsert(&offre)
                .await
                .map_err(|e| ApiError::Internal(e.to_string()))?;

            (offre.id, offre.slug)
        };

        // Création d'une instance brouillon associée
        let instance = domain::Instance {
            id: domain::InstanceId::new(),
            slug: instance_slug.clone(),
            offre_id,
            profil_id: profil.id,
            status: domain::InstanceStatus::Draft,
            resume_json: None,
            cover_letter_json: None,
            notes: serde_json::Value::Null,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            sent_at: None,
        };

        state
            .instance_repo
            .upsert(&instance)
            .await
            .map_err(|e| ApiError::Internal(e.to_string()))?;

        results.push(instance_slug.to_string());
    }

    Ok(Json(
        serde_json::json!({ "status": "ok", "ingested": results }),
    ))
}
