use std::sync::Arc;
use std::time::Duration;

use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::state::{AppState, Command};

#[derive(Deserialize)]
pub struct InjectRequest {
    pub concept: String,
    pub density: f32,
    #[serde(default = "default_volume")]
    pub volume: f32,
}

fn default_volume() -> f32 {
    0.5
}

#[derive(Serialize)]
pub struct InjectResponse {
    pub id: Uuid,
    pub name: String,
    pub density: f32,
    pub area: f32,
    pub initial_layer: f32,
}

/// POST /inject - Inject a new thought into the fluid
pub async fn inject_concept(
    State(state): State<Arc<AppState>>,
    Json(req): Json<InjectRequest>,
) -> Result<Json<InjectResponse>, (StatusCode, String)> {
    // Validate inputs
    if req.density < 0.0 || req.density > 1.0 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Density must be between 0.0 and 1.0".into(),
        ));
    }
    if req.volume < 0.0 || req.volume > 2.0 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Volume must be between 0.0 and 2.0".into(),
        ));
    }

    // Derive area from volume
    let area = if req.density > 0.01 {
        (req.volume / req.density).clamp(0.1, 2.0)
    } else {
        req.volume * 2.0
    };

    // Create response channel
    let (response_tx, response_rx) = oneshot::channel();

    // Send command to simulation
    state
        .command_tx
        .send(Command::Inject {
            name: req.concept.clone(),
            density: req.density,
            area,
            response_tx,
        })
        .await
        .map_err(|_| {
            (
                StatusCode::SERVICE_UNAVAILABLE,
                "Simulation not running".into(),
            )
        })?;

    // Wait for response with timeout
    let id = tokio::time::timeout(Duration::from_secs(5), response_rx)
        .await
        .map_err(|_| {
            (
                StatusCode::GATEWAY_TIMEOUT,
                "Simulation response timeout".into(),
            )
        })?
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create concept".into(),
            )
        })?;

    Ok(Json(InjectResponse {
        id,
        name: req.concept,
        density: req.density,
        area,
        initial_layer: req.density,
    }))
}
