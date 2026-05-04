use crate::errors::ApiError;
use application::generate::{GenerateInput, Livrables};
use domain::{Instance, Profil, ProfilId};
use ports::ProfilRepo;

use super::IngestConfig;

pub(super) async fn resolve_ingest_profile(
    profil_repo: &dyn ProfilRepo,
    profil_id: Option<uuid::Uuid>,
) -> Result<Profil, ApiError> {
    match profil_id {
        Some(pid) => profil_repo
            .get_by_id(ProfilId::from_uuid(pid))
            .await
            .map_err(|e| ApiError::Internal(e.to_string()))?
            .ok_or_else(|| ApiError::BadRequest(format!("Profil {} introuvable", pid))),
        None => profil_repo
            .get_active()
            .await
            .map_err(|e| ApiError::Internal(e.to_string()))?
            .ok_or_else(|| ApiError::BadRequest("Aucun profil actif trouvé".to_string())),
    }
}

pub(super) fn parse_input_items(input: &str) -> Vec<String> {
    let lines: Vec<&str> = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();

    let url_count = lines
        .iter()
        .filter(|line| line.starts_with("http://") || line.starts_with("https://"))
        .count();

    if url_count > 0 {
        lines
            .into_iter()
            .filter(|line| line.starts_with("http://") || line.starts_with("https://"))
            .map(|line| line.to_string())
            .collect()
    } else {
        vec![input.to_string()]
    }
}

pub(super) fn should_generate(config: Option<&IngestConfig>) -> bool {
    config
        .map(|c| c.analysis || c.resume || c.cover)
        .unwrap_or(false)
}

pub(super) fn build_generate_input(
    instance: Instance,
    profil_id: ProfilId,
    config: &IngestConfig,
) -> GenerateInput {
    GenerateInput {
        offre_id: instance.offre_id,
        profil_id,
        existing_instance: Some(instance),
        livrables: Livrables {
            analysis: config.analysis,
            resume: config.resume,
            cover_letter: config.cover,
        },
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
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn should_generate_is_false_without_config() {
        assert!(!should_generate(None));
    }

    #[test]
    fn should_generate_detects_true_flags() {
        assert!(should_generate(Some(&IngestConfig {
            resume: true,
            cover: false,
            analysis: false,
        })));
    }
}
