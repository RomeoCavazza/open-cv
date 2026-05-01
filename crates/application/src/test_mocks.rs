//! Mocks d'infra pour tester les use cases sans toucher Postgres ni le réseau.
//!
//! Disponibles uniquement sous `#[cfg(test)]` ou via la feature `test-mocks`
//! pour qu'ils ne polluent pas le binaire de prod.
//!
//! Philosophie : minimaux, prévisibles, scriptables. Pas de magie.

#![allow(dead_code)]

use std::sync::Mutex;

use async_trait::async_trait;
use ports::{
    CompletionRequest, CompletionResponse, EmbedError, EmbedMode, Embedder, ExtractionRequest,
    LlmClient, LlmError,
};

/// Mock LLM qui renvoie des réponses scriptées.
///
/// Usage :
/// ```ignore
/// let llm = MockLlm::new();
/// llm.queue_extract_json(serde_json::json!({...}));
/// llm.queue_complete("réponse texte");
/// ```
pub struct MockLlm {
    extract_queue: Mutex<Vec<serde_json::Value>>,
    complete_queue: Mutex<Vec<String>>,
}

impl MockLlm {
    pub fn new() -> Self {
        Self {
            extract_queue: Mutex::new(Vec::new()),
            complete_queue: Mutex::new(Vec::new()),
        }
    }

    pub fn queue_extract_json(&self, value: serde_json::Value) {
        self.extract_queue.lock().unwrap().push(value);
    }

    pub fn queue_complete(&self, text: impl Into<String>) {
        self.complete_queue.lock().unwrap().push(text.into());
    }
}

impl Default for MockLlm {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl LlmClient for MockLlm {
    async fn complete(&self, _req: CompletionRequest) -> Result<CompletionResponse, LlmError> {
        let text = self
            .complete_queue
            .lock()
            .unwrap()
            .pop()
            .ok_or_else(|| LlmError::Other("MockLlm: queue complete vide".into()))?;
        Ok(CompletionResponse {
            text,
            model: "mock".into(),
            tokens_in: 0,
            tokens_out: 0,
            latency_ms: 0,
        })
    }

    async fn extract(&self, _req: ExtractionRequest) -> Result<serde_json::Value, LlmError> {
        let value = self
            .extract_queue
            .lock()
            .unwrap()
            .pop()
            .ok_or_else(|| LlmError::Other("MockLlm: queue extract vide".into()))?;
        Ok(value)
    }

    fn name(&self) -> &'static str {
        "mock"
    }
}

/// Mock Embedder qui renvoie des vecteurs déterministes basés sur le hash
/// du texte. Permet de tester les flux RAG sans appeler Voyage/OpenAI.
pub struct MockEmbedder {
    dimension: usize,
}

impl MockEmbedder {
    pub fn new(dimension: usize) -> Self {
        Self { dimension }
    }
}

#[async_trait]
impl Embedder for MockEmbedder {
    async fn embed(&self, texts: &[&str], _mode: EmbedMode) -> Result<Vec<Vec<f32>>, EmbedError> {
        Ok(texts
            .iter()
            .map(|t| pseudo_embedding(t, self.dimension))
            .collect())
    }

    fn dimension(&self) -> usize {
        self.dimension
    }

    fn name(&self) -> &'static str {
        "mock"
    }
}

/// Génère un "embedding" pseudo-aléatoire mais déterministe à partir du texte.
/// Suffisant pour les tests : deux textes identiques → vecteurs identiques.
fn pseudo_embedding(text: &str, dim: usize) -> Vec<f32> {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    text.hash(&mut hasher);
    let seed = hasher.finish();

    (0..dim)
        .map(|i| {
            let mixed = seed.wrapping_add(i as u64).wrapping_mul(2654435761);
            ((mixed % 1000) as f32 / 1000.0) - 0.5
        })
        .collect()
}
