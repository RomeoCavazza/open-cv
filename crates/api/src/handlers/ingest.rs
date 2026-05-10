use crate::errors::ApiError;
use crate::state::AppState;
use application::intake::IntakeInput;
use axum::{extract::State, Json};
use futures::stream::{self, StreamExt};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Semaphore;
use uuid::Uuid;

mod logic;

use logic::{build_generate_input, parse_input_items, resolve_ingest_profile, should_generate};

const MAX_INGEST_ITEMS: usize = 5;
const DEFAULT_INGEST_MAX_PARALLEL: usize = 2;
const DEFAULT_INGEST_PER_HOST_MAX_PARALLEL: usize = 1;
const ABSOLUTE_INGEST_MAX_PARALLEL: usize = 8;

#[derive(Clone, Deserialize)]
#[allow(dead_code)]
pub struct IngestConfig {
    pub resume: bool,
    pub cover: bool,
    pub restitution: bool,
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

    let parsed_items = parse_input_items(&raw_input);
    let rejected_count = parsed_items.len().saturating_sub(MAX_INGEST_ITEMS);
    let items = parsed_items
        .into_iter()
        .take(MAX_INGEST_ITEMS)
        .collect::<Vec<_>>();
    let accepted_count = items.len();

    let profil =
        resolve_ingest_profile(state.intake_uc.profils.as_ref(), payload.profil_id).await?;

    let llm_provider = payload
        .llm_provider
        .as_ref()
        .and_then(|p| state.llm_registry.get(p))
        .cloned();
    let should_run_generation = should_generate(payload.config.as_ref());
    let config = payload.config.clone();
    let profil_id = profil.id;

    let ingest_parallelism = read_usize_env(
        "INGEST_MAX_PARALLEL",
        DEFAULT_INGEST_MAX_PARALLEL,
        ABSOLUTE_INGEST_MAX_PARALLEL,
    );
    let per_host_parallelism = read_usize_env(
        "INGEST_PER_HOST_MAX_PARALLEL",
        DEFAULT_INGEST_PER_HOST_MAX_PARALLEL,
        ABSOLUTE_INGEST_MAX_PARALLEL,
    );

    // Per-host limiter: avoid hammering a single site (anti-bot / 423), while still parallelizing across domains.
    let mut host_limiters: HashMap<String, Arc<Semaphore>> = HashMap::new();
    let indexed_items = items
        .into_iter()
        .enumerate()
        .map(|(idx, item)| {
            let host_key = host_key_for_item(&item);
            host_limiters
                .entry(host_key.clone())
                .or_insert_with(|| Arc::new(Semaphore::new(per_host_parallelism)));
            (idx, item, host_key)
        })
        .collect::<Vec<_>>();

    let host_limiters = Arc::new(host_limiters);

    let task_results = stream::iter(indexed_items.into_iter().map(|(idx, item, host_key)| {
        let state = state.clone();
        let llm_provider = llm_provider.clone();
        let host_limiters = host_limiters.clone();
        let config = config.clone();

        async move {
            let Some(host_limiter) = host_limiters.get(&host_key).cloned() else {
                return (
                    idx,
                    Vec::new(),
                    0usize,
                    Some(serde_json::json!({
                        "input": item,
                        "error": "Limiter introuvable pour cet hôte",
                    })),
                );
            };

            let _host_permit = match host_limiter.acquire_owned().await {
                Ok(permit) => permit,
                Err(_) => {
                    return (
                        idx,
                        Vec::new(),
                        0usize,
                        Some(serde_json::json!({
                            "input": item,
                            "error": "Limiter fermé pour cet hôte",
                        })),
                    );
                }
            };

            let input = IntakeInput {
                raw_input: item.clone(),
                profil_id,
            };

            match state.intake_uc.execute(input, llm_provider.clone()).await {
                Ok(exec_result) => {
                    let outputs = exec_result.outputs;
                    let ignored_count = exec_result.ignored_count;
                    let mut item_results = Vec::new();

                    for output in outputs {
                        if !output.was_duplicate && should_run_generation {
                            let cfg = match config.as_ref() {
                                Some(c) => c,
                                None => {
                                    return (
                                        idx,
                                        item_results,
                                        ignored_count,
                                        Some(serde_json::json!({
                                            "input": item,
                                            "error": "Configuration de génération absente",
                                        })),
                                    );
                                }
                            };

                            let instance = match state
                                .instance_repo
                                .get_by_id(output.instance_id)
                                .await
                            {
                                Ok(Some(instance)) => instance,
                                Ok(None) => {
                                    return (
                                        idx,
                                        item_results,
                                        ignored_count,
                                        Some(serde_json::json!({
                                            "input": item,
                                            "error": "Instance introuvable juste après ingestion",
                                        })),
                                    );
                                }
                                Err(e) => {
                                    return (
                                        idx,
                                        item_results,
                                        ignored_count,
                                        Some(serde_json::json!({
                                            "input": item,
                                            "error": format!("Erreur DB instance: {}", e),
                                        })),
                                    );
                                }
                            };

                            let gen_input = build_generate_input(instance, profil_id, cfg);

                            if let Err(e) = state
                                .generate_uc
                                .execute(gen_input, llm_provider.clone())
                                .await
                            {
                                tracing::error!(
                                    error = %e,
                                    "Erreur lors de la génération des livrables"
                                );
                                return (
                                    idx,
                                    item_results,
                                    ignored_count,
                                    Some(serde_json::json!({
                                        "input": item,
                                        "error": format!("Erreur de génération : {}", e),
                                    })),
                                );
                            }
                        }

                        item_results.push(serde_json::json!({
                            "offer_slug": output.offre_slug,
                            "instance_slug": output.instance_slug,
                            "duplicate": output.was_duplicate,
                        }));
                    }

                    (idx, item_results, ignored_count, None)
                }
                Err(e) => {
                    tracing::error!(error = %e, input = %item, "Échec de l'ingestion d'un item");
                    (
                        idx,
                        Vec::new(),
                        0usize,
                        Some(serde_json::json!({
                            "input": item,
                            "error": e.to_string(),
                        })),
                    )
                }
            }
        }
    }))
    .buffer_unordered(ingest_parallelism)
    .collect::<Vec<_>>()
    .await;

