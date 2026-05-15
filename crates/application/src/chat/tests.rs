use super::*;
use crate::chat::chat_event::ChatEvent;
use async_trait::async_trait;
use chrono::Utc;
use domain::{
    Chunk, ChunkKind, Instance, InstanceId, InstanceStatus, JsonValue as DomainJsonValue, Offre,
    OffreId, OffreStructured, Profil, ProfilId, Slug,
};
use futures::StreamExt;
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
        req: ports::CompletionRequest,
    ) -> Result<ports::BoxStream<'static, Result<ports::StreamChunk, LlmError>>, LlmError> {
        let mut chunks = Vec::new();

        let is_tool_result = req
            .messages
            .last()
            .map(|m| m.role == ports::Role::Tool)
            .unwrap_or(false);
        let wants_mutation = req.messages.iter().any(|m| {
            m.content.iter().any(|c| match c {
                ports::MessageContent::Text(t) => t.contains("modifie"),
                _ => false,
            })
        });

        let is_partial = req.messages.iter().any(|m| {
            m.content.iter().any(|c| match c {
                ports::MessageContent::Text(t) => t.contains("partial"),
                _ => false,
            })
        });
        let is_array_patch = req.messages.iter().any(|m| {
            m.content.iter().any(|c| match c {
                ports::MessageContent::Text(t) => t.contains("array_patch"),
                _ => false,
            })
        });
        let is_object_patch = req.messages.iter().any(|m| {
            m.content.iter().any(|c| match c {
                ports::MessageContent::Text(t) => t.contains("object_patch"),
                _ => false,
            })
        });
        let is_add_experience = req.messages.iter().any(|m| {
            m.content.iter().any(|c| match c {
                ports::MessageContent::Text(t) => t.contains("ajoute_experience"),
                _ => false,
            })
        });
        let is_invalid_resume_list = req.messages.iter().any(|m| {
            m.content.iter().any(|c| match c {
                ports::MessageContent::Text(t) => t.contains("invalid_resume_list"),
                _ => false,
            })
        });
        let is_partial_fail_both = req.messages.iter().any(|m| {
            m.content.iter().any(|c| match c {
                ports::MessageContent::Text(t) => t.contains("partial_fail_both"),
                _ => false,
            })
        });

        if is_partial_fail_both && !is_tool_result {
            chunks.push(Ok(ports::StreamChunk::ToolCallStart {
                id: "call_partial_fail".into(),
                name: "update_documents".into(),
            }));
            chunks.push(Ok(ports::StreamChunk::ToolCallEnd {
                id: "call_partial_fail".into(),
                name: "update_documents".into(),
                arguments: json!({
                    "resume": {
                        "accroche": { "titre": "should_not_apply" }
                    },
                    "cover": {
                        "paragraphes": [
                            { "role": "accroche", "contenu": "invalid via update_documents" }
                        ]
                    },
                    "message": "partial fail both",
                    "commit_message": "Tentative invalide mixte"
                }),
            }));
            chunks.push(Ok(ports::StreamChunk::Done {
                stop_reason: ports::StopReason::ToolUse,
            }));
        } else if is_add_experience && !is_tool_result {
            chunks.push(Ok(ports::StreamChunk::ToolCallStart {
                id: "call_add_exp".into(),
                name: "edit_resume_list".into(),
            }));
            chunks.push(Ok(ports::StreamChunk::ToolCallEnd {
                id: "call_add_exp".into(),
                name: "edit_resume_list".into(),
                arguments: json!({
                    "target": "experiences",
                    "operation": "add",
                    "item": {
                        "poste": "Dev Rust",
                        "entreprise": "X",
                        "periode": "2025",
                        "bullets": ["implémentation"]
                    },
                    "commit_message": "Ajout experience Rust"
                }),
            }));
            chunks.push(Ok(ports::StreamChunk::Done {
                stop_reason: ports::StopReason::ToolUse,
            }));
        } else if is_array_patch && !is_tool_result {
            chunks.push(Ok(ports::StreamChunk::ToolCallStart {
                id: "call_array_patch".into(),
                name: "update_documents".into(),
            }));
            chunks.push(Ok(ports::StreamChunk::ToolCallEnd {
                id: "call_array_patch".into(),
                name: "update_documents".into(),
                arguments: json!({
                    "resume": {
                        "experiences": [
                            { "poste": "Dev Rust", "entreprise": "X", "periode": "2025", "bullets": ["..."] }
                        ]
                    },
                    "message": "array patch",
                    "commit_message": "Tentative patch array"
                }),
            }));
            chunks.push(Ok(ports::StreamChunk::Done {
                stop_reason: ports::StopReason::ToolUse,
            }));
        } else if is_object_patch && !is_tool_result {
            chunks.push(Ok(ports::StreamChunk::ToolCallStart {
                id: "call_object_patch".into(),
                name: "update_documents".into(),
            }));
            chunks.push(Ok(ports::StreamChunk::ToolCallEnd {
                id: "call_object_patch".into(),
                name: "update_documents".into(),
                arguments: json!({
                    "resume": {
                        "experiences": {
                            "poste": "Dev Rust"
                        }
                    },
                    "message": "object patch",
                    "commit_message": "Tentative patch objet"
                }),
            }));
            chunks.push(Ok(ports::StreamChunk::Done {
                stop_reason: ports::StopReason::ToolUse,
            }));
        } else if is_invalid_resume_list && !is_tool_result {
            chunks.push(Ok(ports::StreamChunk::ToolCallStart {
                id: "call_invalid_list".into(),
                name: "edit_resume_list".into(),
            }));
            chunks.push(Ok(ports::StreamChunk::ToolCallEnd {
                id: "call_invalid_list".into(),
                name: "edit_resume_list".into(),
                arguments: json!({
                    "target": "experiences",
                    "operation": "add",
                    "item": "payload_invalide",
                    "commit_message": "Payload invalide"
                }),
            }));
            chunks.push(Ok(ports::StreamChunk::Done {
                stop_reason: ports::StopReason::ToolUse,
            }));
        } else if is_partial && !is_tool_result {
            chunks.push(Ok(ports::StreamChunk::ToolCallStart {
                id: "call_partial".into(),
                name: "update_documents".into(),
            }));
            chunks.push(Ok(ports::StreamChunk::ToolCallEnd {
                id: "call_partial".into(),
                name: "update_documents".into(),
                arguments: json!({
                    "resume": {
                        "accroche": { "titre": "partial update" }
                    },
                    "message": "partial update",
                    "commit_message": "Mise à jour accroche"
                }),
            }));
            chunks.push(Ok(ports::StreamChunk::Done {
                stop_reason: ports::StopReason::ToolUse,
            }));
        } else if wants_mutation && !is_tool_result {
            // Simulation d'un tool call
            chunks.push(Ok(ports::StreamChunk::ToolCallStart {
                id: "call_123".into(),
                name: "update_documents".into(),
            }));
            chunks.push(Ok(ports::StreamChunk::ToolCallEnd {
                id: "call_123".into(),
                name: "update_documents".into(),
                arguments: json!({
                    "resume": {
                        "accroche": {
                            "titre": "updated resume"
                        }
                    },
                    "cover": {
                        "objet": {
                            "libelle": "updated cover"
                        }
                    },
                    "message": "ceci est ignoré car on utilise le second tour",
                    "commit_message": "Mise à jour CV et lettre"
                }),
            }));
            chunks.push(Ok(ports::StreamChunk::Done {
                stop_reason: ports::StopReason::ToolUse,
            }));
        } else if is_tool_result {
            chunks.push(Ok(ports::StreamChunk::TextDelta {
                text: "mise à jour appliquée".into(),
            }));
            chunks.push(Ok(ports::StreamChunk::Done {
                stop_reason: ports::StopReason::EndTurn,
            }));
        } else {
            chunks.push(Ok(ports::StreamChunk::TextDelta {
                text: "token1".into(),
            }));
            chunks.push(Ok(ports::StreamChunk::TextDelta {
                text: "token2".into(),
            }));
            chunks.push(Ok(ports::StreamChunk::Done {
                stop_reason: ports::StopReason::EndTurn,
            }));
        }

        Ok(Box::pin(futures::stream::iter(chunks)))
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

