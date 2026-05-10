//! Resolver — Transformation d'un input brut (URL ou texte) en texte d'offre propre.

use crate::AppError;
use ports::Scraper;
use std::sync::Arc;
use tracing::info;
use url::Url;

pub struct ContentResolver {
    scraper: Arc<dyn Scraper>,
}

impl ContentResolver {
    pub fn new(scraper: Arc<dyn Scraper>) -> Self {
        Self { scraper }
    }

    pub async fn resolve(&self, raw: &str) -> Result<(String, String), AppError> {
        let raw = raw.trim();
        if raw.is_empty() {
            return Err(AppError::Validation("input vide".into()));
        }

        let (raw_text_uncleaned, source_url) = if looks_like_url(raw) {
            let result = self
                .scraper
                .scrape(raw)
                .await
                .map_err(|e| AppError::Validation(format!("Accès refusé par le site ({}). Essayez de copier-coller directement le texte de l'offre au lieu du lien.", e)))?;
            let canonical_source = canonicalize_source_url(&result.final_url);
            (result.raw_text, canonical_source)
        } else if raw.starts_with('{') && raw.ends_with('}') {
            // Tentative d'extraction intelligente si c'est un JSON (ex: export d'une liste d'offres)
            match serde_json::from_str::<serde_json::Value>(raw) {
                Ok(json) => {
                    if let Some(url) = json.get("url").and_then(|v| v.as_str()) {
                        info!(
                            "L'input est un JSON contenant une URL, on lance le scraping de {}",
                            url
                        );
                        let (scraped_text, source_url) = Box::pin(self.resolve(url)).await?;

                        // On injecte les métadonnées du JSON comme contexte pour l'extracteur
                        let mut context = String::new();
                        if let Some(t) = json
                            .get("title")
                            .or_else(|| json.get("intitule"))
                            .and_then(|v| v.as_str())
                        {
                            context.push_str(&format!("CONTEXTE_TITRE: {t}\n"));
                        }
                        if let Some(e) = json
                            .get("company")
                            .or_else(|| json.get("entreprise"))
                            .and_then(|v| v.as_str())
                        {
                            context.push_str(&format!("CONTEXTE_ENTREPRISE: {e}\n"));
                        }

                        let combined = if context.is_empty() {
                            scraped_text
                        } else {
                            format!("{context}\n---\n\n{scraped_text}")
                        };

                        return Ok((combined, source_url));
                    }
                    (raw.to_string(), "manual_json".to_string())
                }
                Err(_) => (raw.to_string(), "manual".to_string()),
            }
        } else {
            if is_direct_prompt(raw) {
                (build_direct_prompt_offer(raw), "manual_prompt".to_string())
            } else if raw.len() < 200 {
                return Err(AppError::Validation(
                    "texte trop court pour être une offre (<200 chars)".into(),
                ));
            } else {
                (raw.to_string(), "manual".to_string())
            }
        };

        let raw_text = clean_raw_text(&raw_text_uncleaned);
        if source_url != "manual_prompt" {
            validate_quality(&raw_text)?;
        }

        Ok((raw_text, source_url))
    }
}

pub(crate) fn looks_like_url(s: &str) -> bool {
    s.starts_with("http://") || s.starts_with("https://")
}

