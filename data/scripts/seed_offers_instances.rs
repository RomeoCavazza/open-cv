use std::{collections::HashMap, env, fs, path::{Path, PathBuf}};

use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
use domain::{
    CoverLetter, Instance, InstanceId, InstanceStatus, Offre, OffreId, OffreStructured,
    Paragraphe, ParagrapheRole, Resume, Slug,
};
use ports::{InstanceRepo, OffreRepo, ProfilRepo};
use serde::Deserialize;
use serde_json::json;
use sha2::{Digest, Sha256};

fn main() {
    if let Err(error) = main_result() {
        eprintln!("Erreur: {error:#}");
        std::process::exit(1);
    }
}

fn main_result() -> Result<()> {
    dotenvy::dotenv_override().ok();
    tracing_subscriber::fmt::init();


    let database_url = env::var("DATABASE_URL").context("DATABASE_URL non défini")?;
    let rt = tokio::runtime::Runtime::new().context("Impossible de créer le runtime Tokio")?;

    rt.block_on(async move {
        let pool = adapter_postgres::connect(&database_url)
            .await
            .context("Connexion Postgres impossible")?;

        sqlx::migrate!("../../migrations")
            .run(&pool)
            .await
            .context("Migrations échouées")?;

        let offre_repo = adapter_postgres::OffreRepoPg::new(pool.clone());
        let instance_repo = adapter_postgres::InstanceRepoPg::new(pool.clone());
        let profil_repo = adapter_postgres::ProfilRepoPg::new(pool.clone());

        let profile = profil_repo
            .get_active()
            .await
            .context("Impossible de charger le profil actif")?
            .context("Aucun profil actif trouvé: lance d'abord seed_profile")?;

        let offer_index = load_offer_list(Path::new("data/offres/liste.json"))?;
        let offer_raw_dir = Path::new("data/offres/raw");

        let mut offers_imported = 0usize;
        for path in sorted_md_files(offer_raw_dir)? {
            let job_id = path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .context("Nom de fichier offre invalide")?;

            let raw_text = fs::read_to_string(&path)
                .with_context(|| format!("Impossible de lire {}", path.display()))?;
            let parsed = parse_offer_seed(job_id, &raw_text, offer_index.get(job_id))?;

            let existing = offre_repo
                .get_by_slug(&parsed.slug)
                .await
                .context("Impossible de vérifier l'offre existante")?;

            let offre = Offre {
                id: existing.as_ref().map(|offre| offre.id).unwrap_or_else(OffreId::new),
                slug: parsed.slug,
                source_url: parsed.source_url,
                source_host: parsed.source_host,
                source_hash: parsed.source_hash,
                entreprise: parsed.entreprise,
                intitule: parsed.intitule,
                localisation: parsed.localisation,
                contrat: parsed.contrat,
                raw_text: parsed.raw_text,
                structured: OffreStructured::default(),
                scraped_at: existing
                    .as_ref()
                    .map(|offre| offre.scraped_at)
                    .unwrap_or(parsed.scraped_at),
                last_seen_at: Utc::now(),
                closed_at: existing.as_ref().and_then(|offre| offre.closed_at),
                categorie: parsed.categorie,
            };

            offre_repo
                .upsert(&offre)
                .await
                .with_context(|| format!("Impossible d'enregistrer l'offre {}", offre.slug))?;
            offers_imported += 1;
        }

        let instances_dir = Path::new("data/instances");
        let mut instances_imported = 0usize;
        for dir in sorted_instance_dirs(instances_dir)? {
            let meta_path = dir.join("meta.json");
            let resume_path = dir.join("resume.json");
            let cover_letter_path = dir.join("cover-letter.json");
            let restitution_path = dir.join("restitution.json");

            let meta: InstanceMeta = read_json_file(&meta_path)
                .with_context(|| format!("Impossible de lire {}", meta_path.display()))?;

            let folder_name = dir.file_name().unwrap().to_string_lossy();
            let instance_slug = Slug::parse(folder_name.as_ref())
                .with_context(|| format!("Slug d'instance invalide : {}", folder_name))?;
            
            let offer_slug = Slug::parse(meta.job_id.clone())
                .with_context(|| format!("Slug d'offre invalide dans meta : {}", meta.job_id))?;

            let offre = offre_repo
                .get_by_slug(&offer_slug)
                .await
                .with_context(|| format!("Impossible de charger l'offre {}", offer_slug))?
                .with_context(|| format!("Offre manquante pour l'instance {} (job_id: {})", folder_name, offer_slug))?;

            let existing = instance_repo
                .get_by_slug(&instance_slug)
                .await
                .with_context(|| format!("Impossible de vérifier l'instance {}", instance_slug))?;

            let resume_json = read_legacy_resume(&resume_path)
                .with_context(|| format!("Impossible de lire {}", resume_path.display()))?;
            let cover_letter_json = read_legacy_cover_letter(&cover_letter_path)
                .with_context(|| format!("Impossible de lire {}", cover_letter_path.display()))?;
            let restitution = if restitution_path.exists() {
                Some(
                    read_json_file(&restitution_path).with_context(|| {
                        format!("Impossible de lire {}", restitution_path.display())
                    })?,
                )
            } else {
                None
            };

            let status = parse_instance_status(&meta.status)?;
            let now = Utc::now();

            let instance = Instance {
                id: existing
                    .as_ref()
                    .map(|instance| instance.id)
                    .unwrap_or_else(InstanceId::new),
                slug: instance_slug,
                offre_id: offre.id,
                profil_id: profile.id,
                status,
                restitution,
                resume_json: Some(resume_json),
                cover_letter_json: Some(cover_letter_json),
                notes: serde_json::from_value(json!({
                    "source_instance_dir": dir.to_string_lossy(),
                    "source_offer": meta.source_offer,
                    "job_id": meta.job_id,
                    "version": meta.version,
                    "status": meta.status,
                })).unwrap(),
                created_at: existing
                    .as_ref()
                    .map(|instance| instance.created_at)
                    .unwrap_or(now),
                updated_at: now,
                sent_at: existing.as_ref().and_then(|instance| instance.sent_at),
            };

            instance_repo
                .upsert(&instance)
                .await
                .with_context(|| format!("Impossible d'enregistrer l'instance {}", instance.slug))?;
            instances_imported += 1;
            tracing::debug!("Imported instance {} for offer {}", instance.slug, meta.job_id);
        }

        println!(
            "Offres importées: {} | Instances importées: {} | Profil actif: {}",
            offers_imported,
            instances_imported,
            profile.label
        );

        Ok(())
    })
}

