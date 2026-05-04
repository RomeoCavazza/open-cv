use crate::errors::ApiError;
use crate::state::AppState;
use application::intake::IntakeInput;
use axum::{extract::State, Json};
use serde::Deserialize;
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
    pub llm_provider: Option<String>,
}

pub async fn ingest_handler(
    State(state): State<AppState>,
    Json(payload): Json<IngestRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let raw_input = payload.input.trim().to_string();

    if raw_input.is_empty() {
        return Err(ApiError::BadRequest("input vide".to_string()));
    }

    // Résolution du profil
    let profil = if let Some(pid) = payload.profil_id {
        state
            .intake_uc
            .profils
            .get_by_id(domain::ProfilId::from_uuid(pid))
            .await
            .map_err(|e| ApiError::Internal(e.to_string()))?
            .ok_or_else(|| ApiError::BadRequest(format!("Profil {} introuvable", pid)))?
    } else {
        state
            .intake_uc
            .profils
            .get_active()
            .await
            .map_err(|e| ApiError::Internal(e.to_string()))?
            .ok_or_else(|| ApiError::BadRequest("Aucun profil actif trouvé".to_string()))?
    };

    // Détection : est-ce une ou plusieurs URLs, ou du texte brut ?
    let items = parse_input_items(&raw_input);

    let llm_provider = payload
        .llm_provider
        .as_ref()
        .and_then(|p| state.llm_registry.get(p))
        .cloned();

    let mut results = Vec::new();

    for item in items {
        let input = IntakeInput {
            raw_input: item,
            profil_id: profil.id,
        };

        match state.intake_uc.execute(input, llm_provider.clone()).await {
            Ok(output) => {
                // Si on a demandé des livrables (analyse, cv, lettre), on lance la génération
                if let Some(config) = &payload.config {
                    if config.analysis || config.resume || config.cover {
                        // On récupère l'instance qu'on vient de créer (ou l'existante)
                        if let Some(instance) = state
                            .instance_repo
                            .get_by_id(output.instance_id)
                            .await
                            .map_err(|e| ApiError::Internal(e.to_string()))?
                        {
                            let gen_input = application::generate::GenerateInput {
                                offre_id: instance.offre_id,
                                profil_id: profil.id,
                                existing_instance: Some(instance), // Crucial pour ne pas créer de doublon
                                livrables: application::generate::Livrables {
                                    restitution: config.analysis,
                                    resume: config.resume,
                                    cover_letter: config.cover,
                                },
                            };

                            // On lance la génération de manière synchrone
                            state
                                .generate_uc
                                .execute(gen_input, llm_provider.clone())
                                .await
                                .map_err(|e| {
                                    ApiError::Internal(format!("Erreur de génération : {}", e))
                                })?;
                        }
                    }
                }

                results.push(serde_json::json!({
                    "slug": output.instance_slug,
                    "duplicate": output.was_duplicate,
                }));
            }
            Err(e) => {
                return Err(ApiError::Internal(format!("Erreur d'ingestion : {}", e)));
            }
        }
    }

    Ok(Json(serde_json::json!({
        "status": "ok",
        "ingested": results.iter().map(|r| r["slug"].as_str().unwrap_or("")).collect::<Vec<_>>(),
    })))
}

/// Parse l'input pour distinguer :
/// - Plusieurs URLs (une par ligne) → chaque URL est un item séparé
/// - Texte brut (pas d'URL) → tout le bloc est UN SEUL item
/// - Mix → les URLs sont traitées individuellement, le reste est ignoré
fn parse_input_items(input: &str) -> Vec<String> {
    let lines: Vec<&str> = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();

    // Compter les URLs
    let url_count = lines
        .iter()
        .filter(|l| l.starts_with("http://") || l.starts_with("https://"))
        .count();

    if url_count > 0 {
        // Mode URL : chaque URL est un item distinct
        // Les lignes non-URL sont ignorées (probablement des séparateurs)
        lines
            .into_iter()
            .filter(|l| l.starts_with("http://") || l.starts_with("https://"))
            .map(|l| l.to_string())
            .collect()
    } else {
        // Mode texte brut : TOUT le bloc = une seule offre
        vec![input.to_string()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_url() {
        let items = parse_input_items("https://example.com/job");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0], "https://example.com/job");
    }

    #[test]
    fn multiple_urls() {
        let input = "https://example.com/job1\nhttps://example.com/job2\nhttps://example.com/job3";
        let items = parse_input_items(input);
        assert_eq!(items.len(), 3);
    }

    #[test]
    fn raw_text_is_single_item() {
        let input = "Alternance Data Analyst\nChez Safran\nMissions:\n- Analyser des données\n- Créer des dashboards";
        let items = parse_input_items(input);
        assert_eq!(items.len(), 1);
        assert!(items[0].contains("Safran"));
        assert!(items[0].contains("dashboards"));
    }

    #[test]
    fn mixed_urls_and_text() {
        let input = "https://example.com/job1\nsome random text\nhttps://example.com/job2";
        let items = parse_input_items(input);
        assert_eq!(items.len(), 2); // Only URLs
    }
}
