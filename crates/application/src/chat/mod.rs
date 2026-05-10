use domain::{Message, MessageRole};
use futures::{stream, StreamExt};
use ports::{
    AnnexeRepo, ChunkRepo, Embedder, InstanceRepo, LlmClient, LlmError, MessageRepo, OffreRepo,
    ProfilRepo, SnapshotRepo,
};
use std::sync::Arc;
use tokio::sync::mpsc;

pub mod chat_event;
pub mod persistence;
pub mod prompt_utils;
pub mod streaming;
pub mod types;

#[cfg(test)]
mod tests;

pub use self::chat_event::ChatEvent;
use self::persistence::ChatContextLoader;
use self::prompt_utils::{
    build_offer_prompt_context, build_profile_prompt_context, extract_chat_history,
    push_chat_history, render_chat_history_for_prompt, wants_identity, wants_mutation, wants_undo,
};
pub use self::types::*;

/// Capacité du channel mpsc pour le pipeline streaming.
/// 32 événements de buffer suffisent largement : les tokens sont consommés
/// quasi-immédiatement par le handler SSE.
const STREAM_CHANNEL_CAPACITY: usize = 32;

#[derive(Clone)]
pub struct ChatWithApplicationUseCase {
    pub loader: ChatContextLoader,
    pub message_repo: Arc<dyn MessageRepo>,
    pub snapshot_repo: Option<Arc<dyn SnapshotRepo>>,
    pub llm_registry: std::collections::HashMap<String, Arc<dyn LlmClient>>,
}

