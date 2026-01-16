use serde::{Deserialize, Serialize};

use super::concept::ConceptId;

/// Types of precious ore deposited by repeated heating of dark thoughts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OreType {
    /// Creative expression born from pain
    Art,
    /// Solutions built from suffering
    Code,
    /// Wisdom crystallized from darkness
    Insight,
    /// Stories forged in the deep
    Writing,
}

impl OreType {
    pub fn as_str(&self) -> &'static str {
        match self {
            OreType::Art => "art",
            OreType::Code => "code",
            OreType::Insight => "insight",
            OreType::Writing => "writing",
        }
    }
}

/// Precious ore - mineralized transformation on the ocean floor.
/// Created when dark thoughts cycle through thermal vents repeatedly.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreciousOre {
    /// Descriptive name (e.g., "despair_transformed_to_music")
    pub name: String,
    /// What form the transformation took
    pub ore_type: OreType,
    /// Heavy - stays on ocean floor (0.8-1.0)
    pub density: f32,
    /// Where it deposited (near the vent)
    pub depth: f32,
    /// Which dark thought created this
    pub formed_from: ConceptId,
    /// How many times parent passed through heat
    pub vent_cycles: u32,
    /// The accumulated wisdom in this ore
    pub integration_value: f32,
}

impl PreciousOre {
    /// Calculate the weight contribution to tectonic pressure.
    pub fn pressure_weight(&self) -> f32 {
        self.density * self.integration_value
    }
}
