//! Adapter LLM Anthropic (Claude).
//!
//! Implémentation complète de `LlmClient` via l'API `/v1/messages`.
//! - `complete` : génération de texte libre
//! - `extract` : génération structurée via Tool Use (structured output)

use async_trait::async_trait;
use ports::{CompletionRequest, CompletionResponse, ExtractionRequest, LlmClient, LlmError, Role};
use serde::{Deserialize, Serialize};
use tracing::instrument;

const DEFAULT_MODEL: &str = "claude-sonnet-4-20250514";
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

    fn headers(&self) -> reqwest::header::HeaderMap {
        use reqwest::header::{HeaderMap, HeaderValue};
        let mut h = HeaderMap::new();
        h.insert("x-api-key", HeaderValue::from_str(&self.api_key).unwrap());
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
}

// ─── Anthropic API types ───────────────────────────────────────────────

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    messages: Vec<AnthropicMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<AnthropicTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<serde_json::Value>,
}

#[derive(Serialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct AnthropicTool {
    name: String,
    description: String,
    input_schema: serde_json::Value,
}

#[derive(Deserialize)]
struct AnthropicResponse {
    content: Vec<ContentBlock>,
    usage: Usage,
    model: String,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum ContentBlock {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "tool_use")]
    ToolUse {
        #[allow(dead_code)]
        id: String,
        #[allow(dead_code)]
        name: String,
        input: serde_json::Value,
    },
}

#[derive(Deserialize)]
struct Usage {
    input_tokens: u32,
    output_tokens: u32,
}

#[derive(Deserialize)]
struct AnthropicError {
    error: AnthropicErrorDetail,
}

#[derive(Deserialize)]
struct AnthropicErrorDetail {
    message: String,
    #[serde(rename = "type")]
    error_type: String,
}

// ─── Trait implementation ──────────────────────────────────────────────

#[async_trait]
impl LlmClient for ClaudeClient {
    #[instrument(skip(self, req), fields(model = %self.model))]
    async fn complete(&self, req: CompletionRequest) -> Result<CompletionResponse, LlmError> {
        let messages: Vec<AnthropicMessage> = req
            .messages
            .iter()
            .map(|m| AnthropicMessage {
                role: match m.role {
                    Role::User => "user".into(),
                    Role::Assistant => "assistant".into(),
                },
                content: m.content.clone(),
            })
            .collect();

        let body = AnthropicRequest {
            model: req.model.unwrap_or_else(|| self.model.clone()),
            max_tokens: req.max_tokens.unwrap_or(4096),
            system: req.system,
            messages,
            temperature: req.temperature,
            tools: None,
            tool_choice: None,
        };

        let start = std::time::Instant::now();

        let resp = self
            .http
            .post(ANTHROPIC_API_URL)
            .headers(self.headers())
            .json(&body)
            .send()
            .await
            .map_err(|e| LlmError::Http(e.to_string()))?;

        let status = resp.status().as_u16();

        if status == 429 {
            let retry_after = resp
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok())
                .unwrap_or(5000);
            return Err(LlmError::RateLimit {
                retry_after_ms: retry_after * 1000,
            });
        }

        let raw = resp
            .text()
            .await
            .map_err(|e| LlmError::Http(e.to_string()))?;

        if status != 200 {
            if let Ok(err) = serde_json::from_str::<AnthropicError>(&raw) {
                return Err(LlmError::ProviderStatus {
                    status,
                    body: format!("{}: {}", err.error.error_type, err.error.message),
                });
            }
            return Err(LlmError::ProviderStatus { status, body: raw });
        }

        let latency_ms = start.elapsed().as_millis() as u64;

        let parsed: AnthropicResponse =
            serde_json::from_str(&raw).map_err(|e| LlmError::Json(e.to_string()))?;

        let text = parsed
            .content
            .iter()
            .filter_map(|b| match b {
                ContentBlock::Text { text } => Some(text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("");

        Ok(CompletionResponse {
            text,
            model: parsed.model,
            tokens_in: parsed.usage.input_tokens,
            tokens_out: parsed.usage.output_tokens,
            latency_ms,
        })
    }

    #[instrument(skip(self, req), fields(model = %self.model, schema = %req.schema_name))]
    async fn extract(&self, req: ExtractionRequest) -> Result<serde_json::Value, LlmError> {
        let tool = AnthropicTool {
            name: req.schema_name.clone(),
            description: req.schema_description.clone(),
            input_schema: req.json_schema,
        };

        let user_content = if req.instruction.is_empty() {
            req.input.clone()
        } else {
            format!("{}\n\n---\n\n{}", req.instruction, req.input)
        };

        let body = AnthropicRequest {
            model: req.model.unwrap_or_else(|| self.model.clone()),
            max_tokens: req.max_tokens.unwrap_or(4096),
            system: req.system,
            messages: vec![AnthropicMessage {
                role: "user".into(),
                content: user_content,
            }],
            temperature: None,
            tools: Some(vec![tool]),
            tool_choice: Some(serde_json::json!({
                "type": "tool",
                "name": req.schema_name,
            })),
        };

        let start = std::time::Instant::now();

        let resp = self
            .http
            .post(ANTHROPIC_API_URL)
            .headers(self.headers())
            .json(&body)
            .send()
            .await
            .map_err(|e| LlmError::Http(e.to_string()))?;

        let status = resp.status().as_u16();

        if status == 429 {
            let retry_after = resp
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok())
                .unwrap_or(5000);
            return Err(LlmError::RateLimit {
                retry_after_ms: retry_after * 1000,
            });
        }

        let raw = resp
            .text()
            .await
            .map_err(|e| LlmError::Http(e.to_string()))?;
        let _latency_ms = start.elapsed().as_millis() as u64;

        if status != 200 {
            if let Ok(err) = serde_json::from_str::<AnthropicError>(&raw) {
                return Err(LlmError::ProviderStatus {
                    status,
                    body: format!("{}: {}", err.error.error_type, err.error.message),
                });
            }
            return Err(LlmError::ProviderStatus { status, body: raw });
        }

        let parsed: AnthropicResponse =
            serde_json::from_str(&raw).map_err(|e| LlmError::Json(e.to_string()))?;

        // Find the tool_use block
        for block in &parsed.content {
            if let ContentBlock::ToolUse { input, .. } = block {
                tracing::info!(
                    tokens_in = parsed.usage.input_tokens,
                    tokens_out = parsed.usage.output_tokens,
                    latency_ms = _latency_ms,
                    "extract completed"
                );
                return Ok(input.clone());
            }
        }

        Err(LlmError::BadStructuredOutput(
            "Aucun bloc tool_use trouvé dans la réponse Claude".into(),
        ))
    }

    fn name(&self) -> &'static str {
        "anthropic"
    }
}