impl ChatWithApplicationUseCase {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        offre_repo: Arc<dyn OffreRepo>,
        instance_repo: Arc<dyn InstanceRepo>,
        profil_repo: Arc<dyn ProfilRepo>,
        annexe_repo: Arc<dyn AnnexeRepo>,
        chunk_repo: Arc<dyn ChunkRepo>,
        message_repo: Arc<dyn MessageRepo>,
        embedder: Arc<dyn Embedder>,
        llm_registry: std::collections::HashMap<String, Arc<dyn LlmClient>>,
    ) -> Self {
        Self {
            loader: ChatContextLoader {
                offres: offre_repo,
                instances: instance_repo,
                profils: profil_repo,
                annexes: annexe_repo,
                chunks: chunk_repo,
                embedder,
            },
            message_repo,
            snapshot_repo: None,
            llm_registry,
        }
    }

    /// Configure le repo de snapshots pour activer l'undo.
    pub fn with_snapshot_repo(mut self, repo: Arc<dyn SnapshotRepo>) -> Self {
        self.snapshot_repo = Some(repo);
        self
    }

    pub async fn execute(&self, req: ChatRequest) -> anyhow::Result<ChatResponse> {
        if let Some(id) = req.instance_id.as_ref().filter(|s| !s.is_empty()) {
            let id = id.to_string();
            return self.execute_instance_chat(&id, req).await;
        }
        self.execute_global_chat(req).await
    }

    /// Retourne un stream de `ChatEvent` avec liveliness intégrée.
    ///
    /// Le pipeline est exécuté dans un `tokio::spawn` qui pousse des
    /// `ChatEvent::Status` à chaque phase de calcul, permettant au frontend
    /// d'afficher des messages de progression en temps réel.
    pub async fn execute_stream(
        &self,
        req: ChatRequest,
    ) -> anyhow::Result<ports::BoxStream<'static, Result<ChatEvent, LlmError>>> {
        let (tx, rx) = mpsc::channel::<Result<ChatEvent, LlmError>>(STREAM_CHANNEL_CAPACITY);
        let this = self.clone();

        tokio::spawn(async move {
            if let Some(id) = req.instance_id.as_ref().filter(|s| !s.is_empty()) {
                let id = id.to_string();
                this.run_instance_pipeline(&id, req, tx).await;
            } else {
                this.run_global_pipeline(req, tx).await;
            }
        });

        // Convertir le receiver mpsc en Stream
        let stream = stream::unfold(rx, |mut rx| async move {
            rx.recv().await.map(|item| (item, rx))
        });

        Ok(Box::pin(stream))
    }

    // ─────────────────────────────────────────────────────────────────
    // Pipeline Instance (mpsc-driven)
    // ─────────────────────────────────────────────────────────────────

    async fn run_instance_pipeline(
        &self,
        instance_id: &str,
        req: ChatRequest,
        tx: mpsc::Sender<Result<ChatEvent, LlmError>>,
    ) {
        let result: anyhow::Result<()> = async {
            tx.send(Ok(ChatEvent::status("Chargement du contexte...")))
                .await
                .ok();

            let (mut instance, profil, offre) =
                self.loader.load_instance_context(instance_id).await?;
            let llm = self.get_llm(&req.llm_provider)?;
            let history = self.message_repo.list_by_instance_id(instance.id).await?;

            tx.send(Ok(ChatEvent::status("Recherche dans votre parcours...")))
                .await
                .ok();

            let rag_context = self
                .loader
                .get_rag_context(instance.profil_id, &req.message)
                .await?;

            let user_msg = Message::new(instance.id, MessageRole::User, req.message.clone());
            self.message_repo.push(&user_msg).await?;

            let wants_mut = wants_mutation(&req.message);
            let wants_id = wants_identity(&req.message);

            // --- Undo : restauration depuis le dernier snapshot ---
            if wants_undo(&req.message) {
                tx.send(Ok(ChatEvent::status(
                    "Restauration de la version précédente...",
                )))
                .await
                .ok();

                if let Some(ref snap_repo) = self.snapshot_repo {
                    if let Ok(Some(snapshot)) = snap_repo.get_latest(instance.id).await {
                        instance.resume_json = snapshot.resume_json;
                        instance.cover_letter_json = snapshot.cover_letter_json;
                        instance.restitution = snapshot.restitution;
                        instance.updated_at = chrono::Utc::now();
                        self.loader.instances.upsert(&instance).await?;

                        let msg = "J'ai restauré la version précédente de vos documents.";
                        let assistant_msg =
                            Message::new(instance.id, MessageRole::Assistant, msg.to_string());
                        self.message_repo.push(&assistant_msg).await?;

                        for chunk in Self::chunk_response_for_stream(msg) {
                            if tx.send(Ok(ChatEvent::token(chunk))).await.is_err() {
                                return Ok(());
                            }
                        }
                        tx.send(Ok(ChatEvent::mutation(instance))).await.ok();
                        tx.send(Ok(ChatEvent::Done)).await.ok();
                        return Ok(());
                    }
                }

                // Pas de snapshot disponible
                let msg = "Aucune version précédente disponible pour cette candidature.";
                let assistant_msg =
                    Message::new(instance.id, MessageRole::Assistant, msg.to_string());
                self.message_repo.push(&assistant_msg).await?;
                for chunk in Self::chunk_response_for_stream(msg) {
                    if tx.send(Ok(ChatEvent::token(chunk))).await.is_err() {
                        return Ok(());
                    }
                }
                tx.send(Ok(ChatEvent::Done)).await.ok();
                return Ok(());
            }

            // --- Mutation : modification du CV/LM ---
            if wants_mut {
                tx.send(Ok(ChatEvent::status("L'IA modifie vos documents...")))
                    .await
                    .ok();

                // Snapshot avant mutation (si le repo est configuré)
                if let Some(ref snap_repo) = self.snapshot_repo {
                    let version = snap_repo.count_by_instance(instance.id).await.unwrap_or(0) + 1;
                    let snapshot = domain::InstanceSnapshot::capture(
                        &instance,
                        version,
                        Some(req.message.clone()),
                    );
                    let _ = snap_repo.save(&snapshot).await;
                }

                let system_prompt = self.select_instance_system_prompt(true, false);
                let user_input = self.build_instance_user_input(
                    &instance,
                    &profil,
                    &offre,
                    &history,
                    &rag_context,
                    &req,
                );

                let response_json = self
                    .call_llm_extract(llm, system_prompt, user_input, true)
                    .await?;
                let new_data: ChatMutationOutput = serde_json::from_value(response_json)?;

                tx.send(Ok(ChatEvent::status("Application des modifications...")))
                    .await
                    .ok();

                let ai_message = self
                    .process_instance_mutation(&mut instance, new_data)
                    .await?;
                let assistant_msg = Message::new(instance.id, MessageRole::Assistant, ai_message);
                self.message_repo.push(&assistant_msg).await?;

                instance.updated_at = chrono::Utc::now();
                self.loader.instances.upsert(&instance).await?;

                // Token chunks pour le message IA
                for chunk in Self::chunk_response_for_stream(&assistant_msg.content) {
                    if tx.send(Ok(ChatEvent::token(chunk))).await.is_err() {
                        return Ok(());
                    }
                }
                // Mutation + Done
                tx.send(Ok(ChatEvent::mutation(instance))).await.ok();
                tx.send(Ok(ChatEvent::Done)).await.ok();
                return Ok(());
            }

            // Non-mutation : streaming LLM
            tx.send(Ok(ChatEvent::status("L'IA réfléchit...")))
                .await
                .ok();

            let user_input = self.build_instance_user_input(
                &instance,
                &profil,
                &offre,
                &history,
                &rag_context,
                &req,
            );
            let system_prompt = self.select_instance_system_prompt(false, wants_id);
            let completion_req = ports::CompletionRequest {
                system: Some(system_prompt.to_string()),
                messages: vec![ports::Message {
                    role: ports::Role::User,
                    content: user_input,
                }],
                model: None,
                max_tokens: Some(4000),
                temperature: None,
            };

            let mut llm_stream = llm.stream(completion_req).await?;
            let mut full_response = String::new();

            while let Some(result) = llm_stream.next().await {
                match result {
                    Ok(token) => {
                        full_response.push_str(&token);
                        if tx.send(Ok(ChatEvent::token(token))).await.is_err() {
                            return Ok(());
                        }
                    }
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        return Ok(());
                    }
                }
            }

            // Persist
            if !full_response.is_empty() {
                let assistant_msg =
                    Message::new(instance.id, MessageRole::Assistant, full_response);
                let _ = self.message_repo.push(&assistant_msg).await;
                let mut instance = instance;
                instance.updated_at = chrono::Utc::now();
                let _ = self.loader.instances.upsert(&instance).await;
            }

            tx.send(Ok(ChatEvent::Done)).await.ok();
            Ok(())
        }
        .await;

        if let Err(e) = result {
            let _ = tx.send(Ok(ChatEvent::error(e.to_string()))).await;
        }
    }

    // ─────────────────────────────────────────────────────────────────
    // Pipeline Global (mpsc-driven)
    // ─────────────────────────────────────────────────────────────────

    async fn run_global_pipeline(
        &self,
        req: ChatRequest,
        tx: mpsc::Sender<Result<ChatEvent, LlmError>>,
    ) {
        let result: anyhow::Result<()> = async {
            tx.send(Ok(ChatEvent::status("Chargement du profil...")))
                .await
                .ok();

            let mut profil = self.loader.load_active_profil().await?;
            let llm = self.get_llm(&req.llm_provider)?;

            tx.send(Ok(ChatEvent::status("Recherche dans votre parcours...")))
                .await
                .ok();

            let rag_context = self.loader.get_rag_context(profil.id, &req.message).await?;
            let chat_history = extract_chat_history(&profil.notes);
            let offres = self.loader.offres.list_all().await?;

            push_chat_history(&mut profil.notes, "user", &req.message);
            self.loader.profils.upsert(&profil).await?;

            tx.send(Ok(ChatEvent::status("L'IA réfléchit...")))
                .await
                .ok();

            let user_input =
                self.build_global_user_input(&profil, &offres, &chat_history, &rag_context, &req);

            let completion_req = ports::CompletionRequest {
                system: Some(crate::prompts::chat::GLOBAL_CHAT_SYSTEM.to_string()),
                messages: vec![ports::Message {
                    role: ports::Role::User,
                    content: user_input,
                }],
                model: None,
                max_tokens: Some(4000),
                temperature: None,
            };

            let mut llm_stream = llm.stream(completion_req).await?;
            let mut full_response = String::new();

            while let Some(result) = llm_stream.next().await {
                match result {
                    Ok(token) => {
                        full_response.push_str(&token);
                        if tx.send(Ok(ChatEvent::token(token))).await.is_err() {
                            return Ok(());
                        }
                    }
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        return Ok(());
                    }
                }
            }

            // Persist
            if !full_response.is_empty() {
                push_chat_history(&mut profil.notes, "assistant", &full_response);
                let _ = self.loader.profils.upsert(&profil).await;
            }

            tx.send(Ok(ChatEvent::Done)).await.ok();
            Ok(())
        }
        .await;

        if let Err(e) = result {
            let _ = tx.send(Ok(ChatEvent::error(e.to_string()))).await;
        }
    }

    // ─────────────────────────────────────────────────────────────────
    // Instance Chat (non-stream, inchangé)
    // ─────────────────────────────────────────────────────────────────

    async fn execute_instance_chat(
        &self,
        instance_id: &str,
        req: ChatRequest,
    ) -> anyhow::Result<ChatResponse> {
        let (mut instance, profil, offre) = self.loader.load_instance_context(instance_id).await?;
        let llm = self.get_llm(&req.llm_provider)?;
        let history = self.message_repo.list_by_instance_id(instance.id).await?;
        let rag_context = self
            .loader
            .get_rag_context(instance.profil_id, &req.message)
            .await?;

        let user_msg = Message::new(instance.id, MessageRole::User, req.message.clone());
        self.message_repo.push(&user_msg).await?;

        let wants_mutation = wants_mutation(&req.message);
        let wants_identity = wants_identity(&req.message);

        let system_prompt = self.select_instance_system_prompt(wants_mutation, wants_identity);
        let user_input = self.build_instance_user_input(
            &instance,
            &profil,
            &offre,
            &history,
            &rag_context,
            &req,
        );

        let response_json = self
            .call_llm_extract(llm, system_prompt, user_input, wants_mutation)
            .await?;
        let new_data: ChatMutationOutput = serde_json::from_value(response_json)?;

        let ai_message = self
            .process_instance_mutation(&mut instance, new_data)
            .await?;

        let assistant_msg = Message::new(instance.id, MessageRole::Assistant, ai_message.clone());
        self.message_repo.push(&assistant_msg).await?;

        instance.updated_at = chrono::Utc::now();
        self.loader.instances.upsert(&instance).await?;

        Ok(ChatResponse {
            updated_instance: Some(instance),
            message: ai_message,
        })
    }

    // ─────────────────────────────────────────────────────────────────
    // Global Chat (non-stream, inchangé)
    // ─────────────────────────────────────────────────────────────────

    async fn execute_global_chat(&self, req: ChatRequest) -> anyhow::Result<ChatResponse> {
        let mut profil = self.loader.load_active_profil().await?;
        let llm = self.get_llm(&req.llm_provider)?;
        let rag_context = self.loader.get_rag_context(profil.id, &req.message).await?;
        let chat_history = extract_chat_history(&profil.notes);
        let offres = self.loader.offres.list_all().await?;

        push_chat_history(&mut profil.notes, "user", &req.message);
        self.loader.profils.upsert(&profil).await?;

        let user_input =
            self.build_global_user_input(&profil, &offres, &chat_history, &rag_context, &req);

        let response_json = self
            .call_llm_extract(
                llm,
                crate::prompts::chat::GLOBAL_CHAT_SYSTEM,
                user_input,
                false,
            )
            .await?;
        let ai_message = response_json["message"]
            .as_str()
            .unwrap_or("Désolé, je n'ai pas pu générer de réponse.")
            .to_string();

        push_chat_history(&mut profil.notes, "assistant", &ai_message);
        self.loader.profils.upsert(&profil).await?;

        Ok(ChatResponse {
            updated_instance: None,
            message: ai_message,
        })
    }

    // ─────────────────────────────────────────────────────────────────
    // Helpers
    // ─────────────────────────────────────────────────────────────────

    fn get_llm(&self, provider: &str) -> anyhow::Result<Arc<dyn LlmClient>> {
        self.llm_registry
            .get(provider)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("LLM '{}' non configuré", provider))
    }

    async fn call_llm_extract(
        &self,
        llm: Arc<dyn LlmClient>,
        system: &str,
        input: Vec<ports::MessageContent>,
        wants_mutation: bool,
    ) -> anyhow::Result<serde_json::Value> {
        let extraction_req = ports::ExtractionRequest {
            system: Some(system.to_string()),
            instruction: "RÉPONDS UNIQUEMENT AVEC DU JSON.".into(),
            input,
            schema_name: "ChatResponse".into(),
            schema_description: "Réponse du chat".into(),
            json_schema: if wants_mutation {
                serde_json::to_value(schemars::schema_for!(ChatMutationOutput))?
            } else {
                serde_json::json!({
                    "type": "object",
                    "properties": {
                        "message": { "type": "string" }
                    },
                    "required": ["message"]
                })
            },
            model: None,
            max_tokens: Some(4000),
        };

        llm.extract(extraction_req)
            .await
            .map(|resp| resp.value)
            .map_err(|e| anyhow::anyhow!(e))
    }

    fn select_instance_system_prompt(
        &self,
        wants_mutation: bool,
        wants_identity: bool,
    ) -> &'static str {
        if wants_mutation {
            crate::prompts::chat::INSTANCE_MUTATION_SYSTEM
        } else if wants_identity {
            crate::prompts::chat::INSTANCE_IDENTITY_SYSTEM
        } else {
            crate::prompts::chat::INSTANCE_DEFAULT_SYSTEM
        }
    }

    fn build_instance_user_input(
        &self,
        instance: &domain::Instance,
        profil: &domain::Profil,
        offre: &domain::Offre,
        history: &[domain::Message],
        rag_context: &str,
        req: &ChatRequest,
    ) -> Vec<ports::MessageContent> {
        let history_prompt = render_chat_history_for_prompt(history);
        let mut contents = vec![ports::MessageContent::Text(format!(
            "IDENTITÉ DE L'UTILISATEUR (Profil complet):\n{}\n\n\
            OFFRE CIBLÉE (fiche brute et structurée):\n{}\n\n\
            ANALYSE DE L'OFFRE CIBLÉE (Restitution):\n{}\n\n\
            FRAGMENTS DE PARCOURS (RAG):\n{}\n\n\
            HISTORIQUE RÉCENT DU CHAT:\n{}\n\n\
            DEMANDE DE L'UTILISATEUR: {}\n\n\
            JSON ACTUEL DU CV: {}\n\n\
            JSON ACTUEL DE LA LETTRE: {}",
            serde_json::to_string_pretty(&build_profile_prompt_context(profil)).unwrap_or_default(),
            serde_json::to_string_pretty(&build_offer_prompt_context(offre)).unwrap_or_default(),
            serde_json::to_string_pretty(&instance.restitution).unwrap_or_default(),
            rag_context,
            history_prompt,
            req.message,
            serde_json::to_string_pretty(&instance.resume_json).unwrap_or_default(),
            serde_json::to_string_pretty(&instance.cover_letter_json).unwrap_or_default()
        ))];

        self.append_attachments(&mut contents, &req.attachments);
        contents
    }

    fn build_global_user_input(
        &self,
        profil: &domain::Profil,
        offres: &[domain::Offre],
        chat_history: &[domain::Message],
        rag_context: &str,
        req: &ChatRequest,
    ) -> Vec<ports::MessageContent> {
        let history_prompt = render_chat_history_for_prompt(chat_history);
        let mut contents = vec![ports::MessageContent::Text(format!(
            "IDENTITÉ DE L'UTILISATEUR (Profil complet):\n{}\n\n\
            OFFRES DISPONIBLES EN BASE ({} offres):\n{}\n\n\
            DÉTAILS DU PARCOURS (RAG):\n{}\n\n\
            HISTORIQUE RÉCENT DU CHAT:\n{}\n\n\
            DEMANDE DE L'UTILISATEUR: {}",
            serde_json::to_string_pretty(&build_profile_prompt_context(profil)).unwrap_or_default(),
            offres.len(),
            serde_json::to_string_pretty(
                &offres
                    .iter()
                    .map(|o| (o.slug.to_string(), o.intitule.clone(), o.entreprise.clone()))
                    .collect::<Vec<_>>()
            )
            .unwrap_or_default(),
            rag_context,
            history_prompt,
            req.message
        ))];

        self.append_attachments(&mut contents, &req.attachments);
        contents
    }

    fn append_attachments(
        &self,
        contents: &mut Vec<ports::MessageContent>,
        attachments: &[ChatAttachment],
    ) {
        use base64::Engine;
        for att in attachments {
            if att.content_type.starts_with("image/") {
                let b64 = att.data.split(',').nth(1).unwrap_or(&att.data);
                if let Ok(data) = base64::engine::general_purpose::STANDARD.decode(b64) {
                    contents.push(ports::MessageContent::Image {
                        data,
                        content_type: att.content_type.clone(),
                    });
                }
            }
        }
    }

    async fn process_instance_mutation(
        &self,
        instance: &mut domain::Instance,
        new_data: ChatMutationOutput,
    ) -> anyhow::Result<String> {
        let ai_message = if new_data.message.trim().is_empty() {
            "J'ai mis à jour les documents selon votre demande.".to_string()
        } else {
            new_data.message
        };

        if let Some(res) = new_data.resume {
            instance.resume_json = Some(res);
            instance.status = domain::InstanceStatus::Ready;
        }
        if let Some(cov) = new_data.cover {
            instance.cover_letter_json = Some(cov);
            instance.status = domain::InstanceStatus::Ready;
        }
        Ok(ai_message)
    }

    fn chunk_response_for_stream(text: &str) -> Vec<String> {
        let mut chunks = Vec::new();
        let mut current = String::new();

        for ch in text.chars() {
            current.push(ch);
            if ch == ' ' || current.len() >= 14 {
                chunks.push(std::mem::take(&mut current));
            }
        }

        if !current.is_empty() {
            chunks.push(current);
        }

        if chunks.is_empty() {
            chunks.push(String::new());
        }

        chunks
    }
}
