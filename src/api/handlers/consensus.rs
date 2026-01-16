use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::simulation::consensus_reactor::VentDominance;
use crate::state::{AppState, Command};

#[derive(Deserialize)]
pub struct ConsensusRequest {
    /// First contradictory position (e.g., "Privacy is absolute")
    pub position_a: String,
    /// Conviction strength of first position (0.1-2.0)
    #[serde(default = "default_heat")]
    pub heat_a: f32,
    /// Second contradictory position (e.g., "Transparency is mandatory")
    pub position_b: String,
    /// Conviction strength of second position (0.1-2.0)
    #[serde(default = "default_heat")]
    pub heat_b: f32,
}

fn default_heat() -> f32 {
    1.0
}

#[derive(Serialize)]
pub struct ConsensusStartResponse {
    pub experiment_id: Uuid,
    pub position_a: String,
    pub position_b: String,
    pub heat_a: f32,
    pub heat_b: f32,
    pub probe_count: usize,
    pub message: String,
}

#[derive(Serialize)]
pub struct ConsensusStatusResponse {
    pub active: bool,
    pub position_a: Option<String>,
    pub position_b: Option<String>,
    pub current_certainty: Option<f32>,
    pub accumulated_jitter: Option<f32>,
    pub peak_jitter: Option<f32>,
    pub ticks_elapsed: Option<u64>,
    pub stable_ticks: Option<u32>,
}

#[derive(Serialize)]
pub struct ConsensusOreResponse {
    pub id: Uuid,
    pub name: String,
    pub ore_type: String,
    pub position_a: String,
    pub position_b: String,
    pub certainty: f32,
    pub quality: String,
    pub is_foundational: bool,
    pub insight: Option<String>,
    pub accumulated_jitter: f32,
    pub crystallization_time: u64,
    pub integration_value: f32,
    /// The extracted phase structure (physical topology) - the "new material"
    pub phase_structure: Option<PhaseStructureResponse>,
}

/// The physical structure extracted at phase transition.
/// This is NOT a compromise - it's what SURVIVES the collision dynamics.
#[derive(Serialize)]
pub struct PhaseStructureResponse {
    pub id: Uuid,
    pub transition_tick: u64,
    pub trigger_jitter: f32,
    /// Territory controlled by position A (0.0-1.0)
    pub vent_a_territory: f32,
    /// Territory controlled by position B (0.0-1.0)
    pub vent_b_territory: f32,
    /// Contested zone where neither dominates
    pub contested_territory: f32,
    /// Depth where territories collide
    pub collision_boundary: f32,
    /// The synthesized "new material" name
    pub material_name: String,
    /// Description of the new material's properties
    pub material_description: String,
    /// Emergent properties (what NEITHER input had)
    pub emergent_properties: Vec<EmergentPropertyResponse>,
    /// Voronoi cells (territory map)
    pub voronoi_cells: Vec<VoronoiCellResponse>,
}

#[derive(Serialize)]
pub struct EmergentPropertyResponse {
    pub name: String,
    pub physical_basis: String,
    pub confidence: f32,
    pub depth_range: (f32, f32),
}

#[derive(Serialize)]
pub struct VoronoiCellResponse {
    pub center: f32,
    pub left_bound: f32,
    pub right_bound: f32,
    pub width: f32,
    pub dominance: String,
}

/// POST /consensus - Start a consensus experiment
///
/// Inject two contradictory positions as thermal vents and watch
/// probe bubbles jostle until a stable insight crystallizes.
///
/// The certainty metric C = 1 / (1 + ∫|Jitter|dt) determines quality:
/// - C → 1: "Foundational Truth" (low jitter, stable convergence)
/// - C → 0: "Noise" (high jitter, chaotic oscillation)
pub async fn start_consensus(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ConsensusRequest>,
) -> Result<Json<ConsensusStartResponse>, (StatusCode, String)> {
    // Validate inputs
    if req.position_a.is_empty() || req.position_b.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Positions cannot be empty".into()));
    }
    if req.heat_a < 0.1 || req.heat_a > 2.0 {
        return Err((
            StatusCode::BAD_REQUEST,
            "heat_a must be between 0.1 and 2.0".into(),
        ));
    }
    if req.heat_b < 0.1 || req.heat_b > 2.0 {
        return Err((
            StatusCode::BAD_REQUEST,
            "heat_b must be between 0.1 and 2.0".into(),
        ));
    }

    // Create response channel
    let (tx, rx) = oneshot::channel();

    // Send command
    state
        .command_tx
        .send(Command::StartConsensusExperiment {
            position_a: req.position_a.clone(),
            heat_a: req.heat_a,
            position_b: req.position_b.clone(),
            heat_b: req.heat_b,
            response_tx: tx,
        })
        .await
        .map_err(|_| {
            (
                StatusCode::SERVICE_UNAVAILABLE,
                "Simulation not running".into(),
            )
        })?;

    // Wait for experiment ID
    let experiment_id = rx.await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to start consensus experiment".into(),
        )
    })?;

    let heat_comparison = if (req.heat_a - req.heat_b).abs() < 0.2 {
        "balanced conviction"
    } else if req.heat_a > req.heat_b {
        "first position stronger"
    } else {
        "second position stronger"
    };

    let message = format!(
        "Consensus Reactor ignited. '{}' collides with '{}' ({}).\n\
         Probe bubbles injected into collision zone. \
         Watching for crystallization...",
        req.position_a, req.position_b, heat_comparison
    );

    Ok(Json(ConsensusStartResponse {
        experiment_id,
        position_a: req.position_a,
        position_b: req.position_b,
        heat_a: req.heat_a,
        heat_b: req.heat_b,
        probe_count: 8, // Hardcoded for now, matches fluid.rs
        message,
    }))
}

