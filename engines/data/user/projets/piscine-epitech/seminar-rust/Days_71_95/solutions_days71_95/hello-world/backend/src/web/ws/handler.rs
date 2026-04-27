//! Handler WebSocket Axum : upgrade HTTP → WS + gestion des événements

use axum::extract::{ws::WebSocketUpgrade, State};
use axum::response::Response;
use uuid::Uuid;

use crate::services::verify_token;
use crate::web::ws::connection::WsConnection;
use crate::web::ws::hub::WsHub;
use crate::web::ws::protocol::{ClientEvent, ServerEvent};
use crate::AppState;

/// Handler principal pour l'upgrade WebSocket
pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

/// Gère une connexion WebSocket après upgrade
async fn handle_socket(socket: axum::extract::ws::WebSocket, state: AppState) {
    let conn_id = Uuid::new_v4();
    tracing::info!("[WS] New connection: {}", conn_id);

    // Enregistrer la connexion dans les métriques
    state.ws_metrics.on_connection().await;

    // Créer la connexion
    let hub = state.ws_hub.clone();
    let connection = WsConnection::new(conn_id, hub.clone());

    // Démarrer la connexion et obtenir le channel d'événements
    let mut event_rx = connection.handle(socket).await;

    // Envoyer HELLO immédiatement
    let hello = ServerEvent::Hello {
        heartbeat_interval: 30000, // 30s
    };
    hub.send_to_connection(conn_id, &hello).await;

    // État de la connexion (authentification)
    let mut user_id: Option<Uuid> = None;
    let mut authenticated = false;

    // Boucle de traitement des événements clients
    while let Some((_event_conn_id, event)) = event_rx.recv().await {
        // Enregistrer le message reçu
        state.ws_metrics.on_message_received().await;

        match event {
            ClientEvent::Identify { token } => {
                // Vérifier le token
                match verify_token(&token, &state.jwt_secret) {
                    Ok(claims) => {
                        // Vérifier que l'utilisateur existe
                        match state.user_repo.find_by_id(claims.sub).await {
                            Ok(Some(user)) => {
                                // Authentification réussie
                                user_id = Some(claims.sub);
                                authenticated = true;
                                hub.associate_user(conn_id, claims.sub).await;

                                // Marquer l'utilisateur comme en ligne
                                crate::services::realtime::handle_user_online(&state, claims.sub)
                                    .await;

                                // Envoyer READY
                                let ready = ServerEvent::Ready {
                                    user_id: claims.sub,
                                    username: user.username.clone(),
                                };
                                send_to_connection(&hub, conn_id, &ready).await;

                                tracing::info!(
                                    "[WS] Connection {} authenticated as user {}",
                                    conn_id,
                                    claims.sub
                                );
                            }
                            Ok(None) => {
                                send_error(&hub, conn_id, "USER_NOT_FOUND", "User not found").await;
                            }
                            Err(e) => {
                                tracing::error!("[WS] DB error: {}", e);
                                send_error(&hub, conn_id, "INTERNAL_ERROR", "Database error").await;
                            }
                        }
                    }
                    Err(_) => {
                        send_error(&hub, conn_id, "INVALID_TOKEN", "Invalid or expired token")
                            .await;
                    }
                }
            }
            ClientEvent::SendMessage {
                channel_id,
                content,
            } => {
                if !authenticated {
                    send_error(&hub, conn_id, "NOT_AUTHENTICATED", "Must identify first").await;
                    continue;
                }

                let uid = user_id.expect("User ID should be set after authentication check");

                // Déléguer au service realtime
                if let Err(e) =
                    crate::services::realtime::handle_send_message(&state, uid, channel_id, content)
                        .await
                {
                    tracing::error!("[WS] Error sending message: {}", e);
                    send_error(&hub, conn_id, "MESSAGE_ERROR", &e.to_string()).await;
                }
            }
            ClientEvent::TypingStart { channel_id } => {
                if !authenticated {
                    continue;
                }

                let uid = user_id.expect("User ID should be set after authentication check");
                crate::services::realtime::handle_typing_start(&state, uid, channel_id).await;
            }
            ClientEvent::TypingStop { channel_id } => {
                if !authenticated {
                    continue;
                }

                let uid = user_id.expect("User ID should be set after authentication check");
                crate::services::realtime::handle_typing_stop(&state, uid, channel_id).await;
            }
            ClientEvent::Heartbeat { seq } => {
                let ack = ServerEvent::HeartbeatAck { seq };
                send_to_connection(&hub, conn_id, &ack).await;
            }
            ClientEvent::Subscribe { channel_id } => {
                if !authenticated {
                    send_error(&hub, conn_id, "NOT_AUTHENTICATED", "Must identify first").await;
                    continue;
                }

                hub.subscribe(conn_id, channel_id).await;
                let subscribed = ServerEvent::Subscribed { channel_id };
                send_to_connection(&hub, conn_id, &subscribed).await;
            }
            ClientEvent::Unsubscribe { channel_id } => {
                hub.unsubscribe(conn_id, channel_id).await;
                let unsubscribed = ServerEvent::Unsubscribed { channel_id };
                send_to_connection(&hub, conn_id, &unsubscribed).await;
            }
            ClientEvent::PresenceUpdate { status } => {
                if !authenticated {
                    send_error(&hub, conn_id, "NOT_AUTHENTICATED", "Must identify first").await;
                    continue;
                }

                let uid = user_id.expect("User ID should be set after authentication check");
                crate::services::realtime::handle_presence_update(&state, uid, status).await;
            }
        }
    }

    // Marquer l'utilisateur comme hors ligne avant de nettoyer
    if let Some(uid) = user_id {
        crate::services::realtime::handle_user_offline(&state, uid).await;
    }

    // Enregistrer la déconnexion dans les métriques
    state.ws_metrics.on_disconnection().await;

    // Nettoyer la connexion
    hub.unregister(conn_id, user_id).await;
}

/// Envoie un événement à une connexion spécifique
async fn send_to_connection(hub: &WsHub, conn_id: Uuid, event: &ServerEvent) {
    hub.send_to_connection(conn_id, event).await;
}

/// Envoie une erreur à une connexion
async fn send_error(hub: &WsHub, conn_id: Uuid, code: &str, message: &str) {
    let error = ServerEvent::Error {
        code: code.to_string(),
        message: message.to_string(),
    };
    hub.send_to_connection(conn_id, &error).await;
}
