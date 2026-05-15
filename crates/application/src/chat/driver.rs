use crate::chat::chat_event::ChatEvent;
use crate::chat::persistence::ChatContextLoader;
use crate::chat::types::{
    ChatRequest, CoverListTarget, EditCoverListOutput, EditResumeListOutput, ListOperation,
    ResumeListTarget,
};
use domain::{Message, MessageRole};
use futures::StreamExt;
use ports::{LlmClient, LlmError, LlmTool, MessageRepo, SnapshotRepo, StopReason, ToolChoice};
use std::sync::Arc;
use tokio::sync::mpsc;

const MAX_TOOL_ITERATIONS: usize = 5;
const RESUME_JSON_PROMPT_LIMIT: usize = 3000;
const COVER_JSON_PROMPT_LIMIT: usize = 2500;

pub struct ConversationDriver {
    pub loader: ChatContextLoader,
    pub message_repo: Arc<dyn MessageRepo>,
    pub snapshot_repo: Option<Arc<dyn SnapshotRepo>>,
    pub llm: Arc<dyn LlmClient>,
}

impl ConversationDriver {
    pub async fn run(
        &self,
        instance_id: String,
        req: ChatRequest,
        tx: mpsc::Sender<Result<ChatEvent, LlmError>>,
    ) -> anyhow::Result<()> {
        let (mut instance, mut profil, offre) =
            self.loader.load_instance_context(&instance_id).await?;
        let history = crate::chat::prompt_utils::extract_chat_history_for_conversation(
            &profil.notes,
            req.conversation_id.as_deref(),
        );
        let rag_context = self
            .loader
            .get_rag_context(instance.profil_id, &req.message)
            .await?;

        let user_msg = Message::new(instance.id, MessageRole::User, req.message.clone());
        self.message_repo.push(&user_msg).await?;
        let _ = crate::chat::prompt_utils::push_chat_history_for_conversation(
            &mut profil.notes,
            req.conversation_id.as_deref(),
            "user",
            &req.message,
            Some(&instance_id),
        );
        let _ = self.loader.profils.upsert(&profil).await;

        let tools = self.build_tools()?;
        let mut messages =
            self.build_initial_messages(&instance, &profil, &offre, &history, &rag_context, &req);
        let mut full_assistant_response = String::new();
        let tool_choice = if is_mutation_request(&req.message) {
            ToolChoice::Required
        } else {
            ToolChoice::Auto
        };

        for iteration in 0..MAX_TOOL_ITERATIONS {
            let completion_req = ports::CompletionRequest {
                system: Some(self.select_system_prompt()),
                messages: messages.clone(),
                model: None,
                max_tokens: Some(4000),
                temperature: None,
                tools: tools.clone(),
                tool_choice: tool_choice.clone(),
            };

            let mut stream = self.llm.stream(completion_req).await?;
            let mut stop_reason = StopReason::EndTurn;
            let mut current_tool_calls = Vec::new();
            let mut assistant_text_in_this_turn = String::new();

            while let Some(chunk_res) = stream.next().await {
                let chunk = chunk_res?;
                match chunk {
                    ports::StreamChunk::TextDelta { text } => {
                        assistant_text_in_this_turn.push_str(&text);
                        full_assistant_response.push_str(&text);
                        tx.send(Ok(ChatEvent::token(text))).await.ok();
                    }
                    ports::StreamChunk::ToolCallStart { .. } => {
                        tx.send(Ok(ChatEvent::status("L'IA prépare des modifications...")))
                            .await
                            .ok();
                    }
                    ports::StreamChunk::ToolCallArgsDelta { .. } => {}
                    ports::StreamChunk::ToolCallEnd {
                        id,
                        name,
                        arguments,
                    } => {
                        tracing::debug!(
                            tool_id = %id,
                            tool_name = %name,
                            tool_args = %arguments,
                            "tool call reçu"
                        );
                        current_tool_calls.push(ports::ToolCall {
                            id,
                            name,
                            arguments,
                        });
                    }
                    ports::StreamChunk::Done {
                        stop_reason: reason,
                    } => {
                        stop_reason = reason;
                    }
                }
            }

            let mut assistant_content = Vec::new();
            if !assistant_text_in_this_turn.is_empty() {
                assistant_content.push(ports::MessageContent::Text(assistant_text_in_this_turn));
            }
            for tc in &current_tool_calls {
                assistant_content.push(ports::MessageContent::ToolUse {
                    id: tc.id.clone(),
                    name: tc.name.clone(),
                    input: tc.arguments.clone(),
                });
            }

            if !assistant_content.is_empty() {
                messages.push(ports::Message {
                    role: ports::Role::Assistant,
                    content: assistant_content,
                });
            }

            if current_tool_calls.is_empty() {
                break;
            }

            for tc in current_tool_calls {
                match tc.name.as_str() {
                    "update_documents" => {
                        tx.send(Ok(ChatEvent::status("Application des modifications...")))
                            .await
                            .ok();

                        let commit_message = tc
                            .arguments
                            .get("commit_message")
                            .and_then(|v| v.as_str())
                            .map(str::to_string)
                            .or_else(|| Some(req.message.clone()));

                        let mut mutation_error = None;
                        let instance_before_mutation = instance.clone();
                        let mut next_resume: Option<domain::Resume> = None;
                        let mut next_cover: Option<domain::CoverLetter> = None;

                        if let Some(resume_patch) = tc.arguments.get("resume") {
                            if !resume_patch.is_null() {
                                let mut current = serde_json::to_value(
                                    instance.resume_json.clone().unwrap_or_default(),
                                )?;
                                if let Err(e) = merge_json_without_arrays(
                                    &mut current,
                                    resume_patch,
                                    "$.resume",
                                ) {
                                    mutation_error = Some(e);
                                } else {
                                    match serde_json::from_value::<domain::Resume>(current) {
                                        Ok(r) => next_resume = Some(r),
                                        Err(e) => {
                                            mutation_error = Some(format!(
                                                "Désérialisation Resume échouée: {}",
                                                e
                                            ))
                                        }
                                    }
                                }
                            }
                        }

                        if mutation_error.is_none() {
                            if let Some(cover_patch) = tc.arguments.get("cover") {
                                if !cover_patch.is_null() {
                                    let mut current = serde_json::to_value(
                                        instance.cover_letter_json.clone().unwrap_or_default(),
                                    )?;
                                    if let Err(e) = merge_json_without_arrays(
                                        &mut current,
                                        cover_patch,
                                        "$.cover",
                                    ) {
                                        mutation_error = Some(e);
                                    } else {
                                        match serde_json::from_value::<domain::CoverLetter>(current)
                                        {
                                            Ok(c) => next_cover = Some(c),
                                            Err(e) => {
                                                mutation_error = Some(format!(
                                                    "Désérialisation CoverLetter échouée: {}",
                                                    e
                                                ))
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        let changed = mutation_error.is_none()
                            && (next_resume.is_some() || next_cover.is_some());

                        if changed {
                            if let Some(resume) = next_resume {
                                instance.resume_json = Some(resume);
                            }
                            if let Some(cover) = next_cover {
                                instance.cover_letter_json = Some(cover);
                            }
                            self.capture_snapshot(&instance_before_mutation, commit_message)
                                .await;
                            instance.updated_at = chrono::Utc::now();
                            self.loader.instances.upsert(&instance).await?;
                            tx.send(Ok(ChatEvent::mutation(instance.clone())))
                                .await
                                .ok();
                            messages.push(ports::Message::tool_result(
                                tc.id,
                                "Modifications appliquées avec succès.",
                            ));
                        } else if let Some(err_msg) = mutation_error {
                            messages.push(ports::Message::tool_result(
                                tc.id,
                                format!(
                                    "Erreur: {} Utilise edit_resume_list/edit_cover_list pour les listes.",
                                    err_msg
                                ),
                            ));
                        } else {
                            messages.push(ports::Message::tool_result(
                                tc.id,
                                "Aucune modification détectée.",
                            ));
                        }
                    }
                    "edit_resume_list" => {
                        tx.send(Ok(ChatEvent::status("Application des modifications...")))
                            .await
                            .ok();
                        match serde_json::from_value::<EditResumeListOutput>(tc.arguments.clone()) {
                            Ok(edit) => {
                                let instance_before_mutation = instance.clone();
                                match apply_resume_list_edit(&mut instance, &edit) {
                                    Ok(()) => {
                                        self.capture_snapshot(
                                            &instance_before_mutation,
                                            Some(edit.commit_message.clone()),
                                        )
                                        .await;
                                        instance.updated_at = chrono::Utc::now();
                                        self.loader.instances.upsert(&instance).await?;
                                        tx.send(Ok(ChatEvent::mutation(instance.clone())))
                                            .await
                                            .ok();
                                        messages.push(ports::Message::tool_result(
                                            tc.id,
                                            "Liste CV mise à jour avec succès.",
                                        ));
                                    }
                                    Err(e) => {
                                        messages.push(ports::Message::tool_result(
                                            tc.id,
                                            format!("Erreur de validation: {}", e),
                                        ));
                                    }
                                }
                            }
                            Err(e) => {
                                messages.push(ports::Message::tool_result(
                                    tc.id,
                                    format!("Erreur de validation: {}", e),
                                ));
                            }
                        }
                    }
                    "edit_cover_list" => {
                        tx.send(Ok(ChatEvent::status("Application des modifications...")))
                            .await
                            .ok();
                        match serde_json::from_value::<EditCoverListOutput>(tc.arguments.clone()) {
                            Ok(edit) => {
                                let instance_before_mutation = instance.clone();
                                match apply_cover_list_edit(&mut instance, &edit) {
                                    Ok(()) => {
                                        self.capture_snapshot(
                                            &instance_before_mutation,
                                            Some(edit.commit_message.clone()),
                                        )
                                        .await;
                                        instance.updated_at = chrono::Utc::now();
                                        self.loader.instances.upsert(&instance).await?;
                                        tx.send(Ok(ChatEvent::mutation(instance.clone())))
                                            .await
                                            .ok();
                                        messages.push(ports::Message::tool_result(
                                            tc.id,
                                            "Liste lettre mise à jour avec succès.",
                                        ));
                                    }
                                    Err(e) => {
                                        messages.push(ports::Message::tool_result(
                                            tc.id,
                                            format!("Erreur de validation: {}", e),
                                        ));
                                    }
                                }
                            }
                            Err(e) => {
                                messages.push(ports::Message::tool_result(
                                    tc.id,
                                    format!("Erreur de validation: {}", e),
                                ));
                            }
                        }
                    }
                    _ => {
                        messages.push(ports::Message::tool_result(
                            tc.id,
                            format!("Tool inconnu: {}", tc.name),
                        ));
                    }
                }
            }

            if stop_reason != StopReason::ToolUse {
                break;
            }

            if iteration + 1 == MAX_TOOL_ITERATIONS {
                tx.send(Ok(ChatEvent::error(
                    "Nombre maximal d'itérations atteint. Arrêt de sécurité.".to_string(),
                )))
                .await
                .ok();
                break;
            }
        }

        if !full_assistant_response.is_empty() {
            let assistant_msg =
                Message::new(instance.id, MessageRole::Assistant, full_assistant_response);
            self.message_repo.push(&assistant_msg).await?;
            let _ = crate::chat::prompt_utils::push_chat_history_for_conversation(
                &mut profil.notes,
                req.conversation_id.as_deref(),
                "assistant",
                &assistant_msg.content,
                Some(&instance_id),
            );
            let _ = self.loader.profils.upsert(&profil).await;
        }

        tx.send(Ok(ChatEvent::Done)).await.ok();
        Ok(())
    }

    fn build_tools(&self) -> anyhow::Result<Vec<LlmTool>> {
        Ok(vec![
            LlmTool {
                name: "update_documents".into(),
                description: "Modifie les champs non-listes du CV et/ou de la lettre. N'utilise pas cet outil pour les listes.".into(),
                input_schema: serde_json::to_value(schemars::schema_for!(crate::chat::types::ChatMutationOutput))?,
            },
            LlmTool {
                name: "edit_resume_list".into(),
                description: "Modifie une liste du CV avec une sémantique explicite (add/update/remove/replace).".into(),
                input_schema: serde_json::to_value(schemars::schema_for!(EditResumeListOutput))?,
            },
            LlmTool {
                name: "edit_cover_list".into(),
                description: "Modifie la liste des paragraphes de la lettre avec une sémantique explicite (add/update/remove/replace).".into(),
                input_schema: serde_json::to_value(schemars::schema_for!(EditCoverListOutput))?,
            },
        ])
    }

    async fn capture_snapshot(&self, instance: &domain::Instance, trigger_message: Option<String>) {
        if let Some(ref snap_repo) = self.snapshot_repo {
            let version = snap_repo.count_by_instance(instance.id).await.unwrap_or(0) + 1;
            let snapshot = domain::InstanceSnapshot::capture(instance, version, trigger_message);
            let _ = snap_repo.save(&snapshot).await;
        }
    }

    fn select_system_prompt(&self) -> String {
        crate::prompts::chat::INSTANCE_DEFAULT_SYSTEM.to_string()
    }

    fn build_initial_messages(
        &self,
        instance: &domain::Instance,
        profil: &domain::Profil,
        offre: &domain::Offre,
        history: &[domain::Message],
        rag_context: &str,
        req: &ChatRequest,
    ) -> Vec<ports::Message> {
        use crate::chat::prompt_utils::{
            build_offer_prompt_context, build_profile_prompt_context,
            render_chat_history_for_prompt,
        };

        let history_prompt = render_chat_history_for_prompt(history);
        let resume_json = truncate_for_prompt(
            serde_json::to_string(&instance.resume_json).unwrap_or_default(),
            RESUME_JSON_PROMPT_LIMIT,
        );
        let cover_json = truncate_for_prompt(
            serde_json::to_string(&instance.cover_letter_json).unwrap_or_default(),
            COVER_JSON_PROMPT_LIMIT,
        );

        let text = format!(
            "IDENTITÉ DE L'UTILISATEUR:\n{}\n\n\
             OFFRE CIBLÉE:\n{}\n\n\
             ANALYSE DE L'OFFRE:\n{}\n\n\
             PARCOURS (RAG):\n{}\n\n\
             HISTORIQUE:\n{}\n\n\
             DEMANDE: {}\n\n\
             JSON CV (résumé): {}\n\n\
             JSON LETTRE (résumé): {}",
            serde_json::to_string(&build_profile_prompt_context(profil)).unwrap_or_default(),
            serde_json::to_string(&build_offer_prompt_context(offre)).unwrap_or_default(),
            serde_json::to_string(&instance.restitution).unwrap_or_default(),
            rag_context,
            history_prompt,
            req.message,
            resume_json,
            cover_json
        );

        vec![ports::Message {
            role: ports::Role::User,
            content: vec![ports::MessageContent::Text(text)],
        }]
    }
}

fn is_mutation_request(message: &str) -> bool {
    let lower = message.to_lowercase();
    let tokens: Vec<String> = lower
        .split_whitespace()
        .take(6)
        .map(|t| {
            t.trim_matches(|c: char| !c.is_alphanumeric() && c != '\'' && c != '’')
                .to_string()
        })
        .filter(|t| !t.is_empty())
        .collect();

    if tokens.is_empty() {
        return false;
    }

    let direct_mutation_keywords = [
        "modifie",
        "modifier",
        "change",
        "changer",
        "ajoute",
        "ajouter",
        "supprime",
        "supprimer",
        "réécris",
        "reecris",
        "réécrire",
    ];

    if tokens
        .iter()
        .any(|t| direct_mutation_keywords.iter().any(|kw| t == kw))
    {
        return true;
    }

    // Cas "met/mets à jour" en début de demande.
    tokens.windows(3).any(|w| {
        (w[0] == "met" || w[0] == "mets")
            && (w[1] == "a" || w[1] == "à")
            && w[2].starts_with("jour")
    })
}

fn truncate_for_prompt(mut text: String, limit: usize) -> String {
    if text.len() <= limit {
        return text;
    }
    text.truncate(limit);
    text.push_str(" ...[truncated]");
    text
}

fn merge_json_without_arrays(
    a: &mut serde_json::Value,
    b: &serde_json::Value,
    path: &str,
) -> Result<(), String> {
    match (a, b) {
        (serde_json::Value::Object(a_obj), serde_json::Value::Object(b_obj)) => {
            for (k, v) in b_obj {
                let child_path = format!("{}.{}", path, k);
                if is_list_field(k) {
                    return Err(format!(
                        "Mutation de liste non autorisée via update_documents sur {}",
                        child_path
                    ));
                }
                if v.is_null() {
                    a_obj.remove(k);
                    continue;
                }

                let entry = a_obj.entry(k.clone()).or_insert(serde_json::Value::Null);
                let targets_array = matches!(entry, serde_json::Value::Array(_));
                let patch_is_array = matches!(v, serde_json::Value::Array(_));
                if patch_is_array && (targets_array || matches!(entry, serde_json::Value::Null)) {
                    return Err(format!(
                        "Mutation de liste non autorisée via update_documents sur {}",
                        child_path
                    ));
                }
                merge_json_without_arrays(entry, v, &child_path)?;
            }
            Ok(())
        }
        (serde_json::Value::Array(_), serde_json::Value::Array(_)) => Err(format!(
            "Mutation de liste non autorisée via update_documents sur {}",
            path
        )),
        (target, patch) => {
            *target = patch.clone();
            Ok(())
        }
    }
}

fn is_list_field(field_name: &str) -> bool {
    matches!(
        field_name,
        "competences" | "experiences" | "formations" | "projets" | "langues" | "paragraphes"
    )
}

fn apply_resume_list_edit(
    instance: &mut domain::Instance,
    edit: &EditResumeListOutput,
) -> anyhow::Result<()> {
    let mut resume = instance.resume_json.clone().unwrap_or_default();

    match edit.target {
        ResumeListTarget::Competences => {
            apply_list_operation(
                &mut resume.competences,
                &edit.operation,
                edit.index,
                edit.item.clone(),
                edit.items.clone(),
            )?;
        }
        ResumeListTarget::Experiences => {
            apply_list_operation(
                &mut resume.experiences,
                &edit.operation,
                edit.index,
                edit.item.clone(),
                edit.items.clone(),
            )?;
        }
        ResumeListTarget::Formations => {
            apply_list_operation(
                &mut resume.formations,
                &edit.operation,
                edit.index,
                edit.item.clone(),
                edit.items.clone(),
            )?;
        }
        ResumeListTarget::Projets => {
            apply_list_operation(
                &mut resume.projets,
                &edit.operation,
                edit.index,
                edit.item.clone(),
                edit.items.clone(),
            )?;
        }
        ResumeListTarget::Langues => {
            apply_list_operation(
                &mut resume.langues,
                &edit.operation,
                edit.index,
                edit.item.clone(),
                edit.items.clone(),
            )?;
        }
    }

    instance.resume_json = Some(resume);
    Ok(())
}

fn apply_cover_list_edit(
    instance: &mut domain::Instance,
    edit: &EditCoverListOutput,
) -> anyhow::Result<()> {
    let mut cover = instance.cover_letter_json.clone().unwrap_or_default();

    match edit.target {
        CoverListTarget::Paragraphes => {
            apply_list_operation(
                &mut cover.paragraphes,
                &edit.operation,
                edit.index,
                edit.item.clone(),
                edit.items.clone(),
            )?;
        }
    }

    instance.cover_letter_json = Some(cover);
    Ok(())
}

fn apply_list_operation<T>(
    target: &mut Vec<T>,
    operation: &ListOperation,
    index: Option<usize>,
    item: Option<serde_json::Value>,
    items: Option<Vec<serde_json::Value>>,
) -> anyhow::Result<()>
where
    T: serde::de::DeserializeOwned,
{
    match operation {
        ListOperation::Add => {
            let raw_item = item.ok_or_else(|| anyhow::anyhow!("item requis pour add"))?;
            let parsed: T = serde_json::from_value(raw_item)?;
            target.push(parsed);
        }
        ListOperation::Update => {
            let idx = index.ok_or_else(|| anyhow::anyhow!("index requis pour update"))?;
            if idx >= target.len() {
                return Err(anyhow::anyhow!(
                    "index {} hors limites (taille actuelle: {})",
                    idx,
                    target.len()
                ));
            }
            let raw_item = item.ok_or_else(|| anyhow::anyhow!("item requis pour update"))?;
            let parsed: T = serde_json::from_value(raw_item)?;
            target[idx] = parsed;
        }
        ListOperation::Remove => {
            let idx = index.ok_or_else(|| anyhow::anyhow!("index requis pour remove"))?;
            if idx >= target.len() {
                return Err(anyhow::anyhow!(
                    "index {} hors limites (taille actuelle: {})",
                    idx,
                    target.len()
                ));
            }
            target.remove(idx);
        }
        ListOperation::Replace => {
            let raw_items = items.ok_or_else(|| anyhow::anyhow!("items requis pour replace"))?;
            let parsed: Vec<T> = raw_items
                .into_iter()
                .map(serde_json::from_value)
                .collect::<Result<Vec<T>, _>>()?;
            *target = parsed;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::is_mutation_request;

    #[test]
    fn mutation_request_detects_direct_action() {
        assert!(is_mutation_request("ajoute une expérience Rust"));
        assert!(is_mutation_request("Peux-tu modifier mon accroche ?"));
        assert!(is_mutation_request("mets à jour mon CV"));
    }

    #[test]
    fn mutation_request_avoids_obvious_false_positive() {
        assert!(!is_mutation_request(
            "explique-moi pourquoi tu n'as pas ajouté Rust"
        ));
    }
}
