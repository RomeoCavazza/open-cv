//! Scraper HTTP avec fallback optionnel ScrapingAnt.
//!
//! Flux:
//! 1) tentative directe via `reqwest`
//! 2) si anti-bot / status hostile / contenu vide -> fallback ScrapingAnt si configuré

use async_trait::async_trait;
use ports::{ScrapeError, ScrapeResult, Scraper};
use scraper::{Html, Selector};
use serde_json::Value;
use std::time::Duration;
use tracing::{debug, warn};

const SCRAPINGANT_DEFAULT_ENDPOINT: &str = "https://api.scrapingant.com/v2/general";
const SCRAPINGANT_MAX_ATTEMPTS: usize = 4;

pub struct HttpScraper {
    http: reqwest::Client,
    scrapingant: Option<ScrapingAntConfig>,
}

#[derive(Clone)]
struct ScrapingAntConfig {
    endpoint: String,
    api_key: String,
    timeout_secs: u64,
    browser: bool,
    proxy_type: Option<String>,
    proxy_country: Option<String>,
}

impl HttpScraper {
    pub fn new() -> Self {
        Self {
            http: reqwest::Client::builder()
                .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:125.0) Gecko/20100101 Firefox/125.0")
                .default_headers({
                    let mut headers = reqwest::header::HeaderMap::new();
                    headers.insert(
                        reqwest::header::ACCEPT,
                        reqwest::header::HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8"),
                    );
                    headers.insert(
                        reqwest::header::ACCEPT_LANGUAGE,
                        reqwest::header::HeaderValue::from_static("en-US,en;q=0.5"),
                    );
                    headers
                })
                .timeout(std::time::Duration::from_secs(30))
                .redirect(reqwest::redirect::Policy::limited(5))
                .build()
                .expect("client HTTP valide"),
            scrapingant: load_scrapingant_config(),
        }
    }

    async fn scrape_direct(&self, parsed: &url::Url) -> Result<ScrapeResult, ScrapeError> {
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

        let bytes = resp
            .bytes()
            .await
            .map_err(|e| ScrapeError::Http(e.to_string()))?;
        to_scrape_result(parsed.as_str(), &final_url, status, &bytes)
    }

    async fn scrape_with_scrapingant(
        &self,
        parsed: &url::Url,
    ) -> Result<ScrapeResult, ScrapeError> {
        let Some(cfg) = &self.scrapingant else {
            return Err(ScrapeError::Other(
                "fallback ScrapingAnt demandé mais non configuré".into(),
            ));
        };

        let timeout = cfg.timeout_secs.to_string();
        let browser = if cfg.browser { "true" } else { "false" };
        let mut last_error: Option<ScrapeError> = None;

        for attempt in 1..=SCRAPINGANT_MAX_ATTEMPTS {
            let mut request = self.http.get(&cfg.endpoint).query(&[
                ("url", parsed.as_str()),
                ("x-api-key", cfg.api_key.as_str()),
                ("browser", browser),
                ("timeout", timeout.as_str()),
            ]);

            if let Some(proxy_country) = &cfg.proxy_country {
                request = request.query(&[("proxy_country", proxy_country.as_str())]);
            }
            if let Some(proxy_type) = &cfg.proxy_type {
                request = request.query(&[("proxy_type", proxy_type.as_str())]);
            }

            match request.send().await {
                Ok(resp) => {
                    let status = resp.status().as_u16();
                    if resp.status().is_success() {
                        let bytes = resp.bytes().await.map_err(|e| {
                            ScrapeError::Http(format!("ScrapingAnt decode error: {e}"))
                        })?;
                        return to_scrape_result(parsed.as_str(), parsed.as_str(), status, &bytes);
                    }

                    let body = resp.text().await.unwrap_or_default();
                    let err = ScrapeError::Other(format!("ScrapingAnt status {}: {}", status, body));
                    let can_retry = should_retry_scrapingant(status, &body)
                        && attempt < SCRAPINGANT_MAX_ATTEMPTS;
                    if can_retry {
                        warn!(
                            attempt,
                            status,
                            source = %parsed,
                            "ScrapingAnt indisponible temporairement, retry"
                        );
                        tokio::time::sleep(scrapingant_retry_backoff(attempt)).await;
                        last_error = Some(err);
                        continue;
                    }
                    return Err(err);
                }
                Err(e) => {
                    let err = ScrapeError::Http(format!("ScrapingAnt transport error: {e}"));
                    if attempt < SCRAPINGANT_MAX_ATTEMPTS {
                        warn!(
                            attempt,
                            source = %parsed,
                            "Erreur transport ScrapingAnt, retry"
                        );
                        tokio::time::sleep(scrapingant_retry_backoff(attempt)).await;
                        last_error = Some(err);
                        continue;
                    }
                    return Err(err);
                }
            }
        }

        Err(last_error
            .unwrap_or_else(|| ScrapeError::Other("ScrapingAnt retry failed".into())))
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
        let parsed =
            url::Url::parse(url).map_err(|e| ScrapeError::InvalidUrl(format!("{url}: {e}")))?;

        match self.scrape_direct(&parsed).await {
            Ok(result) => Ok(result),
            Err(primary_error) => {
                if self.scrapingant.is_some() && should_try_scrapingant_fallback(&primary_error) {
                    warn!(
                        reason = %primary_error,
                        source = %parsed,
                        "fallback ScrapingAnt activé"
                    );
                    match self.scrape_with_scrapingant(&parsed).await {
                        Ok(result) => {
                            debug!(source = %parsed, "scraping réussi via ScrapingAnt");
                            Ok(result)
                        }
                        Err(fallback_error) => Err(ScrapeError::Other(format!(
                            "scraping direct échoué ({primary_error}) puis ScrapingAnt échoué ({fallback_error})"
                        ))),
                    }
                } else {
                    Err(primary_error)
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        "http"
    }
}

fn to_scrape_result(
    source_url: &str,
    final_url: &str,
    status: u16,
    bytes: &[u8],
) -> Result<ScrapeResult, ScrapeError> {
    let (cow, _, _) = encoding_rs::UTF_8.decode(bytes);
    let raw_html = cow.into_owned();

    if is_antibot_page(&raw_html) {
        return Err(ScrapeError::AntiBotDetected);
    }

    let raw_text = extract_main_text(&raw_html);
    if raw_text.len() < 500 {
        return Err(ScrapeError::EmptyContent);
    }

    Ok(ScrapeResult {
        url: source_url.to_string(),
        final_url: final_url.to_string(),
        raw_html,
        raw_text,
        status,
    })
}

fn should_try_scrapingant_fallback(error: &ScrapeError) -> bool {
    match error {
        ScrapeError::AntiBotDetected | ScrapeError::EmptyContent => true,
        ScrapeError::BadStatus(status) => matches!(status, 403 | 408 | 409 | 423 | 429 | 500..=599),
        ScrapeError::Http(_) => true,
        ScrapeError::InvalidUrl(_) | ScrapeError::Other(_) => false,
    }
}

fn should_retry_scrapingant(status: u16, body: &str) -> bool {
    if matches!(status, 408 | 409 | 423 | 429 | 500..=599) {
        return true;
    }
    let lower = body.to_ascii_lowercase();
    lower.contains("concurrency limit reached") || lower.contains("try again")
}

fn scrapingant_retry_backoff(attempt: usize) -> Duration {
    let millis = 800u64.saturating_mul(attempt as u64);
    Duration::from_millis(millis.max(800))
}

fn load_scrapingant_config() -> Option<ScrapingAntConfig> {
    let api_key = std::env::var("SCRAPINGANT_API_KEY").ok()?;
    let api_key = api_key.trim().to_string();
    if api_key.is_empty() {
        return None;
    }

    let endpoint = std::env::var("SCRAPINGANT_ENDPOINT")
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| SCRAPINGANT_DEFAULT_ENDPOINT.to_string());
    let timeout_secs = std::env::var("SCRAPINGANT_TIMEOUT_SECS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .map(|v| v.clamp(5, 60))
        .unwrap_or(45);
    let browser = std::env::var("SCRAPINGANT_BROWSER")
        .ok()
        .map(|v| {
            matches!(
                v.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(true);
    let proxy_type = std::env::var("SCRAPINGANT_PROXY_TYPE")
        .ok()
        .map(|v| v.trim().to_ascii_lowercase())
        .and_then(|v| match v.as_str() {
            "datacenter" | "residential" => Some(v),
            _ => None,
        });
    let proxy_country = std::env::var("SCRAPINGANT_PROXY_COUNTRY")
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty());

    Some(ScrapingAntConfig {
        endpoint,
        api_key,
        timeout_secs,
        browser,
        proxy_type,
        proxy_country,
    })
}

fn is_antibot_page(html: &str) -> bool {
    let lower = html.to_lowercase();
    let cloudflare_challenge_markers = [
        "just a moment...",
        "attention required!",
        "you have been blocked",
        "additional verification required",
        "cf-error-details",
        "cf-mitigated",
        "checking your browser before accessing",
    ];

    cloudflare_challenge_markers
        .iter()
        .any(|marker| lower.contains(marker))
        || lower.contains("datadome")
        || lower.contains("hcaptcha")
        || lower.contains("g-recaptcha")
}

/// Extraction naïve : texte des balises <article>, <main>, ou <body>.
/// Phase 4 : remplacer par une vraie extraction (readability-rs).
fn extract_main_text(html: &str) -> String {
    let doc = Html::parse_document(html);

    if let Some(text) = extract_jobposting_json_ld(&doc) {
        return text;
    }

    if let Some(text) = extract_meta_job_description(&doc) {
        return text;
    }

    // Essaye dans l'ordre : article, main, body
    for sel in &["article", "main", "body"] {
        if let Ok(selector) = Selector::parse(sel) {
            if let Some(el) = doc.select(&selector).next() {
                let text = strip_common_boilerplate(&normalize_text_nodes(el.text()));
                if text.len() >= 500 {
                    return text;
                }
            }
        }
    }

    // Fallback : tout le texte du document.
    strip_common_boilerplate(&normalize_text_nodes(doc.root_element().text()))
}

fn extract_jobposting_json_ld(doc: &Html) -> Option<String> {
    let selector = Selector::parse(r#"script[type="application/ld+json"]"#).ok()?;
    let mut best_candidate: Option<String> = None;

    for script in doc.select(&selector) {
        let payload = script.text().collect::<Vec<_>>().join(" ");
        let trimmed = payload.trim();
        if trimmed.is_empty() {
            continue;
        }

        let Ok(value) = serde_json::from_str::<Value>(trimmed) else {
            continue;
        };

        let mut fragments = Vec::new();
        collect_jobposting_fragments(&value, &mut fragments);
        if fragments.is_empty() {
            continue;
        }

        let candidate = normalize_text_nodes(fragments.iter().map(std::string::String::as_str));
        if candidate.len() < 250 {
            continue;
        }

        if best_candidate
            .as_ref()
            .map(|best| candidate.len() > best.len())
            .unwrap_or(true)
        {
            best_candidate = Some(candidate);
        }
    }

    best_candidate
}

fn collect_jobposting_fragments(value: &Value, out: &mut Vec<String>) {
    match value {
        Value::Array(items) => {
            for item in items {
                collect_jobposting_fragments(item, out);
            }
        }
        Value::Object(map) => {
            if map
                .get("@type")
                .is_some_and(value_contains_jobposting_type)
            {
                push_json_text(map.get("title"), out);
                push_json_text(map.get("description"), out);
                push_json_text(map.get("responsibilities"), out);
                push_json_text(map.get("qualifications"), out);
                push_json_text(map.get("skills"), out);
                push_json_text(map.get("experienceRequirements"), out);
            }

            for value in map.values() {
                collect_jobposting_fragments(value, out);
            }
        }
        _ => {}
    }
}

fn value_contains_jobposting_type(value: &Value) -> bool {
    match value {
        Value::String(kind) => kind.to_ascii_lowercase().contains("jobposting"),
        Value::Array(values) => values.iter().any(value_contains_jobposting_type),
        _ => false,
    }
}

fn push_json_text(value: Option<&Value>, out: &mut Vec<String>) {
    let Some(value) = value else {
        return;
    };
    push_json_text_value(value, out);
}

fn push_json_text_value(value: &Value, out: &mut Vec<String>) {
    match value {
        Value::String(text) => {
            let normalized = normalize_inline_whitespace(text);
            if !normalized.is_empty() {
                out.push(normalized);
            }
        }
        Value::Array(values) => {
            for value in values {
                push_json_text_value(value, out);
            }
        }
        Value::Object(map) => {
            for key in ["@value", "text", "name"] {
                if let Some(text) = map.get(key).and_then(Value::as_str) {
                    let normalized = normalize_inline_whitespace(text);
                    if !normalized.is_empty() {
                        out.push(normalized);
                    }
                }
            }
        }
        _ => {}
    }
}

fn extract_meta_job_description(doc: &Html) -> Option<String> {
    let selector = Selector::parse(
        r#"meta[name="description"],meta[property="og:description"],meta[name="twitter:description"]"#,
    )
    .ok()?;

    let mut parts = Vec::new();
    for meta in doc.select(&selector) {
        if let Some(content) = meta.value().attr("content") {
            let normalized = normalize_inline_whitespace(content);
            if normalized.len() >= 120 {
                parts.push(normalized);
            }
        }
    }

    if parts.is_empty() {
        None
    } else {
        Some(parts.join("\n\n"))
    }
}

fn normalize_text_nodes<'a, I>(nodes: I) -> String
where
    I: Iterator<Item = &'a str>,
{
    let mut lines = Vec::new();
    for node in nodes {
        let normalized = normalize_inline_whitespace(node);
        if !normalized.is_empty() {
            lines.push(normalized);
        }
    }
    lines.join("\n")
}

fn normalize_inline_whitespace(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn strip_common_boilerplate(input: &str) -> String {
    let noisy_terms = [
        "cookie",
        "consent",
        "cookieyes",
        "accepter tout",
        "tout rejeter",
        "toujours actif",
        "politique de confidentialité",
        "privacy policy",
    ];

    input
        .lines()
        .filter_map(|line| {
            let normalized = normalize_inline_whitespace(line);
            if normalized.is_empty() {
                return None;
            }

            let lower = normalized.to_ascii_lowercase();
            let is_noisy_short_line =
                normalized.len() < 220 && noisy_terms.iter().any(|term| lower.contains(term));
            if is_noisy_short_line {
                return None;
            }

            Some(normalized)
        })
        .collect::<Vec<_>>()
        .join("\n")
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

    #[test]
    fn fallback_is_triggered_on_antibot() {
        assert!(should_try_scrapingant_fallback(
            &ScrapeError::AntiBotDetected
        ));
    }

    #[test]
    fn fallback_is_triggered_on_forbidden_status() {
        assert!(should_try_scrapingant_fallback(&ScrapeError::BadStatus(
            403
        )));
    }

    #[test]
    fn fallback_is_not_triggered_on_invalid_url() {
        assert!(!should_try_scrapingant_fallback(&ScrapeError::InvalidUrl(
            "bad".into()
        )));
    }

    #[test]
    fn extracts_jobposting_description_from_json_ld() {
        let html = r#"
        <html><head>
            <script type="application/ld+json">
            {
              "@context": "https://schema.org",
              "@type": "JobPosting",
              "title": "Data Analyst Alternance",
              "description": "Mission: construire des dashboards et analyser des donnees pour les equipes metier."
            }
            </script>
        </head><body><div>Cookie settings</div></body></html>
        "#;

        let text = extract_main_text(html);
        assert!(text.contains("Data Analyst Alternance"));
        assert!(text.contains("dashboards"));
    }

    #[test]
    fn keeps_meaningful_long_lines_even_with_cookie_words() {
        let noisy = "Mission principale analyser les donnees produit et construire des tableaux de bord pour les equipes metier. Cookie consent banner texte annexe. Cette meme ligne reste volontairement longue pour simuler un scrape monocorde qui contient aussi des mots de navigation.";
        let cleaned = strip_common_boilerplate(noisy);
        assert!(cleaned.contains("Mission principale"));
    }

    #[test]
    fn retries_on_scrapingant_concurrency_limit() {
        let body = r#"{"detail":"Free user concurrency limit reached."}"#;
        assert!(should_retry_scrapingant(409, body));
    }
}
