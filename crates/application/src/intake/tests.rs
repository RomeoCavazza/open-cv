#[cfg(test)]
mod tests {
    use crate::intake::extractor::{build_offre_slug, fallback_extraction};
    use crate::intake::resolver::{ContentResolver, looks_like_url};
    use ports::{Scraper, ScrapeResult, ScrapeError};
    use async_trait::async_trait;
    use std::sync::Arc;

    struct MockScraper;
    #[async_trait]
    impl Scraper for MockScraper {
        async fn scrape(&self, url: &str) -> Result<ScrapeResult, ScrapeError> {
            Ok(ScrapeResult {
                url: url.to_string(),
                final_url: url.to_string(),
                raw_html: "<html></html>".into(),
                raw_text: "Offre de test avec mission et profil. Nous cherchons un expert en Rust.".into(),
                status: 200,
            })
        }
        fn name(&self) -> &'static str { "mock" }
    }

    #[test]
    fn url_detection() {
        assert!(looks_like_url("https://example.com/job"));
        assert!(looks_like_url("http://foo.bar"));
        assert!(!looks_like_url("Ceci est du texte brut"));
    }

    #[test]
    fn slug_generation() {
        let slug = build_offre_slug("Safran", "Alternance Data Analyst (Roche la Molière)");
        assert!(slug.as_str().contains("safran"));
        assert!(slug.as_str().contains("data"));
    }

    #[test]
    fn fallback_gives_first_line() {
        let (intitule, entreprise, _, _, structured) =
            fallback_extraction("Mon offre cool\nAutre ligne\nEncore");
        assert_eq!(intitule, "Mon offre cool");
        assert_eq!(entreprise, "Non identifié");
        assert!(structured.resume_court.contains("échouée"));
    }

    #[tokio::test]
    async fn resolver_handles_url() {
        let resolver = ContentResolver::new(Arc::new(MockScraper));
        let (text, url) = resolver.resolve("https://example.com").await.expect("resolve should succeed");
        assert_eq!(url, "https://example.com");
        assert!(text.contains("Rust"));
    }
}
