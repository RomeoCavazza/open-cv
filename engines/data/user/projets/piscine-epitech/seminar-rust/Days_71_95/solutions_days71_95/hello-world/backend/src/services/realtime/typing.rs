//! Gestion des événements "typing" (quelqu'un écrit...)

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::web::ws::protocol::ServerEvent;
use crate::AppState;

/// Cache des événements typing (user_id + channel_id -> timestamp)
/// Expire automatiquement après 3 secondes
type TypingCache = Arc<Mutex<HashMap<(Uuid, Uuid), Instant>>>;

lazy_static::lazy_static! {
    static ref TYPING_CACHE: TypingCache = Arc::new(Mutex::new(HashMap::new()));
}

const TYPING_TIMEOUT: Duration = Duration::from_secs(3);

/// Traite un événement "typing start"
pub async fn handle_typing_start(state: &AppState, user_id: Uuid, channel_id: Uuid) {
    // Vérifier que l'utilisateur a accès au channel
    // (on pourrait ajouter une vérification ici, mais pour éviter trop de DB calls,
    // on fait confiance au fait que l'utilisateur est déjà authentifié)

    // Récupérer le username
    let username = match state.user_repo.get_username(user_id).await {
        Ok(Some(name)) => name,
        _ => return, // User not found, ignore
    };

    // Mettre à jour le cache
    let mut cache = TYPING_CACHE.lock().await;
    let key = (user_id, channel_id);
    let now = Instant::now();

    // Vérifier si c'est un nouveau typing (pas déjà présent)
    let is_new = cache.insert(key, now).is_none();

    if is_new {
        // Nouveau typing, broadcaster l'événement
        let event = ServerEvent::TypingStart {
            channel_id,
            user_id,
            username: username.clone(),
        };

        drop(cache); // Libérer le lock avant l'await
        state.ws_hub.broadcast_to_channel(channel_id, &event).await;
    }
}

/// Traite un événement "typing stop"
pub async fn handle_typing_stop(state: &AppState, user_id: Uuid, channel_id: Uuid) {
    let mut cache = TYPING_CACHE.lock().await;
    let key = (user_id, channel_id);

    if cache.remove(&key).is_some() {
        // Broadcaster l'arrêt
        let event = ServerEvent::TypingStop {
            channel_id,
            user_id,
        };

        drop(cache);
        state.ws_hub.broadcast_to_channel(channel_id, &event).await;
    }
}

/// Nettoyage périodique du cache typing (expiration)
pub async fn cleanup_typing_cache() {
    let mut cache = TYPING_CACHE.lock().await;
    let now = Instant::now();

    cache.retain(|_, timestamp| now.duration_since(*timestamp) < TYPING_TIMEOUT);
}
