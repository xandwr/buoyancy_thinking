use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::state::{AppState, Command};

#[derive(Deserialize)]
pub struct DivisionRequest {
    /// The dividend (V) - number of bubbles to inject
    pub dividend: f32,
    /// The divisor (n) - acoustic frequency creating nodes
    pub divisor: f32,
    /// Salinity boost for Laminar Streamlining (optional, default 0.0)
    /// Higher values dampen "volume overhead" noise, making remainder turbulence clearer
    #[serde(default)]
    pub salinity: f32,
}

#[derive(Serialize)]
pub struct DivisionStartResponse {
    pub experiment_id: Uuid,
    pub dividend: f32,
    pub divisor: f32,
    pub salinity_boost: f32,
    pub expected_quotient: f32,
    pub expected_remainder: f32,
    pub message: String,
}

#[derive(Serialize)]
pub struct ExperimentStatusResponse {
    pub active: bool,
    pub dividend: Option<f32>,
    pub divisor: Option<f32>,
    pub bubble_count: Option<usize>,
    pub node_count: Option<usize>,
    pub accumulated_turbulence: Option<f32>,
    pub ticks_elapsed: Option<u64>,
}

#[derive(Serialize)]
pub struct DivisionResultResponse {
    pub dividend: f32,
    pub divisor: f32,
    pub quotient: f32,
    pub remainder: f32,
    pub is_divisible: bool,
    pub turbulence_energy: f32,
    pub reynolds_number: f32,
    pub ticks_to_settle: u64,
    pub node_occupancy: Vec<u32>,
    pub salinity_boost: f32,
    /// Velocity standard deviation - "arrival jitter" detector
    /// High vσ = micro-cavitation (remainder), Low vσ = laminar (divisible)
    pub velocity_sigma: f32,
    pub velocity_mean: f32,
    /// Peak jitter during settling - THE key remainder detection metric
    /// Captures transient micro-cavitation before damping smooths it out
    pub peak_jitter: f32,
    pub interpretation: String,
}

/// POST /divide - Start a division experiment
///
/// Encodes division as fluid dynamics:
/// - Divisor n → acoustic frequency creating standing wave nodes
/// - Dividend V → stream of bubbles injected
/// - If V/n is integer → laminar flow (bubbles fill nodes perfectly)
/// - If V/n has remainder → turbulence (extra bubbles jostle for position)
pub async fn start_division(
    State(state): State<Arc<AppState>>,
    Json(req): Json<DivisionRequest>,
) -> Result<Json<DivisionStartResponse>, (StatusCode, String)> {
    // Validate inputs
    if req.dividend <= 0.0 {
        return Err((StatusCode::BAD_REQUEST, "Dividend must be positive".into()));
    }
    if req.divisor <= 0.0 {
        return Err((StatusCode::BAD_REQUEST, "Divisor must be positive".into()));
    }
    if req.dividend > 100.0 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Dividend must be <= 100 (too many bubbles cause chaos)".into(),
        ));
    }
    if req.divisor > 20.0 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Divisor must be <= 20 (too many nodes)".into(),
        ));
    }
    if req.salinity < 0.0 || req.salinity > 10.0 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Salinity must be between 0.0 and 10.0".into(),
        ));
    }

    // Create response channel
    let (tx, rx) = oneshot::channel();

    // Send command
    state
        .command_tx
        .send(Command::StartDivisionExperiment {
            dividend: req.dividend,
            divisor: req.divisor,
            salinity_boost: req.salinity,
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
            "Failed to start experiment".into(),
        )
    })?;

    let expected_quotient = (req.dividend / req.divisor).floor();
    let expected_remainder = req.dividend % req.divisor;

    let salinity_note = if req.salinity > 0.0 {
        format!(
            " Laminar Streamlining active (salinity +{:.1}).",
            req.salinity
        )
    } else {
        String::new()
    };

    let message = if expected_remainder < 0.001 {
        format!(
            "Injecting {} bubbles into {} acoustic nodes. Expecting perfect fit (laminar flow).{}",
            req.dividend as u32, req.divisor as u32, salinity_note
        )
    } else {
        format!(
            "Injecting {} bubbles into {} acoustic nodes. {} bubbles won't fit → expect turbulence!{}",
            req.dividend as u32, req.divisor as u32, expected_remainder as u32, salinity_note
        )
    };

    Ok(Json(DivisionStartResponse {
        experiment_id,
        dividend: req.dividend,
        divisor: req.divisor,
        salinity_boost: req.salinity,
        expected_quotient,
        expected_remainder,
        message,
    }))
}

/// GET /divide/status - Get current experiment status
pub async fn get_division_status(
    State(state): State<Arc<AppState>>,
) -> Json<ExperimentStatusResponse> {
    let fluid = state.fluid.read().await;

    if let Some(exp) = fluid.get_experiment_status() {
        let ticks_elapsed = fluid.tick_count.saturating_sub(exp.start_tick);

        Json(ExperimentStatusResponse {
            active: true,
            dividend: Some(exp.problem.dividend),
            divisor: Some(exp.problem.divisor),
            bubble_count: Some(exp.bubble_ids.len()),
            node_count: Some(exp.wave.node_count()),
            accumulated_turbulence: Some(exp.accumulated_turbulence),
            ticks_elapsed: Some(ticks_elapsed),
        })
    } else {
        Json(ExperimentStatusResponse {
            active: false,
            dividend: None,
            divisor: None,
            bubble_count: None,
            node_count: None,
            accumulated_turbulence: None,
            ticks_elapsed: None,
        })
    }
}

/// GET /divide/results - Get all completed experiment results
pub async fn get_division_results(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<DivisionResultResponse>> {
    let fluid = state.fluid.read().await;

    let results: Vec<DivisionResultResponse> = fluid
        .experiment_results
        .iter()
        .map(|r| {
            let interpretation = if r.is_divisible {
                format!(
                    "{} ÷ {} = {} (clean division, laminar flow achieved)",
                    r.dividend, r.divisor, r.quotient
                )
            } else {
                format!(
                    "{} ÷ {} = {} remainder {} (turbulence detected: {:.2} energy units)",
                    r.dividend, r.divisor, r.quotient, r.remainder, r.turbulence_energy
                )
            };

            DivisionResultResponse {
                dividend: r.dividend,
                divisor: r.divisor,
                quotient: r.quotient,
                remainder: r.remainder,
                is_divisible: r.is_divisible,
                turbulence_energy: r.turbulence_energy,
                reynolds_number: r.reynolds_number,
                ticks_to_settle: r.ticks_to_settle,
                node_occupancy: r.node_occupancy.clone(),
                salinity_boost: r.salinity_boost,
                velocity_sigma: r.velocity_sigma,
                velocity_mean: r.velocity_mean,
                peak_jitter: r.peak_jitter,
                interpretation,
            }
        })
        .collect();

    Json(results)
}
