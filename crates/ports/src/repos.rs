//! Traits des repositories — l'interface entre le domaine et la persistence.

use async_trait::async_trait;
use domain::{
    Annexe, AnnexeId, Chunk, Instance, InstanceId, Offre, OffreId, Profil, ProfilId, Slug,
};
use thiserror::Error;

#[async_trait]
pub trait AnnexeRepo: Send + Sync {
    async fn get_by_id(&self, id: AnnexeId) -> Result<Option<Annexe>, RepoError>;
    async fn list_by_profil_id(&self, profil_id: ProfilId) -> Result<Vec<Annexe>, RepoError>;
    async fn upsert(&self, annexe: &Annexe) -> Result<(), RepoError>;
    async fn delete(&self, id: AnnexeId) -> Result<(), RepoError>;
}

#[async_trait]
pub trait OffreRepo: Send + Sync {
    async fn get_by_id(&self, id: OffreId) -> Result<Option<Offre>, RepoError>;
    async fn get_by_slug(&self, slug: &Slug) -> Result<Option<Offre>, RepoError>;
    async fn list_all(&self) -> Result<Vec<Offre>, RepoError>;
    async fn list_recent(&self, limit: u32) -> Result<Vec<Offre>, RepoError>;
    async fn upsert(&self, offre: &Offre) -> Result<(), RepoError>;
    async fn count(&self) -> Result<u64, RepoError>;

    /// Lookup par URL pour la dédup à l'intake.
    async fn find_by_url(&self, url: &str) -> Result<Option<Offre>, RepoError>;

    /// Lookup par hash du contenu pour la dédup à l'intake.
    async fn find_by_content_hash(
        &self,
        source_host: &str,
        hash: &[u8],
    ) -> Result<Option<Offre>, RepoError>;
}

#[async_trait]
pub trait ProfilRepo: Send + Sync {
    async fn get_active(&self) -> Result<Option<Profil>, RepoError>;
    async fn get_by_id(&self, id: ProfilId) -> Result<Option<Profil>, RepoError>;
    async fn list_all(&self) -> Result<Vec<Profil>, RepoError>;
    async fn upsert(&self, profil: &Profil) -> Result<(), RepoError>;
}

#[async_trait]
pub trait ChunkRepo: Send + Sync {
    async fn upsert(&self, chunk: &Chunk) -> Result<(), RepoError>;

    /// Top-K chunks d'un profil par similarité cosinus avec un embedding query.
    async fn top_k_by_embedding(
        &self,
        profil_id: ProfilId,
        query_embedding: &[f32],
        k: u32,
    ) -> Result<Vec<(Chunk, f32)>, RepoError>;
}

#[async_trait]
pub trait InstanceRepo: Send + Sync {
    async fn get_by_id(&self, id: InstanceId) -> Result<Option<Instance>, RepoError>;
    async fn get_by_slug(&self, slug: &Slug) -> Result<Option<Instance>, RepoError>;
    async fn list_recent(&self, limit: u32) -> Result<Vec<Instance>, RepoError>;
    async fn upsert(&self, instance: &Instance) -> Result<(), RepoError>;
    async fn get_by_offre_id(
        &self,
        offre_id: domain::OffreId,
    ) -> Result<Option<Instance>, RepoError>;
    async fn get_by_offre_and_profil(
        &self,
        offre_id: domain::OffreId,
        profil_id: domain::ProfilId,
    ) -> Result<Option<Instance>, RepoError>;
}

#[async_trait]
pub trait MessageRepo: Send + Sync {
    async fn list_by_instance_id(
        &self,
        instance_id: InstanceId,
    ) -> Result<Vec<domain::Message>, RepoError>;
    async fn list_by_profil_id(
        &self,
        profil_id: ProfilId,
    ) -> Result<Vec<domain::Message>, RepoError>;
    async fn push(&self, message: &domain::Message) -> Result<(), RepoError>;
    async fn delete_all_for_instance(&self, instance_id: InstanceId) -> Result<(), RepoError>;
}

#[derive(Debug, Error)]
pub enum RepoError {
    #[error("erreur SQL : {0}")]
    Sql(String),

    #[error("contrainte d'unicité violée : {0}")]
    UniqueViolation(String),

    #[error("référence introuvable : {0}")]
    NotFound(String),

    #[error("autre : {0}")]
    Other(String),
}
