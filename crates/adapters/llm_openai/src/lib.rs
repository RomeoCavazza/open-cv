use async_trait::async_trait;
use ports::{CompletionRequest, CompletionResponse, ExtractionRequest, LlmClient, LlmError, Role};
use serde::{Deserialize, Serialize};
use tracing::instrument;

const DEFAULT_MODEL: &str = "gpt-4o-2024-08-06"; // Modèle supportant Structured Outputs avec strict mode
const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";

pub struct OpenAiClient {
    api_key: String,
    model: String,
    http: reqwest::Client,
}

impl OpenAiClient {
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
        h.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.api_key)).unwrap(),
        );
        h.insert(
            reqwest::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        h
    }
}

// ─── OpenAI API types ──────────────────────────────────────────────────

#[derive(Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<OpenAiMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<OpenAiResponseFormat>,
}

#[derive(Serialize)]
struct OpenAiMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct OpenAiResponseFormat {
    #[serde(rename = "type")]
    format_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    json_schema: Option<OpenAiJsonSchema>,
}

#[derive(Serialize)]
struct OpenAiJsonSchema {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    schema: serde_json::Value,
    strict: bool,
}

#[derive(Deserialize)]
struct OpenAiResponse {
    choices: Vec<Choice>,
    usage: Usage,
    model: String,
}

#[derive(Deserialize)]
struct Choice {
    message: OpenAiResponseMessage,
}

#[derive(Deserialize)]
struct OpenAiResponseMessage {
    content: Option<String>,
}

#[derive(Deserialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
}

#[derive(Deserialize)]
struct OpenAiError {
    error: OpenAiErrorDetail,
}

#[derive(Deserialize)]
struct OpenAiErrorDetail {
    message: String,
    #[serde(rename = "type")]
    error_type: String,
}

// ─── Trait implementation ──────────────────────────────────────────────

#[async_trait]
impl LlmClient for OpenAiClient {
    #[instrument(skip(self, req), fields(model = %self.model))]
    async fn complete(&self, req: CompletionRequest) -> Result<CompletionResponse, LlmError> {
        let mut messages = Vec::new();
        if let Some(sys) = req.system {
            messages.push(OpenAiMessage {
                role: "system".into(),
                content: sys,
            });
        }
        for m in req.messages {
            messages.push(OpenAiMessage {
                role: match m.role {
                    Role::User => "user".into(),
                    Role::Assistant => "assistant".into(),
                },
                content: m.content,
            });
        }

        let body = OpenAiRequest {
            model: req.model.unwrap_or_else(|| self.model.clone()),
            messages,
            max_tokens: req.max_tokens,
            temperature: req.temperature,
            response_format: None,
        };

        let start = std::time::Instant::now();

        let resp = self
            .http
            .post(OPENAI_API_URL)
            .headers(self.headers())
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
            if let Ok(err) = serde_json::from_str::<OpenAiError>(&raw) {
                return Err(LlmError::ProviderStatus {
                    status,
                    body: format!("{}: {}", err.error.error_type, err.error.message),
                });
            }
            return Err(LlmError::ProviderStatus { status, body: raw });
        }

        let latency_ms = start.elapsed().as_millis() as u64;
        let parsed: OpenAiResponse =
            serde_json::from_str(&raw).map_err(|e| LlmError::Json(e.to_string()))?;

        let text = parsed
            .choices
            .first()
            .and_then(|c| c.message.content.clone())
            .unwrap_or_default();

        Ok(CompletionResponse {
            text,
            model: parsed.model,
            tokens_in: parsed.usage.prompt_tokens,
            tokens_out: parsed.usage.completion_tokens,
            latency_ms,
        })
    }

    #[instrument(skip(self, req), fields(model = %self.model, schema = %req.schema_name))]
    async fn extract(&self, req: ExtractionRequest) -> Result<serde_json::Value, LlmError> {
        let mut messages = Vec::new();
        if let Some(sys) = req.system {
            messages.push(OpenAiMessage {
                role: "system".into(),
                content: sys,
            });
        }

        let user_content = if req.instruction.is_empty() {
            req.input.clone()
        } else {
            format!("{}\n\n---\n\n{}", req.instruction, req.input)
        };

        messages.push(OpenAiMessage {
            role: "user".into(),
            content: user_content,
        });

        let body = OpenAiRequest {
            model: req.model.unwrap_or_else(|| self.model.clone()),
            messages,
            max_tokens: req.max_tokens,
            temperature: None,
            response_format: Some(OpenAiResponseFormat {
                format_type: "json_schema".into(),
                json_schema: Some(OpenAiJsonSchema {
                    name: req.schema_name,
                    description: Some(req.schema_description),
                    schema: req.json_schema,
                    strict: true,
                }),
            }),
        };

        let start = std::time::Instant::now();

        let resp = self
            .http
            .post(OPENAI_API_URL)
            .headers(self.headers())
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
            if let Ok(err) = serde_json::from_str::<OpenAiError>(&raw) {
                return Err(LlmError::ProviderStatus {
                    status,
                    body: format!("{}: {}", err.error.error_type, err.error.message),
                });
            }
            return Err(LlmError::ProviderStatus { status, body: raw });
        }

        let _latency_ms = start.elapsed().as_millis() as u64;
        let parsed: OpenAiResponse =
            serde_json::from_str(&raw).map_err(|e| LlmError::Json(e.to_string()))?;

        let content = parsed
            .choices
            .first()
            .and_then(|c| c.message.content.clone())
            .ok_or_else(|| LlmError::BadStructuredOutput("Réponse vide d'OpenAI".into()))?;

        let json = serde_json::from_str(&content).map_err(|e| LlmError::Json(e.to_string()))?;

        Ok(json)
    }

    fn name(&self) -> &'static str {
        "openai"
    }
}
