//! Le trait `Embedder` — abstraction sur les fournisseurs d'embeddings.
//!
//! Séparé de `LlmClient` à dessein : un fournisseur d'embeddings n'est pas
//! nécessairement un fournisseur de complétion (et vice-versa). Anthropic ne
//! fait pas d'embeddings ; Voyage et certains modèles locaux ne font que ça.
//!
//! Implémentations possibles :
//! - `adapters/embed_voyage` (cloud, multilingue, payant)
//! - `adapters/embed_openai` (cloud, moins bon en FR mais 5x moins cher)
//! - `adapters/embed_fastembed` (local, BGE-M3, zéro coût)
//!
//! La dimension du vecteur dépend du modèle :
//! - voyage-3                : 1024
//! - openai text-embedding-3 : 1536 (small) ou 3072 (large)
//! - BGE-M3                  : 1024
//!
//! La colonne SQL est `vector(1024)` par défaut (cf. migration). Si tu
//! changes pour un modèle 1536, il faudra une migration ALTER COLUMN.

use async_trait::async_trait;
use thiserror::Error;

/// Mode d'embedding : indique au fournisseur si on indexe (document)
/// ou si on cherche (query). Certains modèles (Voyage, BGE-M3) tirent un
/// gain de qualité de cette distinction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmbedMode {
    /// Embedding pour stockage / indexation (chunks de profil, raw_text d'offres).
    Document,
    /// Embedding pour requête (texte de l'offre lors d'un retrieve).
    Query,
}

#[async_trait]
pub trait Embedder: Send + Sync {
    /// Embed une liste de textes. La sortie est un vecteur par texte,
    /// dans l'ordre.
    async fn embed(&self, texts: &[&str], mode: EmbedMode) -> Result<Vec<Vec<f32>>, EmbedError>;

    /// Dimension des vecteurs produits. Doit matcher `vector(N)` dans le SQL.
    fn dimension(&self) -> usize;

    /// Identifiant du provider, pour `llm_calls.provider`.
    fn name(&self) -> &'static str;
}

#[derive(Debug, Error)]
pub enum EmbedError {
    #[error("erreur HTTP : {0}")]
    Http(String),

    #[error("provider a renvoyé un statut {status}: {body}")]
    ProviderStatus { status: u16, body: String },

    #[error("rate limit, retry dans {retry_after_ms}ms")]
    RateLimit { retry_after_ms: u64 },

    #[error("dimension inattendue : attendu {expected}, reçu {got}")]
    UnexpectedDimension { expected: usize, got: usize },

    #[error("autre : {0}")]
    Other(String),
}
