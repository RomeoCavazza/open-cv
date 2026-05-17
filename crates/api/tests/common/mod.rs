use async_trait::async_trait;
use domain::{
    Annexe, AnnexeId, Chunk, Instance, InstanceId, Message, Offre, OffreId, Profil, ProfilId, Slug,
};
use ports::{
    AnnexeRepo, ChunkRepo, CompletionRequest, CompletionResponse, EmbedError, EmbedMode, Embedder,
    ExtractionRequest, InstanceRepo, LlmClient, LlmError, MessageRepo, OffreRepo, ProfilRepo,
    RepoError, ScrapeError, ScrapeResult, Scraper, StopReason, StreamChunk,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct MockRepos {
    pub profils: Arc<Mutex<HashMap<ProfilId, Profil>>>,
    pub offres: Arc<Mutex<HashMap<OffreId, Offre>>>,
    pub instances: Arc<Mutex<HashMap<InstanceId, Instance>>>,
}

impl MockRepos {
    pub fn new() -> Self {
        Self {
            profils: Arc::new(Mutex::new(HashMap::new())),
            offres: Arc::new(Mutex::new(HashMap::new())),
            instances: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl ProfilRepo for MockRepos {
    async fn get_active(&self) -> Result<Option<Profil>, RepoError> {
        Ok(self
            .profils
            .lock()
            .unwrap()
            .values()
            .find(|p| p.is_active)
            .cloned())
    }
    async fn get_by_id(&self, id: ProfilId) -> Result<Option<Profil>, RepoError> {
        Ok(self.profils.lock().unwrap().get(&id).cloned())
    }
    async fn list_all(&self) -> Result<Vec<Profil>, RepoError> {
        Ok(self.profils.lock().unwrap().values().cloned().collect())
    }
    async fn upsert(&self, profil: &Profil) -> Result<(), RepoError> {
        self.profils
            .lock()
            .unwrap()
            .insert(profil.id, profil.clone());
        Ok(())
    }
}

#[async_trait]
impl OffreRepo for MockRepos {
    async fn get_by_id(&self, id: OffreId) -> Result<Option<Offre>, RepoError> {
        Ok(self.offres.lock().unwrap().get(&id).cloned())
    }
    async fn get_by_slug(&self, slug: &Slug) -> Result<Option<Offre>, RepoError> {
        Ok(self
            .offres
            .lock()
            .unwrap()
            .values()
            .find(|o| &o.slug == slug)
            .cloned())
    }
    async fn list_all(&self) -> Result<Vec<Offre>, RepoError> {
        Ok(self.offres.lock().unwrap().values().cloned().collect())
    }
    async fn list_recent(&self, limit: u32) -> Result<Vec<Offre>, RepoError> {
        let _ = limit;
        Ok(self.offres.lock().unwrap().values().cloned().collect())
    }
    async fn upsert(&self, offre: &Offre) -> Result<(), RepoError> {
        self.offres.lock().unwrap().insert(offre.id, offre.clone());
        Ok(())
    }
    async fn count(&self) -> Result<u64, RepoError> {
        Ok(self.offres.lock().unwrap().len() as u64)
    }
    async fn find_by_url(&self, _url: &str) -> Result<Option<Offre>, RepoError> {
        Ok(None)
    }
    async fn find_by_content_hash(
        &self,
        source_host: &str,
        hash: &[u8],
    ) -> Result<Option<Offre>, RepoError> {
        let _ = (source_host, hash);
        Ok(None)
    }
}

#[async_trait]
impl InstanceRepo for MockRepos {
    async fn get_by_id(&self, id: InstanceId) -> Result<Option<Instance>, RepoError> {
        Ok(self.instances.lock().unwrap().get(&id).cloned())
    }
    async fn get_by_slug(&self, slug: &Slug) -> Result<Option<Instance>, RepoError> {
        Ok(self
            .instances
            .lock()
            .unwrap()
            .values()
            .find(|i| &i.slug == slug)
            .cloned())
    }
    async fn list_recent(&self, limit: u32) -> Result<Vec<Instance>, RepoError> {
        let _ = limit;
        Ok(self.instances.lock().unwrap().values().cloned().collect())
    }
    async fn upsert(&self, instance: &Instance) -> Result<(), RepoError> {
        self.instances
            .lock()
            .unwrap()
            .insert(instance.id, instance.clone());
        Ok(())
    }
    async fn get_by_offre_id(&self, offre_id: OffreId) -> Result<Option<Instance>, RepoError> {
        let _ = offre_id;
        Ok(None)
    }
    async fn get_by_offre_and_profil(
        &self,
        offre_id: domain::OffreId,
        profil_id: domain::ProfilId,
    ) -> Result<Option<Instance>, RepoError> {
        let _ = (offre_id, profil_id);
        Ok(None)
    }

    async fn update_livrables(
        &self,
        id: InstanceId,
        restitution: Option<domain::Restitution>,
        resume_json: Option<domain::Resume>,
        cover_letter_json: Option<domain::CoverLetter>,
        status: domain::InstanceStatus,
        _updated_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), RepoError> {
        let mut instances = self.instances.lock().unwrap();
        if let Some(instance) = instances.get_mut(&id) {
            if let Some(r) = restitution {
                instance.restitution = Some(r);
            }
            if let Some(r) = resume_json {
                instance.resume_json = Some(r);
            }
            if let Some(c) = cover_letter_json {
                instance.cover_letter_json = Some(c);
            }
            instance.status = status;
        }
        Ok(())
    }
}

#[async_trait]
impl ChunkRepo for MockRepos {
    async fn upsert(&self, chunk: &Chunk) -> Result<(), RepoError> {
        let _ = chunk;
        Ok(())
    }
    async fn top_k_by_embedding(
        &self,
        profil_id: ProfilId,
        query_embedding: &[f32],
        k: u32,
    ) -> Result<Vec<(Chunk, f32)>, RepoError> {
        let _ = (query_embedding, k);
        Ok(vec![(
            Chunk {
                id: domain::ChunkId::new(),
                profil_id,
                kind: domain::ChunkKind::Experience,
                titre: "Dummy Experience".to_string(),
                content: "Experience dummy".to_string(),
                metadata: domain::JsonValue::Object(Default::default()),
                embedding: vec![0.0; 1024],
                created_at: chrono::Utc::now(),
            },
            1.0,
        )])
    }
}

#[async_trait]
impl AnnexeRepo for MockRepos {
    async fn get_by_id(&self, id: AnnexeId) -> Result<Option<Annexe>, RepoError> {
        let _ = id;
        Ok(None)
    }
    async fn list_by_profil_id(&self, profil_id: ProfilId) -> Result<Vec<Annexe>, RepoError> {
        let _ = profil_id;
        Ok(vec![])
    }
    async fn upsert(&self, annexe: &Annexe) -> Result<(), RepoError> {
        let _ = annexe;
        Ok(())
    }
    async fn delete(&self, id: AnnexeId) -> Result<(), RepoError> {
        let _ = id;
        Ok(())
    }
}

#[async_trait]
impl MessageRepo for MockRepos {
    async fn list_by_instance_id(
        &self,
        instance_id: InstanceId,
    ) -> Result<Vec<Message>, RepoError> {
        let _ = instance_id;
        Ok(vec![])
    }
    async fn list_by_profil_id(&self, profil_id: ProfilId) -> Result<Vec<Message>, RepoError> {
        let _ = profil_id;
        Ok(vec![])
    }
    async fn push(&self, message: &Message) -> Result<(), RepoError> {
        let _ = message;
        Ok(())
    }
    async fn delete_all_for_instance(&self, instance_id: InstanceId) -> Result<(), RepoError> {
        let _ = instance_id;
        Ok(())
    }
}

#[async_trait]
impl ports::SnapshotRepo for MockRepos {
    async fn save(&self, snapshot: &domain::InstanceSnapshot) -> Result<(), RepoError> {
        let _ = snapshot;
        Ok(())
    }
    async fn get_latest(
        &self,
        instance_id: InstanceId,
    ) -> Result<Option<domain::InstanceSnapshot>, RepoError> {
        let _ = instance_id;
        Ok(None)
    }
    async fn list_by_instance(
        &self,
        instance_id: InstanceId,
    ) -> Result<Vec<domain::InstanceSnapshot>, RepoError> {
        let _ = instance_id;
        Ok(vec![])
    }
    async fn get_by_version(
        &self,
        instance_id: InstanceId,
        version: i32,
    ) -> Result<Option<domain::InstanceSnapshot>, RepoError> {
        let _ = (instance_id, version);
        Ok(None)
    }
    async fn count_by_instance(&self, instance_id: InstanceId) -> Result<i32, RepoError> {
        let _ = instance_id;
        Ok(0)
    }
}

pub struct MockEmbedder;
#[async_trait]
impl Embedder for MockEmbedder {
    async fn embed(&self, texts: &[&str], mode: EmbedMode) -> Result<Vec<Vec<f32>>, EmbedError> {
        let _ = (texts, mode);
        Ok(vec![vec![0.0; 1024]; texts.len()])
    }
    fn dimension(&self) -> usize {
        1024
    }
    fn name(&self) -> &'static str {
        "mock"
    }
}

pub struct MockLlm;
#[async_trait]
impl LlmClient for MockLlm {
    async fn complete(&self, req: CompletionRequest) -> Result<CompletionResponse, LlmError> {
        let _ = req;
        Ok(CompletionResponse {
            text: "Mock response".to_string(),
            tool_calls: vec![],
            model: "mock".to_string(),
            tokens_in: 0,
            tokens_out: 0,
            latency_ms: 0,
        })
    }
    async fn extract(&self, req: ExtractionRequest) -> Result<ports::ExtractionResponse, LlmError> {
        let value = match req.schema_name.as_str() {
            "MultiOffreExtraction" => serde_json::json!({
                "offres": [{
                    "intitule": "Mock Job",
                    "entreprise": "Mock Corp",
                    "localisation": "Paris",
                    "contrat": "CDI",
                    "resume_court": "A short summary",
                    "stack": ["Rust", "Axum"],
                    "missions": ["Developing", "Testing"],
                    "exigences": ["Degree in CS"],
                    "soft_skills": ["Teamwork"],
                    "niveau_etudes": "Master",
                    "type_contrat": "Full-time",
                    "mots_cles": ["software", "rust"]
                }]
            }),
            "RerankResponse" => serde_json::json!({
                "indices_retenus": [0],
                "raisonnement": "Selected because it is dummy"
            }),
            "CandidaturePlan" => serde_json::json!({
                "angle": "Showcase technical excellence",
                "forces_a_souligner": ["Expertise in Rust"],
                "mots_cles_critiques": ["performance", "safety"],
                "faiblesses_a_adresser": ["None"]
            }),
            "Restitution" => serde_json::json!({
                "synthese": "Mock synthesis",
                "entreprise": "Mock Corp",
                "poste": "Mock Job",
                "profil_recherche": "Mock Profile",
                "fit_score": 80,
                "fit_justification": "Good match",
                "forces": ["Rust"],
                "faiblesses": ["None"],
                "missions": ["Coding"],
                "stack_technique": ["Axum"],
                "exigences": ["Degree"],
                "points_attention": ["None"],
                "questions_entretien": ["Why Rust?"]
            }),
            _ => serde_json::json!({}),
        };
        Ok(ports::ExtractionResponse {
            value,
            raw: "mock-raw-response".into(),
        })
    }

    async fn stream(
        &self,
        _req: ports::CompletionRequest,
    ) -> Result<ports::BoxStream<'static, Result<StreamChunk, LlmError>>, LlmError> {
        let stream = futures::stream::iter(vec![
            Ok(StreamChunk::TextDelta {
                text: "token1".into(),
            }),
            Ok(StreamChunk::TextDelta {
                text: "token2".into(),
            }),
            Ok(StreamChunk::Done {
                stop_reason: StopReason::EndTurn,
            }),
        ]);
        Ok(Box::pin(stream))
    }

    fn name(&self) -> &'static str {
        "mock"
    }
}

pub struct MockScraper;
#[async_trait]
impl Scraper for MockScraper {
    async fn scrape(&self, url: &str) -> Result<ScrapeResult, ScrapeError> {
        Ok(ScrapeResult {
            url: url.to_string(),
            final_url: url.to_string(),
            raw_html: "<html></html>".to_string(),
            raw_text: "Cette offre d'emploi pour une mission d'alternance nécessite un profil dynamique.
            Voici les missions :
            - Développement de fonctionnalités innovantes
            - Tests unitaires et d'intégration
            - Participation aux réunions techniques
            
            Profil recherché :
            - Étudiant en informatique
            - Maîtrise de Rust et Axum
            - Esprit d'équipe et curiosité
            
            Informations complémentaires :
            L'entreprise propose un cadre de travail premium avec des technologies de pointe. 
            Nous valorisons l'expérience et la formation continue de nos collaborateurs. 
            La stack technique est moderne et stimulante. 
            Rejoignez-nous pour relever des défis passionnants et enrichir votre parcours professionnel dans un environnement bienveillant et ambitieux.".to_string(),
            status: 200,
        })
    }
    fn name(&self) -> &'static str {
        "mock"
    }
}