#[tokio::test]
async fn instance_chat_keeps_read_only_questions_in_message_mode() {
    let (instance, profil, offre) = build_test_data();
    let stores = Arc::new(TestStores::new(instance, profil, offre));
    let usecase = build_usecase(stores.clone());

    let instance_id_str = stores.instance.lock().unwrap().id.to_string();
    let response = usecase
        .execute(ChatRequest {
            message: "quel est le poste ?".into(),
            instance_id: Some(instance_id_str),
            conversation_id: None,
            llm_provider: "ollama".into(),
            attachments: vec![],
        })
        .await
        .expect("chat should succeed");

    assert_eq!(response.message, "token1token2");

    let instance_after = stores.instance.lock().unwrap().clone();
    assert_eq!(instance_after.resume_json.unwrap().accroche.titre, "resume");
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
            message: "modifie mon CV".into(),
            instance_id: Some(instance_id_str),
            conversation_id: None,
            llm_provider: "ollama".into(),
            attachments: vec![],
        })
        .await
        .expect("chat should succeed");

    assert_eq!(response.message, "mise à jour appliquée");

    let instance_after = stores.instance.lock().unwrap().clone();
    assert_eq!(
        instance_after.resume_json.unwrap().accroche.titre,
        "updated resume"
    );
    assert_eq!(stores.messages.lock().unwrap().len(), 2);
}