    let mut ordered_task_results = task_results;
    ordered_task_results.sort_by_key(|(idx, _, _, _)| *idx);

    let mut results = Vec::new();
    let mut item_errors = Vec::new();
    let mut ignored_extracted_count = 0usize;
    for (_, item_results, ignored_count, maybe_error) in ordered_task_results {
        results.extend(item_results);
        ignored_extracted_count += ignored_count;
        if let Some(err) = maybe_error {
            item_errors.push(err);
        }
    }

    if results.is_empty() {
        let detail = item_errors
            .first()
            .and_then(|e| e.get("error"))
            .and_then(|e| e.as_str())
            .unwrap_or("Aucun item ingéré.");
        return Err(ApiError::BadRequest(format!(
            "Aucune offre n'a pu être ingérée. {detail}"
        )));
    }

    let job_id = results
        .first()
        .and_then(|r| r["offer_slug"].as_str())
        .unwrap_or("");
    let instance_id = results
        .first()
        .and_then(|r| r["instance_slug"].as_str())
        .unwrap_or("");
    let failed_count = item_errors.len();
    let ignored_demands_count = rejected_count + ignored_extracted_count;
    let warning = {
        let mut parts = Vec::new();
        if rejected_count > 0 {
            parts.push(format!(
                "Seules {} demande(s) ont été prises en compte. {} demande(s) en trop ont été ignorées.",
                MAX_INGEST_ITEMS,
                rejected_count
            ));
        }
        if ignored_extracted_count > 0 {
            parts.push(format!(
                "{} demande(s) ignorée(s) (limite {} par prompt).",
                ignored_extracted_count, MAX_INGEST_ITEMS
            ));
        }
        if failed_count > 0 {
            parts.push(format!(
                "{} demande(s) rejetée(s) (403/anti-bot, etc.).",
                failed_count
            ));
        }
        if parts.is_empty() {
            None
        } else {
            Some(parts.join(" "))
        }
    };

    Ok(Json(serde_json::json!({
        "status": "ok",
        "job_id": job_id,
        "instance_id": instance_id,
        "queue_limit": MAX_INGEST_ITEMS,
        "accepted_count": accepted_count,
        "rejected_count": rejected_count,
        "ignored_extracted_count": ignored_extracted_count,
        "ignored_demands_count": ignored_demands_count,
        "failed_count": failed_count,
        "warning": warning,
        "errors": item_errors,
        "ingested": results
            .iter()
            .map(|r| r["instance_slug"].as_str().unwrap_or(""))
            .collect::<Vec<_>>(),
        "items": results,
    })))
}

fn read_usize_env(name: &str, default: usize, max: usize) -> usize {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|v| *v > 0)
        .map(|v| v.min(max))
        .unwrap_or(default)
}

fn host_key_for_item(item: &str) -> String {
    let trimmed = item.trim();
    if let Some(rest) = trimmed
        .strip_prefix("https://")
        .or_else(|| trimmed.strip_prefix("http://"))
    {
        let host = rest
            .split('/')
            .next()
            .unwrap_or("")
            .trim()
            .to_ascii_lowercase();
        if !host.is_empty() {
            return host;
        }
    }
    "__raw_text__".to_string()
}
