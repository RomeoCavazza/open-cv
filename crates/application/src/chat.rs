use domain::Instance;
use ports::{ChunkRepo, EmbedMode, Embedder, ExtractionRequest, InstanceRepo, LlmClient, ProfilRepo};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;

const MAX_CHAT_HISTORY_ENTRIES: usize = 20;

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    pub instance_id: Option<String>,
    pub llm_provider: String,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub updated_instance: Option<Instance>,
    pub message: String,
}

#[derive(Debug, Clone)]
struct ChatHistoryEntry {
    role: String,
    content: String,
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
    pub instance_repo: Arc<dyn InstanceRepo>,
    pub profil_repo: Arc<dyn ProfilRepo>,
    pub chunk_repo: Arc<dyn ChunkRepo>,
    pub embedder: Arc<dyn Embedder>,
    pub llm_registry: std::collections::HashMap<String, Arc<dyn LlmClient>>,
}

impl ChatWithApplicationUseCase {
    pub fn new(
        instance_repo: Arc<dyn InstanceRepo>,
        profil_repo: Arc<dyn ProfilRepo>,
        chunk_repo: Arc<dyn ChunkRepo>,
        embedder: Arc<dyn Embedder>,
        llm_registry: std::collections::HashMap<String, Arc<dyn LlmClient>>,
    ) -> Self {
        Self {
            instance_repo,
            profil_repo,
            chunk_repo,
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

    async fn execute_instance_chat(&self, instance_id: &str, req: ChatRequest) -> anyhow::Result<ChatResponse> {
        // 1. Récupérer l'instance actuelle
        let instance_uuid = uuid::Uuid::parse_str(instance_id)?;
        let mut instance = self
            .instance_repo
            .get_by_id(domain::InstanceId::from_uuid(instance_uuid))
            .await?
            .ok_or_else(|| anyhow::anyhow!("Instance non trouvée"))?;

        let chat_history = Self::extract_chat_history(&instance.notes);
        info!("Chat (Instance): Historique extrait ({} entrées)", chat_history.len());

        // 1b. SAUVEGARDER LE MESSAGE UTILISATEUR IMMÉDIATEMENT
        Self::push_chat_history(&mut instance.notes, "user", &req.message);
        instance.updated_at = chrono::Utc::now();
        self.instance_repo.upsert(&instance).await?;

        // 2. Choisir le LLM
        let llm = self.get_llm(&req.llm_provider)?;

        // 3. RAG
        let context = self.get_rag_context(instance.profil_id, &req.message).await?;

        // 4. Construire le prompt de mutation
        let system_prompt = "Tu es un expert en recrutement. L'utilisateur veut modifier son CV ou sa lettre de motivation. \
            Tu as accès à son profil complet via le contexte RAG fourni. \
            TU DOIS RÉPONDRE EXCLUSIVEMENT PAR UN OBJET JSON avec ces 3 clés : \
            1. 'resume' : le JSON complet du CV mis à jour (ou null si inchangé). \
            2. 'cover' : le JSON complet de la lettre mis à jour (ou null si inchangé). \
            3. 'message' : une explication détaillée et personnelle de ce que tu as modifié. \
            INTERDICTION DE METTRE DU TEXTE AVANT OU APRÈS LE JSON. SI TU NE MODIFIES PAS UN DOCUMENT, METS null.";

        let history_prompt = Self::render_chat_history_for_prompt(&chat_history);

        let user_prompt = format!(
            "CONTEXTE DU PROFIL (RAG):\n{}\n\n\
            HISTORIQUE RÉCENT DU CHAT:\n{}\n\n\
            DEMANDE DE L'UTILISATEUR: {}\n\n\
            JSON ACTUEL DU CV: {}\n\n\
            JSON ACTUEL DE LA LETTRE: {}",
            context,
            history_prompt,
            req.message,
            serde_json::to_string_pretty(&instance.resume_json)?,
            serde_json::to_string_pretty(&instance.cover_letter_json)?
        );

        // 5. Appeler le LLM
        let response_json = self.call_llm_extract(llm, system_prompt, user_prompt).await?;

        // 6. Analyser la réponse
        let new_data: ChatMutationOutput = serde_json::from_value(response_json)?;

        let ai_message = if new_data.message.trim().is_empty() {
            "J'ai mis à jour ton CV et ta lettre pour coller à la demande.".to_string()
        } else {
            new_data.message
        };

        // Mettre à jour les livrables
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

        // 7. Sauvegarder l'historique et persister
        Self::push_chat_history(&mut instance.notes, "assistant", &ai_message);
        instance.updated_at = chrono::Utc::now();
        self.instance_repo.upsert(&instance).await?;

        Ok(ChatResponse {
            updated_instance: Some(instance),
            message: ai_message,
        })
    }

    async fn execute_global_chat(&self, req: ChatRequest) -> anyhow::Result<ChatResponse> {
        // 1. Récupérer le profil actif
        let mut profil = self
            .profil_repo
            .get_active()
            .await?
            .ok_or_else(|| anyhow::anyhow!("Aucun profil actif trouvé"))?;

        let chat_history = Self::extract_chat_history(&profil.notes);
        info!("Chat (Global): Historique extrait ({} entrées)", chat_history.len());

        // 1b. Sauvegarder le message utilisateur
        Self::push_chat_history(&mut profil.notes, "user", &req.message);
        self.profil_repo.upsert(&profil).await?;

        // 2. Choisir le LLM
        let llm = self.get_llm(&req.llm_provider)?;

        // 3. RAG
        let context = self.get_rag_context(profil.id, &req.message).await?;

        // 4. Prompt Global
        let system_prompt = "Tu es un coach de carrière expert. Tu aides l'utilisateur dans sa recherche d'emploi. \
            Tu as accès à son profil complet. Réponds de manière constructive et encourageante. \
            TU DOIS RÉPONDRE EXCLUSIVEMENT PAR UN OBJET JSON avec une seule clé 'message'. \
            INTERDICTION DE METTRE DU TEXTE AVANT OU APRÈS LE JSON.";

        let history_prompt = Self::render_chat_history_for_prompt(&chat_history);

        let user_prompt = format!(
            "CONTEXTE DU PROFIL (RAG):\n{}\n\n\
            HISTORIQUE RÉCENT DU CHAT:\n{}\n\n\
            DEMANDE DE L'UTILISATEUR: {}",
            context,
            history_prompt,
            req.message
        );

        // 5. Appeler le LLM
        let response_json = self.call_llm_extract(llm, system_prompt, user_prompt).await?;

        // 6. Analyser
        let ai_message = response_json["message"].as_str()
            .unwrap_or("Désolé, je n'ai pas pu générer de réponse.")
            .to_string();

        // 7. Sauvegarder l'historique
        Self::push_chat_history(&mut profil.notes, "assistant", &ai_message);
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

    async fn get_rag_context(&self, profil_id: domain::ProfilId, message: &str) -> anyhow::Result<String> {
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

    async fn call_llm_extract(&self, llm: Arc<dyn LlmClient>, system: &str, input: String) -> anyhow::Result<serde_json::Value> {
        let extraction_req = ExtractionRequest {
            system: Some(system.to_string()),
            instruction: "RÉPONDS UNIQUEMENT AVEC DU JSON.".into(),
            input,
            schema_name: "ChatResponse".into(),
            schema_description: "Réponse du chat".into(),
            json_schema: if system.contains("resume") {
                serde_json::to_value(schemars::schema_for!(ChatMutationOutput)).unwrap()
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

        llm.extract(extraction_req).await.map_err(|e| anyhow::anyhow!(e))
    }

    fn extract_chat_history(notes: &serde_json::Value) -> Vec<ChatHistoryEntry> {
        notes
            .get("chat_history")
            .and_then(|v| v.as_array())
            .map(|entries| {
                entries
                    .iter()
                    .filter_map(|entry| {
                        let role = entry.get("role")?.as_str()?.to_string();
                        let content = entry.get("content")?.as_str()?.to_string();
                        Some(ChatHistoryEntry { role, content })
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    fn render_chat_history_for_prompt(history: &[ChatHistoryEntry]) -> String {
        if history.is_empty() {
            return "Aucun historique".to_string();
        }

        history
            .iter()
            .rev()
            .take(12)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .map(|entry| {
                let label = if entry.role == "assistant" { "IA" } else { "UTILISATEUR" };
                format!("{label}: {}", entry.content)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn push_chat_history(notes: &mut serde_json::Value, role: &str, content: &str) {
        info!("Chat: Pushing history for role: {}", role);
        
        // Initialiser si ce n'est pas un objet
        if !notes.is_object() {
            *notes = serde_json::json!({});
        }

        let obj = notes.as_object_mut().unwrap();
        
        // Récupérer ou créer l'array chat_history
        let history = obj
            .entry("chat_history")
            .or_insert_with(|| serde_json::json!([]))
            .as_array_mut()
            .expect("chat_history should be an array");

        history.push(serde_json::json!({
            "role": role,
            "content": content,
            "ts": chrono::Utc::now().to_rfc3339(),
        }));

        // Limiter la taille
        if history.len() > MAX_CHAT_HISTORY_ENTRIES {
            let excess = history.len() - MAX_CHAT_HISTORY_ENTRIES;
            history.drain(0..excess);
        }
        
        info!("Chat: History size now: {} entries", history.len());
    }
}
