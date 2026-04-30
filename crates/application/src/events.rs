//! Bus d'événements pour le streaming SSE du progrès de génération.
//!
//! Le use case émet des `GenerationEvent` à chaque étape ; le handler HTTP
//! abonne un `tokio::broadcast` channel et streame vers le client.
//!
//! Pourquoi pas un trait `EventSink` injecté ? Trop de cérémonie pour ce qui
//! est un détail technique. `tokio::broadcast` est lock-free, multi-consumer,
//! et le use case n'a qu'à faire `tx.send(event)` — fire-and-forget.

use chrono::{DateTime, Utc};
use domain::InstanceId;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

/// Capacité du buffer broadcast. 64 = largement assez pour un pipeline qui
/// émet ~10 events par génération. Si un consumer est lent, les events
/// les plus anciens sont droppés (acceptable pour de la progress UI).
const BROADCAST_CAPACITY: usize = 64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationEvent {
    pub instance_id: InstanceId,
    pub timestamp: DateTime<Utc>,
    pub step: GenerationStep,
    pub status: StepStatus,
    /// Message humain-lisible pour l'UI ("3 chunks retenus", "Generated CV in 4.2s").
    pub message: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GenerationStep {
    Retrieve,
    Rerank,
    Plan,
    Restitution,
    Resume,
    CoverLetter,
    Validate,
    Persist,
    Done,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepStatus {
    Started,
    Done,
    Failed,
}

/// Bus d'événements partagé. Cloner pour distribuer aux abonnés.
#[derive(Clone)]
pub struct EventBus {
    tx: broadcast::Sender<GenerationEvent>,
}

impl EventBus {
    pub fn new() -> Self {
        let (tx, _rx) = broadcast::channel(BROADCAST_CAPACITY);
        Self { tx }
    }

    /// Émet un événement. Ne fait rien si personne n'écoute (`SendError` ignoré).
    pub fn emit(&self, event: GenerationEvent) {
        let _ = self.tx.send(event);
    }

    /// Helper : émet `step Started`.
    pub fn started(&self, instance_id: InstanceId, step: GenerationStep) {
        self.emit(GenerationEvent {
            instance_id,
            timestamp: Utc::now(),
            step,
            status: StepStatus::Started,
            message: None,
        });
    }

    /// Helper : émet `step Done` avec un message optionnel.
    pub fn done(
        &self,
        instance_id: InstanceId,
        step: GenerationStep,
        message: impl Into<Option<String>>,
    ) {
        self.emit(GenerationEvent {
            instance_id,
            timestamp: Utc::now(),
            step,
            status: StepStatus::Done,
            message: message.into(),
        });
    }

    /// Helper : émet `step Failed`.
    pub fn failed(
        &self,
        instance_id: InstanceId,
        step: GenerationStep,
        error: impl Into<String>,
    ) {
        self.emit(GenerationEvent {
            instance_id,
            timestamp: Utc::now(),
            step,
            status: StepStatus::Failed,
            message: Some(error.into()),
        });
    }

    /// Crée un nouvel abonné. Chaque abonné reçoit tous les events
    /// émis APRÈS sa souscription.
    pub fn subscribe(&self) -> broadcast::Receiver<GenerationEvent> {
        self.tx.subscribe()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}
