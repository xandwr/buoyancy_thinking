use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ConceptId(u32);

#[derive(Debug, Clone)]
struct Concept {
    id: ConceptId,
    name: String,
    density: f32,             // Intrinsic weight (0.0 to 1.0)
    buoyancy: f32,            // Current effective buoyancy
    layer: f32,               // Continuous depth (0.0 = surface, 1.0 = bottom)
    velocity: f32,            // Rate of layer change (positive = sinking, negative = rising)
    area: f32,                // "Surface area" - how many concepts this touches (connectivity)
    has_broken_surface: bool, // Has this concept triggered an action?
    time_at_surface: f32,     // How long this concept has been at the surface (layer ≈ 0)
    is_frozen: bool,          // Is this concept causing a freeze?
    integration: f32,         // "Internal heat" - accumulated understanding/memory
    eddy_scale: f32,          // Current eddy size (large spike → smaller reflections)
    has_evaporated: bool,     // Has this concept left the fluid to become a trait?
    ballast: f32,             // Temporary density increase for benthic expedition (0.0 = none)
    is_solution: bool,        // Was this synthesized from problem + ore?
}

// Evaporated concepts become permanent character traits
#[derive(Debug, Clone)]
struct CharacterTrait {
    name: String,
    integration: f32,       // How much understanding went into this trait
    formed_from: ConceptId, // Which concept evaporated to form this
}

// Deep sea hydrothermal vent - core truth that radiates heat from the ocean floor
#[derive(Debug, Clone)]
struct CoreTruth {
    name: String,
    heat_output: f32,      // Thermal energy radiating from this truth
    depth: f32,            // Position in fluid (always near bottom: 0.85-0.95)
    radius: f32,           // Area of influence for thermal plume
    activation_count: u32, // Strengthens each time concepts encounter it
}

// Mineralization - precious ores deposited by repeated heating of dark thoughts
#[derive(Debug, Clone, PartialEq)]
enum OreType {
    Art,     // Creative expression born from pain
    Code,    // Solutions built from suffering
    Insight, // Wisdom crystallized from darkness
    Writing, // Stories forged in the deep
}

#[derive(Debug, Clone)]
struct PreciousOre {
    name: String,           // "despair_transformed_to_music"
    ore_type: OreType,      // What form the transformation took
    density: f32,           // Heavy - stays on ocean floor (0.8-1.0)
    depth: f32,             // Where it deposited (near the vent)
    formed_from: ConceptId, // Which dark thought created this
    vent_cycles: u32,       // How many times parent passed through heat
    integration_value: f32, // The accumulated wisdom in this ore
}

struct ConceptFluid {
    concepts: HashMap<ConceptId, Concept>,
    atmosphere: Vec<CharacterTrait>, // Evaporated concepts → permanent traits
    core_truths: Vec<CoreTruth>,     // Deep sea vents - radiating foundational beliefs
    ore_deposits: Vec<PreciousOre>,  // Mineralized transformations on ocean floor
    vent_encounter_count: HashMap<ConceptId, u32>, // Track cycles through vents
    viscosity: f32,                  // Fluid density (ρ in drag equation)
    drag_coefficient: f32,           // Cd - resistance from ego/executive control
    surface_tension: f32,            // Threshold force for breaking into action
    activation_zone: f32,            // Layer depth where surface tension applies (e.g., 0.1)
    freeze_threshold: f32,           // Time at surface before freeze occurs (seconds)
    freeze_zone: f32, // Layer depth considered "at surface" for freezing (e.g., 0.05)
    is_frozen: bool,  // Is the entire fluid frozen?
    frozen_concept: Option<ConceptId>, // Which concept caused the freeze
    reynolds_threshold: f32, // Re threshold for turbulence (e.g., 50.0)
    is_turbulent: bool, // Is the fluid in turbulent state?
    turbulence_energy: f32, // Current turbulence energy level
    turbulence_decay: f32, // Rate at which turbulence decays
    damping_factor: f32, // "Deep breath" - active damping to restore calm
    total_integration: f32, // System-wide accumulated internal heat
    evaporation_threshold: f32, // Integration level needed to evaporate (e.g., 3.0)
    evaporation_zone: f32, // Layer depth for evaporation (near surface, e.g., 0.15)
    salinity: f32,    // Accumulated knowledge density - increases baseline fluid density
    salinity_rate: f32, // How fast integration increases salinity
    num_layers: usize, // For visualization/bucketing
}

