//! Protocole WebSocket : types d'événements client ↔ serveur
//! Séparé des modèles DB pour éviter le couplage

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Événements envoyés par le client
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", content = "d")]
pub enum ClientEvent {
    /// Identification avec token JWT (premier message après connexion)
    #[serde(rename = "IDENTIFY")]
    Identify { token: String },

    /// Envoi d'un message dans un channel
    #[serde(rename = "SEND_MESSAGE")]
    SendMessage { channel_id: Uuid, content: String },

    /// Début de frappe dans un channel
    #[serde(rename = "TYPING_START")]
    TypingStart { channel_id: Uuid },

    /// Fin de frappe dans un channel
    #[serde(rename = "TYPING_STOP")]
    TypingStop { channel_id: Uuid },

    /// Heartbeat (pong applicatif)
    #[serde(rename = "HEARTBEAT")]
    Heartbeat { seq: Option<u64> },

    /// Subscription à un channel (pour recevoir les événements)
    #[serde(rename = "SUBSCRIBE")]
    Subscribe { channel_id: Uuid },

    /// Unsubscription d'un channel
    #[serde(rename = "UNSUBSCRIBE")]
    Unsubscribe { channel_id: Uuid },

    /// Mise à jour de présence
    #[serde(rename = "PRESENCE_UPDATE")]
    PresenceUpdate { status: String },
}

/// Événements envoyés par le serveur
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", content = "d")]
pub enum ServerEvent {
    /// Connexion établie, prêt à recevoir IDENTIFY
    #[serde(rename = "HELLO")]
    Hello {
        heartbeat_interval: u64, // ms
    },

    /// Identification réussie
    #[serde(rename = "READY")]
    Ready { user_id: Uuid, username: String },

    /// Erreur (auth, validation, etc.)
    #[serde(rename = "ERROR")]
    Error { code: String, message: String },

    /// Nouveau message reçu
    #[serde(rename = "MESSAGE_CREATE")]
    MessageCreate {
        id: Uuid,
        channel_id: Uuid,
        server_id: Uuid,
        author_id: Uuid,
        username: String,
        content: String,
        created_at: DateTime<Utc>,
        edited_at: Option<DateTime<Utc>>,
    },

    /// Message modifié
    #[serde(rename = "MESSAGE_UPDATE")]
    MessageUpdate {
        id: Uuid,
        channel_id: Uuid,
        content: String,
        edited_at: DateTime<Utc>,
    },

    /// Message supprimé
    #[serde(rename = "MESSAGE_DELETE")]
    MessageDelete { id: Uuid, channel_id: Uuid },

    /// Quelqu'un commence à taper
    #[serde(rename = "TYPING_START")]
    TypingStart {
        channel_id: Uuid,
        user_id: Uuid,
        username: String,
    },

    /// Quelqu'un arrête de taper
    #[serde(rename = "TYPING_STOP")]
    TypingStop { channel_id: Uuid, user_id: Uuid },

    /// Heartbeat ACK
    #[serde(rename = "HEARTBEAT_ACK")]
    HeartbeatAck { seq: Option<u64> },

    /// Subscription confirmée
    #[serde(rename = "SUBSCRIBED")]
    Subscribed { channel_id: Uuid },

    /// Unsubscription confirmée
    #[serde(rename = "UNSUBSCRIBED")]
    Unsubscribed { channel_id: Uuid },

    /// Mise à jour de présence utilisateur
    #[serde(rename = "PRESENCE_UPDATE")]
    PresenceUpdate {
        user_id: Uuid,
        status: String, // "online" | "offline" | "dnd" | "invisible"
    },
}

impl ClientEvent {
    /// Parse depuis JSON
    pub fn from_json(data: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(data)
    }
}

impl ServerEvent {
    /// Convertit en JSON pour envoi
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}
