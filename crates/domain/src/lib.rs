//! Domaine métier — entités pures, sans dépendance infra.
//!
//! Ce crate ne dépend ni de tokio, ni de sqlx, ni de reqwest. Si vous y voyez
//! arriver une de ces dépendances, c'est qu'une responsabilité s'est mal
//! placée — il faut la déplacer dans `application` ou un `adapter`.

pub mod chunk;
pub mod cover_letter;
pub mod errors;
pub mod ids;
pub mod instance;
pub mod json;
pub mod message;
pub mod offre;
pub mod profil;
pub mod restitution;
pub mod resume;
pub mod snapshot;

pub use chunk::*;
pub use cover_letter::*;
pub use errors::*;
pub use ids::*;
pub use instance::*;
pub use json::*;
pub use message::*;
pub use offre::*;
pub use profil::*;
pub use restitution::*;
pub use resume::*;
pub use snapshot::*;
