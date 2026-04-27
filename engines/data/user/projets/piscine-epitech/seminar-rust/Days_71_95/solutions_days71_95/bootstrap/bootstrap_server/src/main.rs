use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

// ============== Endpoint 1 - Health Check ==============

#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
    })
}

// ============== Endpoint 2 - Server Model ==============

#[derive(Clone, Serialize, Deserialize)]
struct Server {
    id: Uuid,
    name: String,
    description: String,
}

#[derive(Deserialize)]
struct CreateServerPayload {
    name: String,
    description: String,
}

#[derive(Deserialize)]
struct UpdateServerPayload {
    name: Option<String>,
    description: Option<String>,
}

// In-memory storage
type SharedServers = Arc<Mutex<HashMap<Uuid, Server>>>;

#[derive(Clone)]
struct AppState {
    servers: SharedServers,
}

// ============== Endpoint 3 - Create Server ==============

async fn create_server(
    State(state): State<AppState>,
    Json(payload): Json<CreateServerPayload>,
) -> Json<Server> {
    let server = Server {
        id: Uuid::new_v4(),
        name: payload.name,
        description: payload.description,
    };

    let mut servers = state.servers.lock().await;
    servers.insert(server.id, server.clone());

    Json(server)
}

// ============== Endpoint 4 - List Servers ==============

async fn list_servers(State(state): State<AppState>) -> Json<Vec<Server>> {
    let servers = state.servers.lock().await;
    let server_list: Vec<Server> = servers.values().cloned().collect();

    Json(server_list)
}

// ============== Endpoint 5 - Get Server by ID ==============

async fn get_server(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Server>, StatusCode> {
    let servers = state.servers.lock().await;
    servers
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

// ============== Endpoint 6 - Update Server ==============

async fn update_server(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateServerPayload>,
) -> Result<Json<Server>, StatusCode> {
    let mut servers = state.servers.lock().await;

    let server = servers.get_mut(&id).ok_or(StatusCode::NOT_FOUND)?;

    if let Some(name) = payload.name {
        server.name = name;
    }
    if let Some(description) = payload.description {
        server.description = description;
    }

    Ok(Json(server.clone()))
}

// ============== Endpoint 7 - Delete Server ==============

async fn delete_server(State(state): State<AppState>, Path(id): Path<Uuid>) -> StatusCode {
    let mut servers = state.servers.lock().await;

    match servers.remove(&id) {
        Some(_) => StatusCode::NO_CONTENT,
        None => StatusCode::NOT_FOUND,
    }
}

// ============== Main ==============

#[tokio::main]
async fn main() {
    let state = AppState {
        servers: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/servers", post(create_server).get(list_servers))
        .route(
            "/servers/{id}",
            get(get_server).put(update_server).delete(delete_server),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
