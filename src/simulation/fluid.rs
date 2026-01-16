use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    concept::{Concept, ConceptId},
    continent::Continent,
    core_truth::CoreTruth,
    ore::{OreType, PreciousOre},
    traits::CharacterTrait,
};
use crate::state::events::FluidEvent;

/// The main container for the consciousness fluid simulation.
/// Contains all concepts, traits, vents, ores, and continents,
/// along with physics parameters for the simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptFluid {
    // === Entities ===
    pub concepts: HashMap<ConceptId, Concept>,
    /// Evaporated concepts → permanent traits (the "atmosphere")
    pub atmosphere: Vec<CharacterTrait>,
    /// Deep sea vents - radiating foundational beliefs
    pub core_truths: Vec<CoreTruth>,
    /// Mineralized transformations on ocean floor
    pub ore_deposits: Vec<PreciousOre>,
    /// Permanent landmasses - solid ground in the fluid
    pub continents: Vec<Continent>,

    // === Tracking ===
    /// Track cycles through vents for mineralization
    pub vent_encounter_count: HashMap<ConceptId, u32>,
    /// Total weight of ore deposits creating tectonic pressure
    pub ocean_floor_pressure: f32,
    /// Critical pressure for tectonic shift
    pub pressure_threshold: f32,
    /// How many times bedrock has shifted
    pub tectonic_shifts: u32,

    // === Physics parameters ===
    /// Fluid density (ρ in drag equation)
    pub viscosity: f32,
    /// Resistance from ego/executive control (Cd)
    pub drag_coefficient: f32,
    /// Threshold force for breaking into action
    pub surface_tension: f32,
    /// Layer depth where surface tension applies
    pub activation_zone: f32,

    // === Freeze mechanics ===
    /// Time at surface before freeze occurs (seconds)
    pub freeze_threshold: f32,
    /// Layer depth considered "at surface" for freezing
    pub freeze_zone: f32,
    /// Is the entire fluid frozen?
    pub is_frozen: bool,
    /// Which concept caused the freeze
    pub frozen_concept: Option<ConceptId>,

    // === Turbulence mechanics ===
    /// Re threshold for turbulence onset
    pub reynolds_threshold: f32,
    /// Is the fluid in turbulent state?
    pub is_turbulent: bool,
    /// Current turbulence energy level
    pub turbulence_energy: f32,
    /// Rate at which turbulence decays
    pub turbulence_decay: f32,
    /// "Deep breath" - active damping strength
    pub damping_factor: f32,

    // === Integration & Evaporation ===
    /// System-wide accumulated internal heat
    pub total_integration: f32,
    /// Integration level needed to evaporate
    pub evaporation_threshold: f32,
    /// Layer depth for evaporation (near surface)
    pub evaporation_zone: f32,

    // === Salinity ===
    /// Accumulated knowledge density
    pub salinity: f32,
    /// How fast integration increases salinity
    pub salinity_rate: f32,

    // === Visualization ===
    /// Number of layers for bucketing
    pub num_layers: usize,
}

impl ConceptFluid {
    pub fn new(
        viscosity: f32,
        drag_coefficient: f32,
        surface_tension: f32,
        activation_zone: f32,
        freeze_threshold: f32,
        freeze_zone: f32,
        reynolds_threshold: f32,
        turbulence_decay: f32,
        num_layers: usize,
        evaporation_threshold: f32,
        evaporation_zone: f32,
    ) -> Self {
        Self {
            concepts: HashMap::new(),
            atmosphere: Vec::new(),
            core_truths: Vec::new(),
            ore_deposits: Vec::new(),
            continents: Vec::new(),
            vent_encounter_count: HashMap::new(),
            ocean_floor_pressure: 0.0,
            pressure_threshold: 15.0,
            tectonic_shifts: 0,
            viscosity,
            drag_coefficient,
            surface_tension,
            activation_zone,
            freeze_threshold,
            freeze_zone,
            is_frozen: false,
            frozen_concept: None,
            reynolds_threshold,
            is_turbulent: false,
            turbulence_energy: 0.0,
            turbulence_decay,
            damping_factor: 0.0,
            total_integration: 0.0,
            evaporation_threshold,
            evaporation_zone,
            salinity: 0.0,
            salinity_rate: 0.1,
            num_layers,
        }
    }

