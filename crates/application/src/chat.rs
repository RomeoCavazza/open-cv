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

#[derive(Debug, Clone, Copy)]
enum ChatOutputKind {
    MessageOnly,
    Mutation,
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
    pub offre_repo: Arc<dyn ports::OffreRepo>,
    pub instance_repo: Arc<dyn InstanceRepo>,
    pub profil_repo: Arc<dyn ProfilRepo>,
    pub chunk_repo: Arc<dyn ChunkRepo>,
    pub embedder: Arc<dyn Embedder>,
    pub llm_registry: std::collections::HashMap<String, Arc<dyn LlmClient>>,
}

impl ChatWithApplicationUseCase {
    pub fn new(
        offre_repo: Arc<dyn ports::OffreRepo>,
        instance_repo: Arc<dyn InstanceRepo>,
        profil_repo: Arc<dyn ProfilRepo>,
        chunk_repo: Arc<dyn ChunkRepo>,
        embedder: Arc<dyn Embedder>,
        llm_registry: std::collections::HashMap<String, Arc<dyn LlmClient>>,
    ) -> Self {
        Self {
            offre_repo,
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

        // 1b. Récupérer le profil pour avoir l'identité
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

        let wants_mutation = Self::wants_mutation(&req.message);
        let wants_identity = Self::wants_identity(&req.message);

        let chat_history = Self::extract_chat_history(&instance.notes);
        info!("Chat (Instance): Historique extrait ({} entrées)", chat_history.len());

        // 1c. SAUVEGARDER LE MESSAGE UTILISATEUR IMMÉDIATEMENT
        Self::push_chat_history(&mut instance.notes, "user", &req.message);
        instance.updated_at = chrono::Utc::now();
        self.instance_repo.upsert(&instance).await?;

        // 2. Choisir le LLM
        let llm = self.get_llm(&req.llm_provider)?;

        // 3. RAG (Fragments spécifiques)
        let rag_context = self.get_rag_context(instance.profil_id, &req.message).await?;

        // 4. Construire le prompt selon l'intention
        let system_prompt = if wants_mutation {
            "Tu es un expert en recrutement et coach de carrière. \
            L'utilisateur veut modifier son CV ou sa lettre. \
            Tu as accès à 4 sources de données : \
            1. SON IDENTITÉ (Profil complet) \
            2. L'OFFRE CIBLÉE (Offre + restitution) \
            3. DES FRAGMENTS DE SON PARCOURS (RAG) \
            4. LES JSON ACTUELS DU CV ET DE LA LETTRE \
            \
            TU DOIS RÉPONDRE EXCLUSIVEMENT PAR UN OBJET JSON avec ces 3 clés : \
            1. 'resume' : le JSON complet du CV mis à jour (ou null si inchangé). \
            2. 'cover' : le JSON complet de la lettre mis à jour (ou null si inchangé). \
            3. 'message' : ton explication ou réponse à l'utilisateur. \
            \
            SI L'UTILISATEUR POSE UNE SIMPLE QUESTION (nom, offre, contexte, contenu actuel), NE MODIFIE RIEN ET METS resume/cover À null. \
            INTERDICTION DE METTRE DU TEXTE AVANT OU APRÈS LE JSON."
        } else if wants_identity {
            "Tu es un assistant de lecture factuelle. \
            L'utilisateur demande son identité. Réponds directement avec le prénom et le nom si ces informations sont présentes dans le profil. \
            N'ajoute aucune explication, aucune mention du CV, aucune mention de la lettre, aucun commentaire sur un document inchangé. \
            Si le nom n'est pas disponible, dis simplement que l'information n'est pas disponible. \
            TU DOIS RÉPONDRE EXCLUSIVEMENT PAR UN OBJET JSON avec une seule clé 'message'."
        } else {
            "Tu es un assistant de lecture factuelle pour une candidature. \
            Réponds à la question de l'utilisateur de manière directe, courte et naturelle, à partir du profil, de l'offre et de la restitution fournis. \
            Ne parle jamais de modification de CV ou de lettre si l'utilisateur ne demande pas explicitement de modification. \
            Ne commente pas les documents avec des formules comme 'maintenue inchangée', 'aucune modification n'a été apportée' ou 'cela correspond parfaitement' sauf si l'utilisateur parle explicitement d'édition. \
            Si un champ n'est pas présent dans les données, dis simplement que l'information n'est pas disponible. \
            N'invente jamais un nom, une offre, une expérience ou une modification. \
            TU DOIS RÉPONDRE EXCLUSIVEMENT PAR UN OBJET JSON avec une seule clé 'message'."
        };

        let history_prompt = Self::render_chat_history_for_prompt(&chat_history);

        let user_prompt = format!(
            "IDENTITÉ DE L'UTILISATEUR (Profil complet):\n{}\n\n\
            OFFRE CIBLÉE (fiche brute et structurée):\n{}\n\n\
            ANALYSE DE L'OFFRE CIBLÉE (Restitution):\n{}\n\n\
            FRAGMENTS DE PARCOURS (RAG):\n{}\n\n\
            HISTORIQUE RÉCENT DU CHAT:\n{}\n\n\
            DEMANDE DE L'UTILISATEUR: {}\n\n\
            JSON ACTUEL DU CV: {}\n\n\
            JSON ACTUEL DE LA LETTRE: {}",
            serde_json::to_string_pretty(&Self::build_profile_prompt_context(&profil))?,
            serde_json::to_string_pretty(&Self::build_offer_prompt_context(&offre))?,
            serde_json::to_string_pretty(&instance.restitution)?,
            rag_context,
            history_prompt,
            req.message,
            serde_json::to_string_pretty(&instance.resume_json)?,
            serde_json::to_string_pretty(&instance.cover_letter_json)?
        );

        // 5. Appeler le LLM
        let response_json = self
            .call_llm_extract(
                llm,
                system_prompt,
                user_prompt,
                if wants_mutation { ChatOutputKind::Mutation } else { ChatOutputKind::MessageOnly },
            )
            .await?;

        // 6. Analyser la réponse
        let new_data: ChatMutationOutput = serde_json::from_value(response_json)?;

        let ai_message = if new_data.message.trim().is_empty() {
            "J'ai mis à jour les documents selon votre demande.".to_string()
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
        let rag_context = self.get_rag_context(profil.id, &req.message).await?;

        // 4. Prompt Global
        let system_prompt = "Tu es un coach de carrière expert. Tu as accès au profil complet de l'utilisateur. \
            Réponds de manière constructive et encourageante. \
            TU DOIS RÉPONDRE EXCLUSIVEMENT PAR UN OBJET JSON avec une seule clé 'message'. \
            INTERDICTION DE METTRE DU TEXTE AVANT OU APRÈS LE JSON.";

        let history_prompt = Self::render_chat_history_for_prompt(&chat_history);

        let user_prompt = format!(
            "IDENTITÉ DE L'UTILISATEUR (Profil complet):\n{}\n\n\
            DÉTAILS DU PARCOURS (RAG):\n{}\n\n\
            HISTORIQUE RÉCENT DU CHAT:\n{}\n\n\
            DEMANDE DE L'UTILISATEUR: {}",
            serde_json::to_string_pretty(&Self::build_profile_prompt_context(&profil))?,
            rag_context,
            history_prompt,
            req.message
        );

        // 5. Appeler le LLM
        let response_json = self
            .call_llm_extract(llm, system_prompt, user_prompt, ChatOutputKind::MessageOnly)
            .await?;

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

    async fn call_llm_extract(
        &self,
        llm: Arc<dyn LlmClient>,
        system: &str,
        input: String,
        kind: ChatOutputKind,
    ) -> anyhow::Result<serde_json::Value> {
        let extraction_req = ExtractionRequest {
            system: Some(system.to_string()),
            instruction: "RÉPONDS UNIQUEMENT AVEC DU JSON.".into(),
            input,
            schema_name: "ChatResponse".into(),
            schema_description: "Réponse du chat".into(),
            json_schema: if matches!(kind, ChatOutputKind::Mutation) {
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

    fn build_profile_prompt_context(profil: &domain::Profil) -> serde_json::Value {
        serde_json::json!({
            "id": profil.id,
            "label": profil.label,
            "content": profil.content,
            "is_active": profil.is_active,
            "resume_template": profil.resume_template,
            "cover_letter_template": profil.cover_letter_template,
            "has_calendar_pdf": profil.calendar_pdf.is_some(),
            "notes": profil.notes,
            "created_at": profil.created_at,
        })
    }

    fn build_offer_prompt_context(offre: &domain::Offre) -> serde_json::Value {
        let raw_text_preview = if offre.raw_text.chars().count() > 4000 {
            offre.raw_text.chars().take(4000).collect::<String>() + "…"
        } else {
            offre.raw_text.clone()
        };

        serde_json::json!({
            "id": offre.id,
            "slug": offre.slug,
            "source_url": offre.source_url,
            "source_host": offre.source_host,
            "entreprise": offre.entreprise,
            "intitule": offre.intitule,
            "localisation": offre.localisation,
            "contrat": offre.contrat,
            "structured": offre.structured,
            "raw_text_preview": raw_text_preview,
            "scraped_at": offre.scraped_at,
            "last_seen_at": offre.last_seen_at,
            "closed_at": offre.closed_at,
            "categorie": offre.categorie,
        })
    }

    fn wants_mutation(message: &str) -> bool {
        let lowered = message.to_lowercase();
        let mutation_markers = [
            "modifie",
            "modifier",
            "change",
            "changer",
            "corrige",
            "corriger",
            "ajoute",
            "ajouter",
            "supprime",
            "enlève",
            "retire",
            "remplace",
            "mets",
            "mettre",
            "réécris",
            "reecris",
            "réécrire",
            "reecrire",
            "actualise",
            "actualiser",
            "adapte",
            "adapter",
            "réorganise",
            "reorganise",
            "modification",
            "édition",
            "edite",
            "éditer",
            "editer",
        ];

        mutation_markers.iter().any(|marker| lowered.contains(marker))
    }

    fn wants_identity(message: &str) -> bool {
        let lowered = message.to_lowercase();
        let identity_markers = [
            "comment je m'appelle",
            "c'est quoi mon nom",
            "quel est mon nom",
            "tu sais comment je m'appelle",
            "tu connais mon nom",
            "je m'appelle comment",
        ];

        identity_markers.iter().any(|marker| lowered.contains(marker))
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

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use chrono::Utc;
    use domain::{Chunk, ChunkKind, Instance, InstanceId, InstanceStatus, Offre, OffreId, OffreStructured, Profil, ProfilId, Slug};
    use ports::{ChunkRepo, EmbedError, EmbedMode, Embedder, ExtractionRequest, InstanceRepo, LlmClient, LlmError, OffreRepo, ProfilRepo, RepoError};
    use serde_json::json;
    use std::sync::{Arc, Mutex};

    struct TestStores {
        instance: Mutex<Instance>,
        profil: Mutex<Profil>,
        offre: Mutex<Offre>,
        requests: Mutex<Vec<ExtractionRequest>>,
    }

    impl TestStores {
        fn new(instance: Instance, profil: Profil, offre: Offre) -> Self {
            Self {
                instance: Mutex::new(instance),
                profil: Mutex::new(profil),
                offre: Mutex::new(offre),
                requests: Mutex::new(Vec::new()),
            }
        }
    }

    struct TestInstanceRepo {
        stores: Arc<TestStores>,
    }

    struct TestProfilRepo {
        stores: Arc<TestStores>,
    }

    struct TestOffreRepo {
        stores: Arc<TestStores>,
    }

    struct TestChunkRepo;

    struct TestEmbedder;

    struct RecordingLlm {
        stores: Arc<TestStores>,
    }

    #[async_trait]
    impl InstanceRepo for TestInstanceRepo {
        async fn get_by_id(&self, id: InstanceId) -> Result<Option<Instance>, RepoError> {
            let instance = self.stores.instance.lock().unwrap().clone();
            Ok((instance.id == id).then_some(instance))
        }

        async fn get_by_slug(&self, slug: &Slug) -> Result<Option<Instance>, RepoError> {
            let instance = self.stores.instance.lock().unwrap().clone();
            Ok((instance.slug.as_str() == slug.as_str()).then_some(instance))
        }

        async fn list_recent(&self, _limit: u32) -> Result<Vec<Instance>, RepoError> {
            Ok(vec![self.stores.instance.lock().unwrap().clone()])
        }

        async fn upsert(&self, instance: &Instance) -> Result<(), RepoError> {
            *self.stores.instance.lock().unwrap() = instance.clone();
            Ok(())
        }

        async fn get_by_offre_id(&self, offre_id: OffreId) -> Result<Option<Instance>, RepoError> {
            let instance = self.stores.instance.lock().unwrap().clone();
            Ok((instance.offre_id == offre_id).then_some(instance))
        }
    }

    #[async_trait]
    impl ProfilRepo for TestProfilRepo {
        async fn get_active(&self) -> Result<Option<Profil>, RepoError> {
            let profil = self.stores.profil.lock().unwrap().clone();
            Ok(profil.is_active.then_some(profil))
        }

        async fn get_by_id(&self, id: ProfilId) -> Result<Option<Profil>, RepoError> {
            let profil = self.stores.profil.lock().unwrap().clone();
            Ok((profil.id == id).then_some(profil))
        }

        async fn list_all(&self) -> Result<Vec<Profil>, RepoError> {
            Ok(vec![self.stores.profil.lock().unwrap().clone()])
        }

        async fn upsert(&self, profil: &Profil) -> Result<(), RepoError> {
            *self.stores.profil.lock().unwrap() = profil.clone();
            Ok(())
        }
    }

    #[async_trait]
    impl OffreRepo for TestOffreRepo {
        async fn get_by_id(&self, id: OffreId) -> Result<Option<Offre>, RepoError> {
            let offre = self.stores.offre.lock().unwrap().clone();
            Ok((offre.id == id).then_some(offre))
        }

        async fn get_by_slug(&self, slug: &Slug) -> Result<Option<Offre>, RepoError> {
            let offre = self.stores.offre.lock().unwrap().clone();
            Ok((offre.slug.as_str() == slug.as_str()).then_some(offre))
        }

        async fn list_recent(&self, _limit: u32) -> Result<Vec<Offre>, RepoError> {
            Ok(vec![self.stores.offre.lock().unwrap().clone()])
        }

        async fn upsert(&self, offre: &Offre) -> Result<(), RepoError> {
            *self.stores.offre.lock().unwrap() = offre.clone();
            Ok(())
        }

        async fn count(&self) -> Result<u64, RepoError> {
            Ok(1)
        }

        async fn find_by_content_hash(
            &self,
            _source_host: &str,
            _hash: &[u8],
        ) -> Result<Option<Offre>, RepoError> {
            Ok(None)
        }
    }

    #[async_trait]
    impl ChunkRepo for TestChunkRepo {
        async fn upsert(&self, _chunk: &Chunk) -> Result<(), RepoError> {
            Ok(())
        }

        async fn top_k_by_embedding(
            &self,
            profil_id: ProfilId,
            _query_embedding: &[f32],
            _k: u32,
        ) -> Result<Vec<(Chunk, f32)>, RepoError> {
            Ok(vec![(
                Chunk {
                    id: domain::ChunkId::new(),
                    profil_id,
                    kind: ChunkKind::Experience,
                    titre: "Expérience test".into(),
                    content: "Travail sur un prototype IA".into(),
                    metadata: json!({"source": "test"}),
                    embedding: vec![0.0, 0.0],
                    created_at: Utc::now(),
                },
                0.99,
            )])
        }
    }

    #[async_trait]
    impl Embedder for TestEmbedder {
        async fn embed(&self, texts: &[&str], _mode: EmbedMode) -> Result<Vec<Vec<f32>>, EmbedError> {
            Ok(texts.iter().map(|_| vec![0.1, 0.2]).collect())
        }

        fn dimension(&self) -> usize {
            2
        }

        fn name(&self) -> &'static str {
            "test-embedder"
        }
    }

    #[async_trait]
    impl LlmClient for RecordingLlm {
        async fn complete(&self, _req: ports::CompletionRequest) -> Result<ports::CompletionResponse, LlmError> {
            Err(LlmError::Other("not used".into()))
        }

        async fn extract(&self, req: ExtractionRequest) -> Result<serde_json::Value, LlmError> {
            self.stores.requests.lock().unwrap().push(req.clone());
            let schema_text = req.json_schema.to_string();

            if schema_text.contains("\"resume\"") {
                Ok(json!({
                    "resume": {"updated": true},
                    "cover": {"updated": true},
                    "message": "mise à jour appliquée"
                }))
            } else {
                Ok(json!({
                    "message": "lecture factuelle"
                }))
            }
        }

        fn name(&self) -> &'static str {
            "recording-llm"
        }
    }

    fn build_test_data() -> (Instance, Profil, Offre) {
        let profil_id = ProfilId::new();
        let offre_id = OffreId::new();
        let instance_id = InstanceId::new();

        let profil = Profil {
            id: profil_id,
            label: "test-profile".into(),
            content: json!({
                "firstname": "Romeo",
                "lastname": "Cavazza",
                "title": "Alternance IA"
            }),
            is_active: true,
            resume_template: Some(json!({"template": "cv"})),
            cover_letter_template: Some(json!({"template": "lettre"})),
            calendar_pdf: None,
            notes: json!({}),
            created_at: Utc::now(),
        };

        let offre = Offre {
            id: offre_id,
            slug: Slug::parse("test_offre").unwrap(),
            source_url: "https://example.com/offre".into(),
            source_host: "example.com".into(),
            source_hash: vec![1, 2, 3],
            entreprise: "Test SA".into(),
            intitule: "Alternance IA".into(),
            localisation: Some("Paris".into()),
            contrat: Some("alternance".into()),
            raw_text: "Développer des outils IA".into(),
            structured: OffreStructured {
                resume_court: "Concevoir des outils IA".into(),
                stack: vec!["Rust".into(), "Python".into()],
                missions: vec!["Construire un assistant".into()],
                exigences: vec!["IA".into()],
                soft_skills: vec!["communication".into()],
                niveau_etudes: Some("Bac+5".into()),
                type_contrat: Some("alternance".into()),
                mots_cles: vec!["LLM".into()],
            },
            scraped_at: Utc::now(),
            last_seen_at: Utc::now(),
            closed_at: None,
            categorie: Some("ia".into()),
        };

        let instance = Instance {
            id: instance_id,
            slug: Slug::parse("test_instance").unwrap(),
            offre_id,
            profil_id,
            status: InstanceStatus::Draft,
            restitution: Some(json!({"summary": "restitution"})),
            resume_json: Some(json!({"current": "resume"})),
            cover_letter_json: Some(json!({"current": "cover"})),
            notes: json!({}),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            sent_at: None,
        };

        (instance, profil, offre)
    }

    fn build_usecase(stores: Arc<TestStores>) -> ChatWithApplicationUseCase {
        ChatWithApplicationUseCase::new(
            Arc::new(TestOffreRepo { stores: stores.clone() }),
            Arc::new(TestInstanceRepo { stores: stores.clone() }),
            Arc::new(TestProfilRepo { stores: stores.clone() }),
            Arc::new(TestChunkRepo),
            Arc::new(TestEmbedder),
            std::iter::once(("ollama".to_string(), Arc::new(RecordingLlm { stores }) as Arc<dyn LlmClient>)).collect(),
        )
    }

    #[test]
    fn detects_read_only_questions() {
        assert!(!ChatWithApplicationUseCase::wants_mutation("comment je m'appelle ? c'est quoi l'offre ?"));
        assert!(!ChatWithApplicationUseCase::wants_mutation("tu vois l'offre, mon cv et ma lettre de motivation ?"));
    }

    #[test]
    fn detects_mutation_requests() {
        assert!(ChatWithApplicationUseCase::wants_mutation("modifie mon CV pour mieux mettre mon titre"));
        assert!(ChatWithApplicationUseCase::wants_mutation("ajoute une expérience dans la lettre"));
    }

    #[tokio::test]
    async fn instance_chat_keeps_read_only_questions_in_message_mode() {
        let (instance, profil, offre) = build_test_data();
        let stores = Arc::new(TestStores::new(instance, profil, offre));
        let usecase = build_usecase(stores.clone());

        let response = usecase
            .execute(ChatRequest {
                message: "comment je m'appelle ? c'est quoi l'offre ?".into(),
                instance_id: Some(stores.instance.lock().unwrap().id.to_string()),
                llm_provider: "ollama".into(),
            })
            .await
            .expect("chat should succeed");

        assert_eq!(response.message, "lecture factuelle");
        assert!(response.updated_instance.is_some());

        let requests = stores.requests.lock().unwrap();
        assert_eq!(requests.len(), 1);
        let schema_text = requests[0].json_schema.to_string();
        assert!(!schema_text.contains("\"resume\""));
        assert!(!schema_text.contains("\"cover\""));

        let instance_after = stores.instance.lock().unwrap().clone();
        assert_eq!(instance_after.resume_json, Some(json!({"current": "resume"})));
        assert_eq!(instance_after.cover_letter_json, Some(json!({"current": "cover"})));
        assert_eq!(instance_after.notes["chat_history"].as_array().map(|v| v.len()), Some(2));
    }

    #[tokio::test]
    async fn instance_chat_updates_documents_for_explicit_mutations() {
        let (instance, profil, offre) = build_test_data();
        let stores = Arc::new(TestStores::new(instance, profil, offre));
        let usecase = build_usecase(stores.clone());

        let response = usecase
            .execute(ChatRequest {
                message: "modifie mon CV et ma lettre pour mettre en avant Rust".into(),
                instance_id: Some(stores.instance.lock().unwrap().id.to_string()),
                llm_provider: "ollama".into(),
            })
            .await
            .expect("chat should succeed");

        assert_eq!(response.message, "mise à jour appliquée");

        let requests = stores.requests.lock().unwrap();
        assert_eq!(requests.len(), 1);
        let schema_text = requests[0].json_schema.to_string();
        assert!(schema_text.contains("\"resume\""));
        assert!(schema_text.contains("\"cover\""));

        let instance_after = stores.instance.lock().unwrap().clone();
        assert_eq!(instance_after.resume_json, Some(json!({"updated": true})));
        assert_eq!(instance_after.cover_letter_json, Some(json!({"updated": true})));
        assert_eq!(instance_after.status, InstanceStatus::Ready);
    }
}
