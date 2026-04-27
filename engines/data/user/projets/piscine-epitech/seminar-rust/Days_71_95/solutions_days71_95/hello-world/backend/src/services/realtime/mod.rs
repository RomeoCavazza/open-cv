//! Service realtime : logique métier pour les événements WebSocket
//! Séparé du transport (web/ws) pour respecter la séparation des responsabilités

pub mod messaging;
pub mod presence;
pub mod typing;

pub use messaging::handle_send_message;
pub use presence::{handle_presence_update, handle_user_offline, handle_user_online};
pub use typing::{handle_typing_start, handle_typing_stop};
