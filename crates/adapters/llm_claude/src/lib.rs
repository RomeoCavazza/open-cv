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
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone)]
struct AnthropicMessage {
    role: String,
    content: Vec<AnthropicContentBlock>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
enum AnthropicContentBlock {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image")]
    Image { source: AnthropicImageSource },
    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        name: String,
        input: serde_json::Value,
    },
    #[serde(rename = "tool_result")]
    ToolResult {
        tool_use_id: String,
        content: String,
    },
}

#[derive(Serialize, Deserialize, Clone)]
struct AnthropicImageSource {
    #[serde(rename = "type")]
    source_type: String, // "base64"
    media_type: String,
    data: String,
}

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
enum ContentBlock {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        name: String,
        input: serde_json::Value,
    },
}

#[derive(Deserialize, Debug)]
struct Usage {
    input_tokens: u32,
    output_tokens: u32,
}

#[derive(Deserialize)]
struct AnthropicError {
    error: AnthropicErrorDetail,
}

#[derive(Deserialize, Debug)]
struct AnthropicErrorDetail {
    message: String,
    #[serde(rename = "type")]
    error_type: String,
}

// ─── Anthropic Streaming Types ─────────────────────────────────────────

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
#[serde(tag = "type")]
enum AnthropicStreamEvent {
    #[serde(rename = "message_start")]
    MessageStart { message: AnthropicStreamMessage },
    #[serde(rename = "content_block_start")]
    ContentBlockStart {
        index: usize,
        content_block: ContentBlock,
    },
    #[serde(rename = "content_block_delta")]
    ContentBlockDelta {
        index: usize,
        delta: ContentBlockDelta,
    },
    #[serde(rename = "content_block_stop")]
    ContentBlockStop { index: usize },
    #[serde(rename = "message_delta")]
    MessageDelta {
        delta: MessageDelta,
        usage: Option<UsageDelta>,
    },
    #[serde(rename = "message_stop")]
    MessageStop,
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "error")]
    Error { error: AnthropicErrorDetail },
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct AnthropicStreamMessage {
    id: String,
    role: String,
    model: String,
    usage: Usage,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
enum ContentBlockDelta {
    #[serde(rename = "text_delta")]
    TextDelta { text: String },
    #[serde(rename = "input_json_delta")]
    InputJsonDelta { partial_json: String },
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct MessageDelta {
    stop_reason: Option<String>,
    stop_sequence: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct UsageDelta {
    output_tokens: u32,
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
                    Role::Tool => "user".into(), // Claude simule le tool result par un message user
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
                        ports::MessageContent::ToolUse { id, name, input } => {
                            AnthropicContentBlock::ToolUse {
                                id: id.clone(),
                                name: name.clone(),
                                input: input.clone(),
                            }
                        }
                        ports::MessageContent::ToolResult {
                            tool_use_id,
                            content,
                        } => AnthropicContentBlock::ToolResult {
                            tool_use_id: tool_use_id.clone(),
                            content: content.clone(),
                        },
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
            tools: if req.tools.is_empty() {
                None
            } else {
                Some(
                    req.tools
                        .into_iter()
                        .map(|t| AnthropicTool {
                            name: t.name,
                            description: t.description,
                            input_schema: t.input_schema,
                        })
                        .collect(),
                )
            },
            tool_choice: match req.tool_choice {
                ports::ToolChoice::Auto => Some(serde_json::json!({ "type": "auto" })),
                ports::ToolChoice::None => None,
                ports::ToolChoice::Required => Some(serde_json::json!({ "type": "any" })),
                ports::ToolChoice::Tool { name } => {
                    Some(serde_json::json!({ "type": "tool", "name": name }))
                }
            },
            stream: None,
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

        let mut text = String::new();
        let mut tool_calls = Vec::new();

        for block in &parsed.content {
            match block {
                ContentBlock::Text { text: t } => text.push_str(t),
                ContentBlock::ToolUse { id, name, input } => {
                    tool_calls.push(ports::ToolCall {
                        id: id.clone(),
                        name: name.clone(),
                        arguments: input.clone(),
                    });
                }
            }
        }

        Ok(CompletionResponse {
            text,
            tool_calls,
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
                            _ => {}
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
            stream: None,
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
        req: CompletionRequest,
    ) -> Result<ports::BoxStream<'static, Result<ports::StreamChunk, LlmError>>, LlmError> {
        let messages: Vec<AnthropicMessage> = req
            .messages
            .iter()
            .map(|m| AnthropicMessage {
                role: match m.role {
                    Role::User => "user".into(),
                    Role::Assistant => "assistant".into(),
                    Role::Tool => "user".into(),
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
                        ports::MessageContent::ToolUse { id, name, input } => {
                            AnthropicContentBlock::ToolUse {
                                id: id.clone(),
                                name: name.clone(),
                                input: input.clone(),
                            }
                        }
                        ports::MessageContent::ToolResult {
                            tool_use_id,
                            content,
                        } => AnthropicContentBlock::ToolResult {
                            tool_use_id: tool_use_id.clone(),
                            content: content.clone(),
                        },
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
            tools: if req.tools.is_empty() {
                None
            } else {
                Some(
                    req.tools
                        .into_iter()
                        .map(|t| AnthropicTool {
                            name: t.name,
                            description: t.description,
                            input_schema: t.input_schema,
                        })
                        .collect(),
                )
            },
            tool_choice: match req.tool_choice {
                ports::ToolChoice::Auto => Some(serde_json::json!({ "type": "auto" })),
                ports::ToolChoice::None => None,
                ports::ToolChoice::Required => Some(serde_json::json!({ "type": "any" })),
                ports::ToolChoice::Tool { name } => {
                    Some(serde_json::json!({ "type": "tool", "name": name }))
                }
            },
            stream: Some(true),
        };

        let resp = self
            .http
            .post(ANTHROPIC_API_URL)
            .headers(self.headers()?)
            .json(&body)
            .send()
            .await
            .map_err(|e| LlmError::Http(e.to_string()))?;

        if resp.status() != 200 {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(LlmError::ProviderStatus { status, body });
        }

        use futures::{StreamExt, TryStreamExt};
        let bytes_stream = resp.bytes_stream();
        let mut buffer = String::new();

        struct ToolState {
            id: String,
            name: String,
            args: String,
        }
        let mut current_tool: Option<ToolState> = None;

        let chunk_stream = bytes_stream
            .map(move |res| {
                let bytes = res.map_err(|e| LlmError::Http(e.to_string()))?;
                let chunk = String::from_utf8_lossy(&bytes);
                buffer.push_str(&chunk);

                let mut chunks = Vec::new();
                while let Some(pos) = buffer.find("\n\n") {
                    let full_block = buffer.drain(..pos + 2).collect::<String>();
                    for line in full_block.lines() {
                        if let Some(data) = line.strip_prefix("data: ") {
                            if let Ok(event) = serde_json::from_str::<AnthropicStreamEvent>(data) {
                                match event {
                                    AnthropicStreamEvent::ContentBlockStart {
                                        content_block: ContentBlock::ToolUse { id, name, .. },
                                        ..
                                    } => {
                                        current_tool = Some(ToolState {
                                            id: id.clone(),
                                            name: name.clone(),
                                            args: String::new(),
                                        });
                                        chunks.push(Ok(ports::StreamChunk::ToolCallStart {
                                            id,
                                            name,
                                        }));
                                    }
                                    AnthropicStreamEvent::ContentBlockDelta { delta, .. } => {
                                        match delta {
                                            ContentBlockDelta::TextDelta { text } => {
                                                chunks.push(Ok(ports::StreamChunk::TextDelta {
                                                    text,
                                                }));
                                            }
                                            ContentBlockDelta::InputJsonDelta { partial_json } => {
                                                if let Some(ref mut tool) = current_tool {
                                                    tool.args.push_str(&partial_json);
                                                    chunks.push(Ok(
                                                        ports::StreamChunk::ToolCallArgsDelta {
                                                            id: tool.id.clone(),
                                                            delta: partial_json,
                                                        },
                                                    ));
                                                }
                                            }
                                        }
                                    }
                                    AnthropicStreamEvent::ContentBlockStop { .. } => {
                                        if let Some(tool) = current_tool.take() {
                                            let arguments = serde_json::from_str(&tool.args)
                                                .unwrap_or(serde_json::Value::Null);
                                            chunks.push(Ok(ports::StreamChunk::ToolCallEnd {
                                                id: tool.id,
                                                name: tool.name,
                                                arguments,
                                            }));
                                        }
                                    }
                                    AnthropicStreamEvent::MessageDelta { delta, .. } => {
                                        if let Some(reason) = delta.stop_reason {
                                            let stop_reason = match reason.as_str() {
                                                "end_turn" => ports::StopReason::EndTurn,
                                                "tool_use" => ports::StopReason::ToolUse,
                                                "max_tokens" => ports::StopReason::MaxTokens,
                                                "stop_sequence" => ports::StopReason::StopSequence,
                                                _ => ports::StopReason::Other(reason),
                                            };
                                            chunks
                                                .push(Ok(ports::StreamChunk::Done { stop_reason }));
                                        }
                                    }
                                    AnthropicStreamEvent::Error { error } => {
                                        chunks.push(Err(LlmError::Other(error.message)));
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                Ok(futures::stream::iter(chunks))
            })
            .try_flatten();

        Ok(Box::pin(chunk_stream))
    }
}
