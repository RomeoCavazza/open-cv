use domain::{Instance, InstanceId, Offre, Profil, ProfilId};
use ports::{AnnexeRepo, ChunkRepo, EmbedMode, Embedder, InstanceRepo, OffreRepo, ProfilRepo};
use std::sync::Arc;

#[derive(Clone)]
pub struct ChatContextLoader {
    pub offres: Arc<dyn OffreRepo>,
    pub instances: Arc<dyn InstanceRepo>,
    pub profils: Arc<dyn ProfilRepo>,
    pub annexes: Arc<dyn AnnexeRepo>,
    pub chunks: Arc<dyn ChunkRepo>,
    pub embedder: Arc<dyn Embedder>,
}

impl ChatContextLoader {
    pub async fn load_instance_context(
        &self,
        instance_id: &str,
    ) -> anyhow::Result<(Instance, Profil, Offre)> {
        let instance_uuid = uuid::Uuid::parse_str(instance_id)?;
        let instance = self
            .instances
            .get_by_id(InstanceId::from_uuid(instance_uuid))
            .await?
            .ok_or_else(|| anyhow::anyhow!("Instance non trouvée"))?;

        let profil = self
            .profils
            .get_by_id(instance.profil_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Profil non trouvé"))?;

        let offre = self
            .offres
            .get_by_id(instance.offre_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Offre non trouvée"))?;

        Ok((instance, profil, offre))
    }

    pub async fn load_active_profil(&self) -> anyhow::Result<Profil> {
        self.profils
            .get_active()
            .await?
            .ok_or_else(|| anyhow::anyhow!("Aucun profil actif trouvé"))
    }

    pub async fn get_rag_context(
        &self,
        profil_id: ProfilId,
        message: &str,
    ) -> anyhow::Result<String> {
        let query_text = format!("{} career context", message);
        let embeddings = self
            .embedder
            .embed(&[&query_text], EmbedMode::Query)
            .await?;

        let query_vec = embeddings
            .first()
            .ok_or_else(|| anyhow::anyhow!("No embeddings returned"))?;

        let chunks = self
            .chunks
            .top_k_by_embedding(profil_id, query_vec, 5)
            .await?;

        Ok(chunks
            .iter()
            .map(|(c, _)| format!("### {} - {}\n{}", c.kind.as_str(), c.titre, c.content))
            .collect::<Vec<_>>()
            .join("\n\n"))
    }
}
