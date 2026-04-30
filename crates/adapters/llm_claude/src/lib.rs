//! Adapter LLM Anthropic (Claude).
//!
//! Phase 2 : implémentation complète des trois méthodes du trait.
//! Ce squelette compile avec des `todo!()` clairement marqués pour que tu
//! saches exactement où mettre la chair.

use async_trait::async_trait;
use ports::{
    CompletionRequest, CompletionResponse, ExtractionRequest, LlmClient, LlmError,
};
use schemars::JsonSchema;
use serde::de::DeserializeOwned;

const DEFAULT_MODEL: &str = "claude-sonnet-4-5"; // ajuster au lancement
const ANTHROPIC_API_URL: &str = "https://api.anthropic.com/v1/messages";
const ANTHROPIC_VERSION: &str = "2023-06-01";

pub struct ClaudeClient {
    api_key: String,
    model: String,
    http: reqwest::Client,
}

impl ClaudeClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            model: DEFAULT_MODEL.into(),
            http: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(120))
                .build()
                .expect("client HTTP valide"),
        }
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }
}

#[async_trait]
impl LlmClient for ClaudeClient {
    async fn complete(
        &self,
        _req: CompletionRequest,
    ) -> Result<CompletionResponse, LlmError> {
        // Phase 2 :
        //   1. Construire le payload Anthropic /v1/messages
        //   2. POST avec headers x-api-key, anthropic-version
        //   3. Parser la réponse, extraire content[0].text + usage
        //   4. Mesurer la latency
        //   5. Renvoyer CompletionResponse
        Err(LlmError::Other(
            "ClaudeClient::complete pas encore implémenté (Phase 2)".into(),
        ))
    }

    async fn extract<T>(&self, _req: ExtractionRequest) -> Result<T, LlmError>
    where
        T: DeserializeOwned + JsonSchema + Send,
    {
        // Phase 2 :
        //   1. Générer le JSON schema depuis T (via `schemars::schema_for!`)
        //   2. Wrapper en outil Anthropic : tools = [{name, description,
        //      input_schema}]
        //   3. Envoyer messages.create avec tool_choice forcé
        //   4. Parser content[].tool_use[0].input → désérialiser en T
        Err(LlmError::Other(
            "ClaudeClient::extract pas encore implémenté (Phase 2)".into(),
        ))
    }

    fn name(&self) -> &'static str {
        "anthropic"
    }
}

#[allow(dead_code)]
fn auth_headers(api_key: &str) -> reqwest::header::HeaderMap {
    use reqwest::header::{HeaderMap, HeaderValue};
    let mut h = HeaderMap::new();
    h.insert("x-api-key", HeaderValue::from_str(api_key).unwrap());
    h.insert(
        "anthropic-version",
        HeaderValue::from_static(ANTHROPIC_VERSION),
    );
    h.insert(
        reqwest::header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    h
}

#[allow(dead_code)]
const _: &str = ANTHROPIC_API_URL;
