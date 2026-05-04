use crate::errors::ApiError;
use crate::state::AppState;
use application::intake::IntakeInput;
use axum::{extract::State, Json};
use serde::Deserialize;
use uuid::Uuid;

mod helpers;

use helpers::{build_generate_input, parse_input_items, resolve_ingest_profile, should_generate};

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

    let profil =
        resolve_ingest_profile(state.intake_uc.profils.as_ref(), payload.profil_id).await?;
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
                if should_generate(payload.config.as_ref()) {
                    if let Some(instance) = state
                        .instance_repo
                        .get_by_id(output.instance_id)
                        .await
                        .map_err(|e| ApiError::Internal(e.to_string()))?
                    {
                        let gen_input = build_generate_input(
                            instance,
                            profil.id,
                            payload.config.as_ref().expect("config presence checked by should_generate"),
                        );

                        state
                            .generate_uc
                            .execute(gen_input, llm_provider.clone())
                            .await
                            .map_err(|e| {
                                ApiError::Internal(format!("Erreur de génération : {}", e))
                            })?;
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
