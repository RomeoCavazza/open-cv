use domain::{Instance, InstanceId, InstanceStatus, Slug};
use ports::{InstanceRepo, LlmClient, OffreRepo};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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
    pub llm_registry: std::collections::HashMap<String, Arc<dyn LlmClient>>,
}

impl ChatWithApplicationUseCase {
    pub fn new(
        instance_repo: Arc<dyn InstanceRepo>,
        llm_registry: std::collections::HashMap<String, Arc<dyn LlmClient>>,
    ) -> Self {
        Self {
            instance_repo,
            llm_registry,
        }
    }

    pub async fn execute(&self, req: ChatRequest) -> anyhow::Result<ChatResponse> {
        // 1. Récupérer l'instance actuelle
        let instance_uuid = uuid::Uuid::parse_str(&req.instance_id)?;
        let mut instance = self.instance_repo.get_by_id(domain::InstanceId::from_uuid(instance_uuid))
            .await?
            .ok_or_else(|| anyhow::anyhow!("Instance non trouvée"))?;

        // 2. Choisir le LLM
        let llm = self.llm_registry.get(&req.llm_provider)
            .or_else(|| self.llm_registry.get("ollama"))
            .ok_or_else(|| anyhow::anyhow!("LLM non configuré"))?;

        // 3. Construire le prompt de mutation
        let system_prompt = "Tu es un expert en recrutement. L'utilisateur veut modifier son CV ou sa lettre de motivation. Tu DOIS renvoyer le JSON complet mis à jour. NE RÉPONDS QUE PAR LE JSON, SANS TEXTE AVANT OU APRÈS.";

        let user_prompt = format!(
            "Message de l'utilisateur: {}\n\nJSON Actuel du CV: {}\n\nJSON Actuel de la Lettre: {}",
            req.message,
            serde_json::to_string_pretty(&instance.resume_json)?,
            serde_json::to_string_pretty(&instance.cover_letter_json)?
        );

        // 4. Appeler le LLM via la méthode complete
        let req = ports::CompletionRequest {
            system: Some(system_prompt.to_string()),
            messages: vec![ports::Message {
                role: ports::Role::User,
                content: user_prompt,
            }],
            model: None,
            max_tokens: Some(4000),
            temperature: Some(0.0),
        };

        let response = llm.complete(req).await?;
        let response_text = response.text;
        
        // 5. Parser la réponse (extraction du JSON si besoin)
        // On essaie de trouver un bloc JSON dans la réponse si elle contient du texte autour
        let json_str = if let Some(start) = response_text.find('{') {
            if let Some(end) = response_text.rfind('}') {
                &response_text[start..=end]
            } else { &response_text }
        } else { &response_text };

        if let Ok(new_data) = serde_json::from_str::<serde_json::Value>(json_str) {
             if let Some(resume) = new_data.get("resume") {
                 instance.resume_json = Some(resume.clone());
             }
             if let Some(cover) = new_data.get("cover") {
                 instance.cover_letter_json = Some(cover.clone());
             }
        }

        // 6. Sauvegarder
        self.instance_repo.upsert(&instance).await?;

        Ok(ChatResponse {
            updated_instance: instance,
        })
    }
}
