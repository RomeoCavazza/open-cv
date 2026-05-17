use super::helpers::{
    build_generation_input, build_slug, harmonize_resume_contact_from_profile,
    sanitize_resume_experiences, truncate, validate_outputs,
};
use super::*;
use chrono::Utc;
use domain::{
    Chunk, ChunkKind, CoverLetter, InstanceId, Objet, Offre, OffreId, OffreStructured, Paragraphe,
    ParagrapheRole, Profil, ProfilId, Resume, Signature, Slug,
};

fn build_test_offre() -> Offre {
    Offre {
        id: OffreId::new(),
        slug: Slug::parse("safran_alternance_ia").unwrap(),
        source_url: "https://example.com/offre".into(),
        source_host: "example.com".into(),
        source_hash: vec![0; 32],
        entreprise: "Safran".into(),
        intitule: "Alternance IA".into(),
        localisation: None,
        contrat: Some("alternance".into()),
        raw_text: "Offre brute".into(),
        structured: OffreStructured {
            resume_court: "Résumé court".into(),
            stack: vec!["Rust".into(), "PostgreSQL".into()],
            missions: vec!["Développer".into(), "Maintenir".into()],
            exigences: vec!["Curiosité".into()],
            soft_skills: vec![],
            niveau_etudes: Some("Bac+5".into()),
            type_contrat: Some("alternance".into()),
            mots_cles: vec!["IA".into()],
        },
        scraped_at: Utc::now(),
        last_seen_at: Utc::now(),
        closed_at: None,
        categorie: Some("data".into()),
    }
}

fn build_test_profil() -> Profil {
    Profil {
        id: ProfilId::new(),
        label: "Profil test".into(),
        content: domain::ProfilContent {
            profile: domain::ProfileSection {
                title: "Ingénieur logiciel".into(),
                location: "Paris, 11e arr.".into(),
                ..Default::default()
            },
            experiences: vec![domain::ExperienceEntry {
                role: "Développeur Rust".into(),
                company: "ACME".into(),
                period: "2023 - 2025".into(),
                description: vec!["Conception backend".into()],
            }],
            ..Default::default()
        },
        is_active: true,
        profile_photo: None,
        calendar_pdf: None,
        resume_template: None,
        cover_letter_template: None,
        notes: JsonValue::Object(Default::default()),
        created_at: Utc::now(),
    }
}

fn build_test_plan() -> CandidaturePlan {
    CandidaturePlan {
        angle: "Mettre en avant l'impact produit".into(),
        forces_a_souligner: vec!["Rust".into(), "SQLx".into()],
        mots_cles_critiques: vec!["RAG".into(), "LLM".into()],
        faiblesses_a_adresser: vec!["Manque d'expérience sectorielle".into()],
    }
}

fn build_test_resume(
    experiences: Vec<domain::Experience>,
    formations: Vec<domain::Formation>,
) -> Resume {
    Resume {
        identite: domain::Identite {
            nom_complet: "Test User".into(),
            photo_url: None,
        },
        accroche: domain::Accroche {
            titre: "ALTERNANCE — DÉVELOPPEUR".into(),
            paragraphe: "Accroche".into(),
            duree: "24 mois".into(),
            rythme: "3j/2j".into(),
        },
        contact: domain::Contact {
            localisation: "Paris".into(),
            telephone: None,
            email: "test@example.com".into(),
            site_web: None,
            linkedin: None,
            github: None,
        },
        competences: vec![domain::GroupeCompetences {
            categorie: "Programmation".into(),
            items: vec!["Rust".into()],
        }],
        experiences,
        formations,
        projets: vec![],
        langues: vec![],
    }
}

