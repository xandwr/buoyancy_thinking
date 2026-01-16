use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::state::AppState;

#[derive(Deserialize)]
pub struct StrataQuery {
    #[serde(default)]
    pub depth_min: Option<f32>,
    #[serde(default)]
    pub depth_max: Option<f32>,
}

#[derive(Serialize)]
pub struct ConceptView {
    pub id: Uuid,
    pub name: String,
    pub layer: f32,
    pub velocity: f32,
    pub density: f32,
    pub buoyancy: f32,
    pub integration: f32,
    pub status: String,
}

#[derive(Serialize)]
pub struct OreView {
    pub name: String,
    pub ore_type: String,
    pub depth: f32,
    pub integration_value: f32,
    pub vent_cycles: u32,
}

#[derive(Serialize)]
pub struct StrataResponse {
    pub depth_range: (f32, f32),
    pub concepts: Vec<ConceptView>,
    pub ores: Vec<OreView>,
    pub total_concepts: usize,
    pub total_ores: usize,
}

/// GET /strata - View concepts and ores within a depth range
pub async fn get_strata(
    State(state): State<Arc<AppState>>,
    Query(query): Query<StrataQuery>,
) -> Json<StrataResponse> {
    let fluid = state.fluid.read().await;

    let depth_min = query.depth_min.unwrap_or(0.0);
    let depth_max = query.depth_max.unwrap_or(1.0);

    // Filter concepts in range
    let concepts: Vec<_> = fluid
        .concepts
        .values()
        .filter(|c| c.layer >= depth_min && c.layer <= depth_max)
        .map(|c| ConceptView {
            id: c.id,
            name: c.name.clone(),
            layer: c.layer,
            velocity: c.velocity,
            density: c.density,
            buoyancy: c.buoyancy,
            integration: c.integration,
            status: c.status().to_string(),
        })
        .collect();

    // Filter ores in range
    let ores: Vec<_> = fluid
        .ore_deposits
        .iter()
        .filter(|o| o.depth >= depth_min && o.depth <= depth_max)
        .map(|o| OreView {
            name: o.name.clone(),
            ore_type: o.ore_type.as_str().to_string(),
            depth: o.depth,
            integration_value: o.integration_value,
            vent_cycles: o.vent_cycles,
        })
        .collect();

    Json(StrataResponse {
        depth_range: (depth_min, depth_max),
        total_concepts: concepts.len(),
        total_ores: ores.len(),
        concepts,
        ores,
    })
}
