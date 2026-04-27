//! Gestion d'une connexion WebSocket individuelle
//! Read loop, write loop, heartbeat, backpressure

use axum::extract::ws::{Message, WebSocket};
use futures_util::{SinkExt, StreamExt};
use std::time::{Duration, Instant};
use tokio::sync::{broadcast, mpsc};
use tokio::time::interval;
use uuid::Uuid;

use crate::web::ws::hub::WsHub;
use crate::web::ws::protocol::{ClientEvent, ServerEvent};

/// Configuration d'une connexion
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);
const MAX_MESSAGE_SIZE: usize = 1024 * 64; // 64KB max

/// État d'une connexion WebSocket
pub struct WsConnection {
    conn_id: Uuid,
    hub: WsHub,
    user_id: Option<Uuid>,
    #[allow(dead_code)] // Réservé pour future implémentation de timeout
    last_heartbeat: Instant,
    #[allow(dead_code)] // Réservé pour future validation
    authenticated: bool,
}

impl WsConnection {
    pub fn new(conn_id: Uuid, hub: WsHub) -> Self {
        Self {
            conn_id,
            hub,
            user_id: None,
            last_heartbeat: Instant::now(),
            authenticated: false,
        }
    }

    /// Boucle principale : gère read + write + heartbeat
    /// Retourne un channel pour recevoir les événements clients
    pub async fn handle(self, socket: WebSocket) -> mpsc::Receiver<(Uuid, ClientEvent)> {
        // Channel pour communiquer les événements clients au handler principal
        let (tx, rx) = mpsc::channel(100);

        // Channel pour envoyer des messages depuis read_task vers write_task
        let (error_tx, mut error_rx) = mpsc::channel::<String>(10);

        // Enregistrer la connexion dans le Hub
        let mut hub_rx = self.hub.register(self.conn_id).await;

        // Split read/write
        let (mut sender, mut receiver) = socket.split();

        let conn_id = self.conn_id;
        let hub = self.hub.clone();
        let user_id = self.user_id;

        // Spawn write loop (envoi des messages du Hub)
        let mut write_task = tokio::spawn(async move {
            let mut interval = interval(HEARTBEAT_INTERVAL);
            loop {
                tokio::select! {
                    // Message du Hub à envoyer
                    result = hub_rx.recv() => {
                        match result {
                            Ok(msg) => {
                                if sender.send(Message::Text(msg.into())).await.is_err() {
                                    break; // Connexion fermée
                                }
                            }
                            Err(broadcast::error::RecvError::Closed) => break,
                            Err(broadcast::error::RecvError::Lagged(skipped)) => {
                                tracing::warn!("[WS] Connection {} lagged, skipped {} messages", conn_id, skipped);
                            }
                        }
                    }
                    // Message d'erreur depuis read_task
                    error_msg = error_rx.recv() => {
                        match error_msg {
                            Some(msg) => {
                                if sender.send(Message::Text(msg.into())).await.is_err() {
                                    break;
                                }
                            }
                            None => break, // Channel fermé
                        }
                    }
                    // Heartbeat ping
                    _ = interval.tick() => {
                        if sender.send(Message::Ping(axum::body::Bytes::new())).await.is_err() {
                            break;
                        }
                    }
                }
            }
        });

        // Read loop (réception des messages du client)
        let conn_id_clone = self.conn_id;
        let error_tx_clone = error_tx.clone();
        let mut read_task = tokio::spawn(async move {
            while let Some(msg) = receiver.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        tracing::debug!(
                            "[WS] Received text message from {}: {}",
                            conn_id_clone,
                            &text[..text.len().min(100)]
                        );
                        if text.len() > MAX_MESSAGE_SIZE {
                            tracing::warn!("[WS] Message too large from {}", conn_id_clone);
                            continue;
                        }

                        // Parser l'événement client
                        match ClientEvent::from_json(&text) {
                            Ok(event) => {
                                tracing::debug!(
                                    "[WS] Parsed event from {}: {:?}",
                                    conn_id_clone,
                                    event
                                );
                                // Envoyer au handler principal via channel
                                if tx.send((conn_id_clone, event)).await.is_err() {
                                    break; // Handler fermé
                                }
                            }
                            Err(e) => {
                                tracing::warn!("[WS] Invalid JSON from {}: {}", conn_id_clone, e);
                                let error = ServerEvent::Error {
                                    code: "INVALID_JSON".to_string(),
                                    message: format!("Invalid JSON: {}", e),
                                };
                                if let Ok(json) = error.to_json() {
                                    let _ = error_tx_clone.send(json).await;
                                }
                            }
                        }
                    }
                    Ok(Message::Binary(_)) => {
                        tracing::warn!("[WS] Binary messages not supported");
                    }
                    Ok(Message::Ping(_)) => {
                        // Ping géré automatiquement par axum
                        // Pas besoin de répondre manuellement
                    }
                    Ok(Message::Pong(_)) => {
                        // Pong reçu (géré automatiquement par axum)
                    }
                    Ok(Message::Close(_)) => {
                        break;
                    }
                    Err(e) => {
                        tracing::error!("[WS] Error receiving message: {}", e);
                        break;
                    }
                }
            }
            drop(error_tx_clone); // Fermer le channel pour signaler la fin
        });

        // Spawn une tâche pour nettoyer quand les loops se terminent
        let hub_cleanup = hub.clone();
        tokio::spawn(async move {
            // Attendre la fin d'un des deux loops
            tokio::select! {
                _ = &mut write_task => {
                    read_task.abort();
                }
                _ = &mut read_task => {
                    write_task.abort();
                }
            }

            // Nettoyer la connexion
            hub_cleanup.unregister(conn_id, user_id).await;
            tracing::info!("[WS] Connection {} closed", conn_id);
        });

        // Retourner le Receiver immédiatement pendant que les tâches tournent
        rx
    }

    #[allow(dead_code)] // Réservé pour future utilisation (debugging, logging)
    pub fn conn_id(&self) -> Uuid {
        self.conn_id
    }

    #[allow(dead_code)] // Réservé pour future utilisation
    pub fn set_user_id(&mut self, user_id: Uuid) {
        self.user_id = Some(user_id);
        self.authenticated = true;
    }

    #[allow(dead_code)] // Réservé pour future utilisation
    pub fn is_authenticated(&self) -> bool {
        self.authenticated
    }

    #[allow(dead_code)] // Réservé pour future utilisation
    pub fn user_id(&self) -> Option<Uuid> {
        self.user_id
    }
}
