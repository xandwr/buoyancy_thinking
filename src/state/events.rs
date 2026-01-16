use serde::Serialize;
use uuid::Uuid;

/// Events emitted by the fluid simulation.
/// Only significant events are broadcast - the "Consciousness Filter"
/// ignores microscopic position updates of every water molecule.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum FluidEvent {
    // === Concept lifecycle (significant only) ===
    /// A new thought has been injected into the fluid
    ConceptInjected {
        id: Uuid,
        name: String,
        density: f32,
        layer: f32,
    },

    /// A thought has broken through the surface into action
    SurfaceBreakthrough {
        id: Uuid,
        name: String,
        kinetic_energy: f32,
    },

    /// A thought bounced off the surface (not enough energy)
    SurfaceBounce {
        id: Uuid,
        name: String,
        kinetic_energy: f32,
        required: f32,
    },

    /// A concept has evaporated into a permanent character trait
    ConceptEvaporated {
        id: Uuid,
        name: String,
        trait_formed: String,
        integration: f32,
    },

    // === Phase changes ===
    /// The fluid has frozen around a dominant thought
    Freeze {
        concept_id: Uuid,
        concept_name: String,
    },

    /// The freeze has been broken (external intervention)
    Thaw,

    /// Turbulence has begun (chaotic state)
    TurbulenceOnset { reynolds_number: f32, energy: f32 },

    /// Turbulence has subsided
    TurbulenceSubsided,

    // === Thermal/Mineralization events ===
    /// A dark thought has deposited ore after cycling through a vent
    Mineralization {
        concept_name: String,
        ore_name: String,
        ore_type: String,
        depth: f32,
        vent_cycles: u32,
        integration_value: f32,
    },

    /// Ore has been deposited, contributing to tectonic pressure
    OreDeposited {
        name: String,
        ore_type: String,
        total_pressure: f32,
        threshold: f32,
    },

    /// A benthic expedition has catalyzed a solution from ore
    OreCatalysis {
        problem: String,
        ore: String,
        solution: String,
        reactivity: f32,
    },

    // === Tectonic events ===
    /// The Great Unconformity - a tectonic shift has created new bedrock
    TectonicShift {
        continent_name: String,
        depth_range: (f32, f32),
        ores_consumed: Vec<String>,
        total_integration: f32,
    },

    // === Core truth events ===
    /// A new core truth (vent) has been added
    CoreTruthFormed {
        name: String,
        depth: f32,
        heat_output: f32,
        radius: f32,
    },

    /// A core truth has been strengthened by an encounter
    CoreTruthStrengthened {
        name: String,
        heat_output: f32,
        activation_count: u32,
    },

    // === Other significant events ===
    /// A character trait has precipitated a new thought
    Precipitation {
        trait_name: String,
        new_concept: String,
        inherited_integration: f32,
    },

    /// Flash heal has diluted salinity
    FlashHeal {
        concepts_added: usize,
        old_salinity: f32,
        new_salinity: f32,
    },

    /// Deep breath applied damping
    DeepBreath { strength: f32 },

    /// Ballast applied for benthic expedition
    BenthicExpedition {
        concept_id: Uuid,
        concept_name: String,
        ballast_amount: f32,
    },
}
