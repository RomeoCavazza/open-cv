//! Ports — les traits que le domaine exige des adapters.
//!
//! Toute interaction avec le monde extérieur (DB, LLM, scraping) passe par
//! un trait défini ici. Le code applicatif dépend de ces traits, jamais d'une
//! implémentation concrète.

pub mod embedder;
pub mod llm;
pub mod repos;
pub mod scraper;

pub use embedder::*;
pub use llm::*;
pub use repos::*;
pub use scraper::*;