fn build_test_cover_letter(roles: &[ParagrapheRole]) -> CoverLetter {
    CoverLetter {
        expediteur: domain::Expediteur {
            identite: domain::Identite {
                nom_complet: "Test User".into(),
                photo_url: None,
            },
            contact: domain::Contact {
                localisation: "Paris".into(),
                telephone: None,
                email: "test@example.com".into(),
                site_web: None,
                linkedin: None,
                github: None,
            },
        },
        destinataire: domain::Destinataire {
            entreprise: "Safran".into(),
            date: "2026-05-04".into(),
        },
        objet: Objet {
            categorie: "ALTERNANCE".into(),
            libelle: "ALTERNANCE - DEV".into(),
        },
        paragraphes: roles
            .iter()
            .map(|role| Paragraphe {
                role: *role,
                contenu: match role {
                    ParagrapheRole::Cloture => "Je reste à votre disposition".into(),
                    _ => "Safran me motive".into(),
                },
            })
            .collect(),
        signature: Signature {
            formule_politesse: "Cordialement,".into(),
            nom: "Test User".into(),
        },
    }
}

fn merge_generated_outputs(
    instance: &mut domain::Instance,
    restitution: Option<domain::Restitution>,
    resume: Option<domain::Resume>,
    cover_letter: Option<domain::CoverLetter>,
) {
    if let Some(r) = restitution {
        instance.restitution = Some(r);
    }
    if let Some(r) = resume {
        instance.resume_json = Some(r);
    }
    if let Some(c) = cover_letter {
        instance.cover_letter_json = Some(c);
    }
}

#[test]
fn truncate_court_inchange() {
    assert_eq!(truncate("hello", 10), "hello");
}

#[test]
fn truncate_long_coupe() {
    let s = "a".repeat(100);
    let out = truncate(&s, 10);
    assert_eq!(out.chars().count(), 11); // 10 + ellipsis
    assert!(out.ends_with('…'));
}

#[test]
fn livrables_par_defaut_tous_actifs() {
    let l = Livrables::default();
    assert!(l.restitution && l.resume && l.cover_letter);
    assert!(!l.aucun());
}

#[test]
fn livrables_aucun_si_tout_off() {
    let l = Livrables {
        restitution: false,
        resume: false,
        cover_letter: false,
    };
    assert!(l.aucun());
}

#[test]
fn build_generation_input_includes_core_sections() {
    let offre = build_test_offre();
    let profil = build_test_profil();
    let plan = build_test_plan();
    let retained = vec![Chunk {
        id: domain::ChunkId::new(),
        profil_id: profil.id,
        kind: ChunkKind::Experience,
        titre: "Stage Rust".into(),
        content: "Développement backend".into(),
        metadata: JsonValue::Object(Default::default()),
        embedding: vec![],
        created_at: Utc::now(),
    }];

    let output = build_generation_input(&offre, &profil, &retained, &plan);

    assert!(output.contains("## OFFRE"));
    assert!(output.contains("Entreprise: Safran"));
    assert!(output.contains("Localisation: non précisé"));
    assert!(output.contains("## PLAN STRATÉGIQUE"));
    assert!(output.contains("Angle: Mettre en avant l'impact produit"));
    assert!(output.contains("### EXPÉRIENCE #1 (STAGE/EMPLOI)"));
    assert!(output.contains("Développement backend"));
}

#[test]
fn build_slug_uses_offer_slug_and_short_instance_id() {
    let offre = build_test_offre();
    let instance_id = InstanceId::from_uuid(uuid::Uuid::from_u128(
        0x1234_5678_90ab_cdef_0000_0000_0000_0000,
    ));

    let slug = build_slug(&offre, instance_id);

    assert_eq!(slug.as_str(), "safran_alternance_ia__12345678");
}

#[test]
fn validate_outputs_rejects_empty_resume() {
    let offre = build_test_offre();
    let resume = build_test_resume(vec![], vec![]);
    let result = validate_outputs(&offre, None, Some(&resume), None);

    assert!(matches!(result, Err(GenerateError::Invalide(message)) if message.contains("CV vide")));
}

#[test]
fn validate_outputs_rejects_incomplete_cover_letter() {
    let offre = build_test_offre();
    // Missing Accroche -> should fail
    let cover_letter =
        build_test_cover_letter(&[ParagrapheRole::Salutation, ParagrapheRole::Cloture]);
    let result = validate_outputs(&offre, None, None, Some(&cover_letter));

    assert!(
        matches!(result, Err(GenerateError::Invalide(message)) if message.contains("lettre incomplète"))
    );
}

