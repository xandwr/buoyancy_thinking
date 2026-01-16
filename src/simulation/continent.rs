use serde::{Deserialize, Serialize};

/// Great Unconformity - permanent continental landmass formed from critical pressure.
/// Continents are solid ground in the fluid; emotions cannot exist in these layers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Continent {
    /// Name derived from ore composition (e.g., "foundation_of_beauty")
    pub name: String,
    /// Layer span where solid land exists (e.g., 0.7-0.9)
    pub depth_range: (f32, f32),
    /// Which ore deposits melted together to form this
    pub formed_from_ores: Vec<String>,
    /// Combined wisdom that solidified into bedrock
    pub total_integration: f32,
    /// How much it blocks fluid flow (0.9 = nearly solid)
    pub impermeability: f32,
    /// Which tectonic shift created this
    pub formation_event: u32,
}

impl Continent {
    /// Check if a depth falls within this continent's range.
    pub fn contains_depth(&self, depth: f32) -> bool {
        depth >= self.depth_range.0 && depth <= self.depth_range.1
    }
}