#[tokio::test]
async fn instance_chat_stream_keeps_read_only_requests_as_token_stream() {
    let (instance, profil, offre) = build_test_data();
    let stores = Arc::new(TestStores::new(instance, profil, offre));
    let usecase = build_usecase(stores.clone());

    let instance_id_str = stores.instance.lock().unwrap().id.to_string();
    let stream = usecase
        .execute_stream(ChatRequest {
            message: "quel est le poste ?".into(),
            instance_id: Some(instance_id_str),
            conversation_id: None,
            llm_provider: "ollama".into(),
            attachments: vec![],
        })
        .await
        .expect("stream should start");

    let events: Vec<ChatEvent> = stream
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .map(|item| item.expect("stream item should be ok"))
        .collect();

    let text: String = events
        .iter()
        .filter_map(|e| match e {
            ChatEvent::Token { content } => Some(content.as_str()),
            _ => None,
        })
        .collect();

    assert_eq!(text, "token1token2");
    assert!(matches!(events.last(), Some(ChatEvent::Done)));
    assert_eq!(stores.messages.lock().unwrap().len(), 2);
}

#[tokio::test]
async fn instance_chat_stream_applies_json_mutation_before_streaming_message() {
    let (instance, profil, offre) = build_test_data();
    let stores = Arc::new(TestStores::new(instance, profil, offre));
    let usecase = build_usecase(stores.clone());

    let instance_id_str = stores.instance.lock().unwrap().id.to_string();
    let stream = usecase
        .execute_stream(ChatRequest {
            message: "modifie mon CV".into(),
            instance_id: Some(instance_id_str),
            conversation_id: None,
            llm_provider: "ollama".into(),
            attachments: vec![],
        })
        .await
        .expect("stream should start");

    let events: Vec<ChatEvent> = stream
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .map(|item| item.expect("stream item should be ok"))
        .collect();

    let text: String = events
        .iter()
        .filter_map(|e| match e {
            ChatEvent::Token { content } => Some(content.as_str()),
            _ => None,
        })
        .collect();

    assert_eq!(text, "mise à jour appliquée");
    assert!(events
        .iter()
        .any(|e| matches!(e, ChatEvent::Mutation { .. })));
    assert!(matches!(events.last(), Some(ChatEvent::Done)));

    let instance_after = stores.instance.lock().unwrap().clone();
    assert_eq!(
        instance_after.resume_json.unwrap().accroche.titre,
        "updated resume"
    );
}
#[tokio::test]
async fn instance_chat_merge_preserves_unmodified_fields() {
    let (mut instance, profil, offre) = build_test_data();
    // 1. On injecte un nom spécifique dans l'identité pour vérifier sa préservation
    let mut resume = instance.resume_json.clone().unwrap_or_default();
    resume.identite.nom_complet = "Romeo".into();
    instance.resume_json = Some(resume);

    let stores = Arc::new(TestStores::new(instance, profil, offre));
    let usecase = build_usecase(stores.clone());

    let instance_id_str = stores.instance.lock().unwrap().id.to_string();
    let _ = usecase
        .execute(ChatRequest {
            message: "partial mutation".into(),
            instance_id: Some(instance_id_str),
            conversation_id: None,
            llm_provider: "ollama".into(),
            attachments: vec![],
        })
        .await
        .expect("chat should succeed");

    let instance_after = stores.instance.lock().unwrap().clone();
    let resume_after = instance_after.resume_json.unwrap();

    // VÉRIFICATION CRITIQUE :
    // - Le titre a été mis à jour par le patch
    assert_eq!(resume_after.accroche.titre, "partial update");
    // - Le nom complet a été PRÉSERVÉ (le merge a fonctionné au lieu de l'écraser par "" ou null)
    assert_eq!(resume_after.identite.nom_complet, "Romeo");
}

