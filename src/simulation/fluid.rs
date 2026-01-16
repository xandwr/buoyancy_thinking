use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    concept::{Concept, ConceptId},
    continent::Continent,
    core_truth::CoreTruth,
    ore::{OreType, PreciousOre},
    standing_wave::{DivisionExperiment, DivisionProblem, DivisionResult, StandingWave},
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

    // === Debug ===
    /// Total simulation ticks
    pub tick_count: u64,

    // === Division Experiments (Analog Computing) ===
    /// Active standing waves for division experiments
    pub standing_waves: Vec<StandingWave>,
    /// Currently running division experiment
    pub active_experiment: Option<DivisionExperiment>,
    /// Completed experiment results
    pub experiment_results: Vec<DivisionResult>,

    // === Non-Newtonian Shear-Thinning Model ===
    /// Base viscosity (at rest)
    pub base_viscosity: f32,
    /// Shear-thinning coefficient: how much viscosity drops under shear
    /// Higher values = more dramatic thinning at high shear rates
    pub shear_thinning_coefficient: f32,
    /// Shear rate threshold: velocity gradient above which thinning activates
    pub shear_threshold: f32,
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
            tick_count: 0,
            standing_waves: Vec::new(),
            active_experiment: None,
            experiment_results: Vec::new(),
            base_viscosity: viscosity,
            shear_thinning_coefficient: 0.8, // Default: 80% viscosity reduction at max shear
            shear_threshold: 0.3,            // Velocity above which thinning kicks in
        }
    }

    /// Create a fluid with default parameters.
    pub fn default() -> Self {
        Self::new(0.5, 1.2, 0.05, 0.1, 2.0, 0.05, 1.0, 0.3, 5, 1.0, 0.3)
    }

    /// Calculate effective viscosity using shear-thinning model.
    /// High velocity (shear) → lower viscosity → allows "remainder screaming"
    /// Low velocity → high viscosity → maintains stability
    pub fn effective_viscosity(&self, velocity: f32) -> f32 {
        let shear_rate = velocity.abs();

        if shear_rate <= self.shear_threshold {
            // Below threshold: full viscosity (Newtonian)
            self.viscosity
        } else {
            // Above threshold: shear-thinning (non-Newtonian)
            // Viscosity drops as shear increases
            let excess_shear = shear_rate - self.shear_threshold;
            let thinning_factor = 1.0 - (self.shear_thinning_coefficient * excess_shear).min(0.9);
            self.viscosity * thinning_factor
        }
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

    // === Division Experiment Methods (Analog Computing) ===

    /// Start a division experiment: encode V ÷ n using standing waves and bubbles.
    ///
    /// The standing wave creates nodes at regular intervals (the divisor).
    /// Bubbles (the dividend) are injected and settle into nodes.
    /// If V/n is integer → laminar flow (bubbles fill nodes perfectly)
    /// If V/n has remainder → turbulence (extra bubbles can't find nodes)
    ///
    /// The `salinity_boost` parameter enables Laminar Streamlining:
    /// - Higher salinity → higher effective viscosity → more damping
    /// - This suppresses "volume overhead" noise from bubble count
    /// - Making "remainder turbulence" more distinct and measurable
    pub fn start_division_experiment_with_salinity(
        &mut self,
        dividend: f32,
        divisor: f32,
        salinity_boost: f32,
    ) -> Uuid {
        // Clear any previous experiment
        if let Some(ref exp) = self.active_experiment {
            // Remove old bubbles
            for id in &exp.bubble_ids {
                self.concepts.remove(id);
            }
        }
        self.standing_waves.clear();

        // Create the problem
        let problem = DivisionProblem::new(dividend, divisor);
        let problem_id = problem.id;

        // Create the standing wave (encodes the divisor)
        let wave = StandingWave::new(divisor, 8.0);
        self.standing_waves.push(wave.clone());

        // Create the experiment tracker
        let mut experiment = DivisionExperiment::new(problem, self.tick_count);
        experiment.wave = wave;

        // Inject bubbles (the dividend) - very buoyant particles
        for i in 0..dividend as usize {
            let id = Uuid::new_v4();
            let bubble_name = format!("bubble_{}", i);

            // Create a light, buoyant bubble
            let mut bubble = Concept::new(id, bubble_name, 0.15, 0.3);
            // Spread bubbles across the depth range so they need to find nodes
            bubble.layer = 0.2 + (i as f32 * 0.1) % 0.6;
            // Give initial random-ish velocity to ensure physics activates
            bubble.velocity = 0.1 * ((i as f32 * 0.7).sin());

            experiment.bubble_ids.push(id);
            self.concepts.insert(id, bubble);
        }

        // Apply Laminar Streamlining: boost salinity to increase effective viscosity
        // This dampens the "volume overhead" noise, making remainder turbulence clearer
        experiment.original_salinity = self.salinity;
        experiment.salinity_boost = salinity_boost;
        self.salinity += salinity_boost;

        self.active_experiment = Some(experiment);

        // Reset turbulence state for clean measurement
        self.is_turbulent = false;
        self.turbulence_energy = 0.0;

        problem_id
    }

    /// Start a division experiment with default salinity (no boost).
    pub fn start_division_experiment(&mut self, dividend: f32, divisor: f32) -> Uuid {
        self.start_division_experiment_with_salinity(dividend, divisor, 0.0)
    }

    /// Check if the current experiment has settled (reached equilibrium).
    pub fn check_experiment_settlement(&mut self) -> Option<DivisionResult> {
        let experiment = self.active_experiment.as_mut()?;

        // Calculate experiment-specific turbulence from bubble velocities
        // This measures how much the bubbles are jostling for position
        let bubble_kinetic_energy: f32 = experiment
            .bubble_ids
            .iter()
            .filter_map(|id| self.concepts.get(id))
            .map(|c| 0.5 * c.velocity.powi(2))
            .sum();

        // Accumulate the kinetic energy as a measure of turbulence
        // More bubbles fighting for nodes = more accumulated energy
        experiment.accumulated_turbulence += bubble_kinetic_energy;

        experiment.peak_reynolds = experiment.peak_reynolds.max(
            experiment
                .bubble_ids
                .iter()
                .filter_map(|id| self.concepts.get(id))
                .map(|c| c.velocity.abs())
                .sum::<f32>()
                / self.viscosity,
        );

        // Check settlement conditions
        let bubble_velocities: Vec<f32> = experiment
            .bubble_ids
            .iter()
            .filter_map(|id| self.concepts.get(id))
            .map(|c| c.velocity.abs())
            .collect();

        let avg_velocity: f32 =
            bubble_velocities.iter().sum::<f32>() / bubble_velocities.len().max(1) as f32;
        let max_velocity: f32 = bubble_velocities.iter().copied().fold(0.0, f32::max);

        // Record velocity for jitter analysis (Time-of-Flight Delta measurement)
        // This captures the "stuttering" / micro-cavitation of remainder bubbles
        experiment.record_velocity(avg_velocity);

        // Settlement: all bubbles nearly stationary
        // Require minimum 60 ticks (1 second) before considering settlement
        let ticks_elapsed = self.tick_count.saturating_sub(experiment.start_tick);
        let min_ticks_for_settlement = 60;
        let is_settled =
            ticks_elapsed >= min_ticks_for_settlement && max_velocity < 0.05 && avg_velocity < 0.02;
        let is_timed_out = experiment.is_timed_out(self.tick_count);

        if is_settled || is_timed_out {
            experiment.settled = true;
            return Some(self.finalize_experiment());
        }

        None
    }

    /// Finalize the experiment and calculate the result.
    fn finalize_experiment(&mut self) -> DivisionResult {
        let experiment = self.active_experiment.take().unwrap();

        // Calculate node occupancy
        let mut node_occupancy = vec![0u32; experiment.wave.node_positions.len()];
        let node_tolerance = experiment.wave.node_spacing / 2.0;

        for bubble_id in &experiment.bubble_ids {
            if let Some(bubble) = self.concepts.get(bubble_id) {
                // Find which node this bubble is at
                for (i, &node_pos) in experiment.wave.node_positions.iter().enumerate() {
                    if (bubble.layer - node_pos).abs() < node_tolerance {
                        node_occupancy[i] += 1;
                        break;
                    }
                }
            }
        }

        // Calculate turbulence-based remainder
        // Key insight: extra bubbles that can't fit in nodes create turbulence
        let mathematical_quotient =
            (experiment.problem.dividend / experiment.problem.divisor).floor();
        let mathematical_remainder = experiment.problem.dividend % experiment.problem.divisor;

        // The turbulence energy correlates with remainder
        // Perfect division → lower turbulence (bubbles settle into nodes)
        // Remainder r → higher turbulence (extra bubbles jostle)
        //
        // Normalize turbulence by ticks to get average energy per tick
        let ticks = (self.tick_count - experiment.start_tick).max(1) as f32;
        let _normalized_turbulence = experiment.accumulated_turbulence / ticks;

        let is_divisible = mathematical_remainder < 0.001;

        // Calculate Reynolds number from final state
        let final_reynolds: f32 = experiment
            .bubble_ids
            .iter()
            .filter_map(|id| self.concepts.get(id))
            .map(|c| c.velocity.abs())
            .sum::<f32>()
            / self.viscosity;

        // Calculate velocity jitter (vσ) - the "Time-of-Flight Delta" metric
        // High vσ = micro-cavitation / stuttering from remainder bubbles competing for nodes
        // Low vσ = laminar, predictable flow (divisible case)
        let (velocity_mean, velocity_sigma) = experiment.calculate_velocity_sigma();

        let result = DivisionResult {
            dividend: experiment.problem.dividend,
            divisor: experiment.problem.divisor,
            is_divisible,
            quotient: mathematical_quotient,
            remainder: mathematical_remainder, // Use mathematical for accuracy, turbulence for verification
            reynolds_number: final_reynolds,
            velocity_sigma,
            velocity_mean,
            peak_jitter: experiment.peak_jitter, // Key metric: captures transient micro-cavitation
            turbulence_energy: experiment.accumulated_turbulence,
            ticks_to_settle: self.tick_count - experiment.start_tick,
            node_occupancy,
            salinity_boost: experiment.salinity_boost,
        };

        // Restore original salinity (remove the Laminar Streamlining boost)
        self.salinity = experiment.original_salinity;

        // Clean up bubbles
        for id in experiment.bubble_ids {
            self.concepts.remove(&id);
        }
        self.standing_waves.clear();

        // Store result
        self.experiment_results.push(result.clone());

        result
    }

    /// Get the current experiment status.
    pub fn get_experiment_status(&self) -> Option<&DivisionExperiment> {
        self.active_experiment.as_ref()
    }

    /// Run one physics tick, returning all significant events that occurred.
    pub fn update(&mut self, dt: f32) -> Vec<FluidEvent> {
        self.tick_count += 1;
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

            // Non-Newtonian shear-thinning: effective viscosity drops at high velocity
            // This allows "remainder bubbles" to scream through local turbulence
            let effective_visc = {
                let shear_rate = concept.velocity.abs();
                if shear_rate <= self.shear_threshold {
                    self.viscosity
                } else {
                    let excess_shear = shear_rate - self.shear_threshold;
                    let thinning_factor =
                        1.0 - (self.shear_thinning_coefficient * excess_shear).min(0.9);
                    self.viscosity * thinning_factor
                }
            };

            let drag_force = if concept.velocity.abs() > 0.001 {
                -0.5 * effective_visc
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

            // Standing wave force (for division experiments)
            let mut wave_force = 0.0;
            for wave in &self.standing_waves {
                wave_force += wave.force_at_depth(concept.layer);
            }

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
            let net_force =
                buoyancy_force + drag_force + surface_force + thermal_force + wave_force;
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
