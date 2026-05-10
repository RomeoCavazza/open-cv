//! Adapter Postgres — implémente les traits de `ports::repos`.

mod annexe;
mod chunk;
mod helpers;
mod instance;
mod message;
mod offre;
mod profil;
mod snapshot;

pub use annexe::AnnexeRepoPg;
pub use chunk::ChunkRepoPg;
pub use instance::InstanceRepoPg;
pub use message::MessageRepoPg;
pub use offre::OffreRepoPg;
pub use profil::ProfilRepoPg;
pub use snapshot::SnapshotRepoPg;

/// Crée le pool Postgres et exécute `MIGRATE` au démarrage.
pub async fn connect(database_url: &str) -> Result<sqlx::PgPool, sqlx::Error> {
    use sqlx::postgres::PgPoolOptions;

    PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(database_url)
        .await
}