#[derive(Debug, Deserialize)]
struct OfferListFile {
    entries: Vec<OfferListEntry>,
}

#[derive(Debug, Clone, Deserialize)]
struct OfferListEntry {
    title: String,
    url: String,
    category: String,
    job_id: String,
}

#[derive(Debug)]
struct ParsedOffer {
    slug: Slug,
    source_url: String,
    source_host: String,
    source_hash: Vec<u8>,
    entreprise: String,
    intitule: String,
    localisation: Option<String>,
    contrat: Option<String>,
    raw_text: String,
    scraped_at: DateTime<Utc>,
    categorie: Option<String>,
}

#[derive(Debug, Deserialize)]
struct InstanceMeta {
    job_id: String,
    source_offer: String,
    status: String,
    version: String,
}

fn load_offer_list(path: &Path) -> Result<HashMap<String, OfferListEntry>> {
    if !path.exists() {
        return Ok(HashMap::new());
    }

    let file = read_json_file::<OfferListFile>(path)
        .with_context(|| format!("Impossible de lire {}", path.display()))?;

    Ok(file
        .entries
        .into_iter()
        .map(|entry| (entry.job_id.clone(), entry))
        .collect())
}

fn parse_offer_seed(job_id: &str, raw_text: &str, list_entry: Option<&OfferListEntry>) -> Result<ParsedOffer> {
    let slug = Slug::parse(job_id.to_string())?;
    let title = list_entry
        .map(|entry| entry.title.clone())
        .or_else(|| extract_first_heading(raw_text))
        .unwrap_or_else(|| job_id.replace('_', " "));

    let source_url = extract_first_url(raw_text)
        .or_else(|| list_entry.map(|entry| entry.url.clone()))
        .context("URL source introuvable")?;
    let source_host = extract_host(&source_url);
    let source_hash = sha256_bytes(raw_text.as_bytes());
    let (entreprise, intitule) = split_offer_title(&title);
    let localisation = extract_prefixed_value(raw_text, &["Emplacement :", "Localisation :", "Lieu :"]);
    let contrat = extract_prefixed_value(raw_text, &["Type de contrat :", "Contrat :"]);
    let scraped_at = extract_offer_date(raw_text).unwrap_or_else(Utc::now);
    let categorie = list_entry.map(|entry| entry.category.clone());

    Ok(ParsedOffer {
        slug,
        source_url,
        source_host,
        source_hash,
        entreprise,
        intitule,
        localisation,
        contrat,
        raw_text: raw_text.to_string(),
        scraped_at,
        categorie,
    })
}

