//! Métriques WebSocket : nombre de connexions, messages/s, etc.

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

/// Métriques WebSocket
#[derive(Clone, Default)]
pub struct WsMetrics {
    inner: Arc<Mutex<MetricsInner>>,
}

#[derive(Default)]
struct MetricsInner {
    /// Nombre total de connexions depuis le démarrage
    total_connections: u64,

    /// Nombre de connexions actives
    active_connections: usize,

    /// Nombre total de messages reçus
    messages_received: u64,

    /// Nombre total de messages envoyés
    messages_sent: u64,

    /// Timestamp du dernier message
    last_message_at: Option<Instant>,

    /// Messages par seconde (moyenne sur 10s)
    messages_per_second: f64,

    /// Compteur de messages dans la fenêtre actuelle
    message_window: Vec<Instant>,
}

impl WsMetrics {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(MetricsInner::default())),
        }
    }

    /// Enregistre une nouvelle connexion
    pub async fn on_connection(&self) {
        let mut inner = self.inner.lock().await;
        inner.total_connections += 1;
        inner.active_connections += 1;
    }

    /// Enregistre une déconnexion
    pub async fn on_disconnection(&self) {
        let mut inner = self.inner.lock().await;
        if inner.active_connections > 0 {
            inner.active_connections -= 1;
        }
    }

    /// Enregistre un message reçu
    pub async fn on_message_received(&self) {
        let mut inner = self.inner.lock().await;
        inner.messages_received += 1;
        inner.last_message_at = Some(Instant::now());
        inner.message_window.push(Instant::now());

        // Nettoyer les messages de plus de 10 secondes
        let cutoff = Instant::now() - Duration::from_secs(10);
        inner.message_window.retain(|&t| t > cutoff);

        // Calculer messages/s
        inner.messages_per_second = inner.message_window.len() as f64 / 10.0;
    }

    /// Enregistre un message envoyé
    pub async fn on_message_sent(&self) {
        let mut inner = self.inner.lock().await;
        inner.messages_sent += 1;
    }

    /// Récupère les métriques actuelles
    pub async fn get_metrics(&self) -> MetricsSnapshot {
        let inner = self.inner.lock().await;
        MetricsSnapshot {
            total_connections: inner.total_connections,
            active_connections: inner.active_connections,
            messages_received: inner.messages_received,
            messages_sent: inner.messages_sent,
            messages_per_second: inner.messages_per_second,
            last_message_at: inner.last_message_at.map(|_| {
                // Timestamp Unix en millisecondes
                use std::time::{SystemTime, UNIX_EPOCH};
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("SystemTime should be after UNIX_EPOCH")
                    .as_millis() as u64
            }),
        }
    }
}

/// Snapshot des métriques (pour affichage)
#[derive(Debug, Clone, serde::Serialize)]
pub struct MetricsSnapshot {
    pub total_connections: u64,
    pub active_connections: usize,
    pub messages_received: u64,
    pub messages_sent: u64,
    pub messages_per_second: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message_at: Option<u64>, // Timestamp Unix en millisecondes
}
