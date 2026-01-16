use std::sync::Arc;

use tokio::sync::{RwLock, broadcast, mpsc};

use super::commands::Command;
use super::events::FluidEvent;
use crate::simulation::ConceptFluid;

/// Shared application state containing the fluid simulation and communication channels.
pub struct AppState {
    /// The simulation state (protected by RwLock for concurrent access)
    pub fluid: Arc<RwLock<ConceptFluid>>,

    /// Channel for sending commands to the simulation loop
    pub command_tx: mpsc::Sender<Command>,

    /// Channel for subscribing to real-time events
    pub event_tx: broadcast::Sender<FluidEvent>,
}

/// Channels passed to the simulation loop task.
pub struct SimulationChannels {
    pub command_rx: mpsc::Receiver<Command>,
    pub event_tx: broadcast::Sender<FluidEvent>,
}

impl AppState {
    /// Create a new AppState with the given fluid.
    /// Returns the state and the channels needed by the simulation loop.
    pub fn new(fluid: ConceptFluid) -> (Self, SimulationChannels) {
        let (command_tx, command_rx) = mpsc::channel(64);
        let (event_tx, _) = broadcast::channel(256);

        let state = Self {
            fluid: Arc::new(RwLock::new(fluid)),
            command_tx,
            event_tx: event_tx.clone(),
        };

        let channels = SimulationChannels {
            command_rx,
            event_tx,
        };

        (state, channels)
    }
}
