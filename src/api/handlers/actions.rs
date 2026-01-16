use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};

use crate::state::{AppState, Command};

// === Thaw ===

#[derive(Serialize)]
pub struct ThawResponse {
    pub status: String,
    pub was_frozen: bool,
}

/// POST /thaw - Break the freeze state
pub async fn thaw(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ThawResponse>, (StatusCode, String)> {
    let was_frozen = {
        let fluid = state.fluid.read().await;
        fluid.is_frozen
    };

    state.command_tx.send(Command::Thaw).await.map_err(|_| {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            "Simulation not running".into(),
        )
    })?;

    let status = if was_frozen {
        "Freeze broken - fluid thawing"
    } else {
        "Fluid was not frozen"
    };

    Ok(Json(ThawResponse {
        status: status.into(),
        was_frozen,
    }))
}

// === Deep Breath ===

#[derive(Deserialize)]
pub struct DeepBreathRequest {
    pub strength: f32,
}

#[derive(Serialize)]
pub struct DeepBreathResponse {
    pub status: String,
    pub strength: f32,
}

/// POST /breath - Apply damping to restore calm
pub async fn deep_breath(
    State(state): State<Arc<AppState>>,
    Json(req): Json<DeepBreathRequest>,
) -> Result<Json<DeepBreathResponse>, (StatusCode, String)> {
    if req.strength < 0.0 || req.strength > 1.0 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Strength must be between 0.0 and 1.0".into(),
        ));
    }

    state
        .command_tx
        .send(Command::DeepBreath {
            strength: req.strength,
        })
        .await
        .map_err(|_| {
            (
                StatusCode::SERVICE_UNAVAILABLE,
                "Simulation not running".into(),
            )
        })?;

    Ok(Json(DeepBreathResponse {
        status: "Deep breath applied - damping turbulence".into(),
        strength: req.strength,
    }))
}

// === Flash Heal ===

#[derive(Deserialize)]
pub struct FlashHealRequest {
    pub concepts: Vec<FreshConcept>,
    pub dilution_strength: f32,
}

#[derive(Deserialize)]
pub struct FreshConcept {
    pub name: String,
    pub density: f32,
    pub area: f32,
}

#[derive(Serialize)]
pub struct FlashHealResponse {
    pub status: String,
    pub concepts_added: usize,
    pub dilution_strength: f32,
}

/// POST /flash-heal - Break crystalline salinity with fresh input
pub async fn flash_heal(
    State(state): State<Arc<AppState>>,
    Json(req): Json<FlashHealRequest>,
) -> Result<Json<FlashHealResponse>, (StatusCode, String)> {
    if req.dilution_strength < 0.0 || req.dilution_strength > 1.0 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Dilution strength must be between 0.0 and 1.0".into(),
        ));
    }

    let count = req.concepts.len();
    let concepts: Vec<_> = req
        .concepts
        .into_iter()
        .map(|c| (c.name, c.density, c.area))
        .collect();

    state
        .command_tx
        .send(Command::FlashHeal {
            concepts,
            dilution_strength: req.dilution_strength,
        })
        .await
        .map_err(|_| {
            (
                StatusCode::SERVICE_UNAVAILABLE,
                "Simulation not running".into(),
            )
        })?;

    Ok(Json(FlashHealResponse {
        status: "Flash heal applied - crystalline structure diluted".into(),
        concepts_added: count,
        dilution_strength: req.dilution_strength,
    }))
}
