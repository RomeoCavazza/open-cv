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
}

#[derive(Serialize, Deserialize)]
struct OllamaMessage {
    role: String,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    images: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct OllamaChatResponse {
    message: Option<OllamaMessage>,
    #[serde(default)]
    content: Option<String>,
    #[allow(dead_code)]
    done: bool,
    done_reason: Option<String>,
    prompt_eval_count: Option<u32>,
    eval_count: Option<u32>,
    #[allow(dead_code)]
    total_duration: Option<u64>,
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
            });
        }
        for m in req.messages {
            let mut text_content = String::new();
            let mut images = Vec::new();
            for content in m.content {
                match content {
                    ports::MessageContent::Text(t) => text_content.push_str(&t),
                    ports::MessageContent::Image { data, .. } => {
                        images.push(base64::engine::general_purpose::STANDARD.encode(data));
                    }
                }
            }
            messages.push(OllamaMessage {
                role: match m.role {
                    Role::User => "user".into(),
                    Role::Assistant => "assistant".into(),
                },
                content: text_content,
                images: if images.is_empty() {
                    None
                } else {
                    Some(images)
                },
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

        Ok(CompletionResponse {
            text: parsed
                .message
                .map(|m| m.content)
                .or(parsed.content)
                .unwrap_or_default(),
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

        let content = parsed
            .message
            .map(|m| m.content)
            .or(parsed.content)
            .unwrap_or_default();
        let content = content.trim();
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
    ) -> Result<BoxStream<'static, Result<String, LlmError>>, LlmError> {
        let mut messages = Vec::new();
        if let Some(sys) = req.system {
            messages.push(OllamaMessage {
                role: "system".into(),
                content: sys,
                images: None,
            });
        }
        for m in req.messages {
            let mut text_content = String::new();
            let mut images = Vec::new();
            for content in m.content {
                match content {
                    ports::MessageContent::Text(t) => text_content.push_str(&t),
                    ports::MessageContent::Image { data, .. } => {
                        images.push(base64::engine::general_purpose::STANDARD.encode(data));
                    }
                }
            }
            messages.push(OllamaMessage {
                role: match m.role {
                    Role::User => "user".into(),
                    Role::Assistant => "assistant".into(),
                },
                content: text_content,
                images: if images.is_empty() {
                    None
                } else {
                    Some(images)
                },
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
        let mut buffer = String::new();

        let token_stream = bytes_stream
            .map(move |res| {
                let bytes = res.map_err(|e| LlmError::Http(e.to_string()))?;
                let chunk = String::from_utf8_lossy(&bytes);
                buffer.push_str(&chunk);

                let mut tokens = Vec::new();
                while let Some(pos) = buffer.find('\n') {
                    let line = buffer.drain(..=pos).collect::<String>();
                    if line.trim().is_empty() {
                        continue;
                    }
                    if let Ok(parsed) = serde_json::from_str::<OllamaChatResponse>(&line) {
                        if let Some(msg) = parsed.message {
                            tokens.push(Ok(msg.content));
                        } else if let Some(content) = parsed.content {
                            tokens.push(Ok(content));
                        }
                    }
                }
                Ok(futures::stream::iter(tokens))
            })
            .try_flatten();

        Ok(Box::pin(token_stream))
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
