use async_trait::async_trait;
use ports::{CompletionRequest, CompletionResponse, ExtractionRequest, LlmClient, LlmError, Role};
use serde::{Deserialize, Serialize};
use tracing::instrument;

pub struct OllamaClient {
    base_url: String,
    model: String,
    http: reqwest::Client,
}

impl OllamaClient {
    pub fn new(base_url: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            model: model.into(),
            http: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(300)) // Ollama peut être lent en local
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
}

#[derive(Deserialize)]
struct OllamaChatResponse {
    message: OllamaMessage,
    prompt_eval_count: Option<u32>,
    eval_count: Option<u32>,
    #[allow(dead_code)]
    total_duration: Option<u64>,
}

#[async_trait]
impl LlmClient for OllamaClient {
    #[instrument(skip(self, req), fields(model = %self.model))]
    async fn complete(&self, req: CompletionRequest) -> Result<CompletionResponse, LlmError> {
        let mut messages = Vec::new();
        if let Some(sys) = req.system {
            messages.push(OllamaMessage {
                role: "system".into(),
                content: sys,
            });
        }
        for m in req.messages {
            messages.push(OllamaMessage {
                role: match m.role {
                    Role::User => "user".into(),
                    Role::Assistant => "assistant".into(),
                },
                content: m.content,
            });
        }

        let body = OllamaChatRequest {
            model: req.model.unwrap_or_else(|| self.model.clone()),
            messages,
            stream: false,
            format: None,
            options: req
                .temperature
                .map(|t| serde_json::json!({ "temperature": t })),
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
            text: parsed.message.content,
            model: self.model.clone(),
            tokens_in: parsed.prompt_eval_count.unwrap_or(0),
            tokens_out: parsed.eval_count.unwrap_or(0),
            latency_ms: start.elapsed().as_millis() as u64,
        })
    }

    #[instrument(skip(self, req), fields(model = %self.model, schema = %req.schema_name))]
    async fn extract(&self, req: ExtractionRequest) -> Result<serde_json::Value, LlmError> {
        let mut messages = Vec::new();
        if let Some(sys) = req.system {
            messages.push(OllamaMessage {
                role: "system".into(),
                content: sys,
            });
        }

        let instruction = format!(
            "{}\n\nTu DOIS répondre UNIQUEMENT avec un objet JSON valide respectant ce schéma :\n{}",
            req.instruction,
            serde_json::to_string_pretty(&req.json_schema).unwrap()
        );

        messages.push(OllamaMessage {
            role: "user".into(),
            content: format!("{}\n\n---\n\n{}", instruction, req.input),
        });

        let body = OllamaChatRequest {
            model: req.model.unwrap_or_else(|| self.model.clone()),
            messages,
            stream: false,
            format: Some(req.json_schema.clone()),
            options: None,
        };

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
        let content = parsed.message.content.trim();
        let cleaned = if content.starts_with("```") {
            content
                .trim_start_matches('`')
                .trim_start_matches("json")
                .trim_end_matches('`')
                .trim()
        } else {
            content
        };

        let json = serde_json::from_str(cleaned).map_err(|e| {
            tracing::error!("Failed to parse JSON from Ollama: {}. Content: {}", e, cleaned);
            LlmError::Json(e.to_string())
        })?;

        Ok(json)
    }

    fn name(&self) -> &'static str {
        "ollama"
    }
}

#[async_trait]
impl ports::Embedder for OllamaClient {
    #[instrument(skip(self, texts), fields(model = %self.model))]
    async fn embed(&self, texts: &[&str], _mode: ports::EmbedMode) -> Result<Vec<Vec<f32>>, ports::EmbedError> {
        let mut results = Vec::new();
        for text in texts {
            let body = serde_json::json!({
                "model": self.model,
                "prompt": text,
                "stream": false
            });

            let url = format!("{}/api/embeddings", self.base_url);
            let resp = self.http.post(&url)
                .json(&body)
                .send()
                .await
                .map_err(|e| ports::EmbedError::Http(e.to_string()))?;

            let status = resp.status().as_u16();
            if status != 200 {
                let err_body = resp.text().await.unwrap_or_default();
                return Err(ports::EmbedError::ProviderStatus { status, body: err_body });
            }

            #[derive(Deserialize)]
            struct OllamaEmbedResponse {
                embedding: Vec<f32>,
            }

            let parsed: OllamaEmbedResponse = resp.json().await
                .map_err(|e| ports::EmbedError::Other(e.to_string()))?;
            
            results.push(parsed.embedding);
        }
        Ok(results)
    }

    fn dimension(&self) -> usize {
        4096 // Qwen 2.5:7b a typiquement 4096 dimensions
    }

    fn name(&self) -> &'static str {
        "ollama"
    }
}
