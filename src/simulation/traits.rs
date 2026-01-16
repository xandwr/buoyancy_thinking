use serde::{Deserialize, Serialize};

use super::concept::ConceptId;

/// Evaporated concepts become permanent character traits.
/// These exist in the "atmosphere" above the fluid and can
/// precipitate new thoughts into the fluid.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterTrait {
    pub name: String,
    /// How much understanding went into this trait
    pub integration: f32,
    /// Which concept evaporated to form this
    pub formed_from: ConceptId,
}

impl CharacterTrait {
    pub fn new(name: String, integration: f32, formed_from: ConceptId) -> Self {
        Self {
            name,
            integration,
            formed_from,
        }
    }
}
