use std::{env, fs, path::Path};

use anyhow::{Context, Result};
use chrono::Utc;
use domain::{
    AnnexeId, ApprenticeshipSection, DocumentSection, EducationEntry, ExperienceEntry,
    JsonValue as DomainJsonValue, LanguageEntry, Profil, ProfilContent, ProfileSection,
    SkillCategoryEntry,
};
use ports::{AnnexeRepo, ProfilRepo};
use serde_json::json;

fn main_result() -> Result<()> {
    dotenvy::dotenv_override().ok();

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

        let profil_repo = adapter_postgres::ProfilRepoPg::new(pool.clone());
        let annexe_repo = adapter_postgres::AnnexeRepoPg::new(pool.clone());

        let seed = load_profile_seed(Path::new("data/user/profil_to_upload.md"))?;
        let photo_path = seed.photo_path.clone();
        let resume_template_path = seed.resume_template_path.clone();
        let cover_letter_template_path = seed.cover_letter_template_path.clone();
        let calendar_document_path = seed.calendar_document_path.clone();

        let photo_bytes = fs::read(&photo_path)
            .context("Impossible de lire la photo de profil")?;
        let resume_template = read_domain_json_value(&resume_template_path)?;
        let cover_letter_template = read_domain_json_value(&cover_letter_template_path)?;
        let calendar_pdf_path = calendar_document_path.as_deref().map(Path::new);
        let calendar_pdf = match calendar_pdf_path {
            Some(path) if path.is_file() => {
                Some(fs::read(path).context("Impossible de lire le PDF complémentaire")?)
            }
            _ => None,
        };

        let existing_profile = profil_repo
            .get_active()
            .await
            .context("Impossible de charger le profil actif")?;

        let profile_id = existing_profile.as_ref().map(|profil| profil.id).unwrap_or_default();
        let created_at = existing_profile
            .as_ref()
            .map(|profil| profil.created_at)
            .unwrap_or_else(Utc::now);

        let mut content = ProfilContent {
            profile: ProfileSection {
                firstname: seed.firstname,
                lastname: seed.lastname,
                title: seed.title,
                offer_type: seed.offer_type,
                pitch: if seed.pitch.contains("[choisir") {
                    "Actuellement étudiant en Master of Science à l’EPITECH (spécialisation Intelligence Artificielle), je souhaite mettre mes compétences en développement logiciel et en intégration de pipelines LLM/Data au service de projets innovants et stimulants.".to_string()
                } else {
                    seed.pitch
                },
                location: seed.location,
                phone: seed.phone,
                email: seed.email,
                linkedin: seed.linkedin,
                website: seed.website,
                github: seed.github,
                image: "persisted:bytea".to_string(),
            },
            apprenticeship: ApprenticeshipSection {
                duration: seed.apprenticeship_duration,
                rhythm: seed.apprenticeship_rhythm,
            },
            experiences: seed.experiences,
            projects: seed.projects,
            education: seed.education,
            skills: seed.skills,
            languages: seed.languages,
            documents: DocumentSection {
                resume_template: Some(resume_template.clone()),
                cover_letter_template: Some(cover_letter_template.clone()),
                apprenticeship_calendar: calendar_pdf_path.map(|path| {
                    serde_json::from_value(json!({
                        "filename": path.file_name().and_then(|name| name.to_str()).unwrap_or("calendar.pdf"),
                        "source_path": path.to_string_lossy(),
                    })).unwrap()
                }),
            },
        };

        // Post-processing corrections
        // 1. Split Harvard edX
        let mut new_education = Vec::new();
        for edu in content.education {
            if edu.school.contains("HarvardX") || edu.school.contains("edX") {
                if edu.degree.contains("CS50x") && edu.degree.contains("CS50W") {
                    new_education.push(EducationEntry {
                        school: "HarvardX, edX".to_string(),
                        degree: "CS50x — Introduction to Computer Science".to_string(),
                        period: "2026".to_string(),
                    });
                    new_education.push(EducationEntry {
                        school: "HarvardX, edX".to_string(),
                        degree: "CS50W — Web Programming with Python and JavaScript".to_string(),
                        period: "2026".to_string(),
                    });
                } else {
                    new_education.push(edu);
                }
            } else {
                new_education.push(edu);
            }
        }
        content.education = new_education;

        // 2. Add Project Links and Fix Titles
        for proj in &mut content.projects {
            let slug = proj.role.to_lowercase().replace(" ", "-");
            let url = match slug.as_str() {
                s if s.contains("setup-os") => "https://github.com/RomeoCavazza/setup-os",
                s if s.contains("shellgeist") => "https://github.com/RomeoCavazza/shellgeist",
                s if s.contains("nvim-config") => "https://github.com/RomeoCavazza/nvim-config",
                s if s.contains("ventoy-toolkit") => "https://github.com/RomeoCavazza/ventoy-toolkit",
                s if s.contains("station-inertielle") => "https://github.com/RomeoCavazza/tco-core",
                s if s.contains("hackathon") => "https://github.com/RomeoCavazza/CS50", // Fallback
                s if s.contains("hyprchroma") => "https://github.com/RomeoCavazza/Hyprchroma",
                s if s.contains("hypr-canvas") => "https://github.com/RomeoCavazza/hypr-canvas",
                s if s.contains("hyprspace") => "https://github.com/RomeoCavazza/Hyprspace",
                s if s.contains("veyl.io") => "https://github.com/RomeoCavazza/veyl.io",
                s if s.contains("hello-world") => "https://github.com/RomeoCavazza/piscine-epitech",
                s if s.contains("job-board") => "https://github.com/RomeoCavazza/piscine-epitech",
                s if s.contains("dryvia") => "https://github.com/RomeoCavazza/e-commerce",
                s if s.contains("dashboard") => "https://github.com/RomeoCavazza/dashboard",
                s if s.contains("landing-page") => "https://github.com/RomeoCavazza/landing-page",
                s if s.contains("no-low-code") => "https://github.com/RomeoCavazza/no-low-code",
                _ => "https://github.com/RomeoCavazza",
            };
            
            // Fix formatting "Role - Company"
            if !proj.company.is_empty() {
                proj.role = format!("{} - {}", proj.role, proj.company);
                proj.company = url.to_string(); // In UI, we can use this as a link if it looks like one
            }
        }

        let profil = Profil {
            id: profile_id,
            label: format!("{} {}", content.profile.firstname, content.profile.lastname),
            content,
            is_active: true,
            profile_photo: Some(photo_bytes),
            calendar_pdf,
            resume_template: Some(resume_template),
            cover_letter_template: Some(cover_letter_template),
            notes: serde_json::from_value(json!({
                "source_markdown": "data/user/profil_to_upload.md",
                "photo_path": photo_path,
                "resume_template_path": resume_template_path,
                "cover_letter_template_path": cover_letter_template_path,
                "calendar_pdf_path": calendar_pdf_path
                    .map(|path| path.to_string_lossy().to_string()),
            })).unwrap(),
            created_at,
        };

        profil_repo
            .upsert(&profil)
            .await
            .context("Impossible d'enregistrer le profil")?;

        println!(
            "Profil importé: {} ({} expériences, {} projets, {} formations)",
            profil.label,
            profil.content.experiences.len(),
            profil.content.projects.len(),
            profil.content.education.len()
        );

        if let Some(path) = calendar_pdf_path {
            let annexe_id = uuid::Uuid::parse_str("c01edada-1111-4444-8888-999999999999").unwrap();
            let annexe = domain::Annexe {
                id: AnnexeId::from_uuid(annexe_id),
                profil_id: profile_id,
                label: "Calendrier d'alternance".to_string(),
                filename: path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("calendar.pdf")
                    .to_string(),
                content_type: "application/pdf".to_string(),
                content: fs::read(path)
                    .context("Impossible de lire le PDF complémentaire")?,
                created_at,
            };

            annexe_repo
                .upsert(&annexe)
                .await
                .context("Impossible d'enregistrer le document complémentaire")?;
        }

        Ok(())
    })
}

