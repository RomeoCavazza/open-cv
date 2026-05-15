use domain::Instance;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    pub instance_id: Option<String>,
    #[serde(default)]
    pub conversation_id: Option<String>,
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
    pub resume: Option<ResumeScalarPatch>,
    #[serde(default)]
    pub cover: Option<CoverScalarPatch>,
    pub commit_message: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ResumeScalarPatch {
    #[serde(default)]
    pub identite: Option<domain::Identite>,
    #[serde(default)]
    pub accroche: Option<domain::Accroche>,
    #[serde(default)]
    pub contact: Option<domain::Contact>,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct CoverScalarPatch {
    #[serde(default)]
    pub expediteur: Option<domain::Expediteur>,
    #[serde(default)]
    pub destinataire: Option<domain::Destinataire>,
    #[serde(default)]
    pub objet: Option<domain::Objet>,
    #[serde(default)]
    pub signature: Option<domain::Signature>,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ResumeListTarget {
    Competences,
    Experiences,
    Formations,
    Projets,
    Langues,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CoverListTarget {
    Paragraphes,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ListOperation {
    Add,
    Update,
    Remove,
    Replace,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct EditResumeListOutput {
    pub target: ResumeListTarget,
    pub operation: ListOperation,
    #[serde(default)]
    pub index: Option<usize>,
    #[serde(default)]
    pub item: Option<serde_json::Value>,
    #[serde(default)]
    pub items: Option<Vec<serde_json::Value>>,
    pub commit_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct EditCoverListOutput {
    pub target: CoverListTarget,
    pub operation: ListOperation,
    #[serde(default)]
    pub index: Option<usize>,
    #[serde(default)]
    pub item: Option<serde_json::Value>,
    #[serde(default)]
    pub items: Option<Vec<serde_json::Value>>,
    pub commit_message: String,
}
