use chrono::Datelike;
use domain::{Chunk, CoverLetter, InstanceId, Offre, Profil, Restitution, Resume, Slug};

use super::{CandidaturePlan, GenerateError};

pub fn build_query_text(offre: &Offre) -> String {
    format!(
        "{} chez {}. Stack: {}. Missions: {}. Exigences: {}.",
        offre.intitule,
        offre.entreprise,
        offre.structured.stack.join(", "),
        offre.structured.missions.join(" ; "),
        offre.structured.exigences.join(" ; "),
    )
}

pub fn build_generation_input(
    offre: &Offre,
    profil: &Profil,
    retained: &[Chunk],
    plan: &CandidaturePlan,
) -> String {
    let now = chrono::Local::now();
    let months = [
        "janvier", "février", "mars", "avril", "mai", "juin",
        "juillet", "août", "septembre", "octobre", "novembre", "décembre",
    ];
    let date_str = format!(
        "{} {} {}",
        now.day(),
        months[now.month() as usize - 1],
        now.year()
    );

    let mut exp_md = String::new();
    for (i, e) in profil.content.experiences.iter().enumerate() {
        exp_md.push_str(&format!("### EXPÉRIENCE #{} (STAGE/EMPLOI)\nPOSTE: {}\nENTREPRISE: {}\nPÉRIODE: {}\nDESCRIPTION:\n{}\n---\n", i+1, e.role, e.company, e.period, e.description.iter().map(|b| format!("- {}", b)).collect::<Vec<_>>().join("\n")));
    }

    let mut proj_md = String::new();
    for (i, p) in profil.content.projects.iter().enumerate() {
        proj_md.push_str(&format!("### PROJET #{} (SIDE-PROJECT PERSONNEL)\nNOM: {}\nTYPE: SIDE-PROJECT — Réalisation personnelle, PAS un stage\nPÉRIODE: {}\nDESCRIPTION TECHNIQUE (2 bullets max dans le CV):\n{}\n---\n", i+1, p.role, p.period, p.description.iter().map(|b| format!("- {}", b)).collect::<Vec<_>>().join("\n")));
    }

    let profile_md = format!(
        "### IDENTITÉ & CONTACT\n- Nom complet : {} {}\n- Email : {}\n- Téléphone : {}\n- Localisation : {}\n\n### LANGUES\n{}\n\n### FORMATIONS\n{}\n\n═══════════════════════════════════════════\n### EXPÉRIENCES DU PROFIL — STAGES & EMPLOIS (À ADAPTER — 3 bullets chacune)\n═══════════════════════════════════════════\n{}\n\n═══════════════════════════════════════════\n### PROJETS DU PROFIL — SIDE-PROJECTS PERSONNELS (À ADAPTER — 2 bullets chacun, JAMAIS 3)\n⚠️ NE PAS MÉLANGER avec les expériences ci-dessus\n═══════════════════════════════════════════\n{}\n\n### SKILLS DU PROFIL (À ADAPTER)\n{}",
        profil.content.profile.firstname, profil.content.profile.lastname,
        profil.content.profile.email,
        profil.content.profile.phone,
        profil.content.profile.location,
        profil.content.languages.iter().map(|l| format!("- {}: {}", l.name, l.level)).collect::<Vec<_>>().join("\n"),
        profil.content.education.iter().map(|e| format!("- {} : {} ({})", e.school, e.degree, e.period)).collect::<Vec<_>>().join("\n"),
        exp_md,
        proj_md,
        profil.content.skills.iter().map(|s| format!("- {}: {}", s.category, s.items.join(", "))).collect::<Vec<_>>().join("\n"),
    );

    let chunks_listing = retained
        .iter()
        .map(|c| format!("### CHUNK PERTINENT — {} ({})\n{}", c.titre, c.kind.as_str(), c.content))
        .collect::<Vec<_>>()
        .join("\n\n---\n\n");

    format!(
        "## INFOS GÉNÉRALES\nDate du jour: {}\n\n## OFFRE CIBLE\nEntreprise: {}\nIntitulé: {}\nLocalisation: {}\n\n## RÉSUMÉ DE L'OFFRE\n{}\n\n## STACK CIBLE\n{}\n\n## MISSIONS CIBLE\n{}\n\n## EXIGENCES CIBLE\n{}\n\n## PLAN STRATÉGIQUE DE CANDIDATURE\nAngle: {}\nForces à souligner: {}\nMots-clés critiques: {}\n\n## BASE DE DONNÉES DU PROFIL CANDIDAT\n{}\n\n## SÉLECTION DE CHUNKS PERTINENTS\n{}",
        date_str,
        offre.entreprise,
        offre.intitule,
        offre.localisation.as_deref().unwrap_or("non précisé"),
        offre.structured.resume_court,
        offre.structured.stack.join(", "),
        offre.structured.missions.join(" ; "),
        offre.structured.exigences.join(" ; "),
        plan.angle,
        plan.forces_a_souligner.join(" ; "),
        plan.mots_cles_critiques.join(", "),
        profile_md,
        chunks_listing,
    )
}

