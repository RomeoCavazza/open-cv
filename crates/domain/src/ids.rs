//! Identifiants typés. Évite de mélanger un `OffreId` et un `InstanceId`
//! au compile-time — c'est gratuit et ça vaut de l'or.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

macro_rules! define_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub Uuid);

        impl $name {
            pub fn new() -> Self {
                Self(Uuid::new_v4())
            }

            pub fn from_uuid(uuid: Uuid) -> Self {
                Self(uuid)
            }

            pub fn as_uuid(&self) -> &Uuid {
                &self.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl From<Uuid> for $name {
            fn from(u: Uuid) -> Self {
                Self(u)
            }
        }

        impl From<$name> for Uuid {
            fn from(id: $name) -> Self {
                id.0
            }
        }
    };
}

define_id!(OffreId);
define_id!(ProfilId);
define_id!(ChunkId);
define_id!(InstanceId);
define_id!(LlmCallId);
define_id!(AnnexeId);

/// Slug humain-lisible (ex: `safran_alternance_ia__a3b2c1d0`).
/// Historiquement stocké dans `data/instances/`, maintenant persisté en base PostgreSQL.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Slug(String);

impl Slug {
    /// Construit un slug en validant le format : [a-z0-9_]+
    pub fn parse(s: impl Into<String>) -> Result<Self, SlugError> {
        let s = s.into();
        if s.is_empty() {
            return Err(SlugError::Empty);
        }
        if !s
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-')
        {
            return Err(SlugError::InvalidChars);
        }
        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn new_v4() -> Self {
        Self(format!(
            "slug_{}",
            uuid::Uuid::new_v4().to_string().replace('-', "_")
        ))
    }
}

impl std::fmt::Display for Slug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SlugError {
    #[error("slug vide")]
    Empty,
    #[error("slug contient des caractères invalides (autorisés : a-z, 0-9, _, -)")]
    InvalidChars,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slug_valide() {
        assert!(Slug::parse("safran_alternance_ia_bordes").is_ok());
        assert!(Slug::parse("dassault-systemes-apprentissage-547531").is_ok());
    }

    #[test]
    fn slug_rejette_majuscules() {
        assert!(Slug::parse("Safran_Alternance").is_err());
    }

    #[test]
    fn slug_autorise_tirets() {
        assert!(Slug::parse("safran-alternance").is_ok());
    }

    #[test]
    fn slug_rejette_vide() {
        assert!(Slug::parse("").is_err());
    }
}
