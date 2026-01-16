use std::sync::Arc;

use axum::{
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio::sync::oneshot;
use tracing::{debug, error, info};
use uuid::Uuid;

use crate::state::{AppState, Command};

/// GET /ws - WebSocket endpoint (Willful Acts - bidirectional)
///
/// This is the channel for deliberate interventions: Benthic expeditions,
/// deep breaths, and sudden injection of fresh concepts.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();

    // Subscribe to event broadcast
    let mut event_rx = state.event_tx.subscribe();

    info!("WebSocket client connected");

    // Send initial state snapshot
    {
        let fluid = state.fluid.read().await;
        let snapshot = serde_json::json!({
            "type": "initial_state",
            "concepts_count": fluid.concepts.len(),
            "core_truths_count": fluid.core_truths.len(),
            "ore_deposits_count": fluid.ore_deposits.len(),
            "continents_count": fluid.continents.len(),
            "is_frozen": fluid.is_frozen,
            "is_turbulent": fluid.is_turbulent,
            "salinity": fluid.salinity,
            "ocean_floor_pressure": fluid.ocean_floor_pressure,
        });

        if let Ok(json) = serde_json::to_string(&snapshot) {
            let _ = sender.send(Message::Text(json.into())).await;
        }
    }

    // Spawn task to forward events to client
    let mut send_task = tokio::spawn(async move {
        while let Ok(event) = event_rx.recv().await {
            if let Ok(json) = serde_json::to_string(&event) {
                if sender.send(Message::Text(json.into())).await.is_err() {
                    break; // Client disconnected
                }
            }
        }
    });

    // Handle incoming messages from client
    let command_tx = state.command_tx.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                debug!("Received WebSocket command: {}", text);
                if let Some(cmd) = parse_ws_command(&text) {
                    if let Err(e) = command_tx.send(cmd).await {
                        error!("Failed to send command: {}", e);
                    }
                }
            }
        }
    });

    // Wait for either task to complete
    tokio::select! {
        _ = &mut send_task => {
            recv_task.abort();
            info!("WebSocket sender task ended");
        }
        _ = &mut recv_task => {
            send_task.abort();
            info!("WebSocket receiver task ended");
        }
    }

    info!("WebSocket client disconnected");
}

/// Commands that can be sent via WebSocket
#[derive(Deserialize)]
#[serde(tag = "command", rename_all = "snake_case")]
enum WsCommand {
    Inject {
        name: String,
        density: f32,
        #[serde(default = "default_volume")]
        volume: f32,
    },
    Ballast {
        id: Uuid,
        weight_delta: f32,
    },
    Thaw,
    DeepBreath {
        strength: f32,
    },
    ModulateBuoyancy {
        id: Uuid,
        delta: f32,
    },
    AddCoreTruth {
        name: String,
        heat_output: f32,
        depth: f32,
        radius: f32,
    },
    FlashHeal {
        concepts: Vec<FreshConcept>,
        dilution_strength: f32,
    },
}

#[derive(Deserialize)]
struct FreshConcept {
    name: String,
    density: f32,
    area: f32,
}

fn default_volume() -> f32 {
    0.5
}

fn parse_ws_command(text: &str) -> Option<Command> {
    let ws_cmd: WsCommand = serde_json::from_str(text).ok()?;

    Some(match ws_cmd {
        WsCommand::Inject {
            name,
            density,
            volume,
        } => {
            let area = if density > 0.01 {
                (volume / density).clamp(0.1, 2.0)
            } else {
                volume * 2.0
            };
            let (tx, _) = oneshot::channel();
            Command::Inject {
                name,
                density,
                area,
                response_tx: tx,
            }
        }
        WsCommand::Ballast { id, weight_delta } => Command::Ballast {
            concept_id: id,
            weight_delta,
        },
        WsCommand::Thaw => Command::Thaw,
        WsCommand::DeepBreath { strength } => Command::DeepBreath { strength },
        WsCommand::ModulateBuoyancy { id, delta } => Command::ModulateBuoyancy {
            concept_id: id,
            delta,
        },
        WsCommand::AddCoreTruth {
            name,
            heat_output,
            depth,
            radius,
        } => Command::AddCoreTruth {
            name,
            heat_output,
            depth,
            radius,
        },
        WsCommand::FlashHeal {
            concepts,
            dilution_strength,
        } => Command::FlashHeal {
            concepts: concepts
                .into_iter()
                .map(|c| (c.name, c.density, c.area))
                .collect(),
            dilution_strength,
        },
    })
}
