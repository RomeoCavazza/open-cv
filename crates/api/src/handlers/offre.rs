use crate::errors::ApiError;
use crate::state::AppState;
use axum::{
    extract::{Path, Query, State},
    response::Json,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ListOffresQuery {
    #[serde(default = "default_limit")]
    pub limit: u32,
}

fn default_limit() -> u32 {
    50
}

fn infer_business_category(slug: &str, title: &str) -> &'static str {
    let haystack = format!("{} {}", slug.to_lowercase(), title.to_lowercase());

    if [
        "data",
        " ai",
        "ia",
        "intelligence artificielle",
        "llm",
        "langchain",
        "gallica",
        "automation",
        "scientist",
        "machine learning",
    ]
    .iter()
    .any(|needle| haystack.contains(needle))
    {
        return "Data Engineering & Data Science";
    }

    if [
        "developpeur",
        "développeur",
        "software",
        "java",
        "api",
        "logiciel",
        "full stack",
        "full-stack",
        "embarqu",
        "engineering",
    ]
    .iter()
    .any(|needle| haystack.contains(needle))
    {
        return "Ingénierie Logicielle Spécialisée (Embarqué, C++, Simulations, Systèmes)";
    }

    if [
        "pilotage",
        "projet",
        "transformation",
        "strategie",
        "stratégie",
    ]
    .iter()
    .any(|needle| haystack.contains(needle))
    {
        return "Pilotage de Projet, Stratégie IT & Transformation Numérique";
    }

    "Autres"
}

fn public_offer_category(slug: &str, title: &str, raw: Option<&str>) -> String {
    let category = raw.unwrap_or("").trim();
    if category.is_empty()
        || category.eq_ignore_ascii_case("inbox")
        || category.eq_ignore_ascii_case("legacy restored")
    {
        infer_business_category(slug, title).to_string()
    } else {
        category.to_string()
    }
}

pub async fn list_offres(
    State(state): State<AppState>,
    Query(q): Query<ListOffresQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let rows = sqlx::query!(
        r#"
        SELECT o.id, o.slug, o.intitule, o.source_url, o.entreprise, o.categorie,
               EXISTS(SELECT 1 FROM instances i WHERE i.offre_id = o.id) as "has_instance!"
        FROM offres o
        ORDER BY o.scraped_at DESC
        LIMIT $1
        "#,
        q.limit as i64
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;

    let entries: Vec<serde_json::Value> = rows
        .iter()
        .map(|r| {
            serde_json::json!({
                "title": r.intitule,
                "url": r.source_url,
                "job_id": r.slug,
                "entreprise": r.entreprise,
                "category": public_offer_category(&r.slug, &r.intitule, r.categorie.as_deref()),
                "status": if r.has_instance { "ready" } else { "draft" },
            })
        })
        .collect();

    Ok(Json(serde_json::json!({
        "entries": entries,
    })))
}

pub async fn get_offre_by_slug(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<domain::Offre>, ApiError> {
    let slug = domain::Slug::parse(slug).map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let usecase = application::GetOffreBySlugUseCase::new(state.offre_repo.clone());
    let offre = usecase.execute(&slug).await?;

    Ok(Json(offre))
}