pub(crate) fn canonicalize_source_url(raw: &str) -> String {
    let parsed = match Url::parse(raw) {
        Ok(url) => url,
        Err(_) => return raw.trim().to_string(),
    };
    let mut url = parsed;
    url.set_fragment(None);
    let host = url.host_str().unwrap_or_default().to_ascii_lowercase();

    if host.contains("linkedin.") {
        if let Some(id) = extract_linkedin_job_id(url.path()) {
            url.set_path(&format!("/jobs/view/{id}"));
        }
        url.set_query(None);
        return url.to_string();
    }

    if host.contains("indeed.") {
        let jk_value = url
            .query_pairs()
            .find_map(|(k, v)| (k == "jk").then(|| v.to_string()));
        url.set_path("/viewjob");
        url.set_query(None);
        if let Some(jk) = jk_value {
            let mut serializer = url::form_urlencoded::Serializer::new(String::new());
            serializer.append_pair("jk", &jk);
            let query = serializer.finish();
            url.set_query(Some(&query));
        }
        return url.to_string();
    }

    // Canonicalisation générique : suppression des fragments et tracking principal.
    let filtered: Vec<(String, String)> = url
        .query_pairs()
        .filter_map(|(k, v)| {
            let key = k.to_ascii_lowercase();
            if key.starts_with("utm_")
                || key == "trk"
                || key == "trackingid"
                || key == "from"
                || key == "src"
                || key == "ref"
            {
                None
            } else {
                Some((k.to_string(), v.to_string()))
            }
        })
        .collect();
    url.set_query(None);
    if !filtered.is_empty() {
        let mut serializer = url::form_urlencoded::Serializer::new(String::new());
        for (k, v) in filtered {
            serializer.append_pair(&k, &v);
        }
        let query = serializer.finish();
        url.set_query(Some(&query));
    }
    url.to_string()
}

fn extract_linkedin_job_id(path: &str) -> Option<String> {
    let marker = "/jobs/view/";
    let idx = path.find(marker)?;
    let tail = &path[idx + marker.len()..];
    let id = tail
        .trim_matches('/')
        .split('/')
        .next()
        .unwrap_or_default()
        .trim();
    if id.is_empty() {
        None
    } else {
        Some(id.to_string())
    }
}

fn is_direct_prompt(raw: &str) -> bool {
    let normalized = raw.trim().to_lowercase();
    let has_prompt_verb = [
        "genere",
        "génère",
        "générer",
        "generate",
        "cree",
        "crée",
        "créer",
        "create",
        "fait",
        "fais",
        "faire",
        "build",
        "rédige",
        "rédiger",
        "write",
    ]
    .iter()
    .any(|kw| normalized.contains(kw));

    let has_deliverable = [
        "cv",
        "resume",
        "résumé",
        "lettre",
        "profil",
        "profile",
        "restitution",
        "analyse",
        "analysis",
    ]
    .iter()
    .any(|kw| normalized.contains(kw));

    has_prompt_verb || has_deliverable
}

fn build_direct_prompt_offer(raw_prompt: &str) -> String {
    let target_title = extract_target_title(raw_prompt);
    format!(
        "__TARGET_TITLE__: {target_title}\n\
         ENTREPRISE: Non spécifié\n\n\
         CONTEXTE DE LA DEMANDE\n\
         {raw_prompt}\n\n\
         DESCRIPTION DÉDUITE\n\
         Il s'agit d'une demande directe pour générer une candidature au poste de {target_title}. \
         L'analyse doit porter sur les standards de l'industrie pour ce métier."
    )
}

fn extract_target_title(raw_prompt: &str) -> String {
    let trimmed = raw_prompt.trim().trim_matches('"').trim_matches('\'');
    if let Some(title) = first_quoted_fragment(trimmed) {
        return normalize_title_candidate(&title);
    }
    let lower = trimmed.to_lowercase();
    let patterns = [
        "génère-moi un cv de",
        "génère-moi une lettre de",
        "génère un cv de",
        "génère une lettre de",
        "génère-moi un",
        "génère-moi une",
        "génère moi un",
        "génère moi une",
        "génère un",
        "génère une",
        "genere un",
        "genere une",
        "générer un",
        "générer une",
        "generate a",
        "generate an",
        "generate",
        "create a",
        "create an",
        "create",
        "crée un",
        "crée une",
        "cree un",
        "cree une",
        "fais-moi un",
        "fais-moi une",
        "fais moi un",
        "fais moi une",
        "fait moi un",
        "fait moi une",
        "rédige un",
        "rédige une",
        "rédiger un",
        "rédiger une",
        "cv de",
        "cv pour",
        "cv",
        "lettre pour",
        "lettre de",
    ];

    for pattern in patterns {
        if let Some(idx) = lower.find(pattern) {
            let start = idx + pattern.len();
            let candidate = trimmed[start..]
                .trim()
                .trim_matches('?')
                .trim_matches('.')
                .trim_end_matches(" stp")
                .trim_end_matches(" s'il te plait")
                .trim_end_matches(" please")
                .trim();
            if !candidate.is_empty() {
                return normalize_title_candidate(candidate);
            }
        }
    }

    normalize_title_candidate(trimmed)
}

