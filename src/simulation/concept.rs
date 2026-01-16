use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for a concept (thought) in the fluid.
/// Uses UUID for API ergonomics - each concept has a "soul fingerprint".
pub type ConceptId = Uuid;

/// A concept (thought) in the fluid medium with physical properties.
/// Concepts float, sink, freeze, evaporate, and break through the surface
/// based on their density, buoyancy, and accumulated integration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Concept {
    pub id: ConceptId,
    pub name: String,
    /// Intrinsic weight (0.0 to 1.0) - how "heavy" this thought is
    pub density: f32,
    /// Current effective buoyancy - how much it wants to rise
    pub buoyancy: f32,
    /// Continuous depth (0.0 = surface, 1.0 = bottom)
    pub layer: f32,
    /// Rate of layer change (positive = sinking, negative = rising)
    pub velocity: f32,
    /// "Surface area" - connectivity to other concepts (affects drag)
    pub area: f32,
    /// Has this concept triggered an action?
    pub has_broken_surface: bool,
    /// How long this concept has been at the surface (layer ≈ 0)
    pub time_at_surface: f32,
    /// Is this concept causing a freeze?
    pub is_frozen: bool,
    /// "Internal heat" - accumulated understanding/memory
    pub integration: f32,
    /// Current eddy size (large spike → smaller reflections)
    pub eddy_scale: f32,
    /// Has this concept left the fluid to become a trait?
    pub has_evaporated: bool,
    /// Temporary density increase for benthic expedition (0.0 = none)
    pub ballast: f32,
    /// Was this synthesized from problem + ore?
    pub is_solution: bool,
}

impl Concept {
    /// Create a new concept with default physics state.
    pub fn new(id: ConceptId, name: String, density: f32, area: f32) -> Self {
        Self {
            id,
            name,
            density,
            buoyancy: density,         // Start with buoyancy = density
            layer: density,            // Initial layer matches density
            velocity: 0.0,             // Start at rest
            area,                      // Connectivity/surface area
            has_broken_surface: false, // Not yet activated
            time_at_surface: 0.0,      // No time at surface yet
            is_frozen: false,          // Not frozen
            integration: 0.0,          // No accumulated understanding yet
            eddy_scale: 0.0,           // No turbulent motion yet
            has_evaporated: false,     // Still in fluid state
            ballast: 0.0,              // No ballast
            is_solution: false,        // Not a solution
        }
    }

    /// Derive volume from density and area.
    /// Volume represents "how much space this thought occupies in consciousness".
    pub fn volume(&self) -> f32 {
        self.density * self.area
    }

    /// Get the current status of this concept as a string.
    pub fn status(&self) -> &'static str {
        if self.is_frozen {
            "frozen"
        } else if self.has_evaporated {
            "evaporated"
        } else if self.velocity < -0.01 {
            "rising"
        } else if self.velocity > 0.01 {
            "sinking"
        } else {
            "floating"
        }
    }
}
