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
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Ok(None);
        }
        self.offres.find_by_url(url).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use domain::{Offre, OffreId, Slug};
    use ports::OffreRepo;

    struct UrlEchoRepo;

    #[async_trait]
    impl OffreRepo for UrlEchoRepo {
        async fn get_by_id(&self, _id: OffreId) -> Result<Option<Offre>, RepoError> {
            Ok(None)
        }
        async fn get_by_slug(&self, _slug: &Slug) -> Result<Option<Offre>, RepoError> {
            Ok(None)
        }
        async fn list_all(&self) -> Result<Vec<Offre>, RepoError> {
            Ok(vec![])
        }
        async fn list_recent(&self, _limit: u32) -> Result<Vec<Offre>, RepoError> {
            Ok(vec![])
        }
        async fn upsert(&self, _offre: &Offre) -> Result<(), RepoError> {
            Ok(())
        }
        async fn count(&self) -> Result<u64, RepoError> {
            Ok(0)
        }
        async fn find_by_url(&self, url: &str) -> Result<Option<Offre>, RepoError> {
            // If this is called for manual sources, the test should fail.
            Err(RepoError::Other(format!(
                "should not query repo for url={url}"
            )))
        }
        async fn find_by_content_hash(
            &self,
            _source_host: &str,
            _hash: &[u8],
        ) -> Result<Option<Offre>, RepoError> {
            Ok(None)
        }
    }

    #[tokio::test]
    async fn skip_url_dedup_for_non_http_sources() {
        let dedup = Deduplicator::new(Arc::new(UrlEchoRepo));

        let r1 = dedup.find_by_url("manual_prompt").await;
        let r2 = dedup.find_by_url("manual").await;
        let r3 = dedup.find_by_url("manual_json").await;

        assert!(r1.is_ok());
        assert!(r2.is_ok());
        assert!(r3.is_ok());
        assert!(r1.unwrap().is_none());
        assert!(r2.unwrap().is_none());
        assert!(r3.unwrap().is_none());
    }
}
