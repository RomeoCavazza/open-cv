use domain::{Message, MessageRole};
use futures::{stream, StreamExt};
use ports::{
    AnnexeRepo, ChunkRepo, Embedder, InstanceRepo, LlmClient, LlmError, MessageRepo, OffreRepo,
    ProfilRepo, SnapshotRepo,
};
use std::sync::Arc;
use tokio::sync::mpsc;

pub mod chat_event;
pub mod driver;
pub mod persistence;
pub mod prompt_utils;
pub mod types;

#[cfg(test)]
mod tests;

pub use self::chat_event::ChatEvent;
use self::driver::ConversationDriver;
use self::persistence::ChatContextLoader;
use self::prompt_utils::{
    build_profile_prompt_context, extract_chat_history_for_conversation,
    push_chat_history_for_conversation, render_chat_history_for_prompt,
};
pub use self::types::*;

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

    pub fn with_snapshot_repo(mut self, repo: Arc<dyn SnapshotRepo>) -> Self {
        self.snapshot_repo = Some(repo);
        self
    }

    pub async fn execute(&self, req: ChatRequest) -> anyhow::Result<ChatResponse> {
        let mut stream = self.execute_stream(req).await?;
        let mut full_message = String::new();
        let mut last_instance = None;

        use futures::StreamExt;
        while let Some(res) = stream.next().await {
            match res? {
                ChatEvent::Token { content } => full_message.push_str(&content),
                ChatEvent::Mutation { instance } => last_instance = Some(*instance),
                _ => {}
            }
        }

        Ok(ChatResponse {
            updated_instance: last_instance,
            message: full_message,
        })
    }

    pub async fn execute_stream(
        &self,
        req: ChatRequest,
    ) -> anyhow::Result<ports::BoxStream<'static, Result<ChatEvent, LlmError>>> {
        let (tx, rx) = mpsc::channel::<Result<ChatEvent, LlmError>>(STREAM_CHANNEL_CAPACITY);
        let this = self.clone();

        tokio::spawn(async move {
            if let Some(id) = req.instance_id.as_ref().filter(|s| !s.is_empty()) {
                let id = id.to_string();
                this.run_instance_pipeline(id, req, tx).await;
            } else {
                this.run_global_pipeline(req, tx).await;
            }
        });

        let stream = stream::unfold(rx, |mut rx| async move {
            rx.recv().await.map(|item| (item, rx))
        });

        Ok(Box::pin(stream))
    }

    /// Liste tous les snapshots d'une candidature.
    pub async fn list_snapshots(
        &self,
        instance_id: &str,
    ) -> anyhow::Result<Vec<domain::InstanceSnapshot>> {
        let snap_repo = self
            .snapshot_repo
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("SnapshotRepo non activé (SnapshotRepo manquant)"))?;
        let (instance, _, _) = self.loader.load_instance_context(instance_id).await?;
        let snapshots = snap_repo.list_by_instance(instance.id).await?;
        Ok(snapshots)
    }

    /// Restaure une version spécifique.
    pub async fn restore_snapshot(
        &self,
        instance_id: &str,
        version: i32,
    ) -> anyhow::Result<domain::Instance> {
        let snap_repo = self
            .snapshot_repo
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("SnapshotRepo non activé (SnapshotRepo manquant)"))?;
        let (mut instance, _, _) = self.loader.load_instance_context(instance_id).await?;
        let snapshot = snap_repo
            .get_by_version(instance.id, version)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Version {} introuvable", version))?;

        let current_version = snap_repo.count_by_instance(instance.id).await.unwrap_or(0) + 1;
        let backup_before_restore = domain::InstanceSnapshot::capture(
            &instance,
            current_version,
            Some(format!(
                "Backup avant restauration de la version {}",
                version
            )),
        );
        let _ = snap_repo.save(&backup_before_restore).await;

        instance.resume_json = snapshot.resume_json;
        instance.cover_letter_json = snapshot.cover_letter_json;
        instance.restitution = snapshot.restitution;
        instance.updated_at = chrono::Utc::now();

        self.loader.instances.upsert(&instance).await?;

        let assistant_msg = Message::new(
            instance.id,
            MessageRole::Assistant,
            format!("J'ai restauré la version {}.", version),
        );
        self.message_repo.push(&assistant_msg).await?;

        Ok(instance)
    }

    /// Annule la dernière modification sur une candidature.
    pub async fn undo(&self, instance_id: &str) -> anyhow::Result<domain::Instance> {
        let (mut instance, _, _) = self.loader.load_instance_context(instance_id).await?;

        let snap_repo = self
            .snapshot_repo
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Undo non activé (SnapshotRepo manquant)"))?;
        let snapshot = snap_repo
            .get_latest(instance.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Aucune version précédente disponible"))?;

        instance.resume_json = snapshot.resume_json;
        instance.cover_letter_json = snapshot.cover_letter_json;
        instance.restitution = snapshot.restitution;
        instance.updated_at = chrono::Utc::now();

        self.loader.instances.upsert(&instance).await?;

        let assistant_msg = Message::new(
            instance.id,
            MessageRole::Assistant,
            "J'ai restauré la version précédente de vos documents.".to_string(),
        );
        self.message_repo.push(&assistant_msg).await?;

        Ok(instance)
    }

    async fn run_instance_pipeline(
        &self,
        instance_id: String,
        req: ChatRequest,
        tx: mpsc::Sender<Result<ChatEvent, LlmError>>,
    ) {
        let llm = match self.get_llm(&req.llm_provider) {
            Ok(llm) => llm,
            Err(e) => {
                tx.send(Ok(ChatEvent::error(e.to_string()))).await.ok();
                return;
            }
        };

        let driver = ConversationDriver {
            loader: self.loader.clone(),
            message_repo: self.message_repo.clone(),
            snapshot_repo: self.snapshot_repo.clone(),
            llm,
        };

        if let Err(e) = driver.run(instance_id, req, tx.clone()).await {
            tx.send(Ok(ChatEvent::error(e.to_string()))).await.ok();
        }
    }

    async fn run_global_pipeline(
        &self,
        req: ChatRequest,
        tx: mpsc::Sender<Result<ChatEvent, LlmError>>,
    ) {
        // ... (Logique globale simplifiée si besoin, mais reste similaire)
        // Pour l'instant on garde la logique globale inchangée mais on pourrait
        // aussi utiliser le Driver.
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
            let chat_history = extract_chat_history_for_conversation(
                &profil.notes,
                req.conversation_id.as_deref(),
            );
            let offres = self.loader.offres.list_all().await?;

            let _conversation_id = push_chat_history_for_conversation(
                &mut profil.notes,
                req.conversation_id.as_deref(),
                "user",
                &req.message,
                None,
            );
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
                tools: Vec::new(),
                tool_choice: ports::ToolChoice::None,
            };

            let mut stream = llm.stream(completion_req).await?;
            let mut full_response = String::new();

            while let Some(chunk_res) = stream.next().await {
                let chunk = chunk_res?;
                if let ports::StreamChunk::TextDelta { text } = chunk {
                    full_response.push_str(&text);
                    if tx.send(Ok(ChatEvent::token(text))).await.is_err() {
                        return Ok(());
                    }
                }
            }

            if !full_response.is_empty() {
                let _ = push_chat_history_for_conversation(
                    &mut profil.notes,
                    req.conversation_id.as_deref(),
                    "assistant",
                    &full_response,
                    None,
                );
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

    fn get_llm(&self, provider: &str) -> anyhow::Result<Arc<dyn LlmClient>> {
        self.llm_registry
            .get(provider)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("LLM '{}' non configuré", provider))
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
}
