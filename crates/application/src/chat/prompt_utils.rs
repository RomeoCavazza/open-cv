use domain::{JsonValue, Message, MessageRole};

pub const MAX_CHAT_HISTORY_ENTRIES: usize = 20;

pub fn build_profile_prompt_context(profil: &domain::Profil) -> serde_json::Value {
    serde_json::json!({
        "id": profil.id,
        "label": profil.label,
        "content": serde_json::to_value(&profil.content).unwrap_or_default(),
        "is_active": profil.is_active,
        "resume_template": profil.resume_template,
        "cover_letter_template": profil.cover_letter_template,
        "has_calendar_pdf": profil.calendar_pdf.is_some(),
        "notes": profil.notes,
        "created_at": profil.created_at,
    })
}

pub fn build_offer_prompt_context(offre: &domain::Offre) -> serde_json::Value {
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

pub fn wants_mutation(message: &str) -> bool {
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

pub fn wants_undo(message: &str) -> bool {
    let lowered = message.to_lowercase();
    let undo_markers = [
        "annule",
        "annuler",
        "reviens",
        "revenir",
        "undo",
        "version précédente",
        "version precedente",
        "défais",
        "defais",
        "restaure",
        "restaurer",
        "rollback",
        "remets comme avant",
        "état précédent",
        "etat precedent",
    ];

    undo_markers
        .iter()
        .any(|marker| lowered.contains(marker))
}

pub fn wants_identity(message: &str) -> bool {
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

pub fn extract_chat_history(notes: &JsonValue) -> Vec<Message> {
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

pub fn render_chat_history_for_prompt(history: &[Message]) -> String {
    if history.is_empty() {
        return "Aucun historique".to_string();
    }

    history
        .iter()
        .rev()
        .take(12)
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

pub fn push_chat_history(notes: &mut JsonValue, role: &str, content: &str) {
    if !notes.is_object() {
        *notes = JsonValue::Object(std::collections::BTreeMap::new());
    }

    let Some(obj) = notes.as_object_mut() else {
        return;
    };

    let history_value = obj
        .entry("chat_history".to_string())
        .or_insert_with(|| JsonValue::Array(Vec::new()));

    if let Some(history) = history_value.as_array_mut() {
        let mut entry = std::collections::BTreeMap::new();
        entry.insert("role".to_string(), JsonValue::String(role.to_string()));
        entry.insert(
            "content".to_string(),
            JsonValue::String(content.to_string()),
        );
        entry.insert(
            "ts".to_string(),
            JsonValue::String(chrono::Utc::now().to_rfc3339()),
        );
        history.push(JsonValue::Object(entry));

        if history.len() > MAX_CHAT_HISTORY_ENTRIES {
            let excess = history.len() - MAX_CHAT_HISTORY_ENTRIES;
            history.drain(0..excess);
        }
    }
}
