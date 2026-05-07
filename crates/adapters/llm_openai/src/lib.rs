use async_trait::async_trait;
use base64::Engine;
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

    fn headers(&self) -> Result<reqwest::header::HeaderMap, LlmError> {
        use reqwest::header::{HeaderMap, HeaderValue};
        let mut h = HeaderMap::new();
        h.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.api_key))
                .map_err(|e| LlmError::Config(e.to_string()))?,
        );
        h.insert(
            reqwest::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        Ok(h)
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
    content: OpenAiContent,
}

#[derive(Serialize)]
#[serde(untagged)]
enum OpenAiContent {
    String(String),
    Blocks(Vec<OpenAiContentBlock>),
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum OpenAiContentBlock {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: OpenAiImageUrl },
}

#[derive(Serialize)]
struct OpenAiImageUrl {
    url: String, // "data:image/jpeg;base64,..."
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
    finish_reason: Option<String>,
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
                content: OpenAiContent::String(sys),
            });
        }
        for m in req.messages {
            messages.push(OpenAiMessage {
                role: match m.role {
                    Role::User => "user".into(),
                    Role::Assistant => "assistant".into(),
                },
                content: OpenAiContent::Blocks(
                    m.content
                        .iter()
                        .map(|c| match c {
                            ports::MessageContent::Text(text) => {
                                OpenAiContentBlock::Text { text: text.clone() }
                            }
                            ports::MessageContent::Image { data, content_type } => {
                                OpenAiContentBlock::ImageUrl {
                                    image_url: OpenAiImageUrl {
                                        url: format!(
                                            "data:{};base64,{}",
                                            content_type,
                                            base64::engine::general_purpose::STANDARD.encode(data)
                                        ),
                                    },
                                }
                            }
                        })
                        .collect(),
                ),
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
            .headers(self.headers()?)
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
    async fn extract(&self, req: ExtractionRequest) -> Result<ports::ExtractionResponse, LlmError> {
        let mut messages = Vec::new();
        if let Some(sys) = req.system {
            messages.push(OpenAiMessage {
                role: "system".into(),
                content: OpenAiContent::String(sys),
            });
        }

        messages.push(OpenAiMessage {
            role: "user".into(),
            content: OpenAiContent::Blocks({
                let mut blocks = Vec::new();
                if !req.instruction.is_empty() {
                    blocks.push(OpenAiContentBlock::Text {
                        text: req.instruction.clone(),
                    });
                }
                for input_content in req.input {
                    match input_content {
                        ports::MessageContent::Text(text) => {
                            blocks.push(OpenAiContentBlock::Text { text })
                        }
                        ports::MessageContent::Image { data, content_type } => {
                            blocks.push(OpenAiContentBlock::ImageUrl {
                                image_url: OpenAiImageUrl {
                                    url: format!(
                                        "data:{};base64,{}",
                                        content_type,
                                        base64::engine::general_purpose::STANDARD.encode(data)
                                    ),
                                },
                            })
                        }
                    }
                }
                blocks
            }),
        });

        let body = OpenAiRequest {
            model: req.model.unwrap_or_else(|| self.model.clone()),
            messages,
            max_tokens: req.max_tokens,
            temperature: None,
            response_format: if req.json_schema.is_string()
                && req.json_schema.as_str() == Some("json")
            {
                Some(OpenAiResponseFormat {
                    format_type: "json_object".into(),
                    json_schema: None,
                })
            } else {
                Some(OpenAiResponseFormat {
                    format_type: "json_schema".into(),
                    json_schema: Some(OpenAiJsonSchema {
                        name: req.schema_name.clone(),
                        description: Some(req.schema_description),
                        schema: req.json_schema,
                        strict: false,
                    }),
                })
            },
        };

        let start = std::time::Instant::now();

        let resp = self
            .http
            .post(OPENAI_API_URL)
            .headers(self.headers()?)
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

        let choice = parsed.choices.first().ok_or_else(|| {
            LlmError::BadStructuredOutput("Aucun choix renvoyé par OpenAI".into())
        })?;

        if let Some(reason) = &choice.finish_reason {
            if reason == "length" {
                return Err(LlmError::Truncated {
                    step: req.schema_name,
                    partial_payload: raw,
                });
            }
        }

        let content = choice
            .message
            .content
            .clone()
            .ok_or_else(|| LlmError::BadStructuredOutput("Réponse vide d'OpenAI".into()))?;

        let json = serde_json::from_str(&content).map_err(|e| LlmError::Json(e.to_string()))?;

        Ok(ports::ExtractionResponse { value: json, raw })
    }

    fn name(&self) -> &'static str {
        "openai"
    }

    async fn stream(
        &self,
        _req: CompletionRequest,
    ) -> Result<ports::BoxStream<'static, Result<String, LlmError>>, LlmError> {
        Err(LlmError::Other(
            "Streaming non implémenté pour OpenAI".into(),
        ))
    }
}