/// GET /consensus/status - Get current consensus experiment status
pub async fn get_consensus_status(
    State(state): State<Arc<AppState>>,
) -> Json<ConsensusStatusResponse> {
    let fluid = state.fluid.read().await;

    if let Some(exp) = fluid.get_consensus_experiment() {
        let ticks_elapsed = fluid.tick_count.saturating_sub(exp.start_tick);

        Json(ConsensusStatusResponse {
            active: true,
            position_a: Some(exp.vent_a.position.clone()),
            position_b: Some(exp.vent_b.position.clone()),
            current_certainty: Some(exp.certainty()),
            accumulated_jitter: Some(exp.accumulated_jitter),
            peak_jitter: Some(exp.peak_jitter),
            ticks_elapsed: Some(ticks_elapsed),
            stable_ticks: Some(exp.stable_ticks),
        })
    } else {
        Json(ConsensusStatusResponse {
            active: false,
            position_a: None,
            position_b: None,
            current_certainty: None,
            accumulated_jitter: None,
            peak_jitter: None,
            ticks_elapsed: None,
            stable_ticks: None,
        })
    }
}

/// Convert a ConsensusOre to API response format
fn ore_to_response(ore: &crate::simulation::ConsensusOre) -> ConsensusOreResponse {
    let phase_structure = ore
        .phase_structure
        .as_ref()
        .map(|ps| PhaseStructureResponse {
            id: ps.id,
            transition_tick: ps.transition_tick,
            trigger_jitter: ps.trigger_jitter,
            vent_a_territory: ps.vent_a_territory,
            vent_b_territory: ps.vent_b_territory,
            contested_territory: ps.contested_territory,
            collision_boundary: ps.collision_boundary,
            material_name: ps.material_name.clone(),
            material_description: ps.material_description.clone(),
            emergent_properties: ps
                .emergent_properties
                .iter()
                .map(|ep| EmergentPropertyResponse {
                    name: ep.name.clone(),
                    physical_basis: ep.physical_basis.clone(),
                    confidence: ep.confidence,
                    depth_range: ep.depth_range,
                })
                .collect(),
            voronoi_cells: ps
                .voronoi_cells
                .iter()
                .map(|vc| VoronoiCellResponse {
                    center: vc.center,
                    left_bound: vc.left_bound,
                    right_bound: vc.right_bound,
                    width: vc.width,
                    dominance: match vc.dominance {
                        VentDominance::VentA => "vent_a".to_string(),
                        VentDominance::VentB => "vent_b".to_string(),
                        VentDominance::Contested => "contested".to_string(),
                        VentDominance::Escaped => "escaped".to_string(),
                    },
                })
                .collect(),
        });

    ConsensusOreResponse {
        id: ore.id,
        name: ore.name.clone(),
        ore_type: ore.ore_type.as_str().to_string(),
        position_a: ore.vent_a.clone(),
        position_b: ore.vent_b.clone(),
        certainty: ore.certainty,
        quality: ore.quality().to_string(),
        is_foundational: ore.is_foundational(),
        insight: ore.insight.clone(),
        accumulated_jitter: ore.accumulated_jitter,
        crystallization_time: ore.crystallization_time,
        integration_value: ore.integration_value,
        phase_structure,
    }
}

/// GET /consensus/ores - Get all crystallized consensus ores
pub async fn get_consensus_ores(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<ConsensusOreResponse>> {
    let fluid = state.fluid.read().await;

    let ores: Vec<ConsensusOreResponse> = fluid
        .get_consensus_ores()
        .iter()
        .map(ore_to_response)
        .collect();

    Json(ores)
}

/// GET /consensus/truths - Get foundational truths (C > 0.8)
pub async fn get_foundational_truths(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<ConsensusOreResponse>> {
    let fluid = state.fluid.read().await;

    let truths: Vec<ConsensusOreResponse> = fluid
        .get_foundational_truths()
        .iter()
        .map(|ore| ore_to_response(ore))
        .collect();

    Json(truths)
}
