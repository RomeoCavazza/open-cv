//! Resolver — Transformation d'un input brut (URL ou texte) en texte d'offre propre.

use crate::AppError;
use ports::Scraper;
use std::sync::Arc;
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
        validate_quality(&raw_text)?;

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
    let has_prompt_verb = ["genere", "génère", "generate", "cree", "crée", "create"]
        .iter()
        .any(|kw| normalized.contains(kw));

    let has_deliverable = ["cv", "resume", "lettre", "profil", "profile"]
        .iter()
        .any(|kw| normalized.contains(kw));

    has_prompt_verb || has_deliverable
}

fn build_direct_prompt_offer(raw_prompt: &str) -> String {
    let target_title = extract_target_title(raw_prompt);
    format!(
        "OFFRE GENERIQUE - PROMPT DIRECT\n\
         Demande utilisateur: {raw_prompt}\n\
         Intitule cible: {target_title}\n\
         Entreprise: Entreprise cible (non specifiee)\n\
         Localisation: Flexible / a definir\n\
         Contrat: A definir\n\n\
         CONTEXTE\n\
         Cette offre est creee a partir d'un prompt direct sans URL source. \
         Le moteur doit produire une candidature generique, credibile et actionnable.\n\n\
         MISSIONS\n\
         - Concevoir et implementer des solutions adaptees au role cible.\n\
         - Prioriser la qualite, la fiabilite, la documentation et la maintenance.\n\
         - Collaborer avec les equipes produit, engineering et operations.\n\
         - Contribuer a l'amelioration continue des pratiques techniques.\n\n\
         PROFIL RECHERCHE\n\
         - Excellente capacite de structuration et de communication.\n\
         - Maitrise des fondamentaux du poste vise et des bonnes pratiques.\n\
         - Approche pragmatique, autonomie, sens des responsabilites.\n\
         - Capacite a apprendre vite et a transmettre les connaissances.\n\n\
         COMPETENCES ET STACK\n\
         - Stack technique ou fonctionnelle a adapter selon le role cible.\n\
         - Experience operationnelle sur des projets concrets.\n\
         - Methodes de travail collaboratives et culture de feedback.\n\n\
         EXIGENCES\n\
         - Rigueur, curiosite, esprit d'initiative.\n\
         - Capacite a traduire un besoin metier en plan d'action clair.\n\
         - Souci de l'impact, des resultats et de la valeur livree."
    )
}

fn extract_target_title(raw_prompt: &str) -> String {
    let trimmed = raw_prompt.trim().trim_matches('"').trim_matches('\'');
    let lower = trimmed.to_lowercase();
    let patterns = [
        "génère un",
        "génère une",
        "genere un",
        "genere une",
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
    ];

    for pattern in patterns {
        if let Some(rest) = lower.strip_prefix(pattern) {
            let start = trimmed.len() - rest.len();
            let candidate = trimmed[start..].trim();
            if !candidate.is_empty() {
                return candidate.to_string();
            }
        }
    }

    trimmed.to_string()
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
