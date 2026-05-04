//! Adapter Postgres — implémente les traits de `ports::repos`.

mod helpers;
mod offre;
mod instance;
mod annexe;
mod message;
mod profil;
mod chunk;

pub use annexe::AnnexeRepoPg;
pub use chunk::ChunkRepoPg;
pub use instance::InstanceRepoPg;
pub use message::MessageRepoPg;
pub use offre::OffreRepoPg;
pub use profil::ProfilRepoPg;

/// Crée le pool Postgres et exécute `MIGRATE` au démarrage.
pub async fn connect(database_url: &str) -> Result<sqlx::PgPool, sqlx::Error> {
    use sqlx::postgres::PgPoolOptions;

    PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(database_url)
        .await
}
