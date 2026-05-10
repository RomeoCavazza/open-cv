use crate::chat::chat_event::ChatEvent;
use crate::chat::prompt_utils::push_chat_history;
use domain::{Instance, Message, MessageRole, Profil};
use futures::{stream, StreamExt};
use ports::{BoxStream, InstanceRepo, LlmError, MessageRepo, ProfilRepo};
use std::sync::Arc;

pub struct StreamOrchestrator;

impl StreamOrchestrator {
    /// Wraps a raw `String` token stream from the LLM into a typed `ChatEvent` stream.
    ///
    /// - Each token is wrapped in `ChatEvent::Token`.
    /// - On stream end, persists the full response as an assistant message
    ///   and emits `ChatEvent::Done`.
    pub fn wrap_instance_stream(
        stream: BoxStream<'static, Result<String, LlmError>>,
        instance: Instance,
        message_repo: Arc<dyn MessageRepo>,
        instance_repo: Arc<dyn InstanceRepo>,
    ) -> BoxStream<'static, Result<ChatEvent, LlmError>> {
        let full_response = String::new();

        let mapped = stream::unfold(
            (
                stream,
                instance,
                full_response,
                message_repo,
                instance_repo,
                false, // done_emitted
            ),
            |(
                mut stream,
                mut instance,
                mut full_response,
                message_repo,
                instance_repo,
                done_emitted,
            )| async move {
                if done_emitted {
                    return None;
                }

                match stream.next().await {
                    Some(Ok(token)) => {
                        full_response.push_str(&token);
                        Some((
                            Ok(ChatEvent::token(token)),
                            (
                                stream,
                                instance,
                                full_response,
                                message_repo,
                                instance_repo,
                                false,
                            ),
                        ))
                    }
                    Some(Err(e)) => Some((
                        Err(e),
                        (
                            stream,
                            instance,
                            full_response,
                            message_repo,
                            instance_repo,
                            false,
                        ),
                    )),
                    None => {
                        if !full_response.is_empty() {
                            let assistant_msg =
                                Message::new(instance.id, MessageRole::Assistant, full_response);
                            let _ = message_repo.push(&assistant_msg).await;
                            instance.updated_at = chrono::Utc::now();
                            let _ = instance_repo.upsert(&instance).await;
                        }
                        // Emit Done then stop
                        Some((
                            Ok(ChatEvent::Done),
                            (
                                stream,
                                instance,
                                String::new(),
                                message_repo,
                                instance_repo,
                                true,
                            ),
                        ))
                    }
                }
            },
        );
        Box::pin(mapped)
    }

    /// Wraps a raw `String` token stream for global (non-instance) chat.
    ///
    /// Same semantics as `wrap_instance_stream` but persists into profil notes.
    pub fn wrap_global_stream(
        stream: BoxStream<'static, Result<String, LlmError>>,
        profil: Profil,
        profil_repo: Arc<dyn ProfilRepo>,
    ) -> BoxStream<'static, Result<ChatEvent, LlmError>> {
        let full_response = String::new();

        let mapped = stream::unfold(
            (stream, profil, full_response, profil_repo, false),
            |(mut stream, mut profil, mut full_response, profil_repo, done_emitted)| async move {
                if done_emitted {
                    return None;
                }

                match stream.next().await {
                    Some(Ok(token)) => {
                        full_response.push_str(&token);
                        Some((
                            Ok(ChatEvent::token(token)),
                            (stream, profil, full_response, profil_repo, false),
                        ))
                    }
                    Some(Err(e)) => {
                        Some((Err(e), (stream, profil, full_response, profil_repo, false)))
                    }
                    None => {
                        if !full_response.is_empty() {
                            push_chat_history(&mut profil.notes, "assistant", &full_response);
                            let _ = profil_repo.upsert(&profil).await;
                        }
                        Some((
                            Ok(ChatEvent::Done),
                            (stream, profil, String::new(), profil_repo, true),
                        ))
                    }
                }
            },
        );
        Box::pin(mapped)
    }
}
