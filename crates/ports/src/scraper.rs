//! Traits scraping — un par stratégie (HTTP, headless, etc.).

use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct ScrapeResult {
    pub url: String,
    pub final_url: String,
    pub raw_html: String,
    pub raw_text: String,
    pub status: u16,
}

#[async_trait]
pub trait Scraper: Send + Sync {
    async fn scrape(&self, url: &str) -> Result<ScrapeResult, ScrapeError>;
    fn name(&self) -> &'static str;
}

#[derive(Debug, Error)]
pub enum ScrapeError {
    #[error("erreur HTTP : {0}")]
    Http(String),

    #[error("status non-OK : {0}")]
    BadStatus(u16),

    #[error("anti-bot détecté (Cloudflare/Datadome/...) — escalade vers chrome ou raw_text manuel")]
    AntiBotDetected,

    #[error("contenu vide ou inutile (< 500c)")]
    EmptyContent,

    #[error("URL invalide : {0}")]
    InvalidUrl(String),

    #[error("autre : {0}")]
    Other(String),
}