    /// Create a fluid with default parameters.
    pub fn default() -> Self {
        Self::new(0.5, 1.2, 0.05, 0.1, 2.0, 0.05, 1.0, 0.3, 5, 1.0, 0.3)
    }

    /// Add a new concept to the fluid.
    pub fn add_concept(&mut self, name: String, density: f32, area: f32) -> ConceptId {
        let id = Uuid::new_v4();
        let concept = Concept::new(id, name, density, area);
        self.concepts.insert(id, concept);
        id
    }

    /// Add a core truth (deep sea vent) to the fluid.
    pub fn add_core_truth(&mut self, name: String, heat_output: f32, depth: f32, radius: f32) {
        let core_truth = CoreTruth::new(name, heat_output, depth, radius);
        self.core_truths.push(core_truth);
    }

    /// Get a concept by ID.
    pub fn get_concept(&self, id: ConceptId) -> Option<&Concept> {
        self.concepts.get(&id)
    }

    /// Get a mutable concept by ID.
    pub fn get_concept_mut(&mut self, id: ConceptId) -> Option<&mut Concept> {
        self.concepts.get_mut(&id)
    }

    /// Benthic expedition - deliberately sink a problem to find solutions in ore deposits.
    pub fn benthic_expedition(&mut self, concept_id: ConceptId, ballast_amount: f32) -> bool {
        if let Some(concept) = self.concepts.get_mut(&concept_id) {
            concept.ballast = ballast_amount;
            true
        } else {
            false
        }
    }

    /// Modulate buoyancy externally.
    pub fn modulate_buoyancy(&mut self, id: ConceptId, delta: f32) {
        if let Some(concept) = self.concepts.get_mut(&id) {
            let effective_delta = delta * (1.0 - concept.density);
            concept.buoyancy = (concept.buoyancy + effective_delta).clamp(0.0, 1.0);
            concept.velocity += effective_delta * 2.0;
        }
    }

    /// Thaw the frozen fluid (external intervention).
    pub fn thaw(&mut self) -> bool {
        if self.is_frozen {
            self.is_frozen = false;
            if let Some(frozen_id) = self.frozen_concept {
                if let Some(concept) = self.concepts.get_mut(&frozen_id) {
                    concept.is_frozen = false;
                    concept.time_at_surface = 0.0;
                    concept.velocity += 0.5;
                }
            }
            self.frozen_concept = None;
            true
        } else {
            false
        }
    }

    /// Deep breath - active damping to restore laminar flow.
    pub fn deep_breath(&mut self, strength: f32) {
        self.damping_factor = strength;
        if self.is_turbulent {
            self.turbulence_energy *= 1.0 - strength;
        }
    }

    /// Precipitation - character trait influences new thought formation.
    pub fn precipitate(
        &mut self,
        trait_index: usize,
        new_concept_name: String,
        density: f32,
        area: f32,
    ) -> Option<(ConceptId, f32)> {
        if trait_index >= self.atmosphere.len() {
            return None;
        }

        let inherited_integration = self.atmosphere[trait_index].integration * 0.3;

        let id = Uuid::new_v4();
        let mut concept = Concept::new(id, new_concept_name, density, area);
        concept.layer = 1.0;
        concept.velocity = 0.5;
        concept.integration = inherited_integration;

        self.concepts.insert(id, concept);
        Some((id, inherited_integration))
    }

    /// Flash-heal: Surge of fresh, naive input to dilute salinity.
    pub fn flash_heal(&mut self, concepts: Vec<(String, f32, f32)>, dilution_strength: f32) -> f32 {
        let old_salinity = self.salinity;
        self.salinity *= 1.0 - dilution_strength;

        if self.is_frozen {
            self.is_frozen = false;
            self.frozen_concept = None;
        }

        for (name, density, area) in concepts {
            let id = Uuid::new_v4();
            let mut concept = Concept::new(id, name, density, area);
            concept.layer = 0.7;
            self.concepts.insert(id, concept);
        }

        old_salinity
    }

