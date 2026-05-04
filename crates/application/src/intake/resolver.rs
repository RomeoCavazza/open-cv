//! Resolver — Transformation d'un input brut (URL ou texte) en texte d'offre propre.

use ports::Scraper;
use std::sync::Arc;
use crate::AppError;

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
            return Err(AppError::Other("input vide".into()));
        }

        let (raw_text_uncleaned, source_url) = if looks_like_url(raw) {
            let result = self
                .scraper
                .scrape(raw)
                .await
                .map_err(|e| AppError::Other(format!("erreur de scraping : {e}")))?;
            (result.raw_text, raw.to_string())
        } else {
            if raw.len() < 200 {
                return Err(AppError::Other(
                    "texte trop court pour être une offre (<200 chars)".into(),
                ));
            }
            (raw.to_string(), "manual".to_string())
        };

        let raw_text = clean_raw_text(&raw_text_uncleaned);
        validate_quality(&raw_text)?;

        Ok((raw_text, source_url))
    }
}

pub(crate) fn looks_like_url(s: &str) -> bool {
    s.starts_with("http://") || s.starts_with("https://")
}

fn clean_raw_text(input: &str) -> String {
    let nav_keywords = [
        "Our Teams", "Our Culture", "How We Hire", "Recruitment Process", "FAQ",
        "See All Jobs", "Cookie", "Imagine New Horizons", "Mentions légales",
        "Careers", "Politique de confidentialité",
    ];

    input
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() { return true; }
            if is_just_markdown_link(trimmed) { return false; }
            if nav_keywords.iter().any(|kw| trimmed.contains(kw)) { return false; }
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
        "mission", "profil", "compétence", "expérience", "formation",
        "responsabilité", "stack", "technologie",
    ];
    let has_business_signal = business_keywords
        .iter()
        .any(|kw| text.to_lowercase().contains(kw));

    if text.len() < 500 && !has_business_signal {
        return Err(AppError::Other(
            "Le contenu fourni semble être pauvre ou manque de signal métier.".into()
        ));
    }
    Ok(())
}