#[tokio::test]
async fn instance_chat_update_documents_does_not_replace_arrays() {
    let (mut instance, profil, offre) = build_test_data();
    let mut resume = instance.resume_json.clone().unwrap_or_default();
    resume.experiences = vec![
        domain::Experience {
            poste: "Dev 1".into(),
            entreprise: "A".into(),
            localisation: None,
            periode: "2023".into(),
            bullets: vec!["x".into()],
        },
        domain::Experience {
            poste: "Dev 2".into(),
            entreprise: "B".into(),
            localisation: None,
            periode: "2024".into(),
            bullets: vec!["y".into()],
        },
    ];
    instance.resume_json = Some(resume);

    let stores = Arc::new(TestStores::new(instance, profil, offre));
    let usecase = build_usecase(stores.clone());
    let instance_id_str = stores.instance.lock().unwrap().id.to_string();

    let _ = usecase
        .execute(ChatRequest {
            message: "array_patch".into(),
            instance_id: Some(instance_id_str),
            conversation_id: None,
            llm_provider: "ollama".into(),
            attachments: vec![],
        })
        .await
        .expect("chat should succeed");

    let instance_after = stores.instance.lock().unwrap().clone();
    let experiences = instance_after.resume_json.unwrap().experiences;
    assert_eq!(experiences.len(), 2);
    assert_eq!(experiences[0].poste, "Dev 1");
    assert_eq!(experiences[1].poste, "Dev 2");
}

#[tokio::test]
async fn instance_chat_update_documents_rejects_object_patch_on_list_fields() {
    let (mut instance, profil, offre) = build_test_data();
    let mut resume = instance.resume_json.clone().unwrap_or_default();
    resume.experiences = vec![domain::Experience {
        poste: "Dev 1".into(),
        entreprise: "A".into(),
        localisation: None,
        periode: "2023".into(),
        bullets: vec!["x".into()],
    }];
    instance.resume_json = Some(resume);

    let stores = Arc::new(TestStores::new(instance, profil, offre));
    let usecase = build_usecase(stores.clone());
    let instance_id_str = stores.instance.lock().unwrap().id.to_string();

    let _ = usecase
        .execute(ChatRequest {
            message: "object_patch".into(),
            instance_id: Some(instance_id_str),
            conversation_id: None,
            llm_provider: "ollama".into(),
            attachments: vec![],
        })
        .await
        .expect("chat should succeed");

    let instance_after = stores.instance.lock().unwrap().clone();
    let experiences = instance_after.resume_json.unwrap().experiences;
    assert_eq!(experiences.len(), 1);
    assert_eq!(experiences[0].poste, "Dev 1");
}

#[tokio::test]
async fn instance_chat_edit_resume_list_adds_experience() {
    let (instance, profil, offre) = build_test_data();
    let stores = Arc::new(TestStores::new(instance, profil, offre));
    let usecase = build_usecase(stores.clone());
    let instance_id_str = stores.instance.lock().unwrap().id.to_string();

    let _ = usecase
        .execute(ChatRequest {
            message: "ajoute_experience".into(),
            instance_id: Some(instance_id_str),
            conversation_id: None,
            llm_provider: "ollama".into(),
            attachments: vec![],
        })
        .await
        .expect("chat should succeed");

    let instance_after = stores.instance.lock().unwrap().clone();
    let experiences = instance_after.resume_json.unwrap().experiences;
    assert_eq!(experiences.len(), 1);
    assert_eq!(experiences[0].poste, "Dev Rust");
    assert_eq!(experiences[0].entreprise, "X");
}

#[tokio::test]
async fn instance_chat_invalid_edit_resume_list_does_not_fail_stream() {
    let (instance, profil, offre) = build_test_data();
    let stores = Arc::new(TestStores::new(instance, profil, offre));
    let usecase = build_usecase(stores.clone());
    let instance_id_str = stores.instance.lock().unwrap().id.to_string();

    let response = usecase
        .execute(ChatRequest {
            message: "invalid_resume_list".into(),
            instance_id: Some(instance_id_str),
            conversation_id: None,
            llm_provider: "ollama".into(),
            attachments: vec![],
        })
        .await
        .expect("chat should succeed despite invalid list payload");

    assert_eq!(response.message, "mise à jour appliquée");
    let instance_after = stores.instance.lock().unwrap().clone();
    assert!(instance_after.resume_json.unwrap().experiences.is_empty());
}

#[tokio::test]
async fn instance_chat_update_documents_is_atomic_on_resume_and_cover() {
    let (instance, profil, offre) = build_test_data();
    let stores = Arc::new(TestStores::new(instance, profil, offre));
    let usecase = build_usecase(stores.clone());
    let instance_id_str = stores.instance.lock().unwrap().id.to_string();

    let _ = usecase
        .execute(ChatRequest {
            message: "partial_fail_both".into(),
            instance_id: Some(instance_id_str),
            conversation_id: None,
            llm_provider: "ollama".into(),
            attachments: vec![],
        })
        .await
        .expect("chat should succeed");

    let instance_after = stores.instance.lock().unwrap().clone();
    let resume_after = instance_after.resume_json.unwrap();
    assert_eq!(resume_after.accroche.titre, "resume");
}
