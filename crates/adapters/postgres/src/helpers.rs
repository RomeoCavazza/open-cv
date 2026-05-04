use chrono::{DateTime, Utc};
use domain::{Annexe, AnnexeId, Chunk, ChunkId, Instance, InstanceId, InstanceStatus, Message, MessageRole, Offre, OffreId, OffreStructured, Profil, ProfilId, Slug};
use ports::RepoError;

pub(super) fn map_sqlx(e: sqlx::Error) -> RepoError {
    match &e {
        sqlx::Error::Database(db) if db.is_unique_violation() => {
            RepoError::UniqueViolation(db.message().to_string())
        }
        _ => RepoError::Sql(e.to_string()),
    }
}

pub(super) fn parse_status(s: &str) -> Result<InstanceStatus, RepoError> {
    match s {
        "draft" => Ok(InstanceStatus::Draft),
        "generating" => Ok(InstanceStatus::Generating),
        "ready" => Ok(InstanceStatus::Ready),
        "sent" => Ok(InstanceStatus::Sent),
        "archived" => Ok(InstanceStatus::Archived),
        "failed" => Ok(InstanceStatus::Failed),
        _ => Err(RepoError::Other(format!("statut inconnu : {s}"))),
    }
}

pub(super) fn parse_role(s: &str) -> Result<MessageRole, RepoError> {
    match s {
        "user" => Ok(MessageRole::User),
        "assistant" => Ok(MessageRole::Assistant),
        "system" => Ok(MessageRole::System),
        _ => Err(RepoError::Other(format!("rôle inconnu : {s}"))),
    }
}

pub(super) fn parse_chunk_kind(s: &str) -> domain::ChunkKind {
    match s {
        "experience" => domain::ChunkKind::Experience,
        "projet" => domain::ChunkKind::Projet,
        "formation" => domain::ChunkKind::Formation,
        "competence" => domain::ChunkKind::Competence,
        "phrase_lettre" => domain::ChunkKind::PhraseLettre,
        _ => domain::ChunkKind::Experience,
    }
}

pub(super) fn parse_embedding(embedding: &str) -> Vec<f32> {
    embedding
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|value| value.parse().unwrap_or(0.0))
        .collect()
}

pub(super) fn build_offre(
    id: uuid::Uuid,
    slug: String,
    source_url: String,
    source_host: String,
    source_hash: Vec<u8>,
    entreprise: String,
    intitule: String,
    localisation: Option<String>,
    contrat: Option<String>,
    raw_text: String,
    structured: serde_json::Value,
    scraped_at: DateTime<Utc>,
    last_seen_at: DateTime<Utc>,
    closed_at: Option<DateTime<Utc>>,
    categorie: Option<String>,
) -> Result<Offre, RepoError> {
    Ok(Offre {
        id: OffreId::from_uuid(id),
        slug: Slug::parse(slug).map_err(|e| RepoError::Other(e.to_string()))?,
        source_url,
        source_host,
        source_hash,
        entreprise,
        intitule,
        localisation,
        contrat,
        raw_text,
        structured: serde_json::from_value::<OffreStructured>(structured)
            .map_err(|e| RepoError::Other(e.to_string()))?,
        scraped_at,
        last_seen_at,
        closed_at,
        categorie,
    })
}

pub(super) fn build_instance(
    id: uuid::Uuid,
    slug: String,
    offre_id: uuid::Uuid,
    profil_id: uuid::Uuid,
    status: String,
    restitution: Option<serde_json::Value>,
    resume_json: Option<serde_json::Value>,
    cover_letter_json: Option<serde_json::Value>,
    notes: serde_json::Value,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    sent_at: Option<DateTime<Utc>>,
) -> Result<Instance, RepoError> {
    Ok(Instance {
        id: InstanceId::from_uuid(id),
        slug: Slug::parse(slug).map_err(|e| RepoError::Other(e.to_string()))?,
        offre_id: domain::OffreId::from_uuid(offre_id),
        profil_id: domain::ProfilId::from_uuid(profil_id),
        status: parse_status(&status)?,
        restitution,
        resume_json,
        cover_letter_json,
        notes,
        created_at,
        updated_at,
        sent_at,
    })
}

pub(super) fn build_annexe(
    id: uuid::Uuid,
    profil_id: uuid::Uuid,
    label: String,
    filename: String,
    content_type: String,
    content: Vec<u8>,
    created_at: DateTime<Utc>,
) -> Annexe {
    Annexe {
        id: AnnexeId::from_uuid(id),
        profil_id: ProfilId::from_uuid(profil_id),
        label,
        filename,
        content_type,
        content,
        created_at,
    }
}

pub(super) fn build_message(
    id: uuid::Uuid,
    instance_id: uuid::Uuid,
    role: String,
    content: String,
    created_at: DateTime<Utc>,
) -> Result<Message, RepoError> {
    Ok(Message {
        id,
        instance_id: InstanceId::from_uuid(instance_id),
        role: parse_role(&role)?,
        content,
        created_at,
    })
}

pub(super) fn build_profil(
    id: uuid::Uuid,
    label: String,
    content: serde_json::Value,
    is_active: bool,
    profile_photo: Option<Vec<u8>>,
    calendar_pdf: Option<Vec<u8>>,
    resume_template: Option<serde_json::Value>,
    cover_letter_template: Option<serde_json::Value>,
    notes: serde_json::Value,
    created_at: DateTime<Utc>,
) -> Profil {
    Profil {
        id: ProfilId::from_uuid(id),
        label,
        content,
        is_active,
        profile_photo,
        calendar_pdf,
        resume_template,
        cover_letter_template,
        notes,
        created_at,
    }
}

pub(super) fn build_chunk(
    id: uuid::Uuid,
    profil_id: uuid::Uuid,
    kind: String,
    titre: String,
    content: String,
    metadata: serde_json::Value,
    embedding: String,
    created_at: DateTime<Utc>,
    distance: f64,
) -> (Chunk, f32) {
    (
        Chunk {
            id: ChunkId::from_uuid(id),
            profil_id: ProfilId::from_uuid(profil_id),
            kind: parse_chunk_kind(&kind),
            titre,
            content,
            metadata,
            embedding: parse_embedding(&embedding),
            created_at,
        },
        distance as f32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parse_status_accepts_known_values() {
        assert!(matches!(parse_status("draft"), Ok(InstanceStatus::Draft)));
        assert!(parse_status("nope").is_err());
    }

    #[test]
    fn parse_role_accepts_known_values() {
        assert!(matches!(parse_role("user"), Ok(MessageRole::User)));
        assert!(parse_role("nope").is_err());
    }

    #[test]
    fn parse_embedding_handles_vectors() {
        assert_eq!(parse_embedding("[1,2,3]"), vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn build_profil_keeps_payload() {
        let profil = build_profil(
            uuid::Uuid::nil(),
            "Label".into(),
            json!({"a": 1}),
            true,
            None,
            None,
            None,
            None,
            json!({}),
            Utc::now(),
        );
        assert_eq!(profil.label, "Label");
    }
}
