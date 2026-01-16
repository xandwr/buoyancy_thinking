use std::sync::Arc;

use tokio::net::TcpListener;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

mod api;
mod runtime;
mod simulation;
mod state;

use api::create_router;
use runtime::run_simulation_loop;
use simulation::ConceptFluid;
use state::AppState;

#[tokio::main]
async fn main() {
    // Initialize tracing
    FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .compact()
        .init();

    info!("Consciousness API starting...");

    // Create initial fluid with default parameters
    let mut fluid = ConceptFluid::default();

    // Add the Primal Axiom - a mind without a core truth is a vacuum
    // "curiosity_exceeds_despair" ensures the first heavy thought encounters heat
    fluid.add_core_truth(
        "curiosity_exceeds_despair".to_string(),
        1.0, // Strong initial heat output
        0.9, // Deep in the fluid (near bottom)
        0.3, // Wide radius to catch sinking thoughts
    );

    info!("Primal Axiom established: 'curiosity_exceeds_despair' vent active at depth 0.9");

    // Create shared state with channels
    let (state, channels) = AppState::new(fluid);
    let state = Arc::new(state);

    // Spawn simulation loop (60Hz)
    let fluid_clone = state.fluid.clone();
    tokio::spawn(async move {
        run_simulation_loop(fluid_clone, channels).await;
    });

    // Create router
    let app = create_router(state);

    // Start server
    let addr = "0.0.0.0:3000";
    info!("Server listening on {}", addr);
    info!("Endpoints:");
    info!("  POST   /inject          - Inject a new thought");
    info!("  PATCH  /ballast         - Force benthic expedition");
    info!("  GET    /vent/:id        - Get vent details");
    info!("  POST   /vent            - Create new core truth");
    info!("  GET    /vents           - List all vents");
    info!("  GET    /strata          - View concepts/ores at depth");
    info!("  POST   /continent       - Trigger tectonic shift");
    info!("  GET    /continents      - List all continents");
    info!("  POST   /thaw            - Break freeze state");
    info!("  POST   /breath          - Apply deep breath damping");
    info!("  POST   /flash-heal      - Dilute salinity with fresh concepts");
    info!("  GET    /state           - Full state snapshot");
    info!("  GET    /events          - SSE stream (Passive Stream)");
    info!("  GET    /ws              - WebSocket (Willful Acts)");

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
