use domain::Instance;
use ports::{ChunkRepo, EmbedMode, Embedder, InstanceRepo, LlmClient};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, warn};

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    pub instance_id: String,
    pub llm_provider: String,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub updated_instance: Instance,
}

pub struct ChatWithApplicationUseCase {
    pub instance_repo: Arc<dyn InstanceRepo>,
    pub chunk_repo: Arc<dyn ChunkRepo>,
    pub embedder: Arc<dyn Embedder>,
    pub llm_registry: std::collections::HashMap<String, Arc<dyn LlmClient>>,
}

impl ChatWithApplicationUseCase {
    pub fn new(
        instance_repo: Arc<dyn InstanceRepo>,
        chunk_repo: Arc<dyn ChunkRepo>,
        embedder: Arc<dyn Embedder>,
        llm_registry: std::collections::HashMap<String, Arc<dyn LlmClient>>,
    ) -> Self {
        Self {
            instance_repo,
            chunk_repo,
            embedder,
            llm_registry,
        }
    }

    pub async fn execute(&self, req: ChatRequest) -> anyhow::Result<ChatResponse> {
        // 1. Récupérer l'instance actuelle
        let instance_uuid = uuid::Uuid::parse_str(&req.instance_id)?;
        let mut instance = self
            .instance_repo
            .get_by_id(domain::InstanceId::from_uuid(instance_uuid))
            .await?
            .ok_or_else(|| anyhow::anyhow!("Instance non trouvée"))?;

        // 2. Choisir le LLM
        let llm = self
            .llm_registry
            .get(&req.llm_provider)
            .or_else(|| self.llm_registry.get("ollama"))
            .ok_or_else(|| anyhow::anyhow!("LLM non configuré"))?;

        // 3. RAG : Récupérer des chunks pertinents basés sur le message de l'utilisateur
        let query_text = format!("{} context for job application", req.message);
        let embeddings = self
            .embedder
            .embed(&[&query_text], EmbedMode::Query)
            .await
            .map_err(|e| anyhow::anyhow!("Embedding failed: {}", e))?;

        let query_vec = embeddings
            .first()
            .ok_or_else(|| anyhow::anyhow!("No embeddings returned"))?;

        let chunks = self
            .chunk_repo
            .top_k_by_embedding(instance.profil_id, query_vec, 5)
            .await?;
        let context = chunks
            .iter()
            .map(|(c, _)| format!("### {} - {}\n{}", c.kind.as_str(), c.titre, c.content))
            .collect::<Vec<_>>()
            .join("\n\n");

        info!(
            "RAG: {} chunks de contexte trouvés pour le chat",
            chunks.len()
        );

        // 4. Construire le prompt de mutation
        let system_prompt = "Tu es un expert en recrutement. L'utilisateur veut modifier son CV ou sa lettre de motivation. \
            Tu as accès à son profil complet via le contexte RAG fourni ci-dessous. \
            Tu DOIS renvoyer le JSON complet mis à jour. \
            CONSIGNE : Si l'utilisateur demande une info présente dans le contexte, utilise-la. \
            RÈGLE : NE RÉPONDS QUE PAR LE JSON, SANS TEXTE AVANT OU APRÈS.";

        let user_prompt = format!(
            "CONTEXTE DU PROFIL (RAG):\n{}\n\n\
            DEMANDE DE L'UTILISATEUR: {}\n\n\
            JSON ACTUEL DU CV: {}\n\n\
            JSON ACTUEL DE LA LETTRE: {}",
            context,
            req.message,
            serde_json::to_string_pretty(&instance.resume_json)?,
            serde_json::to_string_pretty(&instance.cover_letter_json)?
        );

        // 5. Appeler le LLM
        let completion_req = ports::CompletionRequest {
            system: Some(system_prompt.to_string()),
            messages: vec![ports::Message {
                role: ports::Role::User,
                content: user_prompt,
            }],
            model: None,
            max_tokens: Some(4000),
            temperature: Some(0.0),
        };

        let response = llm.complete(completion_req).await?;
        let response_text = response.text;

        // 6. Parser la réponse
        let json_str = if let Some(start) = response_text.find('{') {
            if let Some(end) = response_text.rfind('}') {
                &response_text[start..=end]
            } else {
                &response_text
            }
        } else {
            &response_text
        };

        if let Ok(new_data) = serde_json::from_str::<serde_json::Value>(json_str) {
            if let Some(resume) = new_data.get("resume") {
                instance.resume_json = Some(resume.clone());
            }
            if let Some(cover) = new_data.get("cover") {
                instance.cover_letter_json = Some(cover.clone());
            }
        } else {
            warn!(
                "Chat LLM n'a pas renvoyé un JSON valide : {}",
                response_text
            );
        }

        // 7. Sauvegarder
        instance.updated_at = chrono::Utc::now();
        self.instance_repo.upsert(&instance).await?;

        Ok(ChatResponse {
            updated_instance: instance,
        })
    }
}