fn main() {
    if let Err(error) = main_result() {
        eprintln!("Erreur: {error:#}");
        std::process::exit(1);
    }
}

struct ProfileSeed {
    firstname: String,
    lastname: String,
    title: String,
    offer_type: String,
    pitch: String,
    location: String,
    phone: String,
    email: String,
    linkedin: String,
    website: String,
    github: String,
    photo_path: String,
    apprenticeship_duration: String,
    apprenticeship_rhythm: String,
    experiences: Vec<ExperienceEntry>,
    projects: Vec<ExperienceEntry>,
    education: Vec<EducationEntry>,
    skills: Vec<SkillCategoryEntry>,
    languages: Vec<LanguageEntry>,
    resume_template_path: String,
    cover_letter_template_path: String,
    calendar_document_path: Option<String>,
}

fn load_profile_seed(path: &Path) -> Result<ProfileSeed> {
    let content = fs::read_to_string(path).with_context(|| format!("Impossible de lire {}", path.display()))?;
    let lines: Vec<&str> = content.lines().collect();

    let photo_path = find_prefixed_value(&lines, "photo de profil :")
        .context("photo de profil introuvable")?;

    let firstname = find_prefixed_value(&lines, "Prénom :").context("Prénom introuvable")?;
    let lastname = find_prefixed_value(&lines, "Nom :").context("Nom introuvable")?;
    let title = find_prefixed_value(&lines, "Titre visé :").context("Titre visé introuvable")?;
    let offer_type = find_prefixed_value(&lines, "Type d'offre :").context("Type d'offre introuvable")?;
    let apprenticeship_duration = find_prefixed_value(&lines, "Durée souhaitée :")
        .context("Durée souhaitée introuvable")?;
    let apprenticeship_rhythm = find_prefixed_value(&lines, "Rythme :").context("Rythme introuvable")?;
    let pitch = find_prefixed_value(&lines, "Bio :").context("Bio introuvable")?;
    let location = find_prefixed_value(&lines, "Ville :").context("Ville introuvable")?;
    let phone = find_prefixed_value(&lines, "Téléphone :").context("Téléphone introuvable")?;
    let email = find_prefixed_value(&lines, "email :").context("email introuvable")?;
    let linkedin = find_prefixed_value(&lines, "LinkedIn :").context("LinkedIn introuvable")?;
    let website = find_prefixed_value(&lines, "Site personnel :").context("Site personnel introuvable")?;
    let github = find_prefixed_value(&lines, "Github :").context("Github introuvable")?;

    let experiences = parse_experience_blocks(&slice_between(&lines, "Expériences pro :", "Projets persos :"));
    let projects = parse_project_blocks(&slice_between(&lines, "Projets persos :", "Formations :"));
    let education = parse_education_blocks(&slice_between(&lines, "Formations :", "Compétences"));
    let skills = parse_skill_groups(&slice_between(&lines, "Compétences", "Langues :"));
    let languages = parse_languages(&slice_between(&lines, "Langues :", "Documents :"));

    let resume_template_path = find_prefixed_value(&lines, "Modèle de CV :")
        .context("Modèle de CV introuvable")?;
    let cover_letter_template_path = find_prefixed_value(&lines, "Modèle de lettre :")
        .context("Modèle de lettre introuvable")?;
    let calendar_document_path = extract_document_path(&lines, "Documents supplémentaires :");

    Ok(ProfileSeed {
        firstname,
        lastname,
        title,
        offer_type,
        pitch,
        location,
        phone,
        email,
        linkedin,
        website,
        github,
        photo_path,
        apprenticeship_duration,
        apprenticeship_rhythm,
        experiences,
        projects,
        education,
        skills,
        languages,
        resume_template_path,
        cover_letter_template_path,
        calendar_document_path,
    })
}

