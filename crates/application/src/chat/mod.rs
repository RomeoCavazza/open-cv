use domain::{Message, MessageRole};
use ports::{
    AnnexeRepo, ChunkRepo, Embedder, InstanceRepo, LlmClient, MessageRepo, ProfilRepo, OffreRepo, LlmError,
};
use std::sync::Arc;

pub mod persistence;
pub mod prompt_utils;
pub mod streaming;
pub mod types;

#[cfg(test)]
mod tests;

pub use self::types::*;
use self::persistence::ChatContextLoader;
use self::streaming::StreamOrchestrator;
use self::prompt_utils::{
    build_offer_prompt_context, build_profile_prompt_context, extract_chat_history,
    push_chat_history, render_chat_history_for_prompt, wants_identity, wants_mutation,
};

pub struct ChatWithApplicationUseCase {
    pub loader: ChatContextLoader,
    pub message_repo: Arc<dyn MessageRepo>,
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
            llm_registry,
        }
    }

    pub async fn execute(&self, req: ChatRequest) -> anyhow::Result<ChatResponse> {
        if let Some(id) = req.instance_id.as_ref().filter(|s| !s.is_empty()) {
            let id = id.to_string();
            return self.execute_instance_chat(&id, req).await;
        }
        self.execute_global_chat(req).await
    }

    pub async fn execute_stream(
        &self,
        req: ChatRequest,
    ) -> anyhow::Result<ports::BoxStream<'static, Result<String, LlmError>>> {
        if let Some(id) = req.instance_id.as_ref().filter(|s| !s.is_empty()) {
            let id = id.to_string();
            return self.execute_instance_chat_stream(&id, req).await;
        }
        self.execute_global_chat_stream(req).await
    }

    // --- Instance Chat ---

    async fn execute_instance_chat(
        &self,
        instance_id: &str,
        req: ChatRequest,
    ) -> anyhow::Result<ChatResponse> {
        let (mut instance, profil, offre) = self.loader.load_instance_context(instance_id).await?;
        let llm = self.get_llm(&req.llm_provider)?;
        let history = self.message_repo.list_by_instance_id(instance.id).await?;
        let rag_context = self.loader.get_rag_context(instance.profil_id, &req.message).await?;

        let user_msg = Message::new(instance.id, MessageRole::User, req.message.clone());
        self.message_repo.push(&user_msg).await?;

        let wants_mutation = wants_mutation(&req.message);
        let wants_identity = wants_identity(&req.message);

        let system_prompt = self.select_instance_system_prompt(wants_mutation, wants_identity);
        let user_input = self.build_instance_user_input(&instance, &profil, &offre, &history, &rag_context, &req);

        let response_json = self.call_llm_extract(llm, system_prompt, user_input, wants_mutation).await?;
        let new_data: ChatMutationOutput = serde_json::from_value(response_json)?;

        let ai_message = self.process_instance_mutation(&mut instance, new_data).await?;

        let assistant_msg = Message::new(instance.id, MessageRole::Assistant, ai_message.clone());
        self.message_repo.push(&assistant_msg).await?;

        instance.updated_at = chrono::Utc::now();
        self.loader.instances.upsert(&instance).await?;

        Ok(ChatResponse {
            updated_instance: Some(instance),
            message: ai_message,
        })
    }

    async fn execute_instance_chat_stream(
        &self,
        instance_id: &str,
        req: ChatRequest,
    ) -> anyhow::Result<ports::BoxStream<'static, Result<String, LlmError>>> {
        let (instance, profil, offre) = self.loader.load_instance_context(instance_id).await?;
        let llm = self.get_llm(&req.llm_provider)?;
        let history = self.message_repo.list_by_instance_id(instance.id).await?;
        let rag_context = self.loader.get_rag_context(instance.profil_id, &req.message).await?;

        let user_msg = Message::new(instance.id, MessageRole::User, req.message.clone());
        self.message_repo.push(&user_msg).await?;

        let user_input = self.render_instance_input_simple(&instance, &profil, &offre, &history, &rag_context, &req.message)?;
        
        let completion_req = ports::CompletionRequest {
            system: Some(crate::prompts::chat::INSTANCE_DEFAULT_SYSTEM.to_string()),
            messages: vec![ports::Message::user(user_input)],
            model: None,
            max_tokens: Some(4000),
            temperature: None,
        };

        let stream = llm.stream(completion_req).await?;
        Ok(StreamOrchestrator::wrap_instance_stream(
            stream, 
            instance, 
            self.message_repo.clone(), 
            self.loader.instances.clone()
        ))
    }

    // --- Global Chat ---

    async fn execute_global_chat(&self, req: ChatRequest) -> anyhow::Result<ChatResponse> {
        let mut profil = self.loader.load_active_profil().await?;
        let llm = self.get_llm(&req.llm_provider)?;
        let rag_context = self.loader.get_rag_context(profil.id, &req.message).await?;
        let chat_history = extract_chat_history(&profil.notes);
        let offres = self.loader.offres.list_all().await?;

        push_chat_history(&mut profil.notes, "user", &req.message);
        self.loader.profils.upsert(&profil).await?;

        let user_input = self.build_global_user_input(&profil, &offres, &chat_history, &rag_context, &req);
        
        let response_json = self.call_llm_extract(llm, crate::prompts::chat::GLOBAL_CHAT_SYSTEM, user_input, false).await?;
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

    async fn execute_global_chat_stream(
        &self,
        req: ChatRequest,
    ) -> anyhow::Result<ports::BoxStream<'static, Result<String, LlmError>>> {
        let mut profil = self.loader.load_active_profil().await?;
        let llm = self.get_llm(&req.llm_provider)?;
        let rag_context = self.loader.get_rag_context(profil.id, &req.message).await?;
        let chat_history = extract_chat_history(&profil.notes);
        let offres = self.loader.offres.list_all().await?;

        push_chat_history(&mut profil.notes, "user", &req.message);
        self.loader.profils.upsert(&profil).await?;

        let user_input = self.render_global_input_simple(&profil, &offres, &chat_history, &rag_context, &req.message)?;
        
        let completion_req = ports::CompletionRequest {
            system: Some(crate::prompts::chat::GLOBAL_CHAT_SYSTEM.to_string()),
            messages: vec![ports::Message::user(user_input)],
            model: None,
            max_tokens: Some(4000),
            temperature: None,
        };

        let stream = llm.stream(completion_req).await?;
        Ok(StreamOrchestrator::wrap_global_stream(
            stream, 
            profil, 
            self.loader.profils.clone()
        ))
    }

    // --- Helpers ---

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
            .map_err(|e| anyhow::anyhow!(e))
    }

    fn select_instance_system_prompt(&self, wants_mutation: bool, wants_identity: bool) -> &'static str {
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
            ).unwrap_or_default(),
            rag_context,
            history_prompt,
            req.message
        ))];

        self.append_attachments(&mut contents, &req.attachments);
        contents
    }

    fn append_attachments(&self, contents: &mut Vec<ports::MessageContent>, attachments: &[ChatAttachment]) {
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

    async fn process_instance_mutation(&self, instance: &mut domain::Instance, new_data: ChatMutationOutput) -> anyhow::Result<String> {
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

    fn render_instance_input_simple(&self, instance: &domain::Instance, profil: &domain::Profil, offre: &domain::Offre, history: &[domain::Message], rag_context: &str, message: &str) -> anyhow::Result<String> {
        let history_prompt = render_chat_history_for_prompt(history);
        Ok(format!(
            "IDENTITÉ DE L'UTILISATEUR (Profil complet):\n{}\n\n\
            OFFRE CIBLÉE (fiche brute et structurée):\n{}\n\n\
            ANALYSE DE L'OFFRE CIBLÉE (Restitution):\n{}\n\n\
            FRAGMENTS DE PARCOURS (RAG):\n{}\n\n\
            HISTORIQUE RÉCENT DU CHAT:\n{}\n\n\
            DEMANDE DE L'UTILISATEUR: {}\n\n\
            JSON ACTUEL DU CV: {}\n\n\
            JSON ACTUEL DE LA LETTRE: {}",
            serde_json::to_string_pretty(&build_profile_prompt_context(profil))?,
            serde_json::to_string_pretty(&build_offer_prompt_context(offre))?,
            serde_json::to_string_pretty(&instance.restitution)?,
            rag_context,
            history_prompt,
            message,
            serde_json::to_string_pretty(&instance.resume_json)?,
            serde_json::to_string_pretty(&instance.cover_letter_json)?
        ))
    }

    fn render_global_input_simple(&self, profil: &domain::Profil, offres: &[domain::Offre], chat_history: &[domain::Message], rag_context: &str, message: &str) -> anyhow::Result<String> {
        let history_prompt = render_chat_history_for_prompt(chat_history);
        Ok(format!(
            "IDENTITÉ DE L'UTILISATEUR (Profil complet):\n{}\n\n\
            OFFRES DISPONIBLES EN BASE ({} offres):\n{}\n\n\
            DÉTAILS DU PARCOURS (RAG):\n{}\n\n\
            HISTORIQUE RÉCENT DU CHAT:\n{}\n\n\
            DEMANDE DE L'UTILISATEUR: {}",
            serde_json::to_string_pretty(&build_profile_prompt_context(profil))?,
            offres.len(),
            serde_json::to_string_pretty(
                &offres
                    .iter()
                    .map(|o| (o.slug.to_string(), o.intitule.clone(), o.entreprise.clone()))
                    .collect::<Vec<_>>()
            )?,
            rag_context,
            history_prompt,
            message
        ))
    }
}
