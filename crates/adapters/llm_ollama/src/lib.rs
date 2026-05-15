use async_trait::async_trait;
use base64::Engine;
use futures::{StreamExt, TryStreamExt};
use ports::{
    BoxStream, CompletionRequest, CompletionResponse, ExtractionRequest, LlmClient, LlmError, Role,
};
use serde::{Deserialize, Serialize};
use tracing::instrument;

pub struct OllamaClient {
    base_url: String,
    model: String,
    dimension: usize,
    http: reqwest::Client,
}

impl OllamaClient {
    pub fn new(base_url: impl Into<String>, model: impl Into<String>, dimension: usize) -> Self {
        Self {
            base_url: base_url.into(),
            model: model.into(),
            dimension,
            http: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(600)) // Ollama peut être lent en local
                .build()
                .expect("client HTTP valide"),
        }
    }
}

#[derive(Serialize)]
struct OllamaChatRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<OllamaTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<serde_json::Value>,
}

#[derive(Serialize)]
struct OllamaTool {
    #[serde(rename = "type")]
    tool_type: String,
    function: OllamaFunction,
}

#[derive(Serialize)]
struct OllamaFunction {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Serialize, Deserialize, Clone)]
struct OllamaMessage {
    role: String,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    images: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<OllamaToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct OllamaToolCall {
    #[allow(dead_code)]
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: OllamaFunctionCall,
}

#[derive(Serialize, Deserialize, Clone)]
struct OllamaFunctionCall {
    pub name: String,
    pub arguments: serde_json::Value,
}

#[derive(Deserialize)]
struct OllamaChatResponse {
    message: Option<OllamaMessage>,
    #[serde(default)]
    content: Option<String>,
    done: bool,
    done_reason: Option<String>,
    prompt_eval_count: Option<u32>,
    eval_count: Option<u32>,
}

#[async_trait]
impl LlmClient for OllamaClient {
    #[instrument(skip(self, req), fields(model = %self.model))]
    async fn complete(&self, req: CompletionRequest) -> Result<CompletionResponse, LlmError> {
        let use_json_format = req
            .system
            .as_ref()
            .map(|s| s.contains("JSON"))
            .unwrap_or(false);

        let mut messages = Vec::new();
        if let Some(sys) = req.system {
            messages.push(OllamaMessage {
                role: "system".into(),
                content: sys,
                images: None,
                tool_calls: None,
                tool_call_id: None,
            });
        }
        for m in req.messages {
            let mut text_content = String::new();
            let mut images = Vec::new();
            let mut tool_calls = Vec::new();
            let mut tool_call_id = None;

            for content in m.content {
                match content {
                    ports::MessageContent::Text(t) => text_content.push_str(&t),
                    ports::MessageContent::Image { data, .. } => {
                        images.push(base64::engine::general_purpose::STANDARD.encode(data));
                    }
                    ports::MessageContent::ToolUse { id, name, input } => {
                        tool_calls.push(OllamaToolCall {
                            id: Some(id),
                            tool_type: "function".into(),
                            function: OllamaFunctionCall {
                                name,
                                arguments: input,
                            },
                        });
                    }
                    ports::MessageContent::ToolResult {
                        tool_use_id: tid,
                        content: tc,
                    } => {
                        text_content = tc;
                        tool_call_id = Some(tid);
                    }
                }
            }
            messages.push(OllamaMessage {
                role: match m.role {
                    Role::User => "user".into(),
                    Role::Assistant => "assistant".into(),
                    Role::Tool => "tool".into(),
                },
                content: text_content,
                images: if images.is_empty() {
                    None
                } else {
                    Some(images)
                },
                tool_calls: if tool_calls.is_empty() {
                    None
                } else {
                    Some(tool_calls)
                },
                tool_call_id,
            });
        }

        let body = OllamaChatRequest {
            model: req.model.unwrap_or_else(|| self.model.clone()),
            messages,
            stream: false,
            format: if use_json_format {
                Some(serde_json::json!("json"))
            } else {
                None
            },
            options: req
                .temperature
                .map(|t| serde_json::json!({ "temperature": t, "num_ctx": 16384 }))
                .or_else(|| Some(serde_json::json!({ "num_ctx": 16384 }))),
            tools: if req.tools.is_empty() {
                None
            } else {
                Some(
                    req.tools
                        .into_iter()
                        .map(|t| OllamaTool {
                            tool_type: "function".into(),
                            function: OllamaFunction {
                                name: t.name,
                                description: t.description,
                                parameters: t.input_schema,
                            },
                        })
                        .collect(),
                )
            },
            tool_choice: match req.tool_choice {
                ports::ToolChoice::Auto => Some(serde_json::json!("auto")),
                ports::ToolChoice::None => None,
                ports::ToolChoice::Required => Some(serde_json::json!("required")),
                ports::ToolChoice::Tool { name } => {
                    Some(serde_json::json!({ "type": "function", "function": { "name": name } }))
                }
            },
        };

        let start = std::time::Instant::now();
        let url = format!("{}/api/chat", self.base_url);

        let resp = self
            .http
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| LlmError::Http(e.to_string()))?;

