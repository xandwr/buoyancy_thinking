use serde::{Deserialize, Serialize};

/// Deep sea hydrothermal vent - a core truth that radiates heat from the ocean floor.
/// Core truths are foundational beliefs that create upward thermal currents,
/// transforming heavy/dark thoughts as they pass through the heat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTruth {
    pub name: String,
    /// Thermal energy radiating from this truth
    pub heat_output: f32,
    /// Position in fluid (always near bottom: 0.85-0.95)
    pub depth: f32,
    /// Area of influence for thermal plume
    pub radius: f32,
    /// Strengthens each time concepts encounter it
    pub activation_count: u32,
}

impl CoreTruth {
    pub fn new(name: String, heat_output: f32, depth: f32, radius: f32) -> Self {
        Self {
            name,
            heat_output,
            depth,
            radius,
            activation_count: 0,
        }
    }
}
