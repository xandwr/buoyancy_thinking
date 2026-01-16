use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};

use crate::state::{AppState, Command};

#[derive(Deserialize)]
pub struct TectonicRequest {
    pub pressure_threshold: f32,
}

#[derive(Serialize)]
pub struct ContinentResponse {
    pub name: String,
    pub depth_range: (f32, f32),
    pub total_integration: f32,
    pub impermeability: f32,
    pub formation_event: u32,
    pub formed_from_ores: Vec<String>,
}

#[derive(Serialize)]
pub struct TectonicResponse {
    pub status: String,
    pub current_pressure: f32,
    pub threshold: f32,
}

/// POST /continent - Trigger tectonic shift by setting a new pressure threshold
pub async fn trigger_tectonic(
    State(state): State<Arc<AppState>>,
    Json(req): Json<TectonicRequest>,
) -> Result<Json<TectonicResponse>, (StatusCode, String)> {
    if req.pressure_threshold < 0.0 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Pressure threshold must be non-negative".into(),
        ));
    }

    let current_pressure = {
        let fluid = state.fluid.read().await;
        fluid.ocean_floor_pressure
    };

    // Send command to lower threshold (may trigger immediate tectonic shift)
    state
        .command_tx
        .send(Command::TriggerTectonic {
            pressure_threshold: req.pressure_threshold,
        })
        .await
        .map_err(|_| {
            (
                StatusCode::SERVICE_UNAVAILABLE,
                "Simulation not running".into(),
            )
        })?;

    let status = if current_pressure >= req.pressure_threshold {
        "Tectonic shift imminent - pressure exceeds new threshold"
    } else {
        "Threshold set - waiting for pressure to accumulate"
    };

    Ok(Json(TectonicResponse {
        status: status.into(),
        current_pressure,
        threshold: req.pressure_threshold,
    }))
}

/// GET /continents - List all continents (permanent bedrock)
pub async fn list_continents(State(state): State<Arc<AppState>>) -> Json<Vec<ContinentResponse>> {
    let fluid = state.fluid.read().await;

    let continents: Vec<_> = fluid
        .continents
        .iter()
        .map(|c| ContinentResponse {
            name: c.name.clone(),
            depth_range: c.depth_range,
            total_integration: c.total_integration,
            impermeability: c.impermeability,
            formation_event: c.formation_event,
            formed_from_ores: c.formed_from_ores.clone(),
        })
        .collect();

    Json(continents)
}
