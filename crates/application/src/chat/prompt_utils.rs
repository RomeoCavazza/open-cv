use domain::{JsonValue, Message, MessageRole};
use serde_json::json;

pub const MAX_CHAT_HISTORY_ENTRIES: usize = 20;
const MAX_CHAT_THREADS: usize = 50;
const MAX_THREAD_MESSAGES: usize = 100;

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

pub fn extract_chat_history(notes: &JsonValue) -> Vec<Message> {
    extract_chat_history_for_conversation(notes, None)
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
    let _ = push_chat_history_for_conversation(notes, None, role, content, None);
}

pub fn extract_chat_history_for_conversation(
    notes: &JsonValue,
    conversation_id: Option<&str>,
) -> Vec<Message> {
    let notes_json = serde_json::to_value(notes).unwrap_or_else(|_| json!({}));
    let active_id = conversation_id
        .map(str::to_string)
        .or_else(|| {
            notes_json
                .get("active_chat_id")
                .and_then(|v| v.as_str())
                .map(str::to_string)
        })
        .or_else(|| {
            notes_json
                .get("chat_threads")
                .and_then(|v| v.as_array())
                .and_then(|threads| threads.first())
                .and_then(|thread| thread.get("id"))
                .and_then(|v| v.as_str())
                .map(str::to_string)
        })
        .unwrap_or_else(|| "default".to_string());

    notes_json
        .get("chat_threads")
        .and_then(|v| v.as_array())
        .and_then(|threads| {
            threads.iter().find(|thread| {
                thread
                    .get("id")
                    .and_then(|v| v.as_str())
                    .map(|id| id == active_id)
                    .unwrap_or(false)
            })
        })
        .and_then(|thread| thread.get("messages"))
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

pub fn push_chat_history_for_conversation(
    notes: &mut JsonValue,
    conversation_id: Option<&str>,
    role: &str,
    content: &str,
    instance_id: Option<&str>,
) -> String {
    let mut notes_json = serde_json::to_value(&*notes).unwrap_or_else(|_| json!({}));
    if !notes_json.is_object() {
        notes_json = json!({});
    }

    migrate_legacy_chat_history(&mut notes_json);

    let now = chrono::Utc::now().to_rfc3339();
    let selected_id = conversation_id
        .map(str::to_string)
        .or_else(|| {
            notes_json
                .get("active_chat_id")
                .and_then(|v| v.as_str())
                .map(str::to_string)
        })
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    if notes_json
        .get("chat_threads")
        .and_then(|v| v.as_array())
        .is_none()
    {
        notes_json["chat_threads"] = json!([]);
    }
    let threads = notes_json["chat_threads"]
        .as_array_mut()
        .expect("chat_threads is always an array here");

    let mut idx = threads.iter().position(|thread| {
        thread
            .get("id")
            .and_then(|v| v.as_str())
            .map(|id| id == selected_id)
            .unwrap_or(false)
    });

    if idx.is_none() {
        let title = build_thread_title(content);
        threads.push(json!({
            "id": selected_id,
            "title": title,
            "created_at": now,
            "updated_at": now,
            "messages": []
        }));
        idx = Some(threads.len() - 1);
    }

    let thread = &mut threads[idx.expect("thread index should be defined")];
    if thread.get("messages").and_then(|v| v.as_array()).is_none() {
        thread["messages"] = json!([]);
    }

    let mut msg = json!({
        "role": role,
        "content": content,
        "ts": now
    });
    if let Some(inst_id) = instance_id {
        msg["instance_id"] = json!(inst_id);
    }

    if let Some(messages) = thread.get_mut("messages").and_then(|v| v.as_array_mut()) {
        messages.push(msg);
        if messages.len() > MAX_THREAD_MESSAGES {
            let excess = messages.len() - MAX_THREAD_MESSAGES;
            messages.drain(0..excess);
        }
    }
    thread["updated_at"] = json!(now);
    if thread
        .get("title")
        .and_then(|v| v.as_str())
        .map(str::is_empty)
        .unwrap_or(true)
    {
        thread["title"] = json!(build_thread_title(content));
    }

    if threads.len() > MAX_CHAT_THREADS {
        let excess = threads.len() - MAX_CHAT_THREADS;
        threads.drain(0..excess);
    }

    notes_json["active_chat_id"] = json!(selected_id.clone());
    *notes = serde_json::from_value(notes_json)
        .unwrap_or_else(|_| JsonValue::Object(Default::default()));
    selected_id
}

fn migrate_legacy_chat_history(notes: &mut serde_json::Value) {
    if notes
        .get("chat_threads")
        .and_then(|v| v.as_array())
        .is_some_and(|arr| !arr.is_empty())
    {
        return;
    }

    let legacy = notes
        .get("chat_history")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    if legacy.is_empty() {
        notes["chat_threads"] = json!([]);
        return;
    }

    let default_id = "legacy-default";
    notes["chat_threads"] = json!([{
        "id": default_id,
        "title": "Conversation",
        "created_at": chrono::Utc::now().to_rfc3339(),
        "updated_at": chrono::Utc::now().to_rfc3339(),
        "messages": legacy
    }]);
    notes["active_chat_id"] = json!(default_id);
}

fn build_thread_title(content: &str) -> String {
    let title = content
        .split_whitespace()
        .take(8)
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string();
    if title.is_empty() {
        "Nouveau chat".to_string()
    } else {
        title
    }
}