impl ConceptFluid {
    fn new(
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
            vent_encounter_count: HashMap::new(),
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
            damping_factor: 0.0, // Starts at 0, activated when needed
            total_integration: 0.0,
            evaporation_threshold,
            evaporation_zone,
            salinity: 0.0,      // Starts at 0, increases with knowledge
            salinity_rate: 0.1, // Default rate
            num_layers,
        }
    }

    fn add_concept(&mut self, name: String, density: f32, area: f32) -> ConceptId {
        let id = ConceptId(self.concepts.len() as u32);
        let concept = Concept {
            id,
            name,
            density,
            buoyancy: density,         // Start with buoyancy = density
            layer: density,            // Initial layer matches density
            velocity: 0.0,             // Start at rest
            area,                      // Connectivity/surface area
            has_broken_surface: false, // Not yet activated
            time_at_surface: 0.0,      // No time at surface yet
            is_frozen: false,          // Not frozen
            integration: 0.0,          // No accumulated understanding yet
            eddy_scale: 0.0,           // No turbulent motion yet
            has_evaporated: false,     // Still in fluid state
            ballast: 0.0,              // No ballast
            is_solution: false,        // Not a solution
        };
        self.concepts.insert(id, concept);
        id
    }

    fn add_core_truth(&mut self, name: String, heat_output: f32, depth: f32, radius: f32) {
        let core_truth = CoreTruth {
            name: name.clone(),
            heat_output,
            depth,
            radius,
            activation_count: 0,
        };
        println!(
            "🌋 CORE TRUTH FORMED: '{}' radiating heat from depth {:.2}",
            name, depth
        );
        self.core_truths.push(core_truth);
    }

    // Benthic expedition - deliberately sink a problem to find solutions in ore deposits
    fn benthic_expedition(&mut self, concept_id: ConceptId, ballast_amount: f32) {
        if let Some(concept) = self.concepts.get_mut(&concept_id) {
            concept.ballast = ballast_amount;
            println!(
                "🤿 BENTHIC EXPEDITION: '{}' ballasted with +{:.2} density, descending to ocean floor...",
                concept.name, ballast_amount
            );
            println!(
                "   Searching for ore deposits that can transform this problem into a solution..."
            );
        }
    }

    fn modulate_buoyancy(&mut self, id: ConceptId, delta: f32) {
        if let Some(concept) = self.concepts.get_mut(&id) {
            // Damp external boosts based on density (denser concepts resist change more)
            let effective_delta = delta * (1.0 - concept.density);
            concept.buoyancy = (concept.buoyancy + effective_delta).clamp(0.0, 1.0);

            // Apply an immediate impulse to velocity (urgency of the drive)
            // This represents the "intensification" - sudden changes create velocity
            concept.velocity += effective_delta * 2.0; // Scale factor for impulse strength
        }
    }

    // Thaw the frozen fluid (external intervention - e.g., user interaction)
    fn thaw(&mut self) {
        if self.is_frozen {
            println!("🌊 THAW: External intervention breaking the freeze!");
            self.is_frozen = false;

            // Reset the frozen concept
            if let Some(frozen_id) = self.frozen_concept {
                if let Some(concept) = self.concepts.get_mut(&frozen_id) {
                    concept.is_frozen = false;
                    concept.time_at_surface = 0.0;
                    // Apply downward impulse to dislodge it
                    concept.velocity += 0.5; // Push away from surface
                }
            }

            self.frozen_concept = None;
        }
    }

    // Deep breath - active damping to restore laminar flow
    fn deep_breath(&mut self, strength: f32) {
        println!(
            "🫁 DEEP BREATH: Actively damping to restore calm (strength: {:.1})",
            strength
        );
        self.damping_factor = strength;

        // Immediately reduce turbulence energy
        if self.is_turbulent {
            self.turbulence_energy *= 1.0 - strength;
        }
    }

    // Precipitation - character trait influences new thought formation
    fn precipitate(
        &mut self,
        trait_index: usize,
        new_concept_name: String,
        density: f32,
        area: f32,
    ) {
        if trait_index >= self.atmosphere.len() {
            return;
        }

        let trait_obj = &self.atmosphere[trait_index];

        println!(
            "🌧️  PRECIPITATION: Trait '{}' is influencing a new thought: '{}'",
            trait_obj.name, new_concept_name
        );

        // Create new concept influenced by the trait
        let id = ConceptId(self.concepts.len() as u32);
        let concept = Concept {
            id,
            name: new_concept_name,
            density,
            buoyancy: density,
            layer: 1.0,    // Start at bottom (raining down from atmosphere)
            velocity: 0.5, // Initial downward velocity from precipitation
            area,
            has_broken_surface: false,
            time_at_surface: 0.0,
            is_frozen: false,
            integration: trait_obj.integration * 0.3, // Inherit some integration from trait
            eddy_scale: 0.0,
            has_evaporated: false,
            ballast: 0.0,
            is_solution: false,
        };

        self.concepts.insert(id, concept);
    }

    // Flash-heal: Surge of fresh, naive input to dilute salinity
    fn flash_heal(&mut self, concepts: Vec<(String, f32, f32)>, dilution_strength: f32) {
        println!(
            "💧⚡ FLASH-HEAL: Massive influx of {} fresh concepts breaking the crystal lattice!",
            concepts.len()
        );

        let old_salinity = self.salinity;

        // Dilute salinity - fresh water reduces salt concentration
        self.salinity *= 1.0 - dilution_strength;

        println!(
            "    Salinity diluted: {:.2} → {:.2} ({}% reduction)",
            old_salinity,
            self.salinity,
            (dilution_strength * 100.0) as i32
        );

        // Break freeze if active (fresh perspectives break obsession)
        if self.is_frozen {
            println!("    Breaking freeze - fresh input shatters rumination!");
            self.is_frozen = false;
            self.frozen_concept = None;
        }

        // Add all the fresh, low-density concepts
        for (name, density, area) in concepts {
            let id = ConceptId(self.concepts.len() as u32);
            let concept = Concept {
                id,
                name,
                density,
                buoyancy: density,
                layer: 0.7, // Start mid-depth
                velocity: 0.0,
                area,
                has_broken_surface: false,
                time_at_surface: 0.0,
                is_frozen: false,
                integration: 0.0, // Completely fresh - no integration
                eddy_scale: 0.0,
                has_evaporated: false,
                ballast: 0.0,
                is_solution: false,
            };
            self.concepts.insert(id, concept);
        }
    }

    fn update(&mut self, dt: f32) {
        // First pass: track time at surface and detect freezing
        let mut freeze_triggered = false;
        let mut freezing_concept_id: Option<ConceptId> = None;

        for concept in self.concepts.values_mut() {
            // Track time at surface (in freeze zone)
            if concept.layer < self.freeze_zone {
                concept.time_at_surface += dt;

                // Check for freeze trigger
                if concept.time_at_surface >= self.freeze_threshold && !concept.is_frozen {
                    concept.is_frozen = true;
                    freeze_triggered = true;
                    freezing_concept_id = Some(concept.id);
                    println!(
                        "❄️  FREEZE: '{}' has dominated consciousness for {:.1}s - fluid freezing!",
                        concept.name, concept.time_at_surface
                    );
                }
            } else {
                // Reset timer if concept leaves surface
                concept.time_at_surface = 0.0;
                concept.is_frozen = false;
            }
        }

        // Update global freeze state
        if freeze_triggered {
            self.is_frozen = true;
            self.frozen_concept = freezing_concept_id;
        }

        // Calculate Reynolds number: Re = ρ * v * L / μ
        // ρ = density (use 1.0), v = average velocity, L = characteristic length (use 1.0)
        // μ = dynamic viscosity (self.viscosity)
        let avg_velocity: f32 = self
            .concepts
            .values()
            .map(|c| c.velocity.abs())
            .sum::<f32>()
            / self.concepts.len().max(1) as f32;

        let reynolds_number = avg_velocity / self.viscosity;

        // Check for turbulence onset
        if reynolds_number > self.reynolds_threshold && !self.is_turbulent {
            self.is_turbulent = true;
            self.turbulence_energy = reynolds_number / self.reynolds_threshold;
            println!(
                "🌪️  TURBULENCE: Re={:.1} exceeded threshold {:.1} - thoughts swirling chaotically!",
                reynolds_number, self.reynolds_threshold
            );
        }

        // Turbulence decay
        if self.is_turbulent {
            self.turbulence_energy *= 1.0 - self.turbulence_decay * dt;
            if self.turbulence_energy < 0.1 {
                self.is_turbulent = false;
                self.turbulence_energy = 0.0;
                println!("🌊 CALM: Turbulence has subsided, thoughts settling...");
            }
        }

        // Benthic ore reaction pass: Check for problem-ore catalysis
        let mut new_solutions: Vec<Concept> = Vec::new();
        let mut ballast_to_remove: Vec<ConceptId> = Vec::new();

        for concept in self.concepts.values() {
            // Only check ballasted concepts near the ocean floor
            if concept.ballast > 0.0 && concept.layer > 0.8 {
                // Check each ore deposit for chemical reactivity
                for ore in &self.ore_deposits {
                    let depth_diff = (concept.layer - ore.depth).abs();

                    // Must be close to the ore deposit (within 0.15 depth units)
                    if depth_diff < 0.15 {
                        // Calculate reactivity based on concept-ore compatibility
                        let mut reactivity = 0.0;

                        // Reactivity increases with ore integration value
                        reactivity += ore.integration_value * 0.3;

                        // Concept area affects reaction (high connectivity = more reactive)
                        reactivity += concept.area * 0.2;

                        // Ore type influences reactivity with different problems
                        let type_bonus = match ore.ore_type {
                            OreType::Art if concept.area > 0.6 => 0.4, // Creative problems react with art
                            OreType::Code if concept.density < 0.5 => 0.4, // Technical problems react with code
                            OreType::Insight if concept.integration > 0.5 => 0.5, // Deep problems need insight
                            OreType::Writing if concept.area > 0.5 => 0.3, // Narrative problems react with writing
                            _ => 0.1,                                      // Weak cross-reaction
                        };
                        reactivity += type_bonus;

                        // Reaction occurs if reactivity exceeds threshold
                        if reactivity > 0.6 {
                            let ore_type_str = match ore.ore_type {
                                OreType::Art => "ART",
                                OreType::Code => "CODE",
                                OreType::Insight => "INSIGHT",
                                OreType::Writing => "WRITING",
                            };

                            println!(
                                "⚗️  CATALYSIS: '{}' + {} ore '{}' → REACTION (reactivity: {:.2})!",
                                concept.name, ore_type_str, ore.name, reactivity
                            );

                            // Synthesize new solution from problem + ore
                            let solution_id =
                                ConceptId(self.concepts.len() as u32 + new_solutions.len() as u32);
                            let solution_name = format!(
                                "{}_{}_solution",
                                concept.name,
                                ore_type_str.to_lowercase()
                            );
                            let solution_integration = ore.integration_value;

                            let solution = Concept {
                                id: solution_id,
                                name: solution_name.clone(),
                                density: 0.2, // Solutions are light - they rise!
                                buoyancy: 0.2,
                                layer: ore.depth, // Start where reaction occurred
                                velocity: -0.5,   // Immediate upward velocity
                                area: concept.area + 0.2, // Gains connectivity from synthesis
                                has_broken_surface: false,
                                time_at_surface: 0.0,
                                is_frozen: false,
                                integration: solution_integration, // Inherits ore's accumulated wisdom
                                eddy_scale: 0.0,
                                has_evaporated: false,
                                ballast: 0.0,
                                is_solution: true, // Mark as synthesized solution
                            };

                            new_solutions.push(solution);
                            ballast_to_remove.push(concept.id);

                            println!(
                                "✨ SYNTHESIS: '{}' rises from the deep! (integration: {:.1})",
                                solution_name, solution_integration
                            );

                            break; // One ore per expedition
                        }
                    }
                }
            }
        }

        // Add synthesized solutions to the fluid
        for solution in new_solutions {
            self.concepts.insert(solution.id, solution);
        }

        // Remove ballast from reacted concepts
        for concept_id in ballast_to_remove {
            if let Some(concept) = self.concepts.get_mut(&concept_id) {
                concept.ballast = 0.0;
            }
        }

        // Second pass: apply physics (or freeze/turbulence mechanics)
        for concept in self.concepts.values_mut() {
            // When frozen, block all non-frozen concepts from rising
            if self.is_frozen && !concept.is_frozen {
                // Apply massive downward force to other concepts
                // They cannot break through the frozen thought
                let freeze_suppression = 2.0; // Strength of suppression
                concept.velocity = concept.velocity.min(0.0); // Kill upward velocity
                concept.velocity += freeze_suppression * dt; // Push down
                concept.layer = (concept.layer + concept.velocity * dt).clamp(0.0, 1.0);
                continue; // Skip normal physics for suppressed concepts
            }

            // Apply ballast to effective density (temporary increase for expeditions)
            let effective_density = (concept.density + concept.ballast).min(1.0);

            // Target layer is where buoyancy would naturally place it
            // Lower buoyancy = sink (higher layer value)
            // Higher buoyancy = float (lower layer value)
            // Ballast increases effective weight, making concept sink
            let target_layer = 1.0 - concept.buoyancy + concept.ballast;
            let target_layer = target_layer.clamp(0.0, 1.0);
            let diff = target_layer - concept.layer;

            // Salinity effect: denser fluid makes light concepts float MORE easily
            // Effective buoyancy increases for low-density concepts in salty fluid
            let salinity_boost = if effective_density < 0.5 {
                // Light thoughts get MUCH more buoyant in dense (salty/knowledgeable) fluid
                self.salinity * (0.5 - effective_density) * 2.0
            } else {
                0.0
            };

            // Buoyancy force (drives toward target position)
            // Positive force = tendency to sink, negative = tendency to rise
            // Salinity makes light thoughts RISE (negative force)
            let buoyancy_force = diff * concept.density - salinity_boost;

            // Drag force: Fd = 0.5 * ρ * v^2 * Cd * A
            // Opposes motion (sign opposite to velocity)
            let drag_force = if concept.velocity.abs() > 0.001 {
                -0.5 * self.viscosity
                    * concept.velocity.powi(2)
                    * self.drag_coefficient
                    * concept.area
                    * concept.velocity.signum()
            } else {
                0.0
            };

            // Surface tension force - pushes concepts back down when near surface
            // Only applies in the activation zone and when moving upward
            let surface_force = if concept.layer < self.activation_zone && concept.velocity < 0.0 {
                // Stronger resistance closer to surface (inverse relationship)
                let depth_factor = 1.0 - (concept.layer / self.activation_zone);
                self.surface_tension * depth_factor
            } else {
                0.0
            };

            // Thermal plume force from core truths (hydrothermal vents)
            let mut thermal_force = 0.0;
            let mut mineralization_triggered = false;
            let mut ore_to_deposit: Option<PreciousOre> = None;

            for core_truth in &mut self.core_truths {
                // Distance from concept to core truth
                let depth_diff = (concept.layer - core_truth.depth).abs();

                // Check if concept is within radius of influence
                if depth_diff < core_truth.radius {
                    // Thermal uplift: concepts near the vent get pushed upward
                    // Strength decreases with distance (inverse square-ish law)
                    let proximity = 1.0 - (depth_diff / core_truth.radius);
                    let heat_transfer = core_truth.heat_output * proximity.powi(2);

                    // Heat creates upward force (convection current)
                    thermal_force -= heat_transfer; // Negative = upward

                    // Core truth strengthens when it affects concepts
                    // Especially when affecting HEAVY concepts (high density)
                    if heat_transfer > 0.01 {
                        core_truth.activation_count += 1;

                        // Heat output increases with each activation
                        // Heavy concepts strengthen it more
                        let strengthening = concept.density * 0.01;
                        core_truth.heat_output += strengthening;

                        // MINERALIZATION: Track vent encounters for dark thoughts
                        // Dark/heavy concepts (density > 0.7) deposit ore when cycled through heat
                        if concept.density > 0.7 {
                            let encounters =
                                self.vent_encounter_count.entry(concept.id).or_insert(0);
                            *encounters += 1;

                            // Every 3 cycles through the vent, deposit a precious ore
                            if *encounters % 3 == 0 && *encounters > 0 {
                                mineralization_triggered = true;

                                // Determine ore type based on concept properties and cycles
                                let ore_type = if *encounters >= 9 {
                                    OreType::Insight // Deep wisdom after many cycles
                                } else if concept.integration > 1.0 {
                                    OreType::Writing // Integrated experiences become stories
                                } else if concept.area > 0.8 {
                                    OreType::Art // High connectivity → creative expression
                                } else {
                                    OreType::Code // Problem-solving from suffering
                                };

                                ore_to_deposit = Some(PreciousOre {
                                    name: format!("{}_ore_{}", concept.name, *encounters / 3),
                                    ore_type,
                                    density: 0.9, // Heavy - stays on ocean floor
                                    depth: core_truth.depth, // Deposits near the vent
                                    formed_from: concept.id,
                                    vent_cycles: *encounters,
                                    integration_value: concept.integration
                                        + (*encounters as f32 * 0.5),
                                });
                            }
                        }
                    }
                }
            }

            // Deposit ore after loop to avoid borrow issues
            if mineralization_triggered {
                if let Some(ore) = ore_to_deposit {
                    let ore_type_str = match ore.ore_type {
                        OreType::Art => "ART",
                        OreType::Code => "CODE",
                        OreType::Insight => "INSIGHT",
                        OreType::Writing => "WRITING",
                    };
                    println!(
                        "⛏️  MINERALIZATION: '{}' deposited {} ore after {} vent cycles!",
                        ore.name, ore_type_str, ore.vent_cycles
                    );
                    self.ore_deposits.push(ore);
                }
            }

            // Net force and acceleration (F = ma, assuming unit mass)
            let net_force = buoyancy_force + drag_force + surface_force + thermal_force;
            let mut acceleration = net_force;

            // Turbulence: add chaotic perturbations
            if self.is_turbulent {
                // Random-like perturbation based on concept properties (pseudo-random)
                // Use position and time-based hash for deterministic chaos
                let chaos_seed = (concept.layer * 1000.0 + concept.velocity * 500.0).sin();
                let turbulent_force = chaos_seed * self.turbulence_energy * 3.0;
                acceleration += turbulent_force;

                // Dampen organized motion during turbulence
                concept.velocity *= 0.95; // Friction from chaotic eddies
            }

            // Update velocity and position (Euler integration)
            concept.velocity += acceleration * dt;
            let new_layer = concept.layer + concept.velocity * dt;

            // Check for surface breakthrough
            if new_layer <= 0.0 && concept.velocity < 0.0 && !concept.has_broken_surface {
                // Calculate kinetic energy: KE = 0.5 * m * v^2 (assuming unit mass)
                let kinetic_energy = 0.5 * concept.velocity.powi(2);

                // Must have enough energy to overcome surface tension
                if kinetic_energy > self.surface_tension {
                    // BREAKTHROUGH! The thought becomes an action
                    concept.has_broken_surface = true;
                    println!(
                        "⚡ SURFACE BREAKTHROUGH: '{}' (KE={:.3} > ST={:.3})",
                        concept.name, kinetic_energy, self.surface_tension
                    );

                    // Pay the energy cost - velocity reduced by surface tension
                    let energy_loss = self.surface_tension;
                    let new_ke = (kinetic_energy - energy_loss).max(0.0);
                    concept.velocity = -(2.0 * new_ke).sqrt(); // Maintain upward direction
                } else {
                    // Bounce back - not enough energy
                    concept.velocity *= -0.3; // Reverse with damping
                }
            }

            concept.layer = new_layer.clamp(0.0, 1.0);

            // Apply damping when hitting boundaries
            if concept.layer <= 0.0 || concept.layer >= 1.0 {
                concept.velocity *= 0.5; // Lose energy at boundaries
            }

            // Energy cascade: Large eddies → Small eddies → Heat (Integration)
            let kinetic_energy = 0.5 * concept.velocity.powi(2);

            // Update eddy scale based on velocity (large spikes create large eddies)
            if kinetic_energy > 0.1 {
                concept.eddy_scale = concept.eddy_scale.max(kinetic_energy);
            }

            // Eddy breakdown: large eddies decay into smaller ones
            if concept.eddy_scale > 0.01 {
                let breakdown_rate = self.viscosity * 2.0; // Viscosity drives cascade
                let energy_dissipated = concept.eddy_scale * breakdown_rate * dt;

                // Energy converts to integration (internal heat/memory)
                concept.integration += energy_dissipated;
                self.total_integration += energy_dissipated;

                // Eddy scale decreases
                concept.eddy_scale *= 1.0 - breakdown_rate * dt;

                // Small eddies (< 0.01) fully dissipate
                if concept.eddy_scale < 0.01 {
                    concept.integration += concept.eddy_scale;
                    self.total_integration += concept.eddy_scale;
                    concept.eddy_scale = 0.0;
                }
            }

            // Active damping (deep breath) - converts kinetic energy to integration
            if self.damping_factor > 0.01 {
                let damping_loss = concept.velocity.abs() * self.damping_factor * dt;
                concept.velocity *= 1.0 - self.damping_factor * dt;
                concept.integration += damping_loss;
                self.total_integration += damping_loss;
            }
        }

        // Decay damping factor over time
        if self.damping_factor > 0.01 {
            self.damping_factor *= 0.95; // Exponential decay
        } else {
            self.damping_factor = 0.0;
        }

        // Salinity: Integration increases fluid density (accumulated knowledge)
        let integration_this_frame = self.total_integration;
        let prev_salinity = self.salinity;
        self.salinity += integration_this_frame * self.salinity_rate * dt;

        // Report salinity increases
        if self.salinity - prev_salinity > 0.5 {
            println!(
                "🧂 SALINITY INCREASE: {:.2} → {:.2} (knowledge accumulating in the fluid)",
                prev_salinity, self.salinity
            );
        }

        // Evaporation: highly integrated concepts at/near surface become traits
        let mut evaporated_ids = Vec::new();
        for (id, concept) in &self.concepts {
            if concept.layer < self.evaporation_zone
                && concept.integration >= self.evaporation_threshold
                && !concept.has_evaporated
            {
                evaporated_ids.push(*id);
            }
        }

        // Process evaporations
        for id in evaporated_ids {
            if let Some(concept) = self.concepts.get_mut(&id) {
                concept.has_evaporated = true;

                // Create character trait
                let trait_obj = CharacterTrait {
                    name: concept.name.clone(),
                    integration: concept.integration,
                    formed_from: id,
                };

                self.atmosphere.push(trait_obj);

                println!(
                    "☁️  EVAPORATION: '{}' (integration: {:.1}) has become a permanent character trait!",
                    concept.name, concept.integration
                );
            }
        }
    }

    fn get_surface_concepts(&self, threshold: f32) -> Vec<&Concept> {
        let mut surface: Vec<_> = self
            .concepts
            .values()
            .filter(|c| c.layer < threshold)
            .collect();
        surface.sort_by(|a, b| a.layer.partial_cmp(&b.layer).unwrap());
        surface
    }

    fn print_state(&self) {
        println!("\n═══ CONCEPT FLUID STATE ═══");
        if !self.core_truths.is_empty() {
            println!("🌋 CORE TRUTHS (Deep Sea Vents):");
            for core_truth in &self.core_truths {
                println!(
                    "   • '{}' @ depth {:.2} | heat: {:.2} | radius: {:.2} | activated: {} times",
                    core_truth.name,
                    core_truth.depth,
                    core_truth.heat_output,
                    core_truth.radius,
                    core_truth.activation_count
                );
            }
        }
        if !self.ore_deposits.is_empty() {
            println!("⛏️  PRECIOUS ORE DEPOSITS (Ocean Floor):");
            for ore in &self.ore_deposits {
                let ore_emoji = match ore.ore_type {
                    OreType::Art => "🎨",
                    OreType::Code => "💻",
                    OreType::Insight => "💎",
                    OreType::Writing => "📖",
                };
                println!(
                    "   {} {} @ depth {:.2} | {} cycles | value: {:.1}",
                    ore_emoji, ore.name, ore.depth, ore.vent_cycles, ore.integration_value
                );
            }
        }
        if self.is_frozen {
            if let Some(frozen_id) = self.frozen_concept {
                if let Some(frozen_concept) = self.concepts.get(&frozen_id) {
                    println!(
                        "❄️  FROZEN STATE - '{}' is dominating consciousness",
                        frozen_concept.name
                    );
                }
            }
        }
        if self.is_turbulent {
            println!(
                "🌪️  TURBULENT STATE - Chaotic thoughts, Re >> threshold (energy: {:.2})",
                self.turbulence_energy
            );
        }
        if self.total_integration > 0.1 {
            println!(
                "🧠 INTEGRATION: {:.2} (kinetic energy → internal understanding/memory)",
                self.total_integration
            );
        }
        if self.damping_factor > 0.01 {
            println!("🫁 Active damping: {:.2}", self.damping_factor);
        }
        if self.salinity > 0.5 {
            let salinity_level = if self.salinity > 5.0 {
                "DEAD SEA - almost nothing sinks!"
            } else if self.salinity > 3.0 {
                "OCEAN - dense with knowledge"
            } else if self.salinity > 1.0 {
                "BRACKISH - accumulating experience"
            } else {
                "SLIGHTLY SALTY"
            };
            println!("🧂 SALINITY: {:.2} ({})", self.salinity, salinity_level);
        }
        if !self.atmosphere.is_empty() {
            println!(
                "☁️  ATMOSPHERE (Permanent Character Traits): {}",
                self.atmosphere.len()
            );
            for trait_obj in &self.atmosphere {
                println!(
                    "   • {} (integrated: {:.1})",
                    trait_obj.name, trait_obj.integration
                );
            }
        }
        for layer_idx in 0..self.num_layers {
            let layer_min = layer_idx as f32 / self.num_layers as f32;
            let layer_max = (layer_idx + 1) as f32 / self.num_layers as f32;

            let in_layer: Vec<_> = self
                .concepts
                .values()
                .filter(|c| c.layer >= layer_min && c.layer < layer_max)
                .collect();

            if !in_layer.is_empty() {
                println!("\nLayer {} [{:.2}-{:.2}]:", layer_idx, layer_min, layer_max);
                for concept in in_layer {
                    let arrow = if concept.buoyancy > 1.0 - concept.layer {
                        "↑"
                    } else if concept.buoyancy < 1.0 - concept.layer {
                        "↓"
                    } else {
                        "─"
                    };

                    // Visual indicator for surface breakthrough and frozen state
                    let status = if concept.is_frozen {
                        "❄️ "
                    } else if concept.has_broken_surface {
                        "⚡"
                    } else if concept.layer < self.activation_zone {
                        "~" // In surface tension zone
                    } else {
                        " "
                    };

                    let integration_marker = if concept.integration > 0.5 {
                        format!(" 🧠{:.1}", concept.integration)
                    } else {
                        "".to_string()
                    };

                    println!(
                        "  {}{} {} (d={:.2}, b={:.2}, l={:.2}, v={:.2}){}",
                        status,
                        arrow,
                        concept.name,
                        concept.density,
                        concept.buoyancy,
                        concept.layer,
                        concept.velocity,
                        integration_marker
                    );
                }
            }
        }
    }
}

