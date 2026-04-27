pub use self::error::{Error, Result};

use axum::http::{header, HeaderValue, Method};
use axum::{
    middleware,
    routing::{get, post},
    Json, Router,
};
use mongodb::{Client as MongoClient, Database as MongoDatabase};
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tower_http::cors::CorsLayer; // On utilise axum::http au lieu de http direct

mod ctx;
mod error;
mod handlers;
mod models;
mod repositories;
mod routes;
mod services;
mod web;

use axum::extract::State;
use repositories::{
    ChannelRepository, InviteRepository, MessageRepository, ServerRepository, UserRepository,
};
use web::MetricsSnapshot;
use web::{WsHub, WsMetrics};

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub mongo: MongoDatabase,
    pub jwt_secret: String,
    pub user_repo: UserRepository,
    pub server_repo: ServerRepository,
    pub channel_repo: ChannelRepository,
    pub message_repo: MessageRepository,
    pub invite_repo: InviteRepository,
    pub ws_hub: web::WsHub,
    pub ws_metrics: web::WsMetrics,
}

async fn health() -> &'static str {
    "OK"
}

/// Endpoint pour récupérer les métriques WebSocket
async fn get_ws_metrics(State(state): State<AppState>) -> Json<MetricsSnapshot> {
    Json(state.ws_metrics.get_metrics().await)
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // Initialiser tracing pour les logs
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5433/helloworld".to_string());
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "super_secret_jwt_key_change_in_production".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    // Exécuter les migrations SQLx automatiquement au démarrage
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    let mongodb_url =
        std::env::var("MONGODB_URL").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let mongo_client = MongoClient::with_uri_str(&mongodb_url)
        .await
        .expect("Failed to connect to MongoDB");
    let mongo_db = mongo_client.database("helloworld");

    let user_repo = UserRepository::new(pool.clone());
    let server_repo = ServerRepository::new(pool.clone());
    let channel_repo = ChannelRepository::new(pool.clone());
    let message_repo = MessageRepository::new(mongo_db.clone());
    let invite_repo = InviteRepository::new(pool.clone());

    let ws_hub = WsHub::new();
    let ws_metrics = WsMetrics::new();

    // Démarrer le cleanup périodique du cache typing
    tokio::spawn(async {
        let mut interval = tokio::time::interval(Duration::from_secs(5));
        loop {
            interval.tick().await;
            crate::services::realtime::typing::cleanup_typing_cache().await;
        }
    });

    let state = AppState {
        db: pool,
        mongo: mongo_db,
        jwt_secret,
        user_repo,
        server_repo,
        channel_repo,
        message_repo,
        invite_repo,
        ws_hub,
        ws_metrics,
    };
    let cors = CorsLayer::new()
        .allow_origin(
            "https://hello-world-messagerie-jfk7.vercel.app"
                .parse::<HeaderValue>()
                .unwrap(),
        )
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS, // Très important pour les navigateurs
        ])
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::UPGRADE,
            header::CONNECTION,
        ]);

    let routes_protected = routes::create_router()
        .route(
            "/me",
            get(handlers::user::me).patch(handlers::user::update_me),
        )
        .route("/auth/logout", post(handlers::auth::logout))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            web::mw_require_auth,
        ));

    let routes_public = Router::new()
        .route("/health", get(health))
        .route("/ws/metrics", get(get_ws_metrics))
        .route(
            "/users/{user_id}",
            get(handlers::user_public::get_public_user),
        )
        .route("/ws", get(web::ws_handler))
        .merge(routes::auth::routes());

    let app = Router::new()
        .merge(routes_public)
        .merge(routes_protected)
        .layer(middleware::from_fn_with_state(
            state.clone(),
            web::mw_ctx_resolver,
        ))
        .layer(cors)
        .with_state(state);

    // 1. Récupération du port dynamique de Render
    let port = std::env::var("PORT").unwrap_or_else(|_| "3001".to_string());
    let addr = format!("0.0.0.0:{}", port);

    // 2. Création du listener TCP sur l'adresse de production
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to address {}", addr));

    // 3. Log de confirmation (on affiche l'adresse réelle au lieu de localhost)
    println!("🚀 Server running on http://{}", addr);

    // 4. Démarrage du serveur Axum
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}
