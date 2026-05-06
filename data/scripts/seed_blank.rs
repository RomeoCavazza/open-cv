use anyhow::{Context, Result};
use chrono::Utc;
use domain::{Profil, ProfilContent, ProfileSection};
use adapter_postgres::ProfilRepoPg;
use ports::ProfilRepo;
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

    let profil_repo = ProfilRepoPg::new(pool.clone());

    // Create a blank active profile
    let profil = Profil {
        id: domain::ProfilId::new(),
        label: "Nouveau Profil".to_string(),
        content: ProfilContent {
            profile: ProfileSection {
                firstname: "Nouveau".to_string(),
                lastname: "Candidat".to_string(),
                ..Default::default()
            },
            ..Default::default()
        },
        is_active: true,
        profile_photo: None,
        calendar_pdf: None,
        resume_template: None,
        cover_letter_template: None,
        notes: domain::JsonValue::Object(Default::default()),
        created_at: Utc::now(),
    };

    profil_repo.upsert(&profil).await.context("Failed to upsert blank profile")?;

    println!("Profil vierge créé avec succès !");
    Ok(())
}
