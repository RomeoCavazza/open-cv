use crate::intake::extractor::{build_offre_slug, fallback_extraction};
use crate::intake::resolver::{canonicalize_source_url, looks_like_url, ContentResolver};
use async_trait::async_trait;
use ports::{ScrapeError, ScrapeResult, Scraper};
use std::sync::Arc;

struct MockScraper;
#[async_trait]
impl Scraper for MockScraper {
    async fn scrape(&self, url: &str) -> Result<ScrapeResult, ScrapeError> {
        Ok(ScrapeResult {
            url: url.to_string(),
            final_url: url.to_string(),
            raw_html: "<html></html>".into(),
            raw_text: "Offre de test avec mission et profil. Nous cherchons un expert en Rust."
                .into(),
            status: 200,
        })
    }
    fn name(&self) -> &'static str {
        "mock"
    }
}

struct NoisySingleLineScraper;
#[async_trait]
impl Scraper for NoisySingleLineScraper {
    async fn scrape(&self, url: &str) -> Result<ScrapeResult, ScrapeError> {
        Ok(ScrapeResult {
            url: url.to_string(),
            final_url: url.to_string(),
            raw_html: "<html></html>".into(),
            raw_text: "Mission principale: analyser des donnees, construire des tableaux de bord, accompagner les equipes produit sur les KPI et prioriser les actions. Profil recherche: autonomie, communication, SQL, Python, esprit d'analyse. Cookie banner, politique de confidentialite et autres textes de navigation. Cette phrase allonge le contenu pour depasser les seuils de validation metier et reproduire un scraping en ligne unique sans retours a la ligne.".into(),
            status: 200,
        })
    }
    fn name(&self) -> &'static str {
        "mock_noisy_single_line"
    }
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
fn slug_generation_without_company() {
    let slug = build_offre_slug("Non spécifié", "Développeur Kotlin");
    assert!(!slug.as_str().contains("non_specifie"));
    assert!(slug.as_str().contains("developpeur"));
}

#[test]
fn fallback_gives_first_line() {
    let (intitule, entreprise, _, _, structured) =
        fallback_extraction("Mon offre cool\nAutre ligne\nEncore");
    assert_eq!(intitule, "Mon offre cool");
    assert_eq!(entreprise, "Non spécifié");
    assert!(structured.resume_court.contains("échouée"));
}

#[tokio::test]
async fn resolver_handles_url() {
    let resolver = ContentResolver::new(Arc::new(MockScraper));
    let (text, url) = resolver
        .resolve("https://example.com")
        .await
        .expect("resolve should succeed");
    assert_eq!(url, "https://example.com/");
    assert!(text.contains("Rust"));
}

#[test]
fn canonicalizes_linkedin_url() {
    let out = canonicalize_source_url(
        "https://www.linkedin.com/jobs/view/1234567890/?trk=public_jobs_topcard-title",
    );
    assert_eq!(out, "https://www.linkedin.com/jobs/view/1234567890");
}

#[test]
fn canonicalizes_indeed_url() {
    let out =
        canonicalize_source_url("https://www.indeed.com/viewjob?jk=abc123&from=shareddesktop_copy");
    assert_eq!(out, "https://www.indeed.com/viewjob?jk=abc123");
}

#[tokio::test]
async fn resolver_accepts_direct_prompt() {
    let resolver = ContentResolver::new(Arc::new(MockScraper));
    let (text, source) = resolver
        .resolve("Genere un profil DevOps")
        .await
        .expect("direct prompt should be accepted");
    assert_eq!(source, "manual_prompt");
    assert!(text.to_lowercase().contains("__target_title__"));
    assert!(text.to_lowercase().contains("contexte de la demande"));
    assert!(text.to_lowercase().contains("description déduite"));
}

#[tokio::test]
async fn resolver_prefers_quoted_title_for_direct_prompt() {
    let resolver = ContentResolver::new(Arc::new(MockScraper));
    let (text, source) = resolver
        .resolve(r#"Fais-moi un CV pour un poste en alternance, intitulé exact "Data Engineer"."#)
        .await
        .expect("direct prompt with quoted title should be accepted");
    assert_eq!(source, "manual_prompt");
    assert!(text.contains("__TARGET_TITLE__: Data Engineer"));
}

#[tokio::test]
async fn resolver_normalizes_noisy_cv_wording_in_target_title() {
    let resolver = ContentResolver::new(Arc::new(MockScraper));
    let (text, source) = resolver
        .resolve("CV pour un poste de Développeur Python en alternance")
        .await
        .expect("noisy direct prompt should be normalized");
    assert_eq!(source, "manual_prompt");
    assert!(
        text.contains("__TARGET_TITLE__: Développeur Python"),
        "text réel: {text}"
    );
}

#[tokio::test]
async fn resolver_normalizes_embedded_target_title_marker() {
    let resolver = ContentResolver::new(Arc::new(MockScraper));
    let (text, source) = resolver
        .resolve("__TARGET_TITLE__: CV pour un poste de Développeur Python en alternance")
        .await
        .expect("embedded target title marker should be normalized");
    assert_eq!(source, "manual_prompt");
    assert!(
        text.contains("__TARGET_TITLE__: Développeur Python"),
        "text réel: {text}"
    );
}

#[tokio::test]
async fn resolver_keeps_meaningful_single_line_with_cookie_word() {
    let resolver = ContentResolver::new(Arc::new(NoisySingleLineScraper));
    let (text, source) = resolver
        .resolve("https://example.com/offre")
        .await
        .expect("single line should remain valid");
    assert_eq!(source, "https://example.com/offre");
    assert!(text.to_lowercase().contains("mission principale"));
    assert!(text.to_lowercase().contains("profil recherche"));
}