fn find_prefixed_value(lines: &[&str], prefix: &str) -> Option<String> {
    lines.iter().find_map(|line| {
        let trimmed = line.trim();
        trimmed
            .strip_prefix(prefix)
            .map(|value| value.trim().to_string())
    })
}

fn extract_document_path(lines: &[&str], marker: &str) -> Option<String> {
    let index = lines.iter().position(|line| line.contains(marker))?;
    lines[index]
        .trim()
        .strip_prefix(marker)
        .and_then(|value| {
            let value = value.trim();
            if value.is_empty() {
                lines[index + 1..]
                    .iter()
                    .map(|line| line.trim())
                    .find(|line| !line.is_empty())
                    .map(|line| line.to_string())
            } else {
                Some(value.to_string())
            }
        })
}

fn slice_between<'a>(lines: &'a [&'a str], start_marker: &str, end_marker: &str) -> Vec<&'a str> {
    let start = lines
        .iter()
        .position(|line| line.contains(start_marker))
        .map(|index| index + 1)
        .unwrap_or(0);
    let end = lines[start..]
        .iter()
        .position(|line| line.contains(end_marker))
        .map(|offset| start + offset)
        .unwrap_or(lines.len());

    lines[start..end].to_vec()
}

fn split_blocks<'a>(lines: &'a [&'a str]) -> Vec<Vec<&'a str>> {
    let mut blocks = Vec::new();
    let mut current = Vec::new();

    for line in lines {
        if line.trim().is_empty() {
            if !current.is_empty() {
                blocks.push(current);
                current = Vec::new();
            }
            continue;
        }
        current.push(*line);
    }

    if !current.is_empty() {
        blocks.push(current);
    }

    blocks
}

fn parse_experience_blocks(lines: &[&str]) -> Vec<ExperienceEntry> {
    split_blocks(lines)
        .into_iter()
        .filter_map(|block| {
            if block.len() < 2 {
                return None;
            }

            let role = strip_markdown(block[0]);
            let (company, period) = split_company_period(block[1]);
            let description = block[2..]
                .iter()
                .map(|line| strip_bullet(line))
                .filter(|line| !line.is_empty())
                .collect();

            Some(ExperienceEntry {
                role,
                company,
                period,
                description,
            })
        })
        .filter(|e| !e.role.is_empty() && e.role != "---")
        .collect()
}

fn parse_project_blocks(lines: &[&str]) -> Vec<ExperienceEntry> {
    split_blocks(lines)
        .into_iter()
        .filter_map(|block| {
            if block.is_empty() {
                return None;
            }

            let header = strip_markdown(block[0]);
            let (header_without_period, period) = extract_trailing_period(&header);
            let header_without_link = header_without_period
                .split("(lien :")
                .next()
                .unwrap_or(&header_without_period)
                .trim();
            let (role, company) = split_title_subtitle(header_without_link);
            let description = block[1..]
                .iter()
                .map(|line| strip_bullet(line))
                .filter(|line| !line.is_empty())
                .collect();

            Some(ExperienceEntry {
                role,
                company,
                period,
                description,
            })
        })
        .filter(|e| !e.role.is_empty() && e.role != "---")
        .collect()
}

fn parse_education_blocks(lines: &[&str]) -> Vec<EducationEntry> {
    split_blocks(lines)
        .into_iter()
        .filter_map(|block| {
            if block.is_empty() {
                return None;
            }

            let (school, period) = split_school_period(block[0]);
            let degree = block[1..]
                .iter()
                .map(|line| strip_markdown(line))
                .filter(|line| !line.is_empty())
                .collect::<Vec<_>>()
                .join(" / ");

            Some(EducationEntry { school, degree, period })
        })
        .filter(|e| !e.school.is_empty() && e.school != "---")
        .collect()
}

fn parse_skill_groups(lines: &[&str]) -> Vec<SkillCategoryEntry> {
    let cleaned: Vec<String> = lines
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect();

    let mut result = Vec::new();
    let mut index = 0;

    while index + 1 < cleaned.len() {
        let category = cleaned[index].clone();
        let items = cleaned[index + 1]
            .split(',')
            .map(|item| item.trim().to_string())
            .filter(|item| !item.is_empty())
            .collect();

        result.push(SkillCategoryEntry { category, items });
        index += 2;
    }

    result
}

fn parse_languages(lines: &[&str]) -> Vec<LanguageEntry> {
    lines
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let (name, level) = line.split_once(':')?;
            Some(LanguageEntry {
                name: name.trim().to_string(),
                level: level.trim().to_string(),
            })
        })
        .collect()
}

