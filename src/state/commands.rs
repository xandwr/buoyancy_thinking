use tokio::sync::oneshot;
use uuid::Uuid;

/// Commands sent from API handlers to the simulation loop.
/// These are "Willful Acts" - deliberate interventions in the fluid.
#[derive(Debug)]
pub enum Command {
    /// Inject a new concept into the fluid
    Inject {
        name: String,
        density: f32,
        area: f32,
        response_tx: oneshot::Sender<Uuid>,
    },

    /// Apply ballast to force benthic descent
    Ballast { concept_id: Uuid, weight_delta: f32 },

    /// Modulate buoyancy externally
    ModulateBuoyancy { concept_id: Uuid, delta: f32 },

    /// Trigger manual tectonic shift by lowering threshold
    TriggerTectonic { pressure_threshold: f32 },

    /// Thaw frozen state
    Thaw,

    /// Apply deep breath damping
    DeepBreath { strength: f32 },

    /// Add a core truth (vent)
    AddCoreTruth {
        name: String,
        heat_output: f32,
        depth: f32,
        radius: f32,
    },

    /// Flash heal with fresh concepts
    FlashHeal {
        concepts: Vec<(String, f32, f32)>,
        dilution_strength: f32,
    },

    /// Precipitate a new thought from a character trait
    Precipitate {
        trait_index: usize,
        new_concept_name: String,
        density: f32,
        area: f32,
    },

    /// Start a division experiment (analog computing)
    /// Salinity boost enables Laminar Streamlining for clearer remainder detection
    StartDivisionExperiment {
        dividend: f32,
        divisor: f32,
        salinity_boost: f32,
        response_tx: oneshot::Sender<Uuid>,
    },

    /// Start a consensus experiment (contradictory vent collision)
    /// Injects two opposing positions and crystallizes stable insight
    StartConsensusExperiment {
        position_a: String,
        heat_a: f32,
        position_b: String,
        heat_b: f32,
        response_tx: oneshot::Sender<Uuid>,
    },
}