pub fn build_slug(offre: &Offre, instance_id: InstanceId) -> Slug {
    let short = instance_id.to_string().chars().take(8).collect::<String>();
    let combined = format!("{}__{}", offre.slug.as_str(), short);
    Slug::parse(combined).unwrap_or_else(|_| {
        Slug::parse(format!("instance_{}", short)).expect("short id is always valid")
    })
}

pub fn truncate(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max_chars).collect();
        format!("{truncated}…")
    }
}

pub fn validate_outputs(
    offre: &Offre,
    restitution: Option<&Restitution>,
    resume: Option<&Resume>,
    cover_letter: Option<&CoverLetter>,
) -> Result<(), GenerateError> {
    if let Some(r) = restitution {
        if r.fit_score > 100 {
            tracing::warn!(
                "Validation: score de fit > 100 (score={}). On cap à 100.",
                r.fit_score
            );
        }
    }

    if let Some(r) = resume {
        if r.experiences.is_empty() && r.formations.is_empty() {
            tracing::error!("Validation CV: Aucune expérience ET aucune formation trouvée.");
            return Err(GenerateError::Invalide(
                "CV vide (ni expérience ni formation)".into(),
            ));
        }
        if r.experiences.is_empty() {
            tracing::warn!(
                "Validation CV: Aucune expérience trouvée pour l'offre '{}'",
                offre.intitule
            );
        }
        if r.formations.is_empty() {
            tracing::warn!(
                "Validation CV: Aucune formation trouvée pour l'offre '{}'",
                offre.intitule
            );
        }
    }

    if let Some(cl) = cover_letter {
        if !cl.est_complete() {
            tracing::error!(
                "Validation Lettre: Structure incomplète pour '{}'",
                offre.entreprise
            );
            return Err(GenerateError::Invalide(
                "lettre incomplète (manque salutation/accroche/clôture)".into(),
            ));
        }

        let texte_complet: String = cl
            .paragraphes
            .iter()
            .map(|p| p.contenu.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        let entreprise_lower = offre.entreprise.to_lowercase();

        if !texte_complet.to_lowercase().contains(&entreprise_lower) {
            tracing::warn!(
                "Validation Lettre: L'entreprise '{}' n'est pas mentionnée dans le texte. Validation assouplie.",
                offre.entreprise
            );
        }
    }

    Ok(())
}

pub fn sanitize_resume_experiences(offre: &Offre, profil: &Profil, resume: &mut Resume) -> usize {
    let before = resume.experiences.len();
    resume
        .experiences
        .retain(|exp| !looks_like_target_offer_experience(exp, offre));

    if resume.experiences.is_empty() && !profil.content.experiences.is_empty() {
        resume.experiences = profil
            .content
            .experiences
            .iter()
            .map(|entry| domain::Experience {
                poste: entry.role.clone(),
                entreprise: entry.company.clone(),
                localisation: None,
                periode: entry.period.clone(),
                bullets: entry.description.clone(),
            })
            .collect();
    }

    before.saturating_sub(resume.experiences.len())
}

/// Hard-enforce structural rules on the generated resume:
/// - Max 2 projects, exactly 2 bullets per project
/// - Exactly 5 skill families
/// - Title: clean gendered forms, remove "APPRENTI(E)", enforce max 50 chars
/// - Experiences: fill missing locations from profile
pub fn sanitize_resume_structure(resume: &mut Resume) {
    // Projects: keep at most 2, exactly 2 bullets each
    resume.projets.truncate(2);
    for projet in &mut resume.projets {
        projet.bullets.truncate(2);
        for bullet in &mut projet.bullets {
            *bullet = truncate(bullet, 85);
        }
    }

    // Skills: enforce exactly 5 families
    resume.competences.truncate(5);

    // Title: clean up gendered forms and redundant prefixes BEFORE truncating
    sanitize_title(&mut resume.accroche.titre);

    // Title: enforce max length (65 chars)
    if resume.accroche.titre.chars().count() > 65 {
        let titre = resume.accroche.titre.clone();
        let prefix_and_rest = titre
            .strip_prefix("ALTERNANCE — ")
            .map(|r| ("ALTERNANCE — ", r))
            .or_else(|| titre.strip_prefix("ALTERNANCE – ").map(|r| ("ALTERNANCE — ", r)));

        if let Some((prefix, rest)) = prefix_and_rest {
            let prefix_chars = prefix.chars().count();
            let max_metier = 65_usize.saturating_sub(prefix_chars);
            let short = truncate(rest, max_metier);
            resume.accroche.titre = format!("{}{}", prefix, short.trim_end());
        } else {
            resume.accroche.titre = truncate(&titre, 65);
        }
    }
}

/// Clean up the CV title from common LLM quirks:
/// - Remove gendered double forms: "INGÉNIEURE / INGÉNIEUR" → "INGÉNIEUR"
/// - Remove inclusive parenthetical: "APPRENTI(E)", "INGÉNIEUR(E)"
/// - Remove redundant "APPRENTI(E)" when "ALTERNANCE" is already present
/// - Normalize whitespace
fn sanitize_title(titre: &mut String) {
    let mut t = titre.clone();

    // 1. Remove gendered double forms: "xxxE / xxx" or "xxx / xxxE" patterns
    // Common patterns: INGÉNIEURE / INGÉNIEUR, APPRENTIE / APPRENTI
    let gendered_pairs = [
        ("INGÉNIEURE / INGÉNIEUR", "INGÉNIEUR"),
        ("INGENIEURE / INGENIEUR", "INGÉNIEUR"),
        ("INGÉNIEUR / INGÉNIEURE", "INGÉNIEUR"),
        ("INGÉNIEUR(E)", "INGÉNIEUR"),
        ("INGENIEUR(E)", "INGÉNIEUR"),
        ("INGÉNIEUR·E", "INGÉNIEUR"),
        ("APPRENTIE / APPRENTI", "APPRENTI"),
        ("APPRENTI / APPRENTIE", "APPRENTI"),
        ("APPRENTI(E)", "APPRENTI"),
        ("APPRENTI·E", "APPRENTI"),
        ("CHARGÉ(E)", "CHARGÉ"),
        ("CHARGE(E)", "CHARGÉ"),
        ("Ingénieure / Ingénieur", "Ingénieur"),
        ("Ingenieure / Ingenieur", "Ingénieur"),
        ("Ingénieur(e)", "Ingénieur"),
        ("Apprenti(e)", "Apprenti"),
        ("Chargé(e)", "Chargé"),
    ];

    for (pattern, replacement) in &gendered_pairs {
        if t.contains(pattern) {
            t = t.replace(pattern, replacement);
        }
    }

    // 2. Remove "APPRENTI " prefix if "ALTERNANCE" is already present
    // "ALTERNANCE — APPRENTI CHEF DE PROJET" → "ALTERNANCE — CHEF DE PROJET"
    if t.contains("ALTERNANCE") {
        t = t.replace("APPRENTI ", "");
        t = t.replace("Apprenti ", "");
    }

    // 3. Clean up "EN " prefixes after dash for common bureaucratic patterns
    // "ALTERNANCE — EN INFORMATIQUE" → "ALTERNANCE — INFORMATIQUE" (already handled by LLM usually)

    // 4. Normalize whitespace (collapse multiple spaces, trim)
    let cleaned: String = t.split_whitespace().collect::<Vec<_>>().join(" ");

    *titre = cleaned;
}

pub fn harmonize_resume_contact_from_profile(profil: &Profil, resume: &mut Resume) {
    // 1. Identité & Photo
    resume.identite.nom_complet = format!("{} {}", profil.content.profile.firstname, profil.content.profile.lastname);
    resume.identite.photo_url = Some("/api/profile/active/photo".to_string());
    
    // 2. Contact
    resume.contact.email = profil.content.profile.email.clone();
    resume.contact.telephone = Some(profil.content.profile.phone.clone());
    resume.contact.localisation = profil.content.profile.location.clone();
    resume.contact.linkedin = Some(profil.content.profile.linkedin.clone());
    resume.contact.github = Some(profil.content.profile.github.clone());
    resume.contact.site_web = Some(profil.content.profile.website.clone());

    // 3. Formations (Hardcoded avec regroupement HarvardX)
    let mut formations = Vec::new();
    let mut harvard_degrees = Vec::new();
    let mut harvard_period = String::new();
    
    for e in &profil.content.education {
        if e.school.contains("HarvardX") || e.school.contains("edX") {
            harvard_degrees.push(e.degree.clone());
            if harvard_period.is_empty() {
                harvard_period = e.period.clone();
            }
        } else {
            formations.push(domain::Formation {
                etablissement: e.school.clone(),
                localisation: None,
                periode: e.period.clone(),
                diplome: e.degree.clone(),
                details: None,
            });
        }
    }
    
    if !harvard_degrees.is_empty() {
        formations.push(domain::Formation {
            etablissement: "HarvardX, edX".to_string(),
            localisation: None,
            periode: harvard_period,
            diplome: harvard_degrees.join("\n"),
            details: None,
        });
    }
    resume.formations = formations;

    // 4. Langues (Hardcoded)
    resume.langues = profil.content.languages.iter().map(|l| domain::Langue {
        langue: l.name.clone(),
        niveau: l.level.clone(),
    }).collect();
}

fn looks_like_target_offer_experience(exp: &domain::Experience, offre: &Offre) -> bool {
    let exp_company = canonical_text(&exp.entreprise);
    let offer_company = canonical_text(&offre.entreprise);
    let exp_role = canonical_text(&exp.poste);
    let offer_role = canonical_text(&offre.intitule);

    if exp_company.is_empty() || exp_role.is_empty() || offer_company.is_empty() || offer_role.is_empty() {
        return false;
    }

    let same_company =
        exp_company == offer_company || exp_company.contains(&offer_company) || offer_company.contains(&exp_company);
    if !same_company {
        return false;
    }

    if exp_role == offer_role || exp_role.contains(&offer_role) || offer_role.contains(&exp_role) {
        return true;
    }

    token_overlap(&exp_role, &offer_role) >= 2
}

fn canonical_text(input: &str) -> String {
    input
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn token_overlap(a: &str, b: &str) -> usize {
    use std::collections::HashSet;
    let a_set: HashSet<&str> = a.split_whitespace().collect();
    let b_set: HashSet<&str> = b.split_whitespace().collect();
    a_set.intersection(&b_set).count()
}
