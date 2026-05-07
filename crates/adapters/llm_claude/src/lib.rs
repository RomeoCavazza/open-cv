//! Adapter LLM Anthropic (Claude).
//!
//! Implémentation complète de `LlmClient` via l'API `/v1/messages`.
//! - `complete` : génération de texte libre
//! - `extract` : génération structurée via Tool Use (structured output)

use async_trait::async_trait;
use base64::Engine;
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

    fn headers(&self) -> Result<reqwest::header::HeaderMap, LlmError> {
        use reqwest::header::{HeaderMap, HeaderValue};
        let mut h = HeaderMap::new();
        h.insert(
            "x-api-key",
            HeaderValue::from_str(&self.api_key).map_err(|e| LlmError::Config(e.to_string()))?,
        );
        h.insert(
            "anthropic-version",
            HeaderValue::from_static(ANTHROPIC_VERSION),
        );
        h.insert(
            reqwest::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        Ok(h)
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
    content: Vec<AnthropicContentBlock>,
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum AnthropicContentBlock {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image")]
    Image { source: AnthropicImageSource },
}

#[derive(Serialize)]
struct AnthropicImageSource {
    #[serde(rename = "type")]
    source_type: String, // "base64"
    media_type: String,
    data: String,
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
    stop_reason: Option<String>,
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
                content: m
                    .content
                    .iter()
                    .map(|c| match c {
                        ports::MessageContent::Text(text) => {
                            AnthropicContentBlock::Text { text: text.clone() }
                        }
                        ports::MessageContent::Image { data, content_type } => {
                            AnthropicContentBlock::Image {
                                source: AnthropicImageSource {
                                    source_type: "base64".into(),
                                    media_type: content_type.clone(),
                                    data: base64::engine::general_purpose::STANDARD.encode(data),
                                },
                            }
                        }
                    })
                    .collect(),
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
            .headers(self.headers()?)
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
    async fn extract(&self, req: ExtractionRequest) -> Result<ports::ExtractionResponse, LlmError> {
        let schema = if req.json_schema.is_string() && req.json_schema.as_str() == Some("json") {
            serde_json::json!({ "type": "object" })
        } else {
            req.json_schema
        };

        let tool = AnthropicTool {
            name: req.schema_name.clone(),
            description: req.schema_description.clone(),
            input_schema: schema,
        };

        let body = AnthropicRequest {
            model: req.model.unwrap_or_else(|| self.model.clone()),
            max_tokens: req.max_tokens.unwrap_or(4096),
            system: req.system,
            messages: vec![AnthropicMessage {
                role: "user".into(),
                content: {
                    let mut blocks = Vec::new();
                    if !req.instruction.is_empty() {
                        blocks.push(AnthropicContentBlock::Text {
                            text: req.instruction.clone(),
                        });
                    }
                    for input_content in req.input {
                        match input_content {
                            ports::MessageContent::Text(text) => {
                                blocks.push(AnthropicContentBlock::Text { text })
                            }
                            ports::MessageContent::Image { data, content_type } => {
                                blocks.push(AnthropicContentBlock::Image {
                                    source: AnthropicImageSource {
                                        source_type: "base64".into(),
                                        media_type: content_type,
                                        data: base64::engine::general_purpose::STANDARD
                                            .encode(data),
                                    },
                                })
                            }
                        }
                    }
                    blocks
                },
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
            .headers(self.headers()?)
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

        if let Some(reason) = &parsed.stop_reason {
            if reason == "max_tokens" {
                return Err(LlmError::Truncated {
                    step: req.schema_name,
                    partial_payload: raw,
                });
            }
        }

        // Find the tool_use block
        for block in &parsed.content {
            if let ContentBlock::ToolUse { input, .. } = block {
                tracing::info!(
                    tokens_in = parsed.usage.input_tokens,
                    tokens_out = parsed.usage.output_tokens,
                    latency_ms = _latency_ms,
                    "extract completed"
                );
                return Ok(ports::ExtractionResponse {
                    value: input.clone(),
                    raw,
                });
            }
        }

        Err(LlmError::BadStructuredOutput(format!(
            "Aucun bloc tool_use trouvé dans la réponse Claude (stop_reason: {:?})",
            parsed.stop_reason
        )))
    }

    fn name(&self) -> &'static str {
        "anthropic"
    }

    async fn stream(
        &self,
        _req: CompletionRequest,
    ) -> Result<ports::BoxStream<'static, Result<String, LlmError>>, LlmError> {
        Err(LlmError::Other(
            "Streaming non implémenté pour Claude".into(),
        ))
    }
}