        let status = resp.status().as_u16();
        let raw = resp
            .text()
            .await
            .map_err(|e| LlmError::Http(e.to_string()))?;

        if status != 200 {
            return Err(LlmError::ProviderStatus { status, body: raw });
        }

        let parsed: OllamaChatResponse =
            serde_json::from_str(&raw).map_err(|e| LlmError::Json(e.to_string()))?;

        let msg = parsed
            .message
            .ok_or_else(|| LlmError::Json("Missing message".into()))?;

        let tool_calls = msg
            .tool_calls
            .unwrap_or_default()
            .into_iter()
            .map(|tc| ports::ToolCall {
                id: tc.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
                name: tc.function.name,
                arguments: tc.function.arguments,
            })
            .collect();

        Ok(CompletionResponse {
            text: msg.content,
            tool_calls,
            model: self.model.clone(),
            tokens_in: parsed.prompt_eval_count.unwrap_or(0),
            tokens_out: parsed.eval_count.unwrap_or(0),
            latency_ms: start.elapsed().as_millis() as u64,
        })
    }

    #[instrument(skip(self, req), fields(model = %self.model, schema = %req.schema_name))]
    async fn extract(&self, req: ExtractionRequest) -> Result<ports::ExtractionResponse, LlmError> {
        let mut messages = Vec::new();
        if let Some(sys) = req.system {
            messages.push(OllamaMessage {
                role: "system".into(),
                content: sys,
                images: None,
                tool_calls: None,
                tool_call_id: None,
            });
        }

        let instruction = format!(
            "{}\n\nTu DOIS répondre UNIQUEMENT avec un objet JSON valide respectant ce schéma :\n{}",
            req.instruction,
            serde_json::to_string_pretty(&req.json_schema).expect("schema is always serializable")
        );

        let mut text_input = instruction;
        let mut images = Vec::new();
        for content in req.input {
            match content {
                ports::MessageContent::Text(t) => {
                    text_input.push_str("\n\n---\n\n");
                    text_input.push_str(&t);
                }
                ports::MessageContent::Image { data, .. } => {
                    images.push(base64::engine::general_purpose::STANDARD.encode(data));
                }
                _ => {}
            }
        }

        messages.push(OllamaMessage {
            role: "user".into(),
            content: text_input,
            images: if images.is_empty() {
                None
            } else {
                Some(images)
            },
            tool_calls: None,
            tool_call_id: None,
        });

        let mut cleaned_schema = req.json_schema.clone();
        let mut use_full_schema = true;

        if let Some(obj) = cleaned_schema.as_object_mut() {
            obj.remove("$schema");
            obj.remove("title");

            let schema_str = serde_json::to_string(&cleaned_schema).unwrap_or_default();
            if schema_str.contains("\"$ref\"") {
                tracing::warn!(
                    "Schema for {} contains $ref, falling back to 'json' format for Ollama",
                    req.schema_name
                );
                use_full_schema = false;
            }
        }

        let body = OllamaChatRequest {
            model: req.model.unwrap_or_else(|| self.model.clone()),
            messages,
            stream: false,
            format: if use_full_schema {
                Some(cleaned_schema)
            } else {
                Some(serde_json::json!("json"))
            },
            options: Some(serde_json::json!({
                "num_ctx": 16384,
                "num_predict": req.max_tokens
            })),
            tools: None,
            tool_choice: None,
        };

        let url = format!("{}/api/chat", self.base_url);
        let mut resp = self
            .http
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| LlmError::Http(e.to_string()))?;

        let mut status = resp.status().as_u16();
        let mut raw = resp
            .text()
            .await
            .map_err(|e| LlmError::Http(e.to_string()))?;

