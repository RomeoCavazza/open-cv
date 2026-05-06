use chrono::{DateTime, Utc};
use domain::{
    Annexe, AnnexeId, Chunk, ChunkId, Instance, InstanceId, InstanceStatus, Message, MessageRole,
    Offre, OffreId, OffreStructured, Profil, ProfilId, Slug,
};
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

pub(crate) struct OffreRow {
    pub id: uuid::Uuid,
    pub slug: String,
    pub source_url: String,
    pub source_host: String,
    pub source_hash: Vec<u8>,
    pub entreprise: String,
    pub intitule: String,
    pub localisation: Option<String>,
    pub contrat: Option<String>,
    pub raw_text: String,
    pub structured: serde_json::Value,
    pub scraped_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub categorie: Option<String>,
}

pub(super) fn build_offre(row: OffreRow) -> Result<Offre, RepoError> {
    Ok(Offre {
        id: OffreId::from_uuid(row.id),
        slug: Slug::parse(row.slug).map_err(|e| RepoError::Other(e.to_string()))?,
        source_url: row.source_url,
        source_host: row.source_host,
        source_hash: row.source_hash,
        entreprise: row.entreprise,
        intitule: row.intitule,
        localisation: row.localisation,
        contrat: row.contrat,
        raw_text: row.raw_text,
        structured: serde_json::from_value::<OffreStructured>(row.structured)
            .map_err(|e| RepoError::Other(e.to_string()))?,
        scraped_at: row.scraped_at,
        last_seen_at: row.last_seen_at,
        closed_at: row.closed_at,
        categorie: row.categorie,
    })
}

pub(crate) struct InstanceRow {
    pub id: uuid::Uuid,
    pub slug: String,
    pub offre_id: uuid::Uuid,
    pub profil_id: uuid::Uuid,
    pub status: String,
    pub restitution: Option<serde_json::Value>,
    pub resume_json: Option<serde_json::Value>,
    pub cover_letter_json: Option<serde_json::Value>,
    pub notes: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub sent_at: Option<DateTime<Utc>>,
}

pub(super) fn build_instance(row: InstanceRow) -> Result<Instance, RepoError> {
    Ok(Instance {
        id: InstanceId::from_uuid(row.id),
        slug: Slug::parse(row.slug).map_err(|e| RepoError::Other(e.to_string()))?,
        offre_id: domain::OffreId::from_uuid(row.offre_id),
        profil_id: domain::ProfilId::from_uuid(row.profil_id),
        status: parse_status(&row.status)?,
        restitution: row.restitution.and_then(|value| {
            if value.is_null() {
                None
            } else {
                serde_json::from_value(value).ok()
            }
        }),
        resume_json: row.resume_json.and_then(|value| {
            if value.is_null() {
                None
            } else {
                serde_json::from_value(value).ok()
            }
        }),
        cover_letter_json: row.cover_letter_json.and_then(|value| {
            if value.is_null() {
                None
            } else {
                serde_json::from_value(value).ok()
            }
        }),
        notes: serde_json::from_value(row.notes).unwrap_or_default(),
        created_at: row.created_at,
        updated_at: row.updated_at,
        sent_at: row.sent_at,
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

pub(crate) struct ProfilRow {
    pub id: uuid::Uuid,
    pub label: String,
    pub content: serde_json::Value,
    pub is_active: bool,
    pub profile_photo: Option<Vec<u8>>,
    pub calendar_pdf: Option<Vec<u8>>,
    pub resume_template: Option<serde_json::Value>,
    pub cover_letter_template: Option<serde_json::Value>,
    pub notes: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

pub(super) fn build_profil(row: ProfilRow) -> Profil {
    Profil {
        id: ProfilId::from_uuid(row.id),
        label: row.label,
        content: serde_json::from_value(row.content).unwrap_or_default(),
        is_active: row.is_active,
        profile_photo: row.profile_photo,
        calendar_pdf: row.calendar_pdf,
        resume_template: row
            .resume_template
            .and_then(|v| serde_json::from_value(v).ok()),
        cover_letter_template: row
            .cover_letter_template
            .and_then(|v| serde_json::from_value(v).ok()),
        notes: serde_json::from_value(row.notes).unwrap_or_default(),
        created_at: row.created_at,
    }
}

pub(crate) struct ChunkRow {
    pub id: uuid::Uuid,
    pub profil_id: uuid::Uuid,
    pub kind: String,
    pub titre: String,
    pub content: String,
    pub metadata: serde_json::Value,
    pub embedding: String,
    pub created_at: DateTime<Utc>,
    pub distance: f64,
}

pub(super) fn build_chunk(row: ChunkRow) -> (Chunk, f32) {
    (
        Chunk {
            id: ChunkId::from_uuid(row.id),
            profil_id: ProfilId::from_uuid(row.profil_id),
            kind: parse_chunk_kind(&row.kind),
            titre: row.titre,
            content: row.content,
            metadata: serde_json::from_value(row.metadata).unwrap_or_default(),
            embedding: parse_embedding(&row.embedding),
            created_at: row.created_at,
        },
        row.distance as f32,
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
        let profil = build_profil(ProfilRow {
            id: uuid::Uuid::nil(),
            label: "Label".into(),
            content: json!({"a": 1}),
            is_active: true,
            profile_photo: None,
            calendar_pdf: None,
            resume_template: None,
            cover_letter_template: None,
            notes: json!({}),
            created_at: Utc::now(),
        });
        assert_eq!(profil.label, "Label");
    }
}