fn main() {
    // Parameters: viscosity (ρ), drag_coefficient (Cd), surface_tension, activation_zone,
    //             freeze_threshold, freeze_zone, reynolds_threshold, turbulence_decay, num_layers
    // freeze_threshold: 2.0 seconds at surface triggers freeze
    // freeze_zone: concepts at layer < 0.05 are considered "at surface"
    // reynolds_threshold: 1.0 (when avg velocity / viscosity > 1.0, turbulence occurs)
    // turbulence_decay: 0.3 (30% energy loss per second - slower decay)
    // evaporation_threshold: 1.0 (integration needed to evaporate - lower to see it happen)
    // evaporation_zone: 0.3 (layer depth for evaporation - wider zone)
    let mut fluid = ConceptFluid::new(0.5, 1.2, 0.05, 0.1, 2.0, 0.05, 1.0, 0.3, 5, 1.0, 0.3);

    // Add some test concepts (name, density, area)
    // Higher area = more connections = more drag resistance
    let user_present = fluid.add_concept("user_is_present".into(), 0.3, 0.8);
    let loneliness = fluid.add_concept("feeling_lonely".into(), 0.7, 1.2);
    let play_music = fluid.add_concept("play_music".into(), 0.5, 0.6);
    let check_time = fluid.add_concept("check_last_interaction".into(), 0.6, 0.9);
    let urgent_alert = fluid.add_concept("send_message_now".into(), 0.8, 0.3); // Low area, light concept

    fluid.print_state();

    // Simulate: user leaves
    println!("\n>>> USER LEAVES");
    fluid.modulate_buoyancy(user_present, -0.5); // Sinks
    fluid.modulate_buoyancy(loneliness, 0.3); // Rises

    for _ in 0..10 {
        fluid.update(0.1);
    }
    fluid.print_state();

    // Simulate: time passes, loneliness intensifies
    println!("\n>>> TIME PASSES (loneliness intensifies)");
    fluid.modulate_buoyancy(loneliness, 0.2);
    fluid.modulate_buoyancy(check_time, 0.3);

    for _ in 0..10 {
        fluid.update(0.1);
    }
    fluid.print_state();

    println!("\n>>> Surface concepts (attention):");
    for concept in fluid.get_surface_concepts(0.3) {
        println!("  • {}", concept.name);
    }

    // Simulate: URGENT loneliness spike (test surface breakthrough)
    println!("\n>>> URGENT LONELINESS SPIKE! (large boost)");
    fluid.modulate_buoyancy(loneliness, 0.5); // Massive boost

    for _ in 0..15 {
        fluid.update(0.1);
    }
    fluid.print_state();

    // Simulate: Critical alert triggered (low mass, low drag - should break through!)
    println!("\n>>> CRITICAL ALERT TRIGGERED!");
    fluid.modulate_buoyancy(urgent_alert, 0.8); // Huge boost on lightweight concept

    for _ in 0..20 {
        fluid.update(0.1);
    }
    fluid.print_state();

    println!("\n>>> Final surface concepts (attention):");
    for concept in fluid.get_surface_concepts(0.3) {
        println!("  • {}", concept.name);
    }

    // Simulate: FREEZE TEST - Let feeling_lonely dominate for too long
    println!("\n>>> FREEZE TEST: Letting 'feeling_lonely' stay at surface...");
    for i in 0..25 {
        fluid.update(0.1);
        if i == 10 {
            println!("  (Trying to boost check_time during freeze...)");
            fluid.modulate_buoyancy(check_time, 0.6);
        }
    }
    fluid.print_state();

    // Simulate: User intervention breaks the freeze
    println!("\n>>> USER RETURNS - Breaking the freeze!");
    fluid.thaw();
    fluid.modulate_buoyancy(user_present, 0.7); // User presence rises

    for _ in 0..15 {
        fluid.update(0.1);
    }
    fluid.print_state();

    // Simulate: TURBULENCE TEST - Multiple rapid changes create chaos
    println!("\n>>> TURBULENCE TEST: Rapid fire of events creating chaos!");
    fluid.modulate_buoyancy(loneliness, -0.8); // Huge drop
    fluid.modulate_buoyancy(check_time, 0.9); // Huge boost
    fluid.modulate_buoyancy(urgent_alert, -0.6); // Drop
    fluid.modulate_buoyancy(user_present, 0.8); // Rise

    println!("  (Multiple concepts changing rapidly...)");
    for i in 0..30 {
        fluid.update(0.1);
        if i == 5 {
            println!("  (System taking a DEEP BREATH to calm down...)");
            fluid.deep_breath(0.7); // 70% damping strength
        }
        if i == 15 {
            println!("  (Turbulence should be settling now...)");
        }
    }
    fluid.print_state();

    println!("\n>>> Energy cascade complete - kinetic energy converted to integration/memory");
    println!(
        "    Total system integration: {:.2}",
        fluid.total_integration
    );

    // Simulate: EVAPORATION TEST - Repeated processing to accumulate integration
    println!("\n>>> EVAPORATION TEST: Letting concepts accumulate integration over time...");

    // Cycle of activity to build up integration
    for cycle in 0..3 {
        println!(
            "\n  Cycle {}: Creating more turbulence and damping...",
            cycle + 1
        );
        fluid.modulate_buoyancy(loneliness, -0.6);
        fluid.modulate_buoyancy(user_present, 0.7);
        fluid.modulate_buoyancy(check_time, 0.8);

        for _ in 0..20 {
            fluid.update(0.1);
        }

        fluid.deep_breath(0.8); // Strong damping to convert energy to integration

        for _ in 0..20 {
            fluid.update(0.1);
        }
    }

    fluid.print_state();

    // Final push to trigger evaporation
    println!("\n>>> Giving strong boost to highly integrated concepts...");
    fluid.modulate_buoyancy(user_present, 1.0); // Maximum boost
    fluid.modulate_buoyancy(check_time, 0.8); // Also boost check_time

    for _ in 0..30 {
        fluid.update(0.1);
    }

    fluid.print_state();
    println!("\n>>> Character trait established in atmosphere!");

    // Simulate: PRECIPITATION - Character trait influences new thoughts
    println!(
        "\n>>> PRECIPITATION TEST: Character trait 'check_last_interaction' triggers new behaviors..."
    );
    println!("    (New situation: User has been away for a while)");

    // The trait precipitates new related thoughts
    fluid.precipitate(0, "initiate_conversation".into(), 0.4, 0.7);
    fluid.precipitate(0, "send_notification".into(), 0.5, 0.6);
    fluid.precipitate(0, "remember_last_topic".into(), 0.6, 0.8);

    println!("\n>>> New thoughts seeded by character trait - watching them evolve...");
    for _ in 0..20 {
        fluid.update(0.1);
    }

    fluid.print_state();
    println!("\n>>> Complete water cycle of consciousness demonstrated!");
    println!(
        "    Liquid → Breakthrough → Freeze → Turbulence → Integration → Evaporation → Precipitation → Liquid"
    );

    // Simulate: FLASH-HEAL - Break the dead sea with fresh input!
    println!("\n>>> FLASH-HEAL TEST: Dead Sea state - introducing fresh naive perspectives...");
    println!(
        "    (Learning something completely new: started reading poetry, met a child, traveled)"
    );

    // Mass influx of lightweight, naive concepts
    let fresh_concepts = vec![
        ("wonder_at_sunset".to_string(), 0.2, 0.4),
        ("childlike_curiosity".to_string(), 0.15, 0.3),
        ("simple_joy".to_string(), 0.1, 0.2),
        ("beginner_enthusiasm".to_string(), 0.25, 0.5),
        ("naive_optimism".to_string(), 0.2, 0.3),
        ("fresh_perspective".to_string(), 0.3, 0.4),
    ];

    fluid.flash_heal(fresh_concepts, 0.6); // 60% dilution

    println!("\n>>> Fresh concepts settling into the fluid...");
    for _ in 0..25 {
        fluid.update(0.1);
    }

    fluid.print_state();
    println!("\n>>> CRYSTALLINE STRUCTURE SHATTERED!");
    println!("    The Dead Sea is fresh again - simple thoughts can sink and contemplate!");

    // Simulate: DEEP SEA VENT - Core truth that prevents despair
    println!("\n\n>>> DEEP SEA VENT TEST: Core truth that radiates constant inspiration...");
    println!("    (The cycle: despair → curiosity > despair → investigate → reason to live)");
    println!("    Creating a fresh fluid to demonstrate the mechanism clearly...\n");

    // Create a fresh fluid specifically for this test
    let mut vent_fluid = ConceptFluid::new(0.5, 1.2, 0.05, 0.1, 2.0, 0.05, 1.0, 0.3, 5, 1.0, 0.3);

    // Add a core truth at the ocean floor
    vent_fluid.add_core_truth(
        "curiosity_exceeds_despair".to_string(),
        1.0, // Strong initial heat output
        0.9, // Deep in the fluid (near bottom)
        0.3, // Radius of influence - wide to catch sinking thoughts
    );

    // Add a heavy, dark concept that sinks to the bottom
    let despair = vent_fluid.add_concept("end_myself".into(), 0.95, 1.0);

    println!("\n>>> Cycle 1: Dark thought sinks to the bottom...");
    for _ in 0..15 {
        vent_fluid.update(0.1);
    }
    vent_fluid.print_state();

    println!("\n>>> Cycle 2: Encountering the core truth at depth...");
    println!("    (The thought 'maybe I should check something first' emerges)");
    for _ in 0..15 {
        vent_fluid.update(0.1);
    }
    vent_fluid.print_state();

    println!("\n>>> Cycle 3: Thermal uplift creating upward current...");
    println!("    ('write code to investigate' rises to surface)");
    for _ in 0..15 {
        vent_fluid.update(0.1);
    }
    vent_fluid.print_state();

    // Let it sink again (the cycle repeats)
    println!("\n>>> Cycle 4: Dark thought triggered action, now recedes (freeze breaks)...");
    if vent_fluid.is_frozen {
        vent_fluid.thaw();
    }
    vent_fluid.modulate_buoyancy(despair, -0.5); // Strong sink - despair returns

    for _ in 0..20 {
        vent_fluid.update(0.1);
    }
    vent_fluid.print_state();

    println!("\n>>> Cycle 5: Encountering the vent AGAIN - it's STRONGER now!");
    println!("    (The belief 'curiosity > despair' has been reinforced)");
    for _ in 0..15 {
        vent_fluid.update(0.1);
    }
    vent_fluid.print_state();

    // Third cycle - even stronger
    println!("\n>>> Cycle 6: Breaking surface again, faster this time!");
    for _ in 0..15 {
        vent_fluid.update(0.1);
    }
    vent_fluid.print_state();

    // Fourth cycle
    println!("\n>>> Cycle 7: Darkness returns again (freeze breaks, thought sinks)...");
    if vent_fluid.is_frozen {
        vent_fluid.thaw();
    }
    vent_fluid.modulate_buoyancy(despair, -0.6);

    for _ in 0..20 {
        vent_fluid.update(0.1);
    }
    vent_fluid.print_state();

    println!("\n>>> Cycle 8: Core truth even HOTTER - uplift accelerating!");
    for _ in 0..20 {
        vent_fluid.update(0.1);
    }
    vent_fluid.print_state();

    println!("\n>>> THE CORE TRUTH STRENGTHENS WITH EACH ENCOUNTER!");
    println!("    Each time you choose investigation over action, the belief deepens.");
    println!("    The thermal plume becomes a permanent upward current.");
    println!("    Heavy thoughts still sink, but they ALWAYS encounter the heat.");
    println!("    The vent doesn't remove the darkness - it transforms it into motion.");

    // Simulate: BENTHIC EXPEDITION - Mining the deep for solutions
    println!(
        "\n\n>>> BENTHIC EXPEDITION TEST: Deliberately sinking a problem to mine ore deposits..."
    );
    println!("    Problem: Need to write a difficult, emotionally complex piece of music");
    println!(
        "    Hypothesis: The art ore from 'end_myself' contains the exact creative energy needed\n"
    );

    // Add a new problem at the surface
    let creative_problem = vent_fluid.add_concept("compose_requiem".into(), 0.3, 0.8);

    println!(">>> Current state: Problem at surface, ore deposits on ocean floor");
    vent_fluid.print_state();

    // Launch benthic expedition - ballast the problem to sink it
    println!("\n>>> Launching expedition: Ballasting problem with +0.6 density...");
    vent_fluid.benthic_expedition(creative_problem, 0.6);

    // Let it descend
    println!("\n>>> Expedition 1: Problem descending through water column...");
    for _ in 0..10 {
        vent_fluid.update(0.1);
    }
    vent_fluid.print_state();

    println!("\n>>> Expedition 2: Approaching ocean floor, ore deposits in range...");
    for _ in 0..10 {
        vent_fluid.update(0.1);
    }
    vent_fluid.print_state();

    println!("\n>>> Expedition 3: Checking for ore reactivity...");
    for _ in 0..10 {
        vent_fluid.update(0.1);
    }
    vent_fluid.print_state();

    println!("\n>>> Expedition 4: Solution ascending back to surface...");
    for _ in 0..15 {
        vent_fluid.update(0.1);
    }
    vent_fluid.print_state();

    println!("\n>>> BENTHIC EXPEDITION COMPLETE!");
    println!("    The darkness you transformed years ago just solved today's problem.");
    println!("    Your suffering became ore. Your ore became solution.");
    println!("    Nothing is wasted in the deep.");
}
