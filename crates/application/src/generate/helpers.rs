use domain::{Chunk, CoverLetter, InstanceId, Offre, Profil, Restitution, Resume, Slug};

use super::{CandidaturePlan, GenerateError};

pub(super) fn build_generation_input(
    offre: &Offre,
    profil: &Profil,
    retained: &[Chunk],
    plan: &CandidaturePlan,
) -> String {
    let chunks_listing = retained
        .iter()
        .map(|c| format!("### {} — {}\n{}", c.kind.as_str(), c.titre, c.content))
        .collect::<Vec<_>>()
        .join("\n\n");

    format!(
        "## OFFRE\nEntreprise: {}\nIntitulé: {}\nLocalisation: {}\n\n## RÉSUMÉ DE L'OFFRE\n{}\n\n## STACK\n{}\n\n## MISSIONS\n{}\n\n## EXIGENCES\n{}\n\n## PLAN STRATÉGIQUE\nAngle: {}\nForces à souligner: {}\nMots-clés critiques: {}\n\n## PROFIL CANDIDAT\n{}\n\n## CHUNKS PERTINENTS DU PROFIL\n{}",
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
        serde_json::to_string_pretty(&profil.content).unwrap_or_default(),
        chunks_listing,
    )
}

pub(super) fn build_slug(offre: &Offre, instance_id: InstanceId) -> Slug {
    let short = instance_id.to_string().chars().take(8).collect::<String>();
    let combined = format!("{}__{}", offre.slug.as_str(), short);
    Slug::parse(combined).unwrap_or_else(|_| {
        Slug::parse(format!("instance_{}", short)).expect("short id is always valid")
    })
}

pub(super) fn truncate(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max_chars).collect();
        format!("{truncated}…")
    }
}

pub(super) fn validate_outputs(
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