fn split_company_period(line: &str) -> (String, String) {
    if let Some((company, period)) = line.split_once('|') {
        (company.trim().to_string(), period.trim().to_string())
    } else {
        (line.trim().to_string(), String::new())
    }
}

fn split_school_period(line: &str) -> (String, String) {
    let trimmed = line.trim();
    if let Some(index) = trimmed.rfind('(') {
        if trimmed.ends_with(')') {
            let school = trimmed[..index].trim().to_string();
            let period = trimmed[index + 1..trimmed.len() - 1].trim().to_string();
            return (school, period);
        }
    }

    (trimmed.to_string(), String::new())
}

fn split_title_subtitle(line: &str) -> (String, String) {
    for separator in [" — ", " – ", " - "] {
        if let Some((title, subtitle)) = line.split_once(separator) {
            return (normalize_title(title), subtitle.trim().to_string());
        }
    }

    (normalize_title(line), String::new())
}

fn extract_trailing_period(line: &str) -> (String, String) {
    let trimmed = line.trim();
    if let Some(index) = trimmed.rfind('(') {
        if trimmed.ends_with(')') {
            let period = trimmed[index + 1..trimmed.len() - 1].trim().to_string();
            let before = trimmed[..index].trim().trim_end_matches("(").trim().to_string();
            return (before, period);
        }
    }

    (trimmed.to_string(), String::new())
}

fn normalize_title(title: &str) -> String {
    let trimmed = strip_markdown(title);
    trimmed
        .trim_start_matches(|c: char| c.is_ascii_digit() || c == '.' || c == ' ')
        .trim()
        .to_string()
}

fn strip_markdown(line: &str) -> String {
    line.replace("**", "")
        .replace("`", "")
        .trim()
        .to_string()
}

fn strip_bullet(line: &str) -> String {
    strip_markdown(line).trim_start_matches("- ").trim().to_string()
}

fn read_domain_json_value(path: impl AsRef<Path>) -> Result<DomainJsonValue> {
    let path = path.as_ref();
    let content = fs::read_to_string(path).with_context(|| format!("Impossible de lire {}", path.display()))?;
    serde_json::from_str(&content).with_context(|| format!("JSON invalide dans {}", path.display()))
}
