use super::*;
use crate::chat::prompt_utils::MAX_CHAT_HISTORY_ENTRIES;
use async_trait::async_trait;
use chrono::Utc;
use domain::{
    Chunk, ChunkKind, Instance, InstanceId, InstanceStatus, JsonValue as DomainJsonValue, Offre,
    OffreId, OffreStructured, Profil, ProfilId, Slug,
};
use ports::{
    ChunkRepo, EmbedError, EmbedMode, Embedder, ExtractionRequest, InstanceRepo, LlmClient,
    LlmError, OffreRepo, ProfilRepo, RepoError,
};
use serde_json::json;
use std::sync::{Arc, Mutex};

struct TestStores {
    instance: Mutex<Instance>,
    profil: Mutex<Profil>,
    offre: Mutex<Offre>,
    messages: Mutex<Vec<Message>>,
    requests: Mutex<Vec<ExtractionRequest>>,
}

impl TestStores {
    fn new(instance: Instance, profil: Profil, offre: Offre) -> Self {
        Self {
            instance: Mutex::new(instance),
            profil: Mutex::new(profil),
            offre: Mutex::new(offre),
            messages: Mutex::new(Vec::new()),
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

    async fn get_by_offre_and_profil(
        &self,
        offre_id: domain::OffreId,
        profil_id: domain::ProfilId,
    ) -> Result<Option<Instance>, RepoError> {
        let instance = self.stores.instance.lock().unwrap().clone();
        Ok((instance.offre_id == offre_id && instance.profil_id == profil_id).then_some(instance))
    }

    async fn update_livrables(
        &self,
        _id: InstanceId,
        restitution: Option<domain::Restitution>,
        resume_json: Option<domain::Resume>,
        cover_letter_json: Option<domain::CoverLetter>,
        status: domain::InstanceStatus,
        _updated_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), RepoError> {
        let mut instance = self.stores.instance.lock().unwrap();
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
        Ok(())
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

    async fn list_all(&self) -> Result<Vec<Offre>, RepoError> {
        Ok(vec![self.stores.offre.lock().unwrap().clone()])
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

    async fn find_by_url(&self, _url: &str) -> Result<Option<Offre>, RepoError> {
        Ok(None)
    }

    async fn find_by_content_hash(
        &self,
        _source_host: &str,
        _hash: &[u8],
    ) -> Result<Option<Offre>, RepoError> {
        Ok(None)
    }
}

struct TestAnnexeRepo;

#[async_trait]
impl ports::AnnexeRepo for TestAnnexeRepo {
    async fn get_by_id(&self, _id: domain::AnnexeId) -> Result<Option<domain::Annexe>, RepoError> {
        Ok(None)
    }
    async fn list_by_profil_id(
        &self,
        _profil_id: domain::ProfilId,
    ) -> Result<Vec<domain::Annexe>, RepoError> {
        Ok(vec![])
    }
    async fn upsert(&self, _annexe: &domain::Annexe) -> Result<(), RepoError> {
        Ok(())
    }
    async fn delete(&self, _id: domain::AnnexeId) -> Result<(), RepoError> {
        Ok(())
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
                metadata: DomainJsonValue::Object(Default::default()),
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
    async fn complete(
        &self,
        _req: ports::CompletionRequest,
    ) -> Result<ports::CompletionResponse, LlmError> {
        Err(LlmError::Other("not used".into()))
    }

    async fn extract(&self, req: ExtractionRequest) -> Result<ports::ExtractionResponse, LlmError> {
        self.stores.requests.lock().unwrap().push(req.clone());
        let schema_text = req.json_schema.to_string();

        let value = if schema_text.contains("\"resume\"") {
            json!({
                "resume": domain::Resume {
                    accroche: domain::Accroche {
                        titre: "updated resume".into(),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                "cover": domain::CoverLetter {
                    objet: domain::Objet {
                        libelle: "updated cover".into(),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                "message": "mise à jour appliquée"
            })
        } else {
            json!({
                "message": "lecture factuelle"
            })
        };

        Ok(ports::ExtractionResponse {
            value,
            raw: "mock-raw-response".into(),
        })
    }

    async fn stream(
        &self,
        _req: ports::CompletionRequest,
    ) -> Result<ports::BoxStream<'static, Result<String, LlmError>>, LlmError> {
        let stream = futures::stream::iter(vec![Ok("token1".into()), Ok("token2".into())]);
        Ok(Box::pin(stream))
    }

    fn name(&self) -> &'static str {
        "recording-llm"
    }
}

struct TestMessageRepo {
    stores: Arc<TestStores>,
}

#[async_trait]
impl MessageRepo for TestMessageRepo {
    async fn list_by_instance_id(&self, _id: InstanceId) -> Result<Vec<Message>, RepoError> {
        Ok(self.stores.messages.lock().unwrap().clone())
    }
    async fn list_by_profil_id(&self, _id: ProfilId) -> Result<Vec<Message>, RepoError> {
        Ok(self.stores.messages.lock().unwrap().clone())
    }
    async fn push(&self, message: &Message) -> Result<(), RepoError> {
        self.stores.messages.lock().unwrap().push(message.clone());
        Ok(())
    }
    async fn delete_all_for_instance(&self, _id: InstanceId) -> Result<(), RepoError> {
        self.stores.messages.lock().unwrap().clear();
        Ok(())
    }
}

fn build_test_data() -> (Instance, Profil, Offre) {
    let profil_id = ProfilId::new();
    let offre_id = OffreId::new();
    let instance_id = InstanceId::new();

    let profil = Profil {
        id: profil_id,
        label: "test-profile".into(),
        content: domain::ProfilContent {
            profile: domain::ProfileSection {
                firstname: "Romeo".into(),
                lastname: "Cavazza".into(),
                title: "Alternance IA".into(),
                ..Default::default()
            },
            ..Default::default()
        },
        is_active: true,
        resume_template: Some(DomainJsonValue::Object(Default::default())),
        cover_letter_template: Some(DomainJsonValue::Object(Default::default())),
        profile_photo: None,
        calendar_pdf: None,
        notes: DomainJsonValue::Object(Default::default()),
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
        restitution: Some(domain::Restitution {
            synthese: "restitution".into(),
            ..Default::default()
        }),
        resume_json: Some(domain::Resume {
            accroche: domain::Accroche {
                titre: "resume".into(),
                ..Default::default()
            },
            ..Default::default()
        }),
        cover_letter_json: Some(domain::CoverLetter {
            objet: domain::Objet {
                libelle: "cover".into(),
                ..Default::default()
            },
            ..Default::default()
        }),
        notes: DomainJsonValue::Object(Default::default()),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        sent_at: None,
    };

    (instance, profil, offre)
}

fn build_usecase(stores: Arc<TestStores>) -> ChatWithApplicationUseCase {
    ChatWithApplicationUseCase::new(
        Arc::new(TestOffreRepo {
            stores: stores.clone(),
        }),
        Arc::new(TestInstanceRepo {
            stores: stores.clone(),
        }),
        Arc::new(TestProfilRepo {
            stores: stores.clone(),
        }),
        Arc::new(TestAnnexeRepo),
        Arc::new(TestChunkRepo),
        Arc::new(TestMessageRepo {
            stores: stores.clone(),
        }),
        Arc::new(TestEmbedder),
        std::iter::once((
            "ollama".to_string(),
            Arc::new(RecordingLlm { stores }) as Arc<dyn LlmClient>,
        ))
        .collect(),
    )
}

#[test]
fn detects_read_only_questions() {
    assert!(!wants_mutation(
        "comment je m'appelle ? c'est quoi l'offre ?"
    ));
    assert!(!wants_mutation(
        "tu vois l'offre, mon cv et ma lettre de motivation ?"
    ));
}

#[test]
fn detects_mutation_requests() {
    assert!(wants_mutation("modifie mon CV pour mieux mettre mon titre"));
    assert!(wants_mutation("ajoute une expérience dans la lettre"));
}

#[test]
fn detects_identity_requests() {
    assert!(wants_identity("tu sais comment je m'appelle ?"));
    assert!(wants_identity("c'est quoi mon nom exactement ?"));
    assert!(!wants_identity("tu peux résumer l'offre ?"));
}

#[test]
fn push_chat_history_trims_old_entries() {
    let mut notes = DomainJsonValue::Object(Default::default());

    for idx in 0..=MAX_CHAT_HISTORY_ENTRIES {
        push_chat_history(&mut notes, "user", &format!("message-{idx}"));
    }

    let history = notes
        .get("chat_history")
        .and_then(|value| value.as_array())
        .expect("chat history should exist");

    assert_eq!(history.len(), MAX_CHAT_HISTORY_ENTRIES);
    assert_eq!(
        history.first().and_then(|entry| entry.get("content")),
        Some(&DomainJsonValue::String("message-1".to_string()))
    );
    assert_eq!(
        history.last().and_then(|entry| entry.get("content")),
        Some(&DomainJsonValue::String(format!(
            "message-{}",
            MAX_CHAT_HISTORY_ENTRIES
        )))
    );
}

#[test]
fn render_chat_history_for_prompt_keeps_last_twelve_entries_in_order() {
    let instance_id = InstanceId::new();
    let history: Vec<Message> = (0..13)
        .map(|idx| Message::new(instance_id, MessageRole::User, format!("message-{idx}")))
        .collect();

    let rendered = render_chat_history_for_prompt(&history);

    assert!(rendered.starts_with("UTILISATEUR: message-1"));
    assert!(rendered.contains("UTILISATEUR: message-12"));
    assert!(!rendered.contains("message-0"));
    assert!(!rendered.contains("message-13"));
    assert_eq!(rendered.lines().count(), 12);
}

#[tokio::test]
async fn instance_chat_keeps_read_only_questions_in_message_mode() {
    let (instance, profil, offre) = build_test_data();
    let stores = Arc::new(TestStores::new(instance, profil, offre));
    let usecase = build_usecase(stores.clone());

    let instance_id_str = stores.instance.lock().unwrap().id.to_string();
    let response = usecase
        .execute(ChatRequest {
            message: "comment je m'appelle ? c'est quoi l'offre ?".into(),
            instance_id: Some(instance_id_str),
            llm_provider: "ollama".into(),
            attachments: vec![],
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
    assert_eq!(instance_after.resume_json.unwrap().accroche.titre, "resume");
    assert_eq!(
        instance_after.cover_letter_json.unwrap().objet.libelle,
        "cover"
    );
    assert_eq!(stores.messages.lock().unwrap().len(), 2);
}

#[tokio::test]
async fn instance_chat_updates_documents_for_explicit_mutations() {
    let (instance, profil, offre) = build_test_data();
    let stores = Arc::new(TestStores::new(instance, profil, offre));
    let usecase = build_usecase(stores.clone());

    let instance_id_str = stores.instance.lock().unwrap().id.to_string();
    let response = usecase
        .execute(ChatRequest {
            message: "modifie mon CV et ma lettre pour mettre en avant Rust".into(),
            instance_id: Some(instance_id_str),
            llm_provider: "ollama".into(),
            attachments: vec![],
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
    assert_eq!(
        instance_after.resume_json.unwrap().accroche.titre,
        "updated resume"
    );
    assert_eq!(
        instance_after.cover_letter_json.unwrap().objet.libelle,
        "updated cover"
    );
    assert_eq!(instance_after.status, InstanceStatus::Ready);
    assert_eq!(stores.messages.lock().unwrap().len(), 2);
}
