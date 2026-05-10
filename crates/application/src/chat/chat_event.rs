//! Événements typés émis par le `StreamOrchestrator` vers le handler SSE.
//!
//! Chaque variante de `ChatEvent` correspond à un `event:` SSE distinct,
//! permettant au frontend de dispatcher le traitement sans parser le payload.

use domain::Instance;
use serde::Serialize;

/// Événement typé du pipeline chat streaming.
///
/// Le `StreamOrchestrator` wraps les tokens `String` du `LlmClient` dans des
/// `ChatEvent::Token`, et injecte des `Status` / `Mutation` / `Done` aux
/// points clés du pipeline.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ChatEvent {
    /// Phase de calcul en cours (RAG, scraping, etc.)
    /// Affiché comme indicateur de liveliness dans l'UI.
    Status { content: String },

    /// Token de texte de la réponse IA (streaming mot-à-mot).
    Token { content: String },

    /// Mutation appliquée : l'instance mise à jour après modification du CV/LM.
    /// Le frontend peut rafraîchir les vues document sans recharger.
    Mutation { instance: Box<Instance> },

    /// Fin normale du stream.
    Done,

    /// Erreur survenue pendant le pipeline.
    Error { message: String },
}

impl ChatEvent {
    pub fn status(msg: impl Into<String>) -> Self {
        Self::Status {
            content: msg.into(),
        }
    }

    pub fn token(content: impl Into<String>) -> Self {
        Self::Token {
            content: content.into(),
        }
    }

    pub fn mutation(instance: Instance) -> Self {
        Self::Mutation {
            instance: Box::new(instance),
        }
    }

    pub fn error(msg: impl Into<String>) -> Self {
        Self::Error {
            message: msg.into(),
        }
    }
}
