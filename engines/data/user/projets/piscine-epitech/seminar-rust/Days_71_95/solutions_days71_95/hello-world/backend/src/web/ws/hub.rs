//! Hub WebSocket : gestion centralisée des connexions et broadcasts
//! Pattern singleton partagé via Arc<Mutex<>> dans AppState

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use uuid::Uuid;

use crate::web::ws::protocol::ServerEvent;

/// ID unique d'une connexion WebSocket
pub type ConnectionId = Uuid;

/// Hub central pour gérer toutes les connexions WebSocket
#[derive(Clone)]
pub struct WsHub {
    /// Connexions actives : ConnectionId -> Sender pour envoyer des messages
    connections: Arc<Mutex<HashMap<ConnectionId, broadcast::Sender<String>>>>,

    /// Subscriptions : ChannelId -> Set de ConnectionId
    subscriptions: Arc<Mutex<HashMap<Uuid, HashSet<ConnectionId>>>>,

    /// Connexions par utilisateur : UserId -> Set de ConnectionId (multi-device)
    user_connections: Arc<Mutex<HashMap<Uuid, HashSet<ConnectionId>>>>,
}

impl WsHub {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(Mutex::new(HashMap::new())),
            subscriptions: Arc::new(Mutex::new(HashMap::new())),
            user_connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Enregistre une nouvelle connexion
    pub async fn register(&self, conn_id: ConnectionId) -> broadcast::Receiver<String> {
        let (tx, rx) = broadcast::channel(100); // Buffer de 100 messages max

        let mut connections = self.connections.lock().await;
        connections.insert(conn_id, tx);

        rx
    }

    /// Supprime une connexion
    pub async fn unregister(&self, conn_id: ConnectionId, user_id: Option<Uuid>) {
        // Retirer de toutes les subscriptions
        let mut subscriptions = self.subscriptions.lock().await;
        for (_, conn_set) in subscriptions.iter_mut() {
            conn_set.remove(&conn_id);
        }
        subscriptions.retain(|_, conn_set| !conn_set.is_empty());

        // Retirer de user_connections si user_id fourni
        if let Some(uid) = user_id {
            let mut user_conns = self.user_connections.lock().await;
            if let Some(conn_set) = user_conns.get_mut(&uid) {
                conn_set.remove(&conn_id);
                if conn_set.is_empty() {
                    user_conns.remove(&uid);
                }
            }
        }

        // Retirer la connexion
        let mut connections = self.connections.lock().await;
        connections.remove(&conn_id);
    }

    /// Associe une connexion à un utilisateur
    pub async fn associate_user(&self, conn_id: ConnectionId, user_id: Uuid) {
        let mut user_conns = self.user_connections.lock().await;
        user_conns
            .entry(user_id)
            .or_insert_with(HashSet::new)
            .insert(conn_id);
    }

    /// Subscribe une connexion à un channel
    pub async fn subscribe(&self, conn_id: ConnectionId, channel_id: Uuid) {
        let mut subscriptions = self.subscriptions.lock().await;
        subscriptions
            .entry(channel_id)
            .or_insert_with(HashSet::new)
            .insert(conn_id);
    }

    /// Unsubscribe une connexion d'un channel
    pub async fn unsubscribe(&self, conn_id: ConnectionId, channel_id: Uuid) {
        let mut subscriptions = self.subscriptions.lock().await;
        if let Some(conn_set) = subscriptions.get_mut(&channel_id) {
            conn_set.remove(&conn_id);
            if conn_set.is_empty() {
                subscriptions.remove(&channel_id);
            }
        }
    }

    /// Broadcast un événement à tous les abonnés d'un channel
    pub async fn broadcast_to_channel(&self, channel_id: Uuid, event: &ServerEvent) {
        let event_json = match event.to_json() {
            Ok(json) => json,
            Err(e) => {
                eprintln!("[Hub] Failed to serialize event: {}", e);
                return;
            }
        };

        let subscriptions = self.subscriptions.lock().await;
        let conn_ids = match subscriptions.get(&channel_id) {
            Some(ids) => ids.clone(),
            None => return, // Aucun abonné
        };
        drop(subscriptions);

        let connections = self.connections.lock().await;
        let mut sent = 0;
        let mut errors = 0;

        for conn_id in conn_ids {
            if let Some(tx) = connections.get(&conn_id) {
                match tx.send(event_json.clone()) {
                    Ok(_) => sent += 1,
                    Err(_) => {
                        // Receiver fermé, connexion morte (sera nettoyée au prochain heartbeat)
                        errors += 1;
                    }
                }
            }
        }

        if sent > 0 {
            tracing::debug!(
                "[Hub] Broadcast to channel {}: {} sent, {} errors",
                channel_id,
                sent,
                errors
            );
        }
    }

    /// Broadcast avec tracking des métriques
    pub async fn broadcast_to_channel_with_metrics(
        &self,
        channel_id: Uuid,
        event: &ServerEvent,
        metrics: Option<&crate::web::ws::metrics::WsMetrics>,
    ) {
        self.broadcast_to_channel(channel_id, event).await;

        // Compter les messages envoyés dans les métriques
        if let Some(m) = metrics {
            let subscriptions = self.subscriptions.lock().await;
            if let Some(conn_ids) = subscriptions.get(&channel_id) {
                for _ in conn_ids.iter() {
                    m.on_message_sent().await;
                }
            }
        }
    }

    /// Envoie un événement à une connexion spécifique
    pub async fn send_to_connection(&self, conn_id: ConnectionId, event: &ServerEvent) {
        let event_json = match event.to_json() {
            Ok(json) => json,
            Err(e) => {
                eprintln!("[Hub] Failed to serialize event: {}", e);
                return;
            }
        };

        let connections = self.connections.lock().await;
        if let Some(tx) = connections.get(&conn_id) {
            let _ = tx.send(event_json);
        }
    }

    /// Envoie un événement à un utilisateur spécifique (toutes ses connexions)
    pub async fn send_to_user(&self, user_id: Uuid, event: &ServerEvent) {
        let event_json = match event.to_json() {
            Ok(json) => json,
            Err(e) => {
                eprintln!("[Hub] Failed to serialize event: {}", e);
                return;
            }
        };

        let user_conns = self.user_connections.lock().await;
        let conn_ids = match user_conns.get(&user_id) {
            Some(ids) => ids.clone(),
            None => return,
        };
        drop(user_conns);

        let connections = self.connections.lock().await;
        for conn_id in conn_ids {
            if let Some(tx) = connections.get(&conn_id) {
                let _ = tx.send(event_json.clone());
            }
        }
    }

    /// Broadcast un événement à toutes les connexions actives
    pub async fn broadcast_all(&self, event: &ServerEvent) {
        let event_json = match event.to_json() {
            Ok(json) => json,
            Err(e) => {
                eprintln!("[Hub] Failed to serialize event: {}", e);
                return;
            }
        };

        let connections = self.connections.lock().await;
        for tx in connections.values() {
            let _ = tx.send(event_json.clone());
        }
    }

    /// Nombre de connexions actives
    pub async fn connection_count(&self) -> usize {
        self.connections.lock().await.len()
    }

    /// Nombre d'abonnés à un channel
    pub async fn channel_subscriber_count(&self, channel_id: Uuid) -> usize {
        self.subscriptions
            .lock()
            .await
            .get(&channel_id)
            .map(|s| s.len())
            .unwrap_or(0)
    }
}

impl Default for WsHub {
    fn default() -> Self {
        Self::new()
    }
}
