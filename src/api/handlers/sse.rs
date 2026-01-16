use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;

use axum::{
    extract::State,
    response::sse::{Event, KeepAlive, Sse},
};
use futures::stream::Stream;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::BroadcastStream;

use crate::state::{AppState, FluidEvent};

/// GET /events - Server-Sent Events stream (Passive Stream of the subconscious)
///
/// This is the appropriate channel for background currents and slow-moving state changes.
/// Receives all significant events from the simulation.
pub async fn event_stream(
    State(state): State<Arc<AppState>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state.event_tx.subscribe();

    let stream = BroadcastStream::new(rx).filter_map(|result: Result<FluidEvent, _>| {
        result.ok().map(|event: FluidEvent| {
            let event_type = match &event {
                FluidEvent::SurfaceBreakthrough { .. } => "breakthrough",
                FluidEvent::SurfaceBounce { .. } => "bounce",
                FluidEvent::ConceptInjected { .. } => "injected",
                FluidEvent::ConceptEvaporated { .. } => "evaporated",
                FluidEvent::Freeze { .. } => "freeze",
                FluidEvent::Thaw => "thaw",
                FluidEvent::TurbulenceOnset { .. } => "turbulence_onset",
                FluidEvent::TurbulenceSubsided => "turbulence_subsided",
                FluidEvent::Mineralization { .. } => "mineralization",
                FluidEvent::OreDeposited { .. } => "ore_deposited",
                FluidEvent::OreCatalysis { .. } => "catalysis",
                FluidEvent::TectonicShift { .. } => "tectonic_shift",
                FluidEvent::CoreTruthFormed { .. } => "core_truth_formed",
                FluidEvent::CoreTruthStrengthened { .. } => "core_truth_strengthened",
                FluidEvent::Precipitation { .. } => "precipitation",
                FluidEvent::FlashHeal { .. } => "flash_heal",
                FluidEvent::DeepBreath { .. } => "deep_breath",
                FluidEvent::BenthicExpedition { .. } => "benthic_expedition",
            };

            let json = serde_json::to_string(&event).unwrap_or_default();
            Ok(Event::default().event(event_type).data(json))
        })
    });

    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("ping"),
    )
}
