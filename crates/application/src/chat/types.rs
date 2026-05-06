use domain::Instance;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    pub instance_id: Option<String>,
    pub llm_provider: String,
    #[serde(default)]
    pub attachments: Vec<ChatAttachment>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChatAttachment {
    pub name: String,
    pub content_type: String,
    pub data: String, // Base64 (data:image/jpeg;base64,...)
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub updated_instance: Option<Instance>,
    pub message: String,
}

#[derive(Debug, Clone, Copy)]
pub enum ChatOutputKind {
    MessageOnly,
    Mutation,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ChatMutationOutput {
    #[serde(default)]
    pub resume: Option<domain::Resume>,
    #[serde(default)]
    pub cover: Option<domain::CoverLetter>,
    pub message: String,
}