fn extract_first_heading(raw_text: &str) -> Option<String> {
    raw_text
        .lines()
        .map(|line| line.trim())
        .find(|line| line.starts_with("# "))
        .map(|line| line.trim_start_matches('#').trim().to_string())
}

fn extract_first_url(raw_text: &str) -> Option<String> {
    raw_text.lines().find_map(|line| {
        let trimmed = line.trim();
        let http_index = trimmed.find("http://").or_else(|| trimmed.find("https://"))?;
        let url = &trimmed[http_index..];
        let end = url
            .find(|ch: char| ch.is_whitespace() || ch == ')' || ch == '"')
            .unwrap_or(url.len());
        Some(url[..end].trim_matches(&['[', ']', '(', ')'][..]).to_string())
    })
}

fn extract_host(url: &str) -> String {
    let without_scheme = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url);
    without_scheme
        .split('/')
        .next()
        .unwrap_or(without_scheme)
        .to_string()
}

fn split_offer_title(title: &str) -> (String, String) {
    for separator in [" - ", " — ", " – "] {
        if let Some((entreprise, intitule)) = title.split_once(separator) {
            return (
                entreprise.trim().to_string(),
                intitule.trim().to_string(),
            );
        }
    }

    (title.trim().to_string(), String::new())
}

fn extract_prefixed_value(raw_text: &str, prefixes: &[&str]) -> Option<String> {
    for line in raw_text.lines() {
        let trimmed = line.trim();
        for prefix in prefixes {
            if let Some(value) = trimmed.strip_prefix(prefix) {
                return Some(value.trim().to_string());
            }
        }
    }

    None
}

fn extract_offer_date(raw_text: &str) -> Option<DateTime<Utc>> {
    for line in raw_text.lines() {
        let trimmed = line.trim();
        let value = trimmed
            .strip_prefix("**Date:**")
            .or_else(|| trimmed.strip_prefix("Date:"))
            .or_else(|| trimmed.strip_prefix("Date de récupération:"))
            .or_else(|| trimmed.strip_prefix("**Date de récupération:**"))?;
        let value = value.trim();

        if let Ok(datetime) = DateTime::parse_from_rfc3339(value) {
            return Some(datetime.with_timezone(&Utc));
        }

        if let Ok(date) = NaiveDate::parse_from_str(value, "%Y-%m-%d") {
            if let Some(naive) = date.and_hms_opt(0, 0, 0) {
                return Some(Utc.from_utc_datetime(&naive));
            }
        }

        if let Ok(naive) = NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S") {
            return Some(Utc.from_utc_datetime(&naive));
        }
    }

    None
}

