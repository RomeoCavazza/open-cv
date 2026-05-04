use base64::Engine;
use domain::{Instance, Message, MessageRole};
use ports::{
    AnnexeRepo, ChunkRepo, EmbedMode, Embedder, ExtractionRequest, InstanceRepo, LlmClient,
    MessageRepo, ProfilRepo,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;

use crate::prompts;

const MAX_CHAT_HISTORY_ENTRIES: usize = 20;

mod prompt_utils;

use self::prompt_utils::{
    build_offer_prompt_context, build_profile_prompt_context, extract_chat_history,
    push_chat_history, render_chat_history_for_prompt, wants_identity, wants_mutation,
};

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    pub instance_id: Option<String>,
    pub llm_provider: String,
    #[serde(default)]
    pub attachments: Vec<ChatAttachment>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChatAttachment {
    pub name: String,
    pub content_type: String,
    pub data: String, // Base64 (data:image/jpeg;base64,...)
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub updated_instance: Option<Instance>,
    pub message: String,
}

#[derive(Debug, Clone, Copy)]
enum ChatOutputKind {
    MessageOnly,
    Mutation,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
struct ChatMutationOutput {
    #[serde(default)]
    resume: Option<serde_json::Value>,
    #[serde(default)]
    cover: Option<serde_json::Value>,
    message: String,
}

pub struct ChatWithApplicationUseCase {
    pub offre_repo: Arc<dyn ports::OffreRepo>,
    pub instance_repo: Arc<dyn InstanceRepo>,
    pub profil_repo: Arc<dyn ProfilRepo>,
    pub annexe_repo: Arc<dyn AnnexeRepo>,
    pub chunk_repo: Arc<dyn ChunkRepo>,
    pub message_repo: Arc<dyn MessageRepo>,
    pub embedder: Arc<dyn Embedder>,
    pub llm_registry: std::collections::HashMap<String, Arc<dyn LlmClient>>,
}
impl ChatWithApplicationUseCase {
    pub fn new(
        offre_repo: Arc<dyn ports::OffreRepo>,
        instance_repo: Arc<dyn InstanceRepo>,
        profil_repo: Arc<dyn ProfilRepo>,
        annexe_repo: Arc<dyn AnnexeRepo>,
        chunk_repo: Arc<dyn ChunkRepo>,
        message_repo: Arc<dyn MessageRepo>,
        embedder: Arc<dyn Embedder>,
        llm_registry: std::collections::HashMap<String, Arc<dyn LlmClient>>,
    ) -> Self {
        Self {
            offre_repo,
            instance_repo,
            profil_repo,
            annexe_repo,
            chunk_repo,
            message_repo,
            embedder,
            llm_registry,
        }
    }

    pub async fn execute(&self, req: ChatRequest) -> anyhow::Result<ChatResponse> {
        let instance_id = req.instance_id.clone();
        if let Some(id) = instance_id {
            if !id.is_empty() {
                return self.execute_instance_chat(&id, req).await;
            }
        }
        self.execute_global_chat(req).await
    }

    async fn execute_instance_chat(
        &self,
        instance_id: &str,
        req: ChatRequest,
    ) -> anyhow::Result<ChatResponse> {
        let instance_uuid = uuid::Uuid::parse_str(instance_id)?;
        let mut instance = self
            .instance_repo
            .get_by_id(domain::InstanceId::from_uuid(instance_uuid))
            .await?
            .ok_or_else(|| anyhow::anyhow!("Instance non trouvée"))?;

        let profil = self
            .profil_repo
            .get_by_id(instance.profil_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Profil non trouvé"))?;

        let offre = self
            .offre_repo
            .get_by_id(instance.offre_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Offre non trouvée"))?;

        let wants_mutation = wants_mutation(&req.message);
        let wants_identity = wants_identity(&req.message);

        let history = self.message_repo.list_by_instance_id(instance.id).await?;

        info!(
            "Chat (Instance): Historique extrait ({} messages)",
            history.len()
        );

        let user_msg = Message::new(instance.id, MessageRole::User, req.message.clone());
        self.message_repo.push(&user_msg).await?;

        instance.updated_at = chrono::Utc::now();
        self.instance_repo.upsert(&instance).await?;

        let llm = self.get_llm(&req.llm_provider)?;

        let rag_context = self
            .get_rag_context(instance.profil_id, &req.message)
            .await?;

        let system_prompt = if wants_mutation {
            prompts::chat::INSTANCE_MUTATION_SYSTEM
        } else if wants_identity {
            prompts::chat::INSTANCE_IDENTITY_SYSTEM
        } else {
            prompts::chat::INSTANCE_DEFAULT_SYSTEM
        };

        let history_prompt = render_chat_history_for_prompt(&history);

        let mut user_input = vec![ports::MessageContent::Text(format!(
            "IDENTITÉ DE L'UTILISATEUR (Profil complet):\n{}\n\n\
            ANNEXES DISPONIBLES (en DB):\n{}\n\n\
            OFFRE CIBLÉE (fiche brute et structurée):\n{}\n\n\
            ANALYSE DE L'OFFRE CIBLÉE (Restitution):\n{}\n\n\
            FRAGMENTS DE PARCOURS (RAG):\n{}\n\n\
            HISTORIQUE RÉCENT DU CHAT:\n{}\n\n\
            DEMANDE DE L'UTILISATEUR: {}\n\n\
            JSON ACTUEL DU CV: {}\n\n\
            JSON ACTUEL DE LA LETTRE: {}",
            serde_json::to_string_pretty(&build_profile_prompt_context(&profil))?,
            serde_json::to_string_pretty(&self.annexe_repo.list_by_profil_id(profil.id).await?)?,
            serde_json::to_string_pretty(&build_offer_prompt_context(&offre))?,
            serde_json::to_string_pretty(&instance.restitution)?,
            rag_context,
            history_prompt,
            req.message,
            serde_json::to_string_pretty(&instance.resume_json)?,
            serde_json::to_string_pretty(&instance.cover_letter_json)?
        ))];

        for att in req.attachments {
            if att.content_type.starts_with("image/") {
                let b64 = att.data.split(',').nth(1).unwrap_or(&att.data);
                if let Ok(data) = base64::engine::general_purpose::STANDARD.decode(b64) {
                    user_input.push(ports::MessageContent::Image {
                        data,
                        content_type: att.content_type,
                    });
                }
            }
        }

        let response_json = self
            .call_llm_extract(
                llm,
                system_prompt,
                user_input,
                if wants_mutation {
                    ChatOutputKind::Mutation
                } else {
                    ChatOutputKind::MessageOnly
                },
            )
            .await?;

        let new_data: ChatMutationOutput = serde_json::from_value(response_json)?;

        let ai_message = if new_data.message.trim().is_empty() {
            "J'ai mis à jour les documents selon votre demande.".to_string()
        } else {
            new_data.message
        };

        if let Some(res) = new_data.resume {
            if !res.is_null() {
                instance.resume_json = Some(res);
                instance.status = domain::InstanceStatus::Ready;
            }
        }
        if let Some(cov) = new_data.cover {
            if !cov.is_null() {
                instance.cover_letter_json = Some(cov);
                instance.status = domain::InstanceStatus::Ready;
            }
        }

        let assistant_msg = Message::new(instance.id, MessageRole::Assistant, ai_message.clone());
        self.message_repo.push(&assistant_msg).await?;

        instance.updated_at = chrono::Utc::now();
        self.instance_repo.upsert(&instance).await?;

        Ok(ChatResponse {
            updated_instance: Some(instance),
            message: ai_message,
        })
    }

    async fn execute_global_chat(&self, req: ChatRequest) -> anyhow::Result<ChatResponse> {
        let mut profil = self
            .profil_repo
            .get_active()
            .await?
            .ok_or_else(|| anyhow::anyhow!("Aucun profil actif trouvé"))?;

        let chat_history = extract_chat_history(&profil.notes);
        info!(
            "Chat (Global): Historique extrait ({} entrées)",
            chat_history.len()
        );

        push_chat_history(&mut profil.notes, "user", &req.message);
        self.profil_repo.upsert(&profil).await?;

        let llm = self.get_llm(&req.llm_provider)?;

        let rag_context = self.get_rag_context(profil.id, &req.message).await?;

        let offres = self.offre_repo.list_all().await?;

        let system_prompt = prompts::chat::GLOBAL_CHAT_SYSTEM;

        let history_prompt = render_chat_history_for_prompt(&chat_history);

        let mut user_input = vec![ports::MessageContent::Text(format!(
            "IDENTITÉ DE L'UTILISATEUR (Profil complet):\n{}\n\n\
            ANNEXES DISPONIBLES (en DB):\n{}\n\n\
            OFFRES DISPONIBLES EN BASE ({} offres):\n{}\n\n\
            DÉTAILS DU PARCOURS (RAG):\n{}\n\n\
            HISTORIQUE RÉCENT DU CHAT:\n{}\n\n\
            DEMANDE DE L'UTILISATEUR: {}",
            serde_json::to_string_pretty(&build_profile_prompt_context(&profil))?,
            serde_json::to_string_pretty(&self.annexe_repo.list_by_profil_id(profil.id).await?)?,
            offres.len(),
            serde_json::to_string_pretty(
                &offres
                    .iter()
                    .map(|o| (o.slug.to_string(), o.intitule.clone(), o.entreprise.clone()))
                    .collect::<Vec<_>>()
            )?,
            rag_context,
            history_prompt,
            req.message
        ))];

        for att in req.attachments {
            if att.content_type.starts_with("image/") {
                let b64 = att.data.split(',').nth(1).unwrap_or(&att.data);
                if let Ok(data) = base64::engine::general_purpose::STANDARD.decode(b64) {
                    user_input.push(ports::MessageContent::Image {
                        data,
                        content_type: att.content_type,
                    });
                }
            }
        }

        let response_json = self
            .call_llm_extract(llm, system_prompt, user_input, ChatOutputKind::MessageOnly)
            .await?;

        let ai_message = response_json["message"]
            .as_str()
            .unwrap_or("Désolé, je n'ai pas pu générer de réponse.")
            .to_string();

        push_chat_history(&mut profil.notes, "assistant", &ai_message);
        self.profil_repo.upsert(&profil).await?;

        Ok(ChatResponse {
            updated_instance: None,
            message: ai_message,
        })
    }

    fn get_llm(&self, provider: &str) -> anyhow::Result<Arc<dyn LlmClient>> {
        self.llm_registry
            .get(provider)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("LLM '{}' non configuré", provider))
    }

    async fn get_rag_context(
        &self,
        profil_id: domain::ProfilId,
        message: &str,
    ) -> anyhow::Result<String> {
        let query_text = format!("{} career context", message);
        let embeddings = self
            .embedder
            .embed(&[&query_text], EmbedMode::Query)
            .await?;

        let query_vec = embeddings
            .first()
            .ok_or_else(|| anyhow::anyhow!("No embeddings returned"))?;

        let chunks = self
            .chunk_repo
            .top_k_by_embedding(profil_id, query_vec, 5)
            .await?;

        Ok(chunks
            .iter()
            .map(|(c, _)| format!("### {} - {}\n{}", c.kind.as_str(), c.titre, c.content))
            .collect::<Vec<_>>()
            .join("\n\n"))
    }

    async fn call_llm_extract(
        &self,
        llm: Arc<dyn LlmClient>,
        system: &str,
        input: Vec<ports::MessageContent>,
        kind: ChatOutputKind,
    ) -> anyhow::Result<serde_json::Value> {
        let extraction_req = ExtractionRequest {
            system: Some(system.to_string()),
            instruction: "RÉPONDS UNIQUEMENT AVEC DU JSON.".into(),
            input,
            schema_name: "ChatResponse".into(),
            schema_description: "Réponse du chat".into(),
            json_schema: if matches!(kind, ChatOutputKind::Mutation) {
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
}

#[cfg(test)]
mod tests;
