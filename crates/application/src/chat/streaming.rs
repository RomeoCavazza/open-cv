use crate::chat::prompt_utils::push_chat_history;
use domain::{Instance, Message, MessageRole, Profil};
use futures::{stream, StreamExt};
use ports::{BoxStream, InstanceRepo, LlmError, MessageRepo, ProfilRepo};
use std::sync::Arc;

pub struct StreamOrchestrator;

impl StreamOrchestrator {
    pub fn wrap_instance_stream(
        stream: BoxStream<'static, Result<String, LlmError>>,
        instance: Instance,
        message_repo: Arc<dyn MessageRepo>,
        instance_repo: Arc<dyn InstanceRepo>,
    ) -> BoxStream<'static, Result<String, LlmError>> {
        let full_response = String::new();

        let mapped = stream::unfold(
            (stream, instance, full_response, message_repo, instance_repo),
            |(mut stream, mut instance, mut full_response, message_repo, instance_repo)| async move {
                match stream.next().await {
                    Some(Ok(token)) => {
                        full_response.push_str(&token);
                        Some((
                            Ok(token),
                            (stream, instance, full_response, message_repo, instance_repo),
                        ))
                    }
                    Some(Err(e)) => Some((
                        Err(e),
                        (stream, instance, full_response, message_repo, instance_repo),
                    )),
                    None => {
                        if !full_response.is_empty() {
                            let assistant_msg =
                                Message::new(instance.id, MessageRole::Assistant, full_response);
                            let _ = message_repo.push(&assistant_msg).await;
                            instance.updated_at = chrono::Utc::now();
                            let _ = instance_repo.upsert(&instance).await;
                        }
                        None
                    }
                }
            },
        );
        Box::pin(mapped)
    }

    pub fn wrap_global_stream(
        stream: BoxStream<'static, Result<String, LlmError>>,
        profil: Profil,
        profil_repo: Arc<dyn ProfilRepo>,
    ) -> BoxStream<'static, Result<String, LlmError>> {
        let full_response = String::new();

        let mapped = stream::unfold(
            (stream, profil, full_response, profil_repo),
            |(mut stream, mut profil, mut full_response, profil_repo)| async move {
                match stream.next().await {
                    Some(Ok(token)) => {
                        full_response.push_str(&token);
                        Some((Ok(token), (stream, profil, full_response, profil_repo)))
                    }
                    Some(Err(e)) => Some((Err(e), (stream, profil, full_response, profil_repo))),
                    None => {
                        if !full_response.is_empty() {
                            push_chat_history(&mut profil.notes, "assistant", &full_response);
                            let _ = profil_repo.upsert(&profil).await;
                        }
                        None
                    }
                }
            },
        );
        Box::pin(mapped)
    }
}