fn parse_instance_status(status: &str) -> Result<InstanceStatus> {
    match status.trim().to_lowercase().as_str() {
        "draft" => Ok(InstanceStatus::Draft),
        "generating" => Ok(InstanceStatus::Generating),
        "ready" => Ok(InstanceStatus::Ready),
        "sent" => Ok(InstanceStatus::Sent),
        "archived" => Ok(InstanceStatus::Archived),
        "failed" => Ok(InstanceStatus::Failed),
        other => anyhow::bail!("Statut d'instance inconnu: {}", other),
    }
}

fn sha256_bytes(bytes: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize().to_vec()
}

fn sorted_md_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut paths: Vec<PathBuf> = fs::read_dir(dir)
        .with_context(|| format!("Impossible de parcourir {}", dir.display()))?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("md"))
        .collect();
    paths.sort();
    Ok(paths)
}

fn sorted_instance_dirs(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut paths: Vec<PathBuf> = fs::read_dir(dir)
        .with_context(|| format!("Impossible de parcourir {}", dir.display()))?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.is_dir())
        .collect();
    paths.sort();
    Ok(paths)
}

fn read_json_file<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T> {
    let content = fs::read_to_string(path).with_context(|| format!("Impossible de lire {}", path.display()))?;
    serde_json::from_str(&content).with_context(|| format!("JSON invalide dans {}", path.display()))
}

#[derive(Debug, Deserialize)]
struct LegacyResumeSeed {
    profile: LegacyResumeProfile,
    apprenticeship: LegacyApprenticeship,
    #[serde(default)]
    experiences: Vec<LegacyExperience>,
    #[serde(default)]
    projects: Vec<LegacyExperience>,
    #[serde(default)]
    education: Vec<LegacyEducation>,
    #[serde(default)]
    skills: Vec<LegacySkillCategory>,
    #[serde(default)]
    languages: Vec<LegacyLanguage>,
}

#[derive(Debug, Deserialize)]
struct LegacyResumeProfile {
    name: String,
    title: String,
    image: Option<String>,
    location: String,
    email: String,
    phone: Option<String>,
    website: Option<String>,
    linkedin: Option<String>,
    github: Option<String>,
    pitch: String,
}

#[derive(Debug, Deserialize)]
struct LegacyApprenticeship {
    duration: String,
    rhythm: String,
}