        if status == 500 && raw.contains("invalid JSON schema in format") {
            tracing::warn!("Ollama rejected the JSON schema. Retrying with format: 'json'...");
            let mut fallback_body = body;
            fallback_body.format = Some(serde_json::json!("json"));

            resp = self
                .http
                .post(&url)
                .json(&fallback_body)
                .send()
                .await
                .map_err(|e| LlmError::Http(e.to_string()))?;

            status = resp.status().as_u16();
            raw = resp
                .text()
                .await
                .map_err(|e| LlmError::Http(e.to_string()))?;
        }

        if status != 200 {
            return Err(LlmError::ProviderStatus { status, body: raw });
        }

        let parsed: OllamaChatResponse =
            serde_json::from_str(&raw).map_err(|e| LlmError::Json(e.to_string()))?;

        if let Some(reason) = &parsed.done_reason {
            if reason == "length" {
                return Err(LlmError::Truncated {
                    step: req.schema_name,
                    partial_payload: raw,
                });
            }
        }

        let msg = parsed
            .message
            .ok_or_else(|| LlmError::Json("Missing message".into()))?;
        let content = msg.content.trim();
        let cleaned = if content.starts_with("```") {
            content
                .trim_start_matches('`')
                .trim_start_matches("json")
                .trim_end_matches('`')
                .trim()
        } else {
            content
        };

        let json = serde_json::from_str(cleaned).map_err(|e| LlmError::ParseFailed {
            step: req.schema_name,
            error: e.to_string(),
            payload: cleaned.to_string(),
        })?;

        Ok(ports::ExtractionResponse { value: json, raw })
    }
    fn name(&self) -> &'static str {
        "ollama"
    }

    async fn stream(
        &self,
        req: CompletionRequest,
    ) -> Result<BoxStream<'static, Result<ports::StreamChunk, LlmError>>, LlmError> {
        let mut messages = Vec::new();
        if let Some(sys) = req.system {
            messages.push(OllamaMessage {
                role: "system".into(),
                content: sys,
                images: None,
                tool_calls: None,
                tool_call_id: None,
            });
        }
        for m in req.messages {
            let mut text_content = String::new();
            let mut images = Vec::new();
            let mut tool_calls = Vec::new();
            let mut tool_call_id = None;

            for content in m.content {
                match content {
                    ports::MessageContent::Text(t) => text_content.push_str(&t),
                    ports::MessageContent::Image { data, .. } => {
                        images.push(base64::engine::general_purpose::STANDARD.encode(data));
                    }
                    ports::MessageContent::ToolUse { id, name, input } => {
                        tool_calls.push(OllamaToolCall {
                            id: Some(id),
                            tool_type: "function".into(),
                            function: OllamaFunctionCall {
                                name,
                                arguments: input,
                            },
                        });
                    }
                    ports::MessageContent::ToolResult {
                        tool_use_id: tid,
                        content: tc,
                    } => {
                        text_content = tc;
                        tool_call_id = Some(tid);
                    }
                }
            }
            messages.push(OllamaMessage {
                role: match m.role {
                    Role::User => "user".into(),
                    Role::Assistant => "assistant".into(),
                    Role::Tool => "tool".into(),
                },
                content: text_content,
                images: if images.is_empty() {
                    None
                } else {
                    Some(images)
                },
                tool_calls: if tool_calls.is_empty() {
                    None
                } else {
                    Some(tool_calls)
                },
                tool_call_id,
            });
        }

        let body = OllamaChatRequest {
            model: req.model.unwrap_or_else(|| self.model.clone()),
            messages,
            stream: true,
            format: None,
            options: req
                .temperature
                .map(|t| serde_json::json!({ "temperature": t, "num_ctx": 16384 }))
                .or_else(|| Some(serde_json::json!({ "num_ctx": 16384 }))),
            tools: if req.tools.is_empty() {
                None
            } else {
                Some(
                    req.tools
                        .into_iter()
                        .map(|t| OllamaTool {
                            tool_type: "function".into(),
                            function: OllamaFunction {
                                name: t.name,
                                description: t.description,
                                parameters: t.input_schema,
                            },
                        })
                        .collect(),
                )
            },
            tool_choice: match req.tool_choice {
                ports::ToolChoice::Auto => Some(serde_json::json!("auto")),
                ports::ToolChoice::None => None,
                ports::ToolChoice::Required => Some(serde_json::json!("required")),
                ports::ToolChoice::Tool { name } => {
                    Some(serde_json::json!({ "type": "function", "function": { "name": name } }))
                }
            },
        };

        let url = format!("{}/api/chat", self.base_url);
        let resp = self
            .http
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| LlmError::Http(e.to_string()))?;

        if resp.status() != 200 {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(LlmError::ProviderStatus { status, body });
        }

        let bytes_stream = resp.bytes_stream();

        struct StreamState {
            buffer: String,
            emitted_tool_calls: std::collections::HashSet<String>,
            had_any_tool_call: bool,
        }

        let state = StreamState {
            buffer: String::new(),
            emitted_tool_calls: std::collections::HashSet::new(),
            had_any_tool_call: false,
        };

        let chunk_stream = bytes_stream
            .map_err(|e| LlmError::Http(e.to_string()))
            .scan(state, |state, res| {
                let bytes = match res {
                    Ok(b) => b,
                    Err(e) => return futures::future::ready(Some(Err(e))),
                };

                let chunk = String::from_utf8_lossy(&bytes);
                state.buffer.push_str(&chunk);

                let mut chunks = Vec::new();
                while let Some(pos) = state.buffer.find('\n') {
                    let line = state.buffer.drain(..=pos).collect::<String>();
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }

                    if let Ok(parsed) = serde_json::from_str::<OllamaChatResponse>(line) {
                        if let Some(msg) = &parsed.message {
                            if let Some(tool_calls) = &msg.tool_calls {
                                for (idx, tc) in tool_calls.iter().enumerate() {
                                    // Utiliser un ID stable basé sur l'index si absent
                                    let stable_id =
                                        tc.id.clone().unwrap_or_else(|| format!("call_{}", idx));

                                    if !state.emitted_tool_calls.contains(&stable_id) {
                                        state.emitted_tool_calls.insert(stable_id.clone());
                                        state.had_any_tool_call = true;
                                        chunks.push(Ok(ports::StreamChunk::ToolCallStart {
                                            id: stable_id.clone(),
                                            name: tc.function.name.clone(),
                                        }));
                                    }

                                    chunks.push(Ok(ports::StreamChunk::ToolCallEnd {
                                        id: stable_id,
                                        name: tc.function.name.clone(),
                                        arguments: tc.function.arguments.clone(),
                                    }));
                                }
                            } else if !msg.content.is_empty() {
                                chunks.push(Ok(ports::StreamChunk::TextDelta {
                                    text: msg.content.clone(),
                                }));
                            }
                        } else if let Some(content) = &parsed.content {
                            if !content.is_empty() {
                                chunks.push(Ok(ports::StreamChunk::TextDelta {
                                    text: content.clone(),
                                }));
                            }
                        }

                        if parsed.done {
                            let mut stop_reason = match parsed.done_reason.as_deref() {
                                Some("stop") => ports::StopReason::EndTurn,
                                Some("length") => ports::StopReason::MaxTokens,
                                _ => ports::StopReason::EndTurn,
                            };

                            // Forcer ToolUse si des outils ont été appelés
                            if state.had_any_tool_call {
                                stop_reason = ports::StopReason::ToolUse;
                            }

                            chunks.push(Ok(ports::StreamChunk::Done { stop_reason }));
                        }
                    }
                }
                futures::future::ready(Some(Ok(futures::stream::iter(chunks))))
            })
            .try_flatten();

        Ok(Box::pin(chunk_stream))
    }
}

