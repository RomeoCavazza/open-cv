use sqlx::postgres::PgPoolOptions;
use std::env;
use std::sync::Arc;
use domain::ProfilId;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new().connect(&database_url).await?;
    
    let profil_repo = Arc::new(adapter_postgres::ProfilRepoPg::new(pool.clone()));
    let chunk_repo = Arc::new(adapter_postgres::ChunkRepoPg::new(pool.clone()));
    
    // ON UTILISE MXBAI POUR LES EMBEDDINGS (1024 dims)
    let ollama_embed = Arc::new(adapter_llm_ollama::OllamaClient::new(
        "http://localhost:11434",
        "mxbai-embed-large"
    ));

    let usecase = application::index_profil::IndexProfilUseCase::new(
        profil_repo, 
        chunk_repo, 
        ollama_embed
    );
    
    usecase.execute(ProfilId(uuid::Uuid::nil())).await?;
    println!("✅ Profil indexé avec succès (1024 dims) !");
    Ok(())
}