#[derive(Debug, Deserialize)]
struct LegacyExperience {
    company: String,
    #[serde(default)]
    role: String,
    period: String,
    #[serde(default)]
    description: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct LegacyEducation {
    school: String,
    degree: String,
    #[serde(default)]
    period: String,
}

#[derive(Debug, Deserialize)]
struct LegacySkillCategory {
    category: String,
    #[serde(default)]
    items: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct LegacyLanguage {
    name: String,
    level: String,
}

#[derive(Debug, Deserialize)]
struct LegacyCoverLetterSeed {
    profile: LegacyCoverLetterProfile,
    letter: LegacyLetter,
}

#[derive(Debug, Deserialize)]
struct LegacyCoverLetterProfile {
    name: String,
    address: Option<String>,
    phone: Option<String>,
    email: String,
    links: LegacyLinks,
}

#[derive(Debug, Deserialize)]
struct LegacyLinks {
    #[serde(default)]
    linkedin: Option<String>,
    #[serde(default)]
    github: Option<String>,
    #[serde(default)]
    website: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LegacyLetter {
    company: String,
    date: String,
    subject: String,
    greeting: String,
    #[serde(default)]
    paragraphs: Vec<String>,
    closing: String,
    signature: String,
}

fn read_legacy_resume(path: &Path) -> Result<Resume> {
    let seed: LegacyResumeSeed = read_json_file(path)?;

    Ok(Resume {
        identite: domain::Identite {
            nom_complet: seed.profile.name,
            photo_url: seed.profile.image,
        },
        accroche: domain::Accroche {
            titre: seed.profile.title,
            paragraphe: seed.profile.pitch,
            duree: seed.apprenticeship.duration,
            rythme: seed.apprenticeship.rhythm,
        },
        contact: domain::Contact {
            localisation: seed.profile.location,
            telephone: seed.profile.phone,
            email: seed.profile.email,
            site_web: seed.profile.website,
            linkedin: seed.profile.linkedin,
            github: seed.profile.github,
        },
        competences: seed
            .skills
            .into_iter()
            .map(|skill| domain::GroupeCompetences {
                categorie: skill.category,
                items: skill.items,
            })
            .collect(),
        experiences: seed
            .experiences
            .into_iter()
            .map(|experience| domain::Experience {
                poste: experience.role,
                entreprise: experience.company,
                localisation: None,
                periode: experience.period,
                bullets: experience.description,
            })
            .collect(),
        formations: seed
            .education
            .into_iter()
            .map(|education| domain::Formation {
                etablissement: education.school,
                localisation: None,
                periode: education.period,
                diplome: education.degree,
                details: None,
            })
            .collect(),
        projets: seed
            .projects
            .into_iter()
            .map(|project| domain::Projet {
                nom: if project.role.is_empty() {
                    project.company
                } else {
                    format!("{} — {}", project.role, project.company)
                },
                periode: project.period,
                bullets: project.description,
                lien: None,
            })
            .collect(),
        langues: seed
            .languages
            .into_iter()
            .map(|language| domain::Langue {
                langue: language.name,
                niveau: language.level,
            })
            .collect(),
    })
}

fn read_legacy_cover_letter(path: &Path) -> Result<CoverLetter> {
    let seed: LegacyCoverLetterSeed = read_json_file(path)?;

    let mut paragraphes = vec![Paragraphe {
        role: ParagrapheRole::Salutation,
        contenu: seed.letter.greeting,
    }];

    let roles = [
        ParagrapheRole::Accroche,
        ParagrapheRole::Projets,
        ParagrapheRole::Vous,
        ParagrapheRole::Pourquoi,
    ];

    for (index, paragraph) in seed.letter.paragraphs.into_iter().enumerate() {
        let role = roles.get(index).copied().unwrap_or(ParagrapheRole::Pourquoi);
        paragraphes.push(Paragraphe { role, contenu: paragraph });
    }

    paragraphes.push(Paragraphe {
        role: ParagrapheRole::Cloture,
        contenu: seed.letter.closing,
    });

    let category = seed
        .letter
        .subject
        .split_once(" - ")
        .map(|(head, _)| head.trim().to_string())
        .unwrap_or_else(|| "ALTERNANCE".to_string());

    Ok(CoverLetter {
        expediteur: domain::Expediteur {
            identite: domain::Identite {
                nom_complet: seed.profile.name,
                photo_url: None,
            },
            contact: domain::Contact {
                localisation: seed.profile.address.unwrap_or_default(),
                telephone: seed.profile.phone,
                email: seed.profile.email,
                site_web: seed.profile.links.website,
                linkedin: seed.profile.links.linkedin,
                github: seed.profile.links.github,
            },
        },
        destinataire: domain::Destinataire {
            entreprise: seed.letter.company,
            date: seed.letter.date,
        },
        objet: domain::Objet {
            categorie: category,
            libelle: seed.letter.subject,
        },
        paragraphes,
        signature: domain::Signature {
            formule_politesse: seed
                .letter
                .signature
                .lines()
                .next()
                .unwrap_or("Cordialement,")
                .to_string(),
            nom: seed
                .letter
                .signature
                .lines()
                .last()
                .unwrap_or("Roméo Cavazza")
                .to_string(),
        },
    })
}
