async fn generate_instance(
    State(state): State<AppState>,
    Path(slug_str): Path<String>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<StatusCode, ApiError> {
    let slug = domain::Slug::parse(slug_str.clone()).map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let llm_provider = params
        .get("llm_provider")
        .and_then(|p| state.llm_registry.get(p))
        .cloned();

    let instance = state
        .instance_repo
        .get_by_slug(&slug)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound(format!("Instance {} inconnue", slug_str)))?;

    let restitution = params.get("restitution").map(|v| v == "true").unwrap_or(true);
    let resume = params.get("resume").map(|v| v == "true").unwrap_or(true);
    let cover_letter = params.get("cover_letter").map(|v| v == "true").unwrap_or(true);

    let input = application::generate::GenerateInput {
        offre_id: instance.offre_id,
        profil_id: instance.profil_id,
        existing_instance: Some(instance),
        livrables: application::generate::Livrables {
            restitution,
            resume,
            cover_letter,
        },
    };

    tokio::spawn(async move {
        match state.generate_uc.execute(input, llm_provider).await {
            Ok(_) => info!(slug = %slug_str, "génération terminée avec succès"),
            Err(e) => error!(slug = %slug_str, error = %e, "échec de la génération en arrière-plan"),
        }
    });

    Ok(StatusCode::ACCEPTED)
}
