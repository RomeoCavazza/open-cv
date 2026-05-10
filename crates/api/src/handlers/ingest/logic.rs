use crate::errors::ApiError;
use application::generate::{GenerateInput, Livrables};
use domain::{Instance, Profil, ProfilId};
use ports::ProfilRepo;
use std::collections::HashSet;

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
    let mut urls = Vec::new();
    let mut seen = HashSet::new();

    for token in input.split_whitespace() {
        let candidate = sanitize_url_token(token);
        if candidate.starts_with("http://") || candidate.starts_with("https://") {
            let normalized = candidate.to_string();
            if seen.insert(normalized.clone()) {
                urls.push(normalized);
            }
        }
    }

    if !urls.is_empty() {
        return urls;
    }

    let trimmed = input.trim();
    let quoted_titles = extract_quoted_titles(trimmed);
    if quoted_titles.len() >= 2 {
        return quoted_titles
            .into_iter()
            .map(|title| {
                format!(
                    "Fais-moi un CV pour un poste de {title} en alternance. Mets l'intitulé exact \"{title}\"."
                )
            })
            .collect();
    }

    vec![trimmed.to_string()]
}

fn sanitize_url_token(token: &str) -> &str {
    token.trim_matches(|c: char| {
        matches!(
            c,
            '"' | '\'' | '<' | '>' | '(' | ')' | '[' | ']' | '{' | '}' | ',' | ';'
        )
    })
}

fn extract_quoted_titles(input: &str) -> Vec<String> {
    let mut titles = Vec::new();
    let mut seen = HashSet::new();
    let mut current = String::new();
    let mut in_quote = false;
    let mut quote_char = '"';

    for ch in input.chars() {
        if !in_quote && (ch == '"' || ch == '\'' || ch == '“' || ch == '”') {
            in_quote = true;
            quote_char = ch;
            current.clear();
            continue;
        }

        if in_quote {
            let is_closing = (quote_char == '"' && ch == '"')
                || (quote_char == '\'' && ch == '\'')
                || ((quote_char == '“' || quote_char == '”') && (ch == '”' || ch == '“'));
            if is_closing {
                let candidate = current.trim().to_string();
                if !candidate.is_empty() {
                    let key = candidate.to_ascii_lowercase();
                    if seen.insert(key) {
                        titles.push(candidate);
                    }
                }
                in_quote = false;
                current.clear();
                continue;
            }
            current.push(ch);
        }
    }

    titles
}

pub(super) fn should_generate(config: Option<&IngestConfig>) -> bool {
    config
        .map(|c| c.restitution || c.resume || c.cover)
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
            restitution: config.restitution,
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
    fn extracts_urls_inside_sentences() {
        let input = "LinkedIn: https://www.linkedin.com/jobs/view/1234567890/?trk=abc then Indeed https://www.indeed.com/viewjob?jk=abc123&from=share";
        let items = parse_input_items(input);
        assert_eq!(items.len(), 2);
        assert!(items[0].contains("linkedin.com/jobs/view/1234567890"));
        assert!(items[1].contains("indeed.com/viewjob?jk=abc123"));
    }

    #[test]
    fn ignores_trailing_punctuation_for_urls() {
        let input = "- https://www.linkedin.com/jobs/view/12345/, \n(https://www.indeed.com/viewjob?jk=abcde);";
        let items = parse_input_items(input);
        assert_eq!(items.len(), 2);
        assert_eq!(items[0], "https://www.linkedin.com/jobs/view/12345/");
        assert_eq!(items[1], "https://www.indeed.com/viewjob?jk=abcde");
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
            restitution: false,
        })));
    }

    #[test]
    fn splits_multi_prompt_into_distinct_titles() {
        let input = r#"Crée 2 candidatures distinctes : 1) Data Scientist en alternance, intitulé exact "Data Scientist" ; 2) Data Engineer en alternance, intitulé exact "Data Engineer"."#;
        let items = parse_input_items(input);
        assert_eq!(items.len(), 2);
        assert!(items[0].contains("Data Scientist"));
        assert!(items[1].contains("Data Engineer"));
        assert_ne!(items[0], items[1]);
    }
}
