use std::sync::Arc;
use std::time::Duration;

use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use crate::simulation::ConceptFluid;
use crate::state::{Command, FluidEvent, SimulationChannels};

/// Tick rate for the simulation (60Hz)
const TICK_RATE_HZ: u64 = 60;
/// Delta time per tick
const DT: f32 = 1.0 / TICK_RATE_HZ as f32;

/// Run the simulation loop at 60Hz.
/// Processes commands from the API and broadcasts significant events.
pub async fn run_simulation_loop(
    fluid: Arc<RwLock<ConceptFluid>>,
    mut channels: SimulationChannels,
) {
    let mut interval = tokio::time::interval(Duration::from_micros(1_000_000 / TICK_RATE_HZ));
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

    info!("Simulation loop started at {}Hz", TICK_RATE_HZ);

    loop {
        interval.tick().await;

        // Acquire write lock for this tick
        let mut fluid_guard = fluid.write().await;

        // Process all pending commands
        while let Ok(cmd) = channels.command_rx.try_recv() {
            process_command(&mut fluid_guard, cmd, &channels.event_tx);
        }

        // Run physics update
        let events = fluid_guard.update(DT);

        // Release lock before broadcasting
        drop(fluid_guard);

        // Broadcast significant events (ignore errors if no subscribers)
        for event in events {
            debug!("Broadcasting event: {:?}", event);
            let _ = channels.event_tx.send(event);
        }
    }
}

/// Process a command from the API.
fn process_command(
    fluid: &mut ConceptFluid,
    cmd: Command,
    event_tx: &tokio::sync::broadcast::Sender<FluidEvent>,
) {
    match cmd {
        Command::Inject {
            name,
            density,
            area,
            response_tx,
        } => {
            let id = fluid.add_concept(name.clone(), density, area);
            info!("Injected concept '{}' with id {}", name, id);

            // Send event
            let _ = event_tx.send(FluidEvent::ConceptInjected {
                id,
                name,
                density,
                layer: density, // Initial layer = density
            });

            // Send response
            let _ = response_tx.send(id);
        }

        Command::Ballast {
            concept_id,
            weight_delta,
        } => {
            if let Some(concept) = fluid.get_concept(concept_id) {
                let name = concept.name.clone();
                if fluid.benthic_expedition(concept_id, weight_delta) {
                    info!(
                        "Benthic expedition: '{}' ballasted with {}",
                        name, weight_delta
                    );
                    let _ = event_tx.send(FluidEvent::BenthicExpedition {
                        concept_id,
                        concept_name: name,
                        ballast_amount: weight_delta,
                    });
                }
            } else {
                warn!("Ballast command for unknown concept: {}", concept_id);
            }
        }

        Command::ModulateBuoyancy { concept_id, delta } => {
            fluid.modulate_buoyancy(concept_id, delta);
            debug!("Modulated buoyancy for {} by {}", concept_id, delta);
        }

        Command::TriggerTectonic { pressure_threshold } => {
            fluid.set_pressure_threshold(pressure_threshold);
            info!("Tectonic pressure threshold set to {}", pressure_threshold);
        }

        Command::Thaw => {
            if fluid.thaw() {
                info!("Fluid thawed");
                let _ = event_tx.send(FluidEvent::Thaw);
            }
        }

        Command::DeepBreath { strength } => {
            fluid.deep_breath(strength);
            info!("Deep breath applied with strength {}", strength);
            let _ = event_tx.send(FluidEvent::DeepBreath { strength });
        }

        Command::AddCoreTruth {
            name,
            heat_output,
            depth,
            radius,
        } => {
            fluid.add_core_truth(name.clone(), heat_output, depth, radius);
            info!("Added core truth '{}' at depth {}", name, depth);
            let _ = event_tx.send(FluidEvent::CoreTruthFormed {
                name,
                depth,
                heat_output,
                radius,
            });
        }

        Command::FlashHeal {
            concepts,
            dilution_strength,
        } => {
            let count = concepts.len();
            let old_salinity = fluid.flash_heal(concepts, dilution_strength);
            info!(
                "Flash heal: {} concepts, salinity {} -> {}",
                count,
                old_salinity,
                old_salinity * (1.0 - dilution_strength)
            );
            let _ = event_tx.send(FluidEvent::FlashHeal {
                concepts_added: count,
                old_salinity,
                new_salinity: old_salinity * (1.0 - dilution_strength),
            });
        }

        Command::Precipitate {
            trait_index,
            new_concept_name,
            density,
            area,
        } => {
            if let Some((_, inherited)) =
                fluid.precipitate(trait_index, new_concept_name.clone(), density, area)
            {
                let trait_name = fluid
                    .atmosphere
                    .get(trait_index)
                    .map(|t| t.name.clone())
                    .unwrap_or_default();

                info!(
                    "Precipitation: '{}' from trait '{}'",
                    new_concept_name, trait_name
                );
                let _ = event_tx.send(FluidEvent::Precipitation {
                    trait_name,
                    new_concept: new_concept_name,
                    inherited_integration: inherited,
                });
            }
        }
    }
}
