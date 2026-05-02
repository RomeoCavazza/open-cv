use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new().connect(&database_url).await?;

    // 1. Lister les instances qui n'ont pas de lettre
    let instances = sqlx::query!("SELECT slug FROM instances WHERE cover_letter_json IS NULL")
        .fetch_all(&pool).await?;

    println!("🚀 Lancement du batch de {} lettres de motivation...", instances.len());

    let client = reqwest::Client::new();

    for (i, inst) in instances.iter().enumerate() {
        println!("[{}/{}] Génération pour {}...", i+1, instances.len(), inst.slug);
        
        let res = client.post(format!("http://127.0.0.1:8000/api/instances/{}/generate", inst.slug))
            .json(&serde_json::json!({ "deliverables": { "cover": true, "resume": false, "restitution": false } }))
            .send().await;

        match res {
            Ok(r) if r.status().is_success() => println!("  ✅ Succès"),
            Ok(r) => println!("  ❌ Erreur HTTP {}", r.status()),
            Err(e) => println!("  ❌ Erreur réseau: {}", e),
        }
    }

    println!("🏁 Batch terminé !");
    Ok(())
}
