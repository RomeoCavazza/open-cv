use anyhow::{Context, Result};
use chrono::Utc;
use domain::{Chunk, ChunkId, ChunkKind};
use ports::{ChunkRepo, Embedder, ProfilRepo};
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").context("DATABASE_URL must be set")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let profil_repo = adapter_postgres::ProfilRepoPg::new(pool.clone());
    let chunk_repo = adapter_postgres::ChunkRepoPg::new(pool.clone());

    // 1. Récupérer le profil actif
    let profil = profil_repo
        .get_active()
        .await?
        .context("Aucun profil actif trouvé")?;

    println!("Vidage de la table chunks...");
    sqlx::query("TRUNCATE TABLE chunks").execute(&pool).await?;

    println!("Chunking du profil: {}", profil.label);

    // 2. Initialiser l'Embedder (Ollama)
    let ollama_base =
        env::var("OLLAMA_BASE_URL").unwrap_or_else(|_| "http://localhost:11434".into());
    let embed_model = env::var("OLLAMA_EMBED_MODEL").unwrap_or_else(|_| "mxbai-embed-large".into());
    let embedder = adapter_llm_ollama::OllamaClient::new(ollama_base, embed_model, 1024);

    let mut total_chunks = 0;

    // --- EXPÉRIENCES ---
    for exp in &profil.content.experiences {
        let content = format!(
            "{} chez {}\nPeriod: {}\nDescription:\n{}",
            exp.role,
            exp.company,
            exp.period,
            exp.description.join("\n")
        );
        let chunk = create_chunk(
            &profil.id,
            ChunkKind::Experience,
            &exp.role,
            &content,
            &embedder,
        )
        .await?;
        chunk_repo.upsert(&chunk).await?;
        total_chunks += 1;
    }

    // --- PROJETS ---
    for proj in &profil.content.projects {
        let content = format!(
            "Projet: {}\nRole: {}\nDescription:\n{}",
            proj.company,
            proj.role,
            proj.description.join("\n")
        );
        let chunk = create_chunk(
            &profil.id,
            ChunkKind::Projet,
            &proj.role,
            &content,
            &embedder,
        )
        .await?;
        chunk_repo.upsert(&chunk).await?;
        total_chunks += 1;
    }

    // --- FORMATIONS ---
    for edu in &profil.content.education {
        let content = format!(
            "Diplôme: {}\nÉcole: {}\nPériode: {}",
            edu.degree, edu.school, edu.period
        );
        let chunk = create_chunk(
            &profil.id,
            ChunkKind::Formation,
            &edu.degree,
            &content,
            &embedder,
        )
        .await?;
        chunk_repo.upsert(&chunk).await?;
        total_chunks += 1;
    }

    // --- COMPÉTENCES ---
    for cat in &profil.content.skills {
        let content = format!("Compétences ({}): {}", cat.category, cat.items.join(", "));
        let chunk = create_chunk(
            &profil.id,
            ChunkKind::Competence,
            &cat.category,
            &content,
            &embedder,
        )
        .await?;
        chunk_repo.upsert(&chunk).await?;
        total_chunks += 1;
    }

    println!("Success! {} chunks générés et indexés.", total_chunks);
    Ok(())
}

async fn create_chunk(
    profil_id: &domain::ProfilId,
    kind: ChunkKind,
    titre: &str,
    content: &str,
    embedder: &dyn Embedder,
) -> Result<Chunk> {
    let embeddings = embedder
        .embed(&[content], ports::EmbedMode::Document)
        .await
        .map_err(|e| anyhow::anyhow!("Embedding error: {}", e))?;

    let embedding = embeddings
        .first()
        .cloned()
        .context("No embedding returned")?;

    Ok(Chunk {
        id: ChunkId::new(),
        profil_id: *profil_id,
        kind,
        titre: titre.to_string(),
        content: content.to_string(),
        metadata: domain::JsonValue::Object(Default::default()),
        embedding,
        created_at: Utc::now(),
    })
}