fn first_quoted_fragment(input: &str) -> Option<String> {
    let mut in_quote = false;
    let mut quote_char = '"';
    let mut current = String::new();

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
                    return Some(candidate);
                }
                in_quote = false;
                current.clear();
                continue;
            }
            current.push(ch);
        }
    }

    None
}

fn normalize_title_candidate(input: &str) -> String {
    let mut title = input.trim().to_string();
    if title.is_empty() {
        return "Poste non spécifié".to_string();
    }

    let lower = title.to_lowercase();
    if lower.starts_with("__target_title__:") {
        title = title["__target_title__:".len()..].trim().to_string();
    }

    let leading_noise = [
        "cv pour un poste de",
        "cv pour le poste de",
        "cv pour un poste",
        "cv pour le poste",
        "cv pour",
        "pour un poste de",
        "pour le poste de",
        "un poste de",
        "le poste de",
        "un poste",
        "le poste",
        "poste de",
        "poste",
        "de",
    ];
    loop {
        let lower = title.to_lowercase();
        let mut changed = false;
        for prefix in leading_noise {
            if lower.starts_with(prefix) {
                title = title[prefix.len()..].trim().to_string();
                changed = true;
                break;
            }
        }
        if !changed {
            break;
        }
    }

    // Nettoie les détails de formulation utilisateur qui polluent le nom métier.
    let trailing_noise = [
        "en alternance",
        "alternance",
        "intitulé exact",
        "intitule exact",
    ];
    loop {
        let lower = title.to_lowercase();
        let mut changed = false;
        for suffix in trailing_noise {
            if lower.ends_with(suffix) {
                let cut = title.len().saturating_sub(suffix.len());
                title = title[..cut]
                    .trim()
                    .trim_matches(':')
                    .trim_matches('-')
                    .trim()
                    .to_string();
                changed = true;
                break;
            }
        }
        if !changed {
            break;
        }
    }

    let cleaned = title
        .trim()
        .trim_matches('"')
        .trim_matches('\'')
        .trim_matches('?')
        .trim_matches('.')
        .trim()
        .to_string();
    if cleaned.is_empty() {
        "Poste non spécifié".to_string()
    } else {
        cleaned
    }
}

fn clean_raw_text(input: &str) -> String {
    let nav_keywords = [
        "our teams",
        "our culture",
        "how we hire",
        "recruitment process",
        "faq",
        "see all jobs",
        "cookie",
        "imagine new horizons",
        "mentions légales",
        "careers",
        "politique de confidentialité",
    ];

    input
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                return true;
            }
            if is_just_markdown_link(trimmed) {
                return false;
            }
            // Ne supprime les lignes "navigation/cookie" que si elles sont courtes :
            // certaines pages scrappées sont une seule ligne longue contenant aussi le vrai contenu métier.
            let lower = trimmed.to_ascii_lowercase();
            if trimmed.len() < 220 && nav_keywords.iter().any(|kw| lower.contains(kw)) {
                return false;
            }
            true
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn is_just_markdown_link(line: &str) -> bool {
    let trimmed = line.trim_start_matches(|c: char| c == '-' || c == '*' || c.is_whitespace());
    trimmed.starts_with('[') && trimmed.contains("](")
}

fn validate_quality(text: &str) -> Result<(), AppError> {
    let business_keywords = [
        "mission",
        "profil",
        "compétence",
        "expérience",
        "formation",
        "responsabilité",
        "stack",
        "technologie",
    ];
    let has_business_signal = business_keywords
        .iter()
        .any(|kw| text.to_lowercase().contains(kw));

    if text.len() < 500 && !has_business_signal {
        return Err(AppError::Validation(
            "Le contenu fourni semble être pauvre ou manque de signal métier.".into(),
        ));
    }
    Ok(())
}