#[async_trait]
impl ports::Embedder for OllamaClient {
    #[instrument(skip(self, texts), fields(model = %self.model))]
    async fn embed(
        &self,
        texts: &[&str],
        _mode: ports::EmbedMode,
    ) -> Result<Vec<Vec<f32>>, ports::EmbedError> {
        let mut results = Vec::new();
        for text in texts {
            let body = serde_json::json!({
                "model": self.model,
                "prompt": text,
                "stream": false
            });

            let url = format!("{}/api/embeddings", self.base_url);
            let resp = self
                .http
                .post(&url)
                .json(&body)
                .send()
                .await
                .map_err(|e| ports::EmbedError::Http(e.to_string()))?;

            let status = resp.status().as_u16();
            if status != 200 {
                let err_body = resp.text().await.unwrap_or_default();
                return Err(ports::EmbedError::ProviderStatus {
                    status,
                    body: err_body,
                });
            }

            #[derive(Deserialize)]
            struct OllamaEmbedResponse {
                embedding: Vec<f32>,
            }

            let parsed: OllamaEmbedResponse = resp
                .json()
                .await
                .map_err(|e| ports::EmbedError::Other(e.to_string()))?;

            results.push(parsed.embedding);
        }
        Ok(results)
    }

    fn dimension(&self) -> usize {
        self.dimension
    }

    fn name(&self) -> &'static str {
        "ollama"
    }
}