#[test]
fn validate_outputs_accepts_minimal_cover_letter() {
    let offre = build_test_offre();
    // Only Accroche -> should pass
    let cover_letter = build_test_cover_letter(&[ParagrapheRole::Accroche]);
    let result = validate_outputs(&offre, None, None, Some(&cover_letter));

    assert!(result.is_ok());
}

#[test]
fn merge_generated_outputs_preserves_existing_resume_and_cover_letter() {
    let offre = build_test_offre();
    let profil = build_test_profil();
    let mut instance = domain::Instance {
        id: InstanceId::new(),
        slug: Slug::parse("instance_test").unwrap(),
        offre_id: offre.id,
        profil_id: profil.id,
        status: domain::InstanceStatus::Ready,
        restitution: None,
        resume_json: Some(build_test_resume(vec![], vec![])),
        cover_letter_json: Some(build_test_cover_letter(&[
            ParagrapheRole::Salutation,
            ParagrapheRole::Accroche,
            ParagrapheRole::Cloture,
        ])),
        notes: JsonValue::Object(Default::default()),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        sent_at: None,
    };
    let previous_resume_title = instance
        .resume_json
        .as_ref()
        .map(|resume| resume.accroche.titre.clone());
    let previous_cover_letter_paragraph_count = instance
        .cover_letter_json
        .as_ref()
        .map(|cover_letter| cover_letter.paragraphes.len());
    let new_restitution = domain::Restitution {
        fit_score: 82,
        ..Default::default()
    };

    merge_generated_outputs(&mut instance, Some(new_restitution.clone()), None, None);

    assert_eq!(
        instance.restitution.as_ref().map(|r| r.fit_score),
        Some(new_restitution.fit_score)
    );
    assert_eq!(
        instance
            .resume_json
            .as_ref()
            .map(|resume| resume.accroche.titre.clone()),
        previous_resume_title
    );
    assert_eq!(
        instance
            .cover_letter_json
            .as_ref()
            .map(|cover_letter| cover_letter.paragraphes.len()),
        previous_cover_letter_paragraph_count
    );
}

#[test]
fn sanitize_resume_experiences_removes_target_offer_projection() {
    let offre = Offre {
        entreprise: "Ministère du Travail, de la Santé et des Solidarités".into(),
        intitule: "Appui au pilotage stratégique du système d'information de la Sécurité Sociale"
            .into(),
        ..build_test_offre()
    };
    let profil = build_test_profil();
    let mut resume = build_test_resume(
        vec![
            domain::Experience {
                poste: "Appui au pilotage stratégique du système d'information de la Sécurité Sociale".into(),
                entreprise: "Ministère du Travail, de la Santé et des Solidarités".into(),
                localisation: None,
                periode: "15 mai 2026 - 14 mai 2028".into(),
                bullets: vec!["Appui stratégique".into()],
            },
            domain::Experience {
                poste: "Développeur Rust".into(),
                entreprise: "ACME".into(),
                localisation: None,
                periode: "2023 - 2025".into(),
                bullets: vec!["Conception backend".into()],
            },
        ],
        vec![domain::Formation {
            etablissement: "EPITECH".into(),
            localisation: None,
            periode: "2026 - 2028".into(),
            diplome: "MSc".into(),
            details: None,
        }],
    );

    let removed = sanitize_resume_experiences(&offre, &profil, &mut resume);

    assert_eq!(removed, 1);
    assert_eq!(resume.experiences.len(), 1);
    assert_eq!(resume.experiences[0].entreprise, "ACME");
}

#[test]
fn harmonize_resume_contact_from_profile_overrides_location() {
    let profil = build_test_profil();
    let mut resume = build_test_resume(
        vec![domain::Experience {
            poste: "Dev".into(),
            entreprise: "ACME".into(),
            localisation: None,
            periode: "2024".into(),
            bullets: vec![],
        }],
        vec![domain::Formation {
            etablissement: "EPITECH".into(),
            localisation: None,
            periode: "2026 - 2028".into(),
            diplome: "MSc".into(),
            details: None,
        }],
    );

    resume.contact.localisation = "Paris, 7e arrondissement".into();
    harmonize_resume_contact_from_profile(&profil, &mut resume);

    assert_eq!(resume.contact.localisation, "Paris, 11e arr.");
}
