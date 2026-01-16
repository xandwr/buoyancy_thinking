use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

use crate::state::{AppState, Command};

#[derive(Serialize)]
pub struct VentResponse {
    pub name: String,
    pub heat_output: f32,
    pub depth: f32,
    pub radius: f32,
    pub activation_count: u32,
}

/// GET /vent/:id - Get details of a specific vent
pub async fn get_vent(
    State(state): State<Arc<AppState>>,
    Path(id): Path<usize>,
) -> Result<Json<VentResponse>, (StatusCode, String)> {
    let fluid = state.fluid.read().await;

    let vent = fluid
        .core_truths
        .get(id)
        .ok_or((StatusCode::NOT_FOUND, format!("Vent {} not found", id)))?;

    Ok(Json(VentResponse {
        name: vent.name.clone(),
        heat_output: vent.heat_output,
        depth: vent.depth,
        radius: vent.radius,
        activation_count: vent.activation_count,
    }))
}

/// GET /vents - List all core truths
pub async fn list_vents(State(state): State<Arc<AppState>>) -> Json<Vec<VentResponse>> {
    let fluid = state.fluid.read().await;

    let vents: Vec<_> = fluid
        .core_truths
        .iter()
        .map(|v| VentResponse {
            name: v.name.clone(),
            heat_output: v.heat_output,
            depth: v.depth,
            radius: v.radius,
            activation_count: v.activation_count,
        })
        .collect();

    Json(vents)
}

#[derive(Deserialize)]
pub struct CreateVentRequest {
    pub name: String,
    pub heat_output: f32,
    pub depth: f32,
    pub radius: f32,
}

/// POST /vent - Create a new core truth (vent)
pub async fn create_vent(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateVentRequest>,
) -> Result<Json<VentResponse>, (StatusCode, String)> {
    // Validate
    if req.depth < 0.0 || req.depth > 1.0 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Depth must be between 0.0 and 1.0".into(),
        ));
    }
    if req.radius <= 0.0 || req.radius > 1.0 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Radius must be between 0.0 and 1.0".into(),
        ));
    }
    if req.heat_output < 0.0 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Heat output must be non-negative".into(),
        ));
    }

    // Send command
    state
        .command_tx
        .send(Command::AddCoreTruth {
            name: req.name.clone(),
            heat_output: req.heat_output,
            depth: req.depth,
            radius: req.radius,
        })
        .await
        .map_err(|_| {
            (
                StatusCode::SERVICE_UNAVAILABLE,
                "Simulation not running".into(),
            )
        })?;

    Ok(Json(VentResponse {
        name: req.name,
        heat_output: req.heat_output,
        depth: req.depth,
        radius: req.radius,
        activation_count: 0,
    }))
}
