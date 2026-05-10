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
    let haystack = format!(" {} {} ", slug.to_lowercase(), title.to_lowercase());

    if [
        "data",
        " ai ",
        " ia ",
        "intelligence artificielle",
        "llm",
        "langchain",
        "machine learning",
        "scientist",
        "statistique",
        "analyst",
        " business intelligence ",
        " bi ",
        " sql ",
        "pandas",
        "spark",
        "tableau",
        "powerbi",
    ]
    .iter()
    .any(|n| haystack.contains(n))
    {
        return "Data";
    }
    if [
        "developpeur",
        "développeur",
        "software",
        "java",
        "python",
        "javascript",
        "js",
        "typescript",
        "ts",
        "kotlin",
        "swift",
        "ios",
        "android",
        "mobile",
        "api",
        "logiciel",
        "full stack",
        "fullstack",
        "frontend",
        "backend",
        "front-end",
        "back-end",
        "embarqu",
        "engineering",
        "devops",
        "c++",
        "rust",
        "react",
        "node",
    ]
    .iter()
    .any(|n| haystack.contains(n))
    {
        return "Software";
    }
    if [
        "pilotage",
        "projet",
        "transformation",
        "strategie",
        "stratégie",
        "product",
        "manager",
        "management",
        "agile",
        "scrum",
        "owner",
    ]
    .iter()
    .any(|n| haystack.contains(n))
    {
        return "Product";
    }
    if ["innovation", "fablab", "pédagog", "learning", "recherche"]
        .iter()
        .any(|n| haystack.contains(n))
    {
        return "Innovation";
    }
    if ["design", "ui", "ux", "graphiste", "maquette"]
        .iter()
        .any(|n| haystack.contains(n))
    {
        return "Design";
    }
    if ["marketing", "communication", "seo", "content", "réseaux sociaux"]
        .iter()
        .any(|n| haystack.contains(n))
    {
        return "Marketing";
    }
    "Autres"
}

fn public_offer_category(slug: &str, title: &str, raw: Option<&str>) -> String {
    let category = raw.unwrap_or("").trim();
    let category_lc = category.to_ascii_lowercase();

    let is_old_hardcoded = category_lc.contains("ingénierie logicielle")
        || category_lc.contains("pilotage de projet")
        || category_lc.contains("transformation numérique")
        || category_lc.contains("digital learning");

    let should_infer = category.is_empty()
        || is_old_hardcoded
        || matches!(
            category_lc.as_str(),
            "inbox"
                | "legacy restored"
                | "autres"
                | "autre"
                | "others"
                | "other"
                | "data engineering & data science"
        );

    if should_infer {
        infer_business_category(slug, title).to_string()
    } else {
        let mut clean = category.to_string();
        if clean.len() > 35 {
            clean = format!("{}...", &clean[..32]);
        }
        clean
    }
}

pub async fn list_offres(
    State(state): State<AppState>,
    Query(q): Query<ListOffresQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let rows = sqlx::query!(
        r#"
        SELECT o.id, o.slug, o.intitule, o.source_url, o.entreprise, o.categorie,
               i.status::text as "instance_status?"
        FROM offres o
        LEFT JOIN instances i ON i.offre_id = o.id
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
                "status": r.instance_status.as_deref().unwrap_or("draft"),
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
