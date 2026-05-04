use domain::{Message, MessageRole};

use super::MAX_CHAT_HISTORY_ENTRIES;

pub(super) fn build_profile_prompt_context(profil: &domain::Profil) -> serde_json::Value {
    serde_json::json!({
        "id": profil.id,
        "label": profil.label,
        "content": profil.content,
        "is_active": profil.is_active,
        "resume_template": profil.resume_template,
        "cover_letter_template": profil.cover_letter_template,
        "has_calendar_pdf": profil.calendar_pdf.is_some(),
        "notes": profil.notes,
        "created_at": profil.created_at,
    })
}

pub(super) fn build_offer_prompt_context(offre: &domain::Offre) -> serde_json::Value {
    let raw_text_preview = if offre.raw_text.chars().count() > 4000 {
        offre.raw_text.chars().take(4000).collect::<String>() + "…"
    } else {
        offre.raw_text.clone()
    };

    serde_json::json!({
        "id": offre.id,
        "slug": offre.slug,
        "source_url": offre.source_url,
        "source_host": offre.source_host,
        "entreprise": offre.entreprise,
        "intitule": offre.intitule,
        "localisation": offre.localisation,
        "contrat": offre.contrat,
        "structured": offre.structured,
        "raw_text_preview": raw_text_preview,
        "scraped_at": offre.scraped_at,
        "last_seen_at": offre.last_seen_at,
        "closed_at": offre.closed_at,
        "categorie": offre.categorie,
    })
}

pub(super) fn wants_mutation(message: &str) -> bool {
    let lowered = message.to_lowercase();
    let mutation_markers = [
        "modifie",
        "modifier",
        "change",
        "changer",
        "corrige",
        "corriger",
        "ajoute",
        "ajouter",
        "supprime",
        "enlève",
        "retire",
        "remplace",
        "mets",
        "mettre",
        "réécris",
        "reecris",
        "réécrire",
        "reecrire",
        "actualise",
        "actualiser",
        "adapte",
        "adapter",
        "réorganise",
        "reorganise",
        "modification",
        "édition",
        "edite",
        "éditer",
        "editer",
    ];

    mutation_markers
        .iter()
        .any(|marker| lowered.contains(marker))
}

pub(super) fn wants_identity(message: &str) -> bool {
    let lowered = message.to_lowercase();
    let identity_markers = [
        "comment je m'appelle",
        "c'est quoi mon nom",
        "quel est mon nom",
        "tu sais comment je m'appelle",
        "tu connais mon nom",
        "je m'appelle comment",
    ];

    identity_markers
        .iter()
        .any(|marker| lowered.contains(marker))
}

pub(super) fn extract_chat_history(notes: &serde_json::Value) -> Vec<Message> {
    notes
        .get("chat_history")
        .and_then(|v| v.as_array())
        .map(|entries| {
            entries
                .iter()
                .filter_map(|entry| {
                    let role_str = entry.get("role")?.as_str()?;
                    let role = match role_str {
                        "user" => MessageRole::User,
                        "assistant" => MessageRole::Assistant,
                        _ => MessageRole::System,
                    };
                    let content = entry.get("content")?.as_str()?.to_string();
                    Some(Message {
                        id: uuid::Uuid::new_v4(),
                        instance_id: domain::InstanceId::new(),
                        role,
                        content,
                        created_at: chrono::Utc::now(),
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

pub(super) fn render_chat_history_for_prompt(history: &[Message]) -> String {
    if history.is_empty() {
        return "Aucun historique".to_string();
    }

    history
        .iter()
        .rev()
        .take(12)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .map(|m| {
            let label = match m.role {
                MessageRole::Assistant => "IA",
                MessageRole::User => "UTILISATEUR",
                MessageRole::System => "SYSTEME",
            };
            format!("{label}: {}", m.content)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub(super) fn push_chat_history(notes: &mut serde_json::Value, role: &str, content: &str) {
    tracing::info!("Chat: Pushing history for role: {}", role);

    if !notes.is_object() {
        *notes = serde_json::json!({});
    }

    let Some(obj) = notes.as_object_mut() else {
        return;
    };

    let history_value = obj
        .entry("chat_history")
        .or_insert_with(|| serde_json::json!([]));
    if !history_value.is_array() {
        *history_value = serde_json::json!([]);
    }
    let Some(history) = history_value.as_array_mut() else {
        return;
    };

    history.push(serde_json::json!({
        "role": role,
        "content": content,
        "ts": chrono::Utc::now().to_rfc3339(),
    }));

    if history.len() > MAX_CHAT_HISTORY_ENTRIES {
        let excess = history.len() - MAX_CHAT_HISTORY_ENTRIES;
        history.drain(0..excess);
    }

    tracing::info!("Chat: History size now: {} entries", history.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use domain::{Offre, OffreId, OffreStructured, Profil, ProfilId, Slug};
    use serde_json::json;

    fn build_test_profile() -> Profil {
        Profil {
            id: ProfilId::new(),
            label: "Test Profil".into(),
            content: json!({"foo": "bar"}),
            is_active: true,
            profile_photo: None,
            calendar_pdf: None,
            resume_template: None,
            cover_letter_template: None,
            notes: json!({}),
            created_at: Utc::now(),
        }
    }

    fn build_test_offer() -> Offre {
        Offre {
            id: OffreId::new(),
            slug: Slug::parse("test_offer").unwrap(),
            source_url: "https://example.com/job".into(),
            source_host: "example.com".into(),
            source_hash: vec![0; 32],
            entreprise: "Example SA".into(),
            intitule: "Alternance Dev".into(),
            localisation: Some("Paris".into()),
            contrat: Some("alternance".into()),
            raw_text: "x".repeat(4100),
            structured: OffreStructured::default(),
            scraped_at: Utc::now(),
            last_seen_at: Utc::now(),
            closed_at: None,
            categorie: None,
        }
    }

    #[test]
    fn mutation_and_identity_detection_work() {
        assert!(wants_mutation("modifie mon CV"));
        assert!(!wants_mutation("résume l'offre"));
        assert!(wants_identity("tu sais comment je m'appelle ?"));
        assert!(!wants_identity("tu peux résumer l'offre ?"));
    }

    #[test]
    fn prompt_context_builders_include_expected_fields() {
        let profil = build_test_profile();
        let offre = build_test_offer();

        let profile_json = build_profile_prompt_context(&profil);
        let offer_json = build_offer_prompt_context(&offre);

        assert_eq!(profile_json["label"], json!("Test Profil"));
        assert_eq!(offer_json["entreprise"], json!("Example SA"));
        assert_eq!(
            offer_json["raw_text_preview"]
                .as_str()
                .unwrap()
                .chars()
                .count(),
            4001
        );
    }

    #[test]
    fn chat_history_helpers_round_trip() {
        let mut notes = json!({});
        for idx in 0..=MAX_CHAT_HISTORY_ENTRIES {
            push_chat_history(&mut notes, "user", &format!("message-{idx}"));
        }

        let history = extract_chat_history(&notes);
        assert_eq!(history.len(), MAX_CHAT_HISTORY_ENTRIES);

        let rendered = render_chat_history_for_prompt(&history);
        assert!(rendered.contains("UTILISATEUR: message-1"));
        assert!(!rendered.contains("message-0"));
    }
}
