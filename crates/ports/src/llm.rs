//! Le trait `LlmClient` — l'abstraction qui tient toute la stack IA.
//!
//! Cf. doc d'archi §6.2. Plusieurs implémentations interchangeables :
//! - `adapters/llm_claude` : Anthropic API, `tool_use` pour structured output
//! - `adapters/llm_mistral` : Mistral API, `response_format: json_schema`
//! - `adapters/embed_voyage` : embeddings via Voyage AI

use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct CompletionRequest {
    pub system: Option<String>,
    pub messages: Vec<Message>,
    pub model: Option<String>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone)]
pub struct CompletionResponse {
    pub text: String,
    pub model: String,
    pub tokens_in: u32,
    pub tokens_out: u32,
    pub latency_ms: u64,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    User,
    Assistant,
}

#[derive(Debug, Clone)]
pub struct ExtractionRequest {
    pub system: Option<String>,
    pub instruction: String,
    pub input: String,
    pub schema_name: String,
    pub schema_description: String,
    pub json_schema: serde_json::Value,
    pub model: Option<String>,
    pub max_tokens: Option<u32>,
}

#[async_trait]
pub trait LlmClient: Send + Sync {
    /// Génération texte libre.
    async fn complete(&self, req: CompletionRequest)
        -> Result<CompletionResponse, LlmError>;

    /// Génération structurée. On précise un schéma JSON, on récupère un JSON.
    async fn extract(&self, req: ExtractionRequest) -> Result<serde_json::Value, LlmError>;

    /// Identifiant du provider, utilisé dans `llm_calls.provider`.
    fn name(&self) -> &'static str;
}

#[derive(Debug, Error)]
pub enum LlmError {
    #[error("erreur HTTP vers le provider : {0}")]
    Http(String),

    #[error("provider a renvoyé un statut {status}: {body}")]
    ProviderStatus { status: u16, body: String },

    #[error("rate limit atteint, retry après {retry_after_ms}ms")]
    RateLimit { retry_after_ms: u64 },

    #[error("structured output invalide : {0}")]
    BadStructuredOutput(String),

    #[error("désérialisation JSON impossible : {0}")]
    Json(String),

    #[error("autre : {0}")]
    Other(String),
}
