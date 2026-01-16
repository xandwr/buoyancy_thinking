use std::sync::Arc;

use axum::{Json, extract::State};
use serde::Serialize;
use uuid::Uuid;

use crate::state::AppState;

#[derive(Serialize)]
pub struct ConceptSummary {
    pub id: Uuid,
    pub name: String,
    pub layer: f32,
    pub velocity: f32,
    pub density: f32,
    pub buoyancy: f32,
    pub integration: f32,
    pub status: String,
    pub is_frozen: bool,
    pub has_broken_surface: bool,
}

#[derive(Serialize)]
pub struct CoreTruthSummary {
    pub name: String,
    pub heat_output: f32,
    pub depth: f32,
    pub radius: f32,
    pub activation_count: u32,
}

#[derive(Serialize)]
pub struct OreSummary {
    pub name: String,
    pub ore_type: String,
    pub depth: f32,
    pub integration_value: f32,
}

#[derive(Serialize)]
pub struct ContinentSummary {
    pub name: String,
    pub depth_range: (f32, f32),
    pub total_integration: f32,
}

#[derive(Serialize)]
pub struct TraitSummary {
    pub name: String,
    pub integration: f32,
}

#[derive(Serialize)]
pub struct FluidStateResponse {
    // Entities
    pub concepts: Vec<ConceptSummary>,
    pub core_truths: Vec<CoreTruthSummary>,
    pub ore_deposits: Vec<OreSummary>,
    pub continents: Vec<ContinentSummary>,
    pub atmosphere: Vec<TraitSummary>,

    // Global state
    pub is_frozen: bool,
    pub is_turbulent: bool,
    pub turbulence_energy: f32,
    pub total_integration: f32,
    pub salinity: f32,
    pub ocean_floor_pressure: f32,
    pub pressure_threshold: f32,
    pub tectonic_shifts: u32,
}

/// GET /state - Full state snapshot
pub async fn get_full_state(State(state): State<Arc<AppState>>) -> Json<FluidStateResponse> {
    let fluid = state.fluid.read().await;

    let concepts: Vec<_> = fluid
        .concepts
        .values()
        .map(|c| ConceptSummary {
            id: c.id,
            name: c.name.clone(),
            layer: c.layer,
            velocity: c.velocity,
            density: c.density,
            buoyancy: c.buoyancy,
            integration: c.integration,
            status: c.status().to_string(),
            is_frozen: c.is_frozen,
            has_broken_surface: c.has_broken_surface,
        })
        .collect();

    let core_truths: Vec<_> = fluid
        .core_truths
        .iter()
        .map(|v| CoreTruthSummary {
            name: v.name.clone(),
            heat_output: v.heat_output,
            depth: v.depth,
            radius: v.radius,
            activation_count: v.activation_count,
        })
        .collect();

    let ore_deposits: Vec<_> = fluid
        .ore_deposits
        .iter()
        .map(|o| OreSummary {
            name: o.name.clone(),
            ore_type: o.ore_type.as_str().to_string(),
            depth: o.depth,
            integration_value: o.integration_value,
        })
        .collect();

    let continents: Vec<_> = fluid
        .continents
        .iter()
        .map(|c| ContinentSummary {
            name: c.name.clone(),
            depth_range: c.depth_range,
            total_integration: c.total_integration,
        })
        .collect();

    let atmosphere: Vec<_> = fluid
        .atmosphere
        .iter()
        .map(|t| TraitSummary {
            name: t.name.clone(),
            integration: t.integration,
        })
        .collect();

    Json(FluidStateResponse {
        concepts,
        core_truths,
        ore_deposits,
        continents,
        atmosphere,
        is_frozen: fluid.is_frozen,
        is_turbulent: fluid.is_turbulent,
        turbulence_energy: fluid.turbulence_energy,
        total_integration: fluid.total_integration,
        salinity: fluid.salinity,
        ocean_floor_pressure: fluid.ocean_floor_pressure,
        pressure_threshold: fluid.pressure_threshold,
        tectonic_shifts: fluid.tectonic_shifts,
    })
}
