use std::sync::Arc;

use axum::{
    Router,
    routing::{get, patch, post},
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use super::handlers;
use crate::state::AppState;

/// Create the API router with all endpoints.
pub fn create_router(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        // === Concept operations ===
        .route("/inject", post(handlers::inject_concept))
        .route("/ballast", patch(handlers::apply_ballast))
        // === Core truths (vents) ===
        .route("/vent", post(handlers::create_vent))
        .route("/vent/{id}", get(handlers::get_vent))
        .route("/vents", get(handlers::list_vents))
        // === Strata (depth queries) ===
        .route("/strata", get(handlers::get_strata))
        // === Continents (tectonic) ===
        .route("/continent", post(handlers::trigger_tectonic))
        .route("/continents", get(handlers::list_continents))
        // === Actions ===
        .route("/thaw", post(handlers::thaw))
        .route("/breath", post(handlers::deep_breath))
        .route("/flash-heal", post(handlers::flash_heal))
        // === Division Experiments (Analog Computing) ===
        .route("/divide", post(handlers::start_division))
        .route("/divide/status", get(handlers::get_division_status))
        .route("/divide/results", get(handlers::get_division_results))
        // === State queries ===
        .route("/state", get(handlers::get_full_state))
        // === Real-time streams ===
        .route("/events", get(handlers::event_stream)) // SSE (Passive Stream)
        .route("/ws", get(handlers::ws_handler)) // WebSocket (Willful Acts)
        // === Middleware ===
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
