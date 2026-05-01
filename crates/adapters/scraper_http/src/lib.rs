//! Scraper HTTP simple — pas de JS, pas de bypass anti-bot.
//!
//! Détecte Cloudflare/Datadome et renvoie `AntiBotDetected` pour escalade.
//! Phase 4 ajoutera `scraper_chrome` (chromiumoxide) en cas de besoin.

use async_trait::async_trait;
use ports::{ScrapeError, ScrapeResult, Scraper};
use scraper::{Html, Selector};

pub struct HttpScraper {
    http: reqwest::Client,
}

impl HttpScraper {
    pub fn new() -> Self {
        Self {
            http: reqwest::Client::builder()
                .user_agent(
                    "Mozilla/5.0 (compatible; alternance-bot/0.1; \
                     +contact:romeo.cavazza@example.org)",
                )
                .timeout(std::time::Duration::from_secs(30))
                .redirect(reqwest::redirect::Policy::limited(5))
                .build()
                .expect("client HTTP valide"),
        }
    }
}

impl Default for HttpScraper {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Scraper for HttpScraper {
    async fn scrape(&self, url: &str) -> Result<ScrapeResult, ScrapeError> {
        let parsed = url::Url::parse(url)
            .map_err(|e| ScrapeError::InvalidUrl(format!("{url}: {e}")))?;

        let resp = self
            .http
            .get(parsed.clone())
            .send()
            .await
            .map_err(|e| ScrapeError::Http(e.to_string()))?;

        let status = resp.status().as_u16();
        let final_url = resp.url().to_string();

        if !resp.status().is_success() {
            return Err(ScrapeError::BadStatus(status));
        }

        // Récupère les bytes pour gérer l'encoding manuellement (latin-1, etc.).
        let bytes = resp
            .bytes()
            .await
            .map_err(|e| ScrapeError::Http(e.to_string()))?;
        let (cow, _, _) = encoding_rs::UTF_8.decode(&bytes);
        let raw_html = cow.into_owned();

        // Détection anti-bot grossière.
        if is_antibot_page(&raw_html) {
            return Err(ScrapeError::AntiBotDetected);
        }

        // Extraction texte naïve. Phase 4 : remplacer par readability-rs.
        let raw_text = extract_main_text(&raw_html);

        if raw_text.len() < 500 {
            return Err(ScrapeError::EmptyContent);
        }

        Ok(ScrapeResult {
            url: url.to_string(),
            final_url,
            raw_html,
            raw_text,
            status,
        })
    }

    fn name(&self) -> &'static str {
        "http"
    }
}

fn is_antibot_page(html: &str) -> bool {
    let lower = html.to_lowercase();
    lower.contains("just a moment...")
        || lower.contains("cloudflare")
        || lower.contains("datadome")
        || lower.contains("captcha")
}

/// Extraction naïve : texte des balises <article>, <main>, ou <body>.
/// Phase 4 : remplacer par une vraie extraction (readability-rs).
fn extract_main_text(html: &str) -> String {
    let doc = Html::parse_document(html);

    // Essaye dans l'ordre : article, main, body
    for sel in &["article", "main", "body"] {
        if let Ok(selector) = Selector::parse(sel) {
            if let Some(el) = doc.select(&selector).next() {
                let text = el
                    .text()
                    .collect::<Vec<_>>()
                    .join(" ")
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ");
                if text.len() >= 500 {
                    return text;
                }
            }
        }
    }

    // Fallback : tout le texte du document.
    doc.root_element()
        .text()
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detecte_cloudflare() {
        let html = r#"<html><body><h1>Just a Moment...</h1></body></html>"#;
        assert!(is_antibot_page(html));
    }

    #[test]
    fn ignore_html_normal() {
        let html = r#"<html><body><article>contenu réel</article></body></html>"#;
        assert!(!is_antibot_page(html));
    }
}