    /// Set pressure threshold for tectonic shifts.
    pub fn set_pressure_threshold(&mut self, threshold: f32) {
        self.pressure_threshold = threshold;
    }

    /// Run one physics tick, returning all significant events that occurred.
    pub fn update(&mut self, dt: f32) -> Vec<FluidEvent> {
        let mut events = Vec::new();

        // === Pass 1: Track time at surface and detect freezing ===
        let mut freeze_triggered = false;
        let mut freezing_concept_id: Option<ConceptId> = None;
        let mut freezing_concept_name: Option<String> = None;

        for concept in self.concepts.values_mut() {
            if concept.layer < self.freeze_zone {
                concept.time_at_surface += dt;

                if concept.time_at_surface >= self.freeze_threshold && !concept.is_frozen {
                    concept.is_frozen = true;
                    freeze_triggered = true;
                    freezing_concept_id = Some(concept.id);
                    freezing_concept_name = Some(concept.name.clone());
                }
            } else {
                concept.time_at_surface = 0.0;
                concept.is_frozen = false;
            }
        }

        if freeze_triggered {
            self.is_frozen = true;
            self.frozen_concept = freezing_concept_id;
            if let (Some(id), Some(name)) = (freezing_concept_id, freezing_concept_name) {
                events.push(FluidEvent::Freeze {
                    concept_id: id,
                    concept_name: name,
                });
            }
        }

        // === Pass 2: Calculate Reynolds number and turbulence ===
        let avg_velocity: f32 = self
            .concepts
            .values()
            .map(|c| c.velocity.abs())
            .sum::<f32>()
            / self.concepts.len().max(1) as f32;

        let reynolds_number = avg_velocity / self.viscosity;

        if reynolds_number > self.reynolds_threshold && !self.is_turbulent {
            self.is_turbulent = true;
            self.turbulence_energy = reynolds_number / self.reynolds_threshold;
            events.push(FluidEvent::TurbulenceOnset {
                reynolds_number,
                energy: self.turbulence_energy,
            });
        }

        if self.is_turbulent {
            self.turbulence_energy *= 1.0 - self.turbulence_decay * dt;
            if self.turbulence_energy < 0.1 {
                self.is_turbulent = false;
                self.turbulence_energy = 0.0;
                events.push(FluidEvent::TurbulenceSubsided);
            }
        }

        // === Pass 3: Benthic ore reaction (problem-ore catalysis) ===
        let mut new_solutions: Vec<Concept> = Vec::new();
        let mut ballast_to_remove: Vec<ConceptId> = Vec::new();
        let mut catalysis_events: Vec<FluidEvent> = Vec::new();

        for concept in self.concepts.values() {
            if concept.ballast > 0.0 && concept.layer > 0.8 {
                for ore in &self.ore_deposits {
                    let depth_diff = (concept.layer - ore.depth).abs();

                    if depth_diff < 0.15 {
                        let mut reactivity = ore.integration_value * 0.3 + concept.area * 0.2;

                        let type_bonus = match ore.ore_type {
                            OreType::Art if concept.area > 0.6 => 0.4,
                            OreType::Code if concept.density < 0.5 => 0.4,
                            OreType::Insight if concept.integration > 0.5 => 0.5,
                            OreType::Writing if concept.area > 0.5 => 0.3,
                            _ => 0.1,
                        };
                        reactivity += type_bonus;

                        if reactivity > 0.6 {
                            let solution_id = Uuid::new_v4();
                            let solution_name =
                                format!("{}_{}_solution", concept.name, ore.ore_type.as_str());

                            let mut solution = Concept::new(
                                solution_id,
                                solution_name.clone(),
                                0.2,
                                concept.area + 0.2,
                            );
                            solution.layer = ore.depth;
                            solution.velocity = -0.5;
                            solution.integration = ore.integration_value;
                            solution.is_solution = true;

                            catalysis_events.push(FluidEvent::OreCatalysis {
                                problem: concept.name.clone(),
                                ore: ore.name.clone(),
                                solution: solution_name,
                                reactivity,
                            });

                            new_solutions.push(solution);
                            ballast_to_remove.push(concept.id);
                            break;
                        }
                    }
                }
            }
        }

        for solution in new_solutions {
            self.concepts.insert(solution.id, solution);
        }

        for concept_id in ballast_to_remove {
            if let Some(concept) = self.concepts.get_mut(&concept_id) {
                concept.ballast = 0.0;
            }
        }

        events.extend(catalysis_events);

        // === Pass 4: Physics simulation ===
        let mut ore_to_deposit: Vec<PreciousOre> = Vec::new();
        let mut mineralization_events: Vec<FluidEvent> = Vec::new();
        let mut breakthrough_events: Vec<FluidEvent> = Vec::new();

        // Collect core truth updates
        let mut core_truth_strengthened: Vec<(usize, f32)> = Vec::new();

        for concept in self.concepts.values_mut() {
            // When frozen, block all non-frozen concepts from rising
            if self.is_frozen && !concept.is_frozen {
                let freeze_suppression = 2.0;
                concept.velocity = concept.velocity.min(0.0);
                concept.velocity += freeze_suppression * dt;
                concept.layer = (concept.layer + concept.velocity * dt).clamp(0.0, 1.0);
                continue;
            }

            let effective_density = (concept.density + concept.ballast).min(1.0);
            let target_layer = (1.0 - concept.buoyancy + concept.ballast).clamp(0.0, 1.0);
            let diff = target_layer - concept.layer;

            let salinity_boost = if effective_density < 0.5 {
                self.salinity * (0.5 - effective_density) * 2.0
            } else {
                0.0
            };

            let buoyancy_force = diff * concept.density - salinity_boost;

            let drag_force = if concept.velocity.abs() > 0.001 {
                -0.5 * self.viscosity
                    * concept.velocity.powi(2)
                    * self.drag_coefficient
                    * concept.area
                    * concept.velocity.signum()
            } else {
                0.0
            };

            let surface_force = if concept.layer < self.activation_zone && concept.velocity < 0.0 {
                let depth_factor = 1.0 - (concept.layer / self.activation_zone);
                self.surface_tension * depth_factor
            } else {
                0.0
            };

            // Thermal plume force from core truths
            let mut thermal_force = 0.0;

            for (truth_idx, core_truth) in self.core_truths.iter().enumerate() {
                let depth_diff = (concept.layer - core_truth.depth).abs();

                if depth_diff < core_truth.radius {
                    let proximity = 1.0 - (depth_diff / core_truth.radius);
                    let heat_transfer = core_truth.heat_output * proximity.powi(2);
                    thermal_force -= heat_transfer;

                    if heat_transfer > 0.01 {
                        core_truth_strengthened.push((truth_idx, concept.density * 0.01));

                        // Mineralization for dark thoughts
                        if concept.density > 0.7 {
                            let encounters =
                                self.vent_encounter_count.entry(concept.id).or_insert(0);
                            *encounters += 1;

                            if *encounters % 3 == 0 && *encounters > 0 {
                                let ore_type = if *encounters >= 9 {
                                    OreType::Insight
                                } else if concept.integration > 1.0 {
                                    OreType::Writing
                                } else if concept.area > 0.8 {
                                    OreType::Art
                                } else {
                                    OreType::Code
                                };

                                let ore_name = format!("{}_ore_{}", concept.name, *encounters / 3);
                                let integration_value =
                                    concept.integration + (*encounters as f32 * 0.5);

                                let ore = PreciousOre {
                                    name: ore_name.clone(),
                                    ore_type,
                                    density: 0.9,
                                    depth: core_truth.depth,
                                    formed_from: concept.id,
                                    vent_cycles: *encounters,
                                    integration_value,
                                };

                                mineralization_events.push(FluidEvent::Mineralization {
                                    concept_name: concept.name.clone(),
                                    ore_name,
                                    ore_type: ore_type.as_str().to_string(),
                                    depth: core_truth.depth,
                                    vent_cycles: *encounters,
                                    integration_value,
                                });

                                ore_to_deposit.push(ore);
                            }
                        }
                    }
                }
            }

            // Net force and acceleration
            let net_force = buoyancy_force + drag_force + surface_force + thermal_force;
            let mut acceleration = net_force;

            // Turbulence perturbations
            if self.is_turbulent {
                let chaos_seed = (concept.layer * 1000.0 + concept.velocity * 500.0).sin();
                let turbulent_force = chaos_seed * self.turbulence_energy * 3.0;
                acceleration += turbulent_force;
                concept.velocity *= 0.95;
            }

            // Update velocity and position
            concept.velocity += acceleration * dt;
            let new_layer = concept.layer + concept.velocity * dt;

            // Surface breakthrough check
            if new_layer <= 0.0 && concept.velocity < 0.0 && !concept.has_broken_surface {
                let kinetic_energy = 0.5 * concept.velocity.powi(2);

                if kinetic_energy > self.surface_tension {
                    concept.has_broken_surface = true;
                    breakthrough_events.push(FluidEvent::SurfaceBreakthrough {
                        id: concept.id,
                        name: concept.name.clone(),
                        kinetic_energy,
                    });

                    let energy_loss = self.surface_tension;
                    let new_ke = (kinetic_energy - energy_loss).max(0.0);
                    concept.velocity = -(2.0 * new_ke).sqrt();
                } else {
                    breakthrough_events.push(FluidEvent::SurfaceBounce {
                        id: concept.id,
                        name: concept.name.clone(),
                        kinetic_energy,
                        required: self.surface_tension,
                    });
                    concept.velocity *= -0.3;
                }
            }

            concept.layer = new_layer.clamp(0.0, 1.0);

            // Boundary damping
            if concept.layer <= 0.0 || concept.layer >= 1.0 {
                concept.velocity *= 0.5;
            }

            // Continental collision
            for continent in &self.continents {
                if continent.contains_depth(concept.layer) {
                    let impermeability = continent.impermeability;

                    if concept.velocity > 0.0 {
                        concept.layer = continent.depth_range.0 - 0.01;
                        concept.velocity = -concept.velocity.abs() * (1.0 - impermeability);
                    } else {
                        concept.layer = continent.depth_range.1 + 0.01;
                        concept.velocity = concept.velocity.abs() * (1.0 - impermeability);
                    }
                    concept.velocity *= 0.3;
                    break;
                }
            }

            // Energy cascade: eddies → integration
            let kinetic_energy = 0.5 * concept.velocity.powi(2);
            if kinetic_energy > 0.1 {
                concept.eddy_scale = concept.eddy_scale.max(kinetic_energy);
            }

            if concept.eddy_scale > 0.01 {
                let breakdown_rate = self.viscosity * 2.0;
                let energy_dissipated = concept.eddy_scale * breakdown_rate * dt;
                concept.integration += energy_dissipated;
                self.total_integration += energy_dissipated;
                concept.eddy_scale *= 1.0 - breakdown_rate * dt;

                if concept.eddy_scale < 0.01 {
                    concept.integration += concept.eddy_scale;
                    self.total_integration += concept.eddy_scale;
                    concept.eddy_scale = 0.0;
                }
            }

            // Active damping
            if self.damping_factor > 0.01 {
                let damping_loss = concept.velocity.abs() * self.damping_factor * dt;
                concept.velocity *= 1.0 - self.damping_factor * dt;
                concept.integration += damping_loss;
                self.total_integration += damping_loss;
            }
        }

        // Apply core truth strengthening
        for (idx, strengthening) in core_truth_strengthened {
            if let Some(truth) = self.core_truths.get_mut(idx) {
                truth.activation_count += 1;
                truth.heat_output += strengthening;
            }
        }

        // Deposit ores
        for ore in ore_to_deposit {
            let ore_weight = ore.pressure_weight();
            self.ocean_floor_pressure += ore_weight;

            events.push(FluidEvent::OreDeposited {
                name: ore.name.clone(),
                ore_type: ore.ore_type.as_str().to_string(),
                total_pressure: self.ocean_floor_pressure,
                threshold: self.pressure_threshold,
            });

            self.ore_deposits.push(ore);
        }

        events.extend(mineralization_events);
        events.extend(breakthrough_events);

        // Decay damping factor
        if self.damping_factor > 0.01 {
            self.damping_factor *= 0.95;
        } else {
            self.damping_factor = 0.0;
        }

        // Salinity increase
        self.salinity += self.total_integration * self.salinity_rate * dt;

        // === Pass 5: Evaporation ===
        let mut evaporated_ids = Vec::new();
        for (id, concept) in &self.concepts {
            if concept.layer < self.evaporation_zone
                && concept.integration >= self.evaporation_threshold
                && !concept.has_evaporated
            {
                evaporated_ids.push(*id);
            }
        }

        for id in evaporated_ids {
            if let Some(concept) = self.concepts.get_mut(&id) {
                concept.has_evaporated = true;

                let trait_obj = CharacterTrait::new(concept.name.clone(), concept.integration, id);

                events.push(FluidEvent::ConceptEvaporated {
                    id,
                    name: concept.name.clone(),
                    trait_formed: concept.name.clone(),
                    integration: concept.integration,
                });

                self.atmosphere.push(trait_obj);
            }
        }

        // === Pass 6: Tectonic shift check ===
        if self.ocean_floor_pressure >= self.pressure_threshold {
            let mut ore_type_counts = HashMap::new();
            let mut total_integration = 0.0;
            let mut ore_names = Vec::new();

            for ore in &self.ore_deposits {
                *ore_type_counts.entry(&ore.ore_type).or_insert(0) += 1;
                total_integration += ore.integration_value;
                ore_names.push(ore.name.clone());
            }

            let dominant_ore_type = ore_type_counts
                .iter()
                .max_by_key(|(_, count)| *count)
                .map(|(ore_type, _)| *ore_type)
                .unwrap_or(&OreType::Insight);

            let continent_name = match dominant_ore_type {
                OreType::Art => "foundation_of_beauty",
                OreType::Code => "bedrock_of_logic",
                OreType::Insight => "pillar_of_wisdom",
                OreType::Writing => "archive_of_story",
            };

            let avg_ore_depth = self.ore_deposits.iter().map(|o| o.depth).sum::<f32>()
                / self.ore_deposits.len().max(1) as f32;

            let continent_span = 0.15;
            let depth_range = (
                (avg_ore_depth - continent_span / 2.0).max(0.6),
                (avg_ore_depth + continent_span / 2.0).min(0.95),
            );

            let continent = Continent {
                name: continent_name.to_string(),
                depth_range,
                formed_from_ores: ore_names.clone(),
                total_integration,
                impermeability: 0.9,
                formation_event: self.tectonic_shifts + 1,
            };

            events.push(FluidEvent::TectonicShift {
                continent_name: continent_name.to_string(),
                depth_range,
                ores_consumed: ore_names,
                total_integration,
            });

            self.continents.push(continent);
            self.tectonic_shifts += 1;
            self.ocean_floor_pressure = 0.0;
            self.ore_deposits.clear();
        }

        events
    }

    /// Get concepts in the surface zone.
    pub fn get_surface_concepts(&self, threshold: f32) -> Vec<&Concept> {
        let mut surface: Vec<_> = self
            .concepts
            .values()
            .filter(|c| c.layer < threshold)
            .collect();
        surface.sort_by(|a, b| a.layer.partial_cmp(&b.layer).unwrap());
        surface
    }

    /// Get concepts within a depth range.
    pub fn get_concepts_in_range(&self, min_depth: f32, max_depth: f32) -> Vec<&Concept> {
        self.concepts
            .values()
            .filter(|c| c.layer >= min_depth && c.layer <= max_depth)
            .collect()
    }

    /// Get ores within a depth range.
    pub fn get_ores_in_range(&self, min_depth: f32, max_depth: f32) -> Vec<&PreciousOre> {
        self.ore_deposits
            .iter()
            .filter(|o| o.depth >= min_depth && o.depth <= max_depth)
            .collect()
    }
}
