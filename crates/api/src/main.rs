use api::create_app;
use api::state::AppState;
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "api=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    tracing::info!("connexion à Postgres...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    tracing::info!("application des migrations...");
    sqlx::migrate!("../../migrations").run(&pool).await?;

    // Initialisation des Repos
    let offre_repo = Arc::new(adapter_postgres::OffreRepoPg::new(pool.clone()));
    let instance_repo = Arc::new(adapter_postgres::InstanceRepoPg::new(pool.clone()));
    let profil_repo = Arc::new(adapter_postgres::ProfilRepoPg::new(pool.clone()));
    let chunk_repo = Arc::new(adapter_postgres::ChunkRepoPg::new(pool.clone()));
    let annexe_repo = Arc::new(adapter_postgres::AnnexeRepoPg::new(pool.clone()));
    let message_repo = Arc::new(adapter_postgres::MessageRepoPg::new(pool.clone()));

    // LLM Registry
    let mut llm_map: HashMap<String, Arc<dyn ports::LlmClient>> = HashMap::new();
    
    // Anthropic
    if let Ok(key) = std::env::var("ANTHROPIC_API_KEY") {
        if !key.is_empty() {
            llm_map.insert("claude".into(), Arc::new(adapter_llm_claude::ClaudeClient::new(key)));
            tracing::info!("LLM: Anthropic (Claude) activé");
        }
    }
    
    // OpenAI
    if let Ok(key) = std::env::var("OPENAI_API_KEY") {
        if !key.is_empty() {
            llm_map.insert("openai".into(), Arc::new(adapter_llm_openai::OpenAiClient::new(key)));
            tracing::info!("LLM: OpenAI activé");
        }
    }

    // Ollama (local)
    let ollama_base = std::env::var("OLLAMA_BASE_URL").unwrap_or_else(|_| "http://localhost:11434".into());
    let ollama_model = std::env::var("OLLAMA_MODEL").unwrap_or_else(|_| "qwen2.5:7b".into());
    let ollama_client = Arc::new(adapter_llm_ollama::OllamaClient::new(ollama_base.clone(), ollama_model.clone(), 1024));
    llm_map.insert("ollama".into(), ollama_client.clone());
    tracing::info!("LLM: Ollama activé ({} @ {})", ollama_model, ollama_base);

    let default_llm = ollama_client.clone();

    // Embedder (Ollama)
    let embed_base = std::env::var("OLLAMA_BASE_URL").unwrap_or_else(|_| "http://localhost:11434".into());
    let embed_model = std::env::var("OLLAMA_EMBED_MODEL").unwrap_or_else(|_| "mxbai-embed-large".into());
    tracing::info!("Embedder: Ollama activé ({} @ {})", embed_model, embed_base);
    let embedder: Arc<dyn ports::Embedder> = Arc::new(adapter_llm_ollama::OllamaClient::new(
        embed_base,
        embed_model,
        1024,
    ));

    let event_bus = Arc::new(application::events::EventBus::new());

    let generate_uc = Arc::new(application::generate::GenerateApplicationUseCase::new(
        offre_repo.clone(),
        profil_repo.clone(),
        chunk_repo.clone(),
        instance_repo.clone(),
        default_llm.clone(),
        embedder.clone(),
        event_bus.clone(),
    ));

    let scraper: Arc<dyn ports::Scraper> = Arc::new(adapter_scraper_http::HttpScraper::new());
    let intake_uc = Arc::new(application::intake::IntakeOffreUseCase::new(
        offre_repo.clone(),
        instance_repo.clone(),
        profil_repo.clone(),
        default_llm.clone(),
        scraper,
    ));

    let state = AppState {
        pool: pool.clone(),
        offre_repo,
        instance_repo,
        profil_repo,
        generate_uc,
        intake_uc,
        chunk_repo,
        annexe_repo,
        message_repo,
        embedder,
        llm_registry: Arc::new(llm_map),
    };

    let app = create_app(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await?;
    tracing::info!("écoute sur http://{}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
