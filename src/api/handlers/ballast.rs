use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::state::{AppState, Command};

#[derive(Deserialize)]
pub struct BallastRequest {
    pub id: Uuid,
    pub weight_delta: f32,
}

#[derive(Serialize)]
pub struct BallastResponse {
    pub id: Uuid,
    pub weight_delta: f32,
    pub status: String,
}

/// PATCH /ballast - Apply ballast to force benthic expedition
pub async fn apply_ballast(
    State(state): State<Arc<AppState>>,
    Json(req): Json<BallastRequest>,
) -> Result<Json<BallastResponse>, (StatusCode, String)> {
    // Validate
    if req.weight_delta < -1.0 || req.weight_delta > 1.0 {
        return Err((
            StatusCode::BAD_REQUEST,
            "weight_delta must be between -1.0 and 1.0".into(),
        ));
    }

    // Send command
    state
        .command_tx
        .send(Command::Ballast {
            concept_id: req.id,
            weight_delta: req.weight_delta,
        })
        .await
        .map_err(|_| {
            (
                StatusCode::SERVICE_UNAVAILABLE,
                "Simulation not running".into(),
            )
        })?;

    let status = if req.weight_delta > 0.0 {
        "Benthic expedition initiated - concept descending"
    } else {
        "Ballast released - concept ascending"
    };

    Ok(Json(BallastResponse {
        id: req.id,
        weight_delta: req.weight_delta,
        status: status.into(),
    }))
}
