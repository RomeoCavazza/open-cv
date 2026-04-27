//! Module WebSocket : transport temps réel
//! Séparé de la logique métier (services/realtime)

pub mod connection;
pub mod handler;
pub mod hub;
pub mod metrics;
pub mod protocol;

pub use handler::ws_handler;
pub use hub::WsHub;
pub use metrics::{MetricsSnapshot, WsMetrics};
