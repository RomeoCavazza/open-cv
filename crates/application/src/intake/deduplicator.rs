//! Deduplicator — Logique de calcul de hash et de détection de doublons.

use domain::Offre;
use ports::{OffreRepo, RepoError};
use sha2::{Digest, Sha256};
use std::sync::Arc;

pub struct Deduplicator {
    offres: Arc<dyn OffreRepo>,
}

impl Deduplicator {
    pub fn new(offres: Arc<dyn OffreRepo>) -> Self {
        Self { offres }
    }

    pub fn compute_hash(&self, text: &str) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(text.as_bytes());
        hasher.finalize().to_vec()
    }

    pub fn resolve_host(&self, url: &str) -> String {
        if url.starts_with("http") {
            url::Url::parse(url)
                .ok()
                .and_then(|u| u.host_str().map(|h| h.to_string()))
                .unwrap_or_else(|| "external".to_string())
        } else {
            "manual".to_string()
        }
    }

    pub async fn find_existing(&self, host: &str, hash: &[u8]) -> Result<Option<Offre>, RepoError> {
        self.offres.find_by_content_hash(host, hash).await
    }

    pub async fn find_by_url(&self, url: &str) -> Result<Option<Offre>, RepoError> {
        if url == "manual" {
            return Ok(None);
        }
        self.offres.find_by_url(url).await
    }
}
