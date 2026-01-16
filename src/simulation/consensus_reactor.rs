use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::concept::ConceptId;

// ============================================================================
// PHASE TRANSITION EXTRACTION
// ============================================================================
// When jitter crosses a threshold, we freeze velocity vectors and extract
// the physical structure. This isn't a compromise—it's what SURVIVES.

/// A frozen probe state at the moment of phase transition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrozenProbe {
    pub id: ConceptId,
    /// Position in depth space (0.0-1.0)
    pub depth: f32,
    /// Velocity at freeze moment
    pub frozen_velocity: f32,
    /// Which vent dominated this probe's motion
    pub dominant_vent: VentDominance,
    /// Distance to nearest Voronoi neighbor
    pub nearest_neighbor_dist: f32,
    /// Local density (probes per unit depth)
    pub local_density: f32,
}

/// Which contradictory vent dominated a probe's final state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VentDominance {
    /// Probe settled closer to vent A's influence
    VentA,
    /// Probe settled closer to vent B's influence
    VentB,
    /// Probe is in the collision zone (contested territory)
    Contested,
    /// Probe escaped both influences (boundary case)
    Escaped,
}

/// A Voronoi cell in the 1D depth space.
/// Represents a "territory" controlled by a probe.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoronoiCell {
    /// The probe that owns this cell
    pub owner_id: ConceptId,
    /// Center position (the probe's depth)
    pub center: f32,
    /// Left boundary of the cell
    pub left_bound: f32,
    /// Right boundary of the cell
    pub right_bound: f32,
    /// Cell width (territory size)
    pub width: f32,
    /// Which vent dominates this cell
    pub dominance: VentDominance,
}

/// Emergent property extracted from the phase structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergentProperty {
    /// Name of the property
    pub name: String,
    /// The physical basis for this property
    pub physical_basis: String,
    /// Confidence based on structural stability
    pub confidence: f32,
    /// Depth range where this property manifests
    pub depth_range: (f32, f32),
}

/// The complete phase structure extracted at transition.
/// This is the "new material" that forms—not a compromise, but what survives.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseStructure {
    /// Unique identifier
    pub id: Uuid,
    /// When the phase transition occurred (tick)
    pub transition_tick: u64,
    /// Jitter level that triggered the transition
    pub trigger_jitter: f32,
    /// The frozen probe states
    pub frozen_probes: Vec<FrozenProbe>,
    /// Voronoi tessellation of the depth space
    pub voronoi_cells: Vec<VoronoiCell>,
    /// Territory controlled by vent A (fraction of depth space)
    pub vent_a_territory: f32,
    /// Territory controlled by vent B (fraction of depth space)
    pub vent_b_territory: f32,
    /// Contested zone size (fraction)
    pub contested_territory: f32,
    /// The collision boundary depth (where territories meet)
    pub collision_boundary: f32,
    /// Emergent properties extracted from the structure
    pub emergent_properties: Vec<EmergentProperty>,
    /// The synthesized "new material" name
    pub material_name: String,
    /// Description of the new material's properties
    pub material_description: String,
    /// Original positions for reference
    pub position_a: String,
    pub position_b: String,
}

impl PhaseStructure {
    /// Extract emergent properties from the physical structure.
    /// These are properties that NEITHER input position had—they emerge
    /// from the collision dynamics.
    pub fn extract_emergent_properties(&mut self) {
        self.emergent_properties.clear();

        // Property 1: Contextual Sovereignty
        // If there's a clear collision boundary with territories on each side,
        // the emergent property is "context-dependent application"
        if self.contested_territory < 0.3
            && self.vent_a_territory > 0.2
            && self.vent_b_territory > 0.2
        {
            let boundary_sharpness = 1.0 - self.contested_territory;
            self.emergent_properties.push(EmergentProperty {
                name: "Contextual Sovereignty".to_string(),
                physical_basis: format!(
                    "Clear boundary at depth {:.2} separates domains. \
                     Above: {} territory ({:.0}%). Below: {} territory ({:.0}%).",
                    self.collision_boundary,
                    "Position A",
                    self.vent_a_territory * 100.0,
                    "Position B",
                    self.vent_b_territory * 100.0
                ),
                confidence: boundary_sharpness,
                depth_range: (0.0, 1.0),
            });
        }

        // Property 2: Gradient Transition
        // If the contested zone is large, the emergent property is
        // "graduated application" (not binary, but scaled)
        if self.contested_territory > 0.3 {
            self.emergent_properties.push(EmergentProperty {
                name: "Gradient Transition".to_string(),
                physical_basis: format!(
                    "Large contested zone ({:.0}%) indicates no sharp boundary. \
                     Properties blend across depth {:.2} to {:.2}.",
                    self.contested_territory * 100.0,
                    self.collision_boundary - self.contested_territory / 2.0,
                    self.collision_boundary + self.contested_territory / 2.0
                ),
                confidence: self.contested_territory,
                depth_range: (
                    (self.collision_boundary - self.contested_territory / 2.0).max(0.0),
                    (self.collision_boundary + self.contested_territory / 2.0).min(1.0),
                ),
            });
        }

        // Property 3: Asymmetric Dominance
        // If one territory is much larger, that position has "structural advantage"
        let territory_ratio = self.vent_a_territory / self.vent_b_territory.max(0.001);
        if territory_ratio > 2.0 || territory_ratio < 0.5 {
            let (dominant, dominated, ratio) = if territory_ratio > 1.0 {
                ("Position A", "Position B", territory_ratio)
            } else {
                ("Position B", "Position A", 1.0 / territory_ratio)
            };
            self.emergent_properties.push(EmergentProperty {
                name: "Structural Advantage".to_string(),
                physical_basis: format!(
                    "{} captures {:.1}x more territory than {}. \
                     This isn't preference—it's physical sustainability.",
                    dominant, ratio, dominated
                ),
                confidence: (ratio - 1.0).min(1.0),
                depth_range: (0.0, 1.0),
            });
        }

        // Property 4: Density Stratification
        // If probes cluster at different densities in different zones,
        // we have "level-dependent behavior"
        let surface_probes: Vec<_> = self
            .frozen_probes
            .iter()
            .filter(|p| p.depth < 0.3)
            .collect();
        let deep_probes: Vec<_> = self
            .frozen_probes
            .iter()
            .filter(|p| p.depth > 0.7)
            .collect();

        if !surface_probes.is_empty() && !deep_probes.is_empty() {
            let surface_density: f32 = surface_probes.iter().map(|p| p.local_density).sum::<f32>()
                / surface_probes.len() as f32;
            let deep_density: f32 =
                deep_probes.iter().map(|p| p.local_density).sum::<f32>() / deep_probes.len() as f32;

            let density_ratio = surface_density / deep_density.max(0.001);
            if density_ratio > 1.5 || density_ratio < 0.67 {
                let (sparse, dense) = if density_ratio > 1.0 {
                    ("deep/private", "surface/public")
                } else {
                    ("surface/public", "deep/private")
                };
                self.emergent_properties.push(EmergentProperty {
                    name: "Density Stratification".to_string(),
                    physical_basis: format!(
                        "Probes cluster {} at {} levels, sparse at {} levels. \
                         Information has natural depth-dependent visibility.",
                        if density_ratio > 1.0 {
                            "densely"
                        } else {
                            "sparsely"
                        },
                        dense,
                        sparse
                    ),
                    confidence: (density_ratio - 1.0).abs().min(1.0),
                    depth_range: (0.0, 1.0),
                });
            }
        }
    }

    /// Generate the "new material" name and description.
    /// This is the key insight: the ore is NOT a compromise.
    pub fn synthesize_material(&mut self) {
        // Analyze the structure to determine what new material formed
        let has_boundary = self.contested_territory < 0.3;
        let has_gradient = self.contested_territory > 0.3;
        let has_asymmetry = (self.vent_a_territory - self.vent_b_territory).abs() > 0.3;
        let has_stratification = self
            .emergent_properties
            .iter()
            .any(|p| p.name == "Density Stratification");

        // Generate material name based on dominant structural features
        self.material_name = if has_boundary && has_stratification {
            "Contextual Sovereignty".to_string()
        } else if has_gradient && !has_asymmetry {
            "Graduated Synthesis".to_string()
        } else if has_asymmetry && has_boundary {
            "Dominant Resolution".to_string()
        } else if has_gradient && has_stratification {
            "Stratified Gradient".to_string()
        } else if self.contested_territory > 0.6 {
            "Persistent Tension".to_string()
        } else {
            "Emergent Equilibrium".to_string()
        };

        // Generate description
        self.material_description = match self.material_name.as_str() {
            "Contextual Sovereignty" => {
                format!(
                    "Data that is {} in aggregate (surface, depth < {:.2}) \
                     but {} in detail (mineralized, depth > {:.2}). \
                     The boundary at {:.2} is not a compromise—it's where \
                     the physics naturally separates concerns.",
                    self.position_a,
                    self.collision_boundary,
                    self.position_b,
                    self.collision_boundary,
                    self.collision_boundary
                )
            }
            "Graduated Synthesis" => {
                format!(
                    "No sharp boundary between '{}' and '{}'. \
                     Instead, a gradient zone ({:.0}% of depth space) where \
                     both properties blend proportionally. \
                     This isn't fence-sitting—it's continuous adaptation.",
                    self.position_a,
                    self.position_b,
                    self.contested_territory * 100.0
                )
            }
            "Dominant Resolution" => {
                let (winner, loser) = if self.vent_a_territory > self.vent_b_territory {
                    (&self.position_a, &self.position_b)
                } else {
                    (&self.position_b, &self.position_a)
                };
                format!(
                    "'{}' structurally dominates '{}' \
                     (territory ratio: {:.1}x). This isn't opinion—\
                     it's what survives the 60Hz collision dynamics.",
                    winner,
                    loser,
                    (self.vent_a_territory / self.vent_b_territory.max(0.001))
                        .max(self.vent_b_territory / self.vent_a_territory.max(0.001))
                )
            }
            "Stratified Gradient" => {
                format!(
                    "Different density at different depths: \
                     the system naturally creates {} behavior near surface, \
                     {} behavior in the deep. The gradient between them \
                     is the actual policy.",
                    self.position_a, self.position_b
                )
            }
            "Persistent Tension" => {
                format!(
                    "'{}' and '{}' remain in dynamic tension. \
                     The contested zone ({:.0}%) never resolves. \
                     This IS the answer: the oscillation itself \
                     is the stable state.",
                    self.position_a,
                    self.position_b,
                    self.contested_territory * 100.0
                )
            }
            _ => {
                format!(
                    "Equilibrium between '{}' and '{}' at boundary {:.2}.",
                    self.position_a, self.position_b, self.collision_boundary
                )
            }
        };
    }
}

/// Types of consensus ore that crystallize from contradictory vents.
/// Each type represents a different resolution pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsensusOreType {
    /// Both positions hold simultaneously (quantum superposition)
    /// "Privacy AND transparency, depending on context"
    Synthesis,
    /// A novel third position emerges that transcends both
    /// "Neither privacy nor transparency—radical trust"
    Transcendence,
    /// One position dissolves the other through superior coherence
    /// "Transparency wins because it's more robust"
    Dissolution,
    /// The contradiction itself becomes the stable insight
    /// "The tension IS the answer"
    Paradox,
    /// Both positions cancel out, revealing a deeper structure
    /// "Neither—the question was wrong"
    Nullification,
}

impl ConsensusOreType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ConsensusOreType::Synthesis => "synthesis",
            ConsensusOreType::Transcendence => "transcendence",
            ConsensusOreType::Dissolution => "dissolution",
            ConsensusOreType::Paradox => "paradox",
            ConsensusOreType::Nullification => "nullification",
        }
    }
}

/// Crystallized consensus from the reactor.
/// Represents stable insight extracted from contradictory inputs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusOre {
    /// Unique identifier
    pub id: Uuid,
    /// Descriptive name (e.g., "privacy_transparency_synthesis")
    pub name: String,
    /// What type of resolution occurred
    pub ore_type: ConsensusOreType,
    /// The first contradictory position
    pub vent_a: String,
    /// The second contradictory position
    pub vent_b: String,
    /// Certainty metric: C = 1 / (1 + ∫|Jitter|dt)
    /// C → 1 means "Foundational Truth"
    /// C → 0 means "Noise"
    pub certainty: f32,
    /// Total accumulated jitter during crystallization
    pub accumulated_jitter: f32,
    /// How many ticks it took to crystallize
    pub crystallization_time: u64,
    /// The emergent insight (if any)
    pub insight: Option<String>,
    /// Integration value for downstream processing
    pub integration_value: f32,
    /// The extracted phase structure (physical topology)
    /// This is the "new material" - not a compromise, but what survives
    pub phase_structure: Option<PhaseStructure>,
}

impl ConsensusOre {
    /// Is this a foundational truth? (C > 0.8)
    pub fn is_foundational(&self) -> bool {
        self.certainty > 0.8
    }

    /// Is this mostly noise? (C < 0.2)
    pub fn is_noise(&self) -> bool {
        self.certainty < 0.2
    }

    /// Quality classification
    pub fn quality(&self) -> &'static str {
        if self.certainty > 0.9 {
            "foundational_truth"
        } else if self.certainty > 0.7 {
            "strong_insight"
        } else if self.certainty > 0.5 {
            "tentative_insight"
        } else if self.certainty > 0.3 {
            "weak_signal"
        } else {
            "noise"
        }
    }
}

/// A contradictory vent pair injected into the reactor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContradictoryVent {
    /// Identifier for this vent
    pub id: Uuid,
    /// The position this vent represents
    pub position: String,
    /// Heat output (conviction strength)
    pub heat_output: f32,
    /// Current thermal energy
    pub energy: f32,
    /// Depth in the fluid (where it injects)
    pub depth: f32,
    /// Radius of influence
    pub radius: f32,
}

impl ContradictoryVent {
    pub fn new(position: String, heat_output: f32, depth: f32, radius: f32) -> Self {
        Self {
            id: Uuid::new_v4(),
            position,
            heat_output,
            energy: heat_output,
            depth,
            radius,
        }
    }
}

/// A consensus experiment tracking the collision of contradictory vents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusExperiment {
    /// Unique experiment ID
    pub id: Uuid,
    /// First contradictory vent
    pub vent_a: ContradictoryVent,
    /// Second contradictory vent
    pub vent_b: ContradictoryVent,
    /// Probe bubbles caught in the thermal collision
    pub probe_ids: Vec<ConceptId>,
    /// Accumulated jitter: ∫|Jitter|dt
    pub accumulated_jitter: f32,
    /// Peak jitter observed
    pub peak_jitter: f32,
    /// Velocity history for jitter calculation
    pub velocity_history: Vec<f32>,
    /// Tick when experiment started
    pub start_tick: u64,
    /// Has crystallization completed?
    pub crystallized: bool,
    /// Minimum ticks before considering crystallization
    pub min_crystallization_time: u64,
    /// Maximum ticks before forced crystallization
    pub max_crystallization_time: u64,
    /// Jitter threshold for "settled" state
    pub jitter_threshold: f32,
    /// Consecutive low-jitter ticks
    pub stable_ticks: u32,
    /// Required consecutive stable ticks to crystallize
    pub stability_requirement: u32,
    /// Phase transition threshold (jitter level that triggers freeze)
    pub phase_transition_threshold: f32,
    /// Has phase transition occurred?
    pub phase_transitioned: bool,
    /// The extracted phase structure (if transition occurred)
    pub phase_structure: Option<PhaseStructure>,
    /// Probe snapshots for phase extraction (depth, velocity pairs)
    pub probe_snapshots: Vec<(ConceptId, f32, f32)>,
}

impl ConsensusExperiment {
    pub fn new(position_a: String, heat_a: f32, position_b: String, heat_b: f32) -> Self {
        // Vents positioned at opposite sides of the reactor zone (0.4-0.6 depth)
        let vent_a = ContradictoryVent::new(position_a, heat_a, 0.4, 0.2);
        let vent_b = ContradictoryVent::new(position_b, heat_b, 0.6, 0.2);

        Self {
            id: Uuid::new_v4(),
            vent_a,
            vent_b,
            probe_ids: Vec::new(),
            accumulated_jitter: 0.0,
            peak_jitter: 0.0,
            velocity_history: Vec::with_capacity(120), // 2 seconds at 60Hz
            start_tick: 0,
            crystallized: false,
            min_crystallization_time: 60,  // Minimum 1 second
            max_crystallization_time: 600, // Maximum 10 seconds
            jitter_threshold: 0.02,
            stable_ticks: 0,
            stability_requirement: 30,        // Half second of stability
            phase_transition_threshold: 0.05, // Jitter below this triggers phase extraction
            phase_transitioned: false,
            phase_structure: None,
            probe_snapshots: Vec::new(),
        }
    }

    /// Record probe snapshot for phase extraction.
    pub fn record_probe_snapshot(&mut self, id: ConceptId, depth: f32, velocity: f32) {
        // Update or add snapshot
        if let Some(pos) = self
            .probe_snapshots
            .iter()
            .position(|(pid, _, _)| *pid == id)
        {
            self.probe_snapshots[pos] = (id, depth, velocity);
        } else {
            self.probe_snapshots.push((id, depth, velocity));
        }
    }

    /// Check if phase transition should occur.
    /// Returns true if jitter drops below threshold after initial turbulence.
    pub fn should_phase_transition(&self, current_tick: u64) -> bool {
        if self.phase_transitioned {
            return false;
        }

        let elapsed = current_tick.saturating_sub(self.start_tick);

        // Need some initial turbulence before we can detect settling
        if elapsed < self.min_crystallization_time / 2 {
            return false;
        }

        // Need accumulated jitter to have meaningful transition
        if self.accumulated_jitter < 0.1 {
            return false;
        }

        // Transition when current jitter drops significantly below peak
        let current = self.current_jitter();
        current < self.phase_transition_threshold && self.peak_jitter > 0.1
    }

    /// Extract the phase structure by freezing current probe states.
    pub fn extract_phase_structure(&mut self, current_tick: u64) -> PhaseStructure {
        self.phase_transitioned = true;

        // Sort probes by depth for Voronoi computation
        let mut sorted_probes: Vec<_> = self.probe_snapshots.clone();
        sorted_probes.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        // Compute Voronoi cells (1D tessellation)
        let mut voronoi_cells = Vec::new();
        let vent_a_depth = self.vent_a.depth;
        let vent_b_depth = self.vent_b.depth;
        let collision_center = (vent_a_depth + vent_b_depth) / 2.0;

        for (i, (id, depth, velocity)) in sorted_probes.iter().enumerate() {
            // Determine cell boundaries (midpoints to neighbors)
            let left_bound = if i == 0 {
                0.0
            } else {
                (sorted_probes[i - 1].1 + depth) / 2.0
            };
            let right_bound = if i == sorted_probes.len() - 1 {
                1.0
            } else {
                (depth + sorted_probes[i + 1].1) / 2.0
            };

            // Determine dominance based on position relative to vents
            let dominance = if *depth < vent_a_depth - 0.1 {
                VentDominance::Escaped
            } else if *depth < collision_center - 0.05 {
                VentDominance::VentA
            } else if *depth > vent_b_depth + 0.1 {
                VentDominance::Escaped
            } else if *depth > collision_center + 0.05 {
                VentDominance::VentB
            } else {
                VentDominance::Contested
            };

            voronoi_cells.push(VoronoiCell {
                owner_id: *id,
                center: *depth,
                left_bound,
                right_bound,
                width: right_bound - left_bound,
                dominance,
            });
        }

        // Compute territory fractions
        let mut vent_a_territory = 0.0f32;
        let mut vent_b_territory = 0.0f32;
        let mut contested_territory = 0.0f32;

        for cell in &voronoi_cells {
            match cell.dominance {
                VentDominance::VentA => vent_a_territory += cell.width,
                VentDominance::VentB => vent_b_territory += cell.width,
                VentDominance::Contested => contested_territory += cell.width,
                VentDominance::Escaped => {} // Not counted
            }
        }

        // Normalize (escaped territory isn't part of the policy space)
        let total = vent_a_territory + vent_b_territory + contested_territory;
        if total > 0.0 {
            vent_a_territory /= total;
            vent_b_territory /= total;
            contested_territory /= total;
        }

        // Find collision boundary (where territories meet)
        let collision_boundary = voronoi_cells
            .iter()
            .filter(|c| c.dominance == VentDominance::Contested)
            .map(|c| c.center)
            .sum::<f32>()
            / voronoi_cells
                .iter()
                .filter(|c| c.dominance == VentDominance::Contested)
                .count()
                .max(1) as f32;

        // Create frozen probes with computed properties
        let frozen_probes: Vec<FrozenProbe> = sorted_probes
            .iter()
            .enumerate()
            .map(|(i, (id, depth, velocity))| {
                // Find nearest neighbor distance
                let nearest_dist = if sorted_probes.len() > 1 {
                    let left_dist = if i > 0 {
                        (depth - sorted_probes[i - 1].1).abs()
                    } else {
                        f32::MAX
                    };
                    let right_dist = if i < sorted_probes.len() - 1 {
                        (sorted_probes[i + 1].1 - depth).abs()
                    } else {
                        f32::MAX
                    };
                    left_dist.min(right_dist)
                } else {
                    1.0
                };

                // Local density (inverse of average spacing)
                let local_density = if nearest_dist > 0.0 {
                    1.0 / nearest_dist
                } else {
                    10.0 // Very dense
                };

                // Determine dominance
                let dominance = if *depth < collision_center - 0.05 {
                    VentDominance::VentA
                } else if *depth > collision_center + 0.05 {
                    VentDominance::VentB
                } else {
                    VentDominance::Contested
                };

                FrozenProbe {
                    id: *id,
                    depth: *depth,
                    frozen_velocity: *velocity,
                    dominant_vent: dominance,
                    nearest_neighbor_dist: nearest_dist,
                    local_density,
                }
            })
            .collect();

        let mut structure = PhaseStructure {
            id: Uuid::new_v4(),
            transition_tick: current_tick,
            trigger_jitter: self.current_jitter(),
            frozen_probes,
            voronoi_cells,
            vent_a_territory,
            vent_b_territory,
            contested_territory,
            collision_boundary: if collision_boundary.is_nan() {
                collision_center
            } else {
                collision_boundary
            },
            emergent_properties: Vec::new(),
            material_name: String::new(),
            material_description: String::new(),
            position_a: self.vent_a.position.clone(),
            position_b: self.vent_b.position.clone(),
        };

        // Extract emergent properties and synthesize material
        structure.extract_emergent_properties();
        structure.synthesize_material();

        self.phase_structure = Some(structure.clone());
        structure
    }

    /// Record velocity for jitter calculation.
    pub fn record_velocity(&mut self, avg_velocity: f32) {
        self.velocity_history.push(avg_velocity);

        // Keep only last 120 samples (2 seconds at 60Hz)
        if self.velocity_history.len() > 120 {
            self.velocity_history.remove(0);
        }
    }

    /// Calculate current jitter as |dv/dt| (velocity derivative).
    pub fn current_jitter(&self) -> f32 {
        if self.velocity_history.len() < 2 {
            return 0.0;
        }

        let n = self.velocity_history.len();
        let v_curr = self.velocity_history[n - 1];
        let v_prev = self.velocity_history[n - 2];

        (v_curr - v_prev).abs()
    }

    /// Calculate total jitter integral: ∫|Jitter|dt
    pub fn jitter_integral(&self) -> f32 {
        if self.velocity_history.len() < 2 {
            return 0.0;
        }

        let dt = 1.0 / 60.0; // Assuming 60Hz
        let mut integral = 0.0;

        for i in 1..self.velocity_history.len() {
            let jitter = (self.velocity_history[i] - self.velocity_history[i - 1]).abs();
            integral += jitter * dt;
        }

        integral
    }

    /// Calculate certainty: C = 1 / (1 + ∫|Jitter|dt)
    pub fn certainty(&self) -> f32 {
        1.0 / (1.0 + self.accumulated_jitter)
    }

    /// Determine ore type based on crystallization dynamics.
    pub fn determine_ore_type(&self) -> ConsensusOreType {
        let certainty = self.certainty();
        let jitter_ratio = if self.accumulated_jitter > 0.0 {
            self.peak_jitter / self.accumulated_jitter
        } else {
            0.0
        };

        // Analyze the crystallization pattern
        let heat_ratio = self.vent_a.heat_output / self.vent_b.heat_output.max(0.001);
        let heat_imbalance = (heat_ratio - 1.0).abs();

        if certainty > 0.9 && jitter_ratio < 0.1 {
            // Very stable, smooth convergence → both positions merge
            ConsensusOreType::Synthesis
        } else if certainty > 0.7 && heat_imbalance < 0.3 {
            // Stable but with tension → transcends both positions
            ConsensusOreType::Transcendence
        } else if certainty > 0.5 && heat_imbalance > 0.5 {
            // One vent dominated → stronger position wins
            ConsensusOreType::Dissolution
        } else if certainty < 0.3 && self.peak_jitter > 0.5 {
            // Very low certainty, high chaos → nullification
            ConsensusOreType::Nullification
        } else {
            // Persistent oscillation → the paradox IS the answer
            ConsensusOreType::Paradox
        }
    }

    /// Generate insight based on ore type.
    pub fn generate_insight(&self, ore_type: ConsensusOreType) -> String {
        let a = &self.vent_a.position;
        let b = &self.vent_b.position;

        match ore_type {
            ConsensusOreType::Synthesis => {
                format!(
                    "Both '{}' and '{}' hold: context determines which applies",
                    a, b
                )
            }
            ConsensusOreType::Transcendence => {
                format!(
                    "Beyond '{}' vs '{}': a third way emerges from their collision",
                    a, b
                )
            }
            ConsensusOreType::Dissolution => {
                let winner = if self.vent_a.heat_output > self.vent_b.heat_output {
                    a
                } else {
                    b
                };
                format!(
                    "'{}' dissolves opposition through superior coherence",
                    winner
                )
            }
            ConsensusOreType::Paradox => {
                format!("The tension between '{}' and '{}' IS the insight", a, b)
            }
            ConsensusOreType::Nullification => {
                format!(
                    "'{}' vs '{}' reveals a false dichotomy—the question dissolves",
                    a, b
                )
            }
        }
    }

    /// Check if ready to crystallize.
    pub fn check_crystallization(&mut self, current_tick: u64) -> bool {
        let elapsed = current_tick.saturating_sub(self.start_tick);

        // Too early
        if elapsed < self.min_crystallization_time {
            return false;
        }

        // Timeout - force crystallization
        if elapsed >= self.max_crystallization_time {
            self.crystallized = true;
            return true;
        }

        // Check stability
        let current_jitter = self.current_jitter();
        if current_jitter < self.jitter_threshold {
            self.stable_ticks += 1;
        } else {
            self.stable_ticks = 0;
        }

        // Stable long enough
        if self.stable_ticks >= self.stability_requirement {
            self.crystallized = true;
            return true;
        }

        false
    }

    /// Finalize and create the consensus ore.
    pub fn crystallize(&self, current_tick: u64) -> ConsensusOre {
        let ore_type = self.determine_ore_type();
        let certainty = self.certainty();

        // Use phase structure material name if available, otherwise generate insight
        let (insight, name) = if let Some(ref structure) = self.phase_structure {
            (
                Some(structure.material_description.clone()),
                format!(
                    "{}_{}_{}",
                    self.vent_a.position.replace(' ', "_").to_lowercase(),
                    self.vent_b.position.replace(' ', "_").to_lowercase(),
                    structure.material_name.replace(' ', "_").to_lowercase()
                ),
            )
        } else {
            (
                Some(self.generate_insight(ore_type)),
                format!(
                    "{}_{}_{}",
                    self.vent_a.position.replace(' ', "_").to_lowercase(),
                    self.vent_b.position.replace(' ', "_").to_lowercase(),
                    ore_type.as_str()
                ),
            )
        };

        ConsensusOre {
            id: Uuid::new_v4(),
            name,
            ore_type,
            vent_a: self.vent_a.position.clone(),
            vent_b: self.vent_b.position.clone(),
            certainty,
            accumulated_jitter: self.accumulated_jitter,
            crystallization_time: current_tick.saturating_sub(self.start_tick),
            insight,
            integration_value: certainty * 2.0, // Higher certainty = more valuable
            phase_structure: self.phase_structure.clone(),
        }
    }

    /// Calculate thermal collision force at a given depth.
    /// Returns (net_force, collision_intensity)
    pub fn thermal_collision_at(&self, depth: f32) -> (f32, f32) {
        let force_a = self.vent_a.force_at(depth);
        let force_b = self.vent_b.force_at(depth);

        // Net force (where they balance = the "collision zone")
        let net_force = force_a + force_b;

        // Collision intensity (where both are strong = maximum interference)
        let collision = force_a.abs() * force_b.abs();

        (net_force, collision)
    }
}

impl ContradictoryVent {
    /// Calculate thermal force at a given depth.
    /// Positive = push down, Negative = push up
    pub fn force_at(&self, depth: f32) -> f32 {
        let diff = depth - self.depth;
        let dist = diff.abs();

        if dist > self.radius {
            return 0.0;
        }

        let proximity = 1.0 - (dist / self.radius);
        let magnitude = self.heat_output * proximity.powi(2);

        // Push away from vent center
        if diff > 0.0 {
            magnitude // Push down (concept is below vent)
        } else {
            -magnitude // Push up (concept is above vent)
        }
    }
}

/// The Consensus Reactor - extracts stable truths from contradictory inputs.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConsensusReactor {
    /// Currently active experiment
    pub active_experiment: Option<ConsensusExperiment>,
    /// Crystallized consensus ores
    pub ore_deposits: Vec<ConsensusOre>,
    /// Completed experiment results
    pub experiment_history: Vec<ConsensusOre>,
    /// Total experiments run
    pub total_experiments: u32,
}

impl ConsensusReactor {
    pub fn new() -> Self {
        Self::default()
    }

    /// Start a new consensus experiment with two contradictory positions.
    pub fn start_experiment(
        &mut self,
        position_a: String,
        heat_a: f32,
        position_b: String,
        heat_b: f32,
        current_tick: u64,
    ) -> Uuid {
        let mut experiment = ConsensusExperiment::new(position_a, heat_a, position_b, heat_b);
        experiment.start_tick = current_tick;
        let id = experiment.id;

        self.active_experiment = Some(experiment);
        self.total_experiments += 1;

        id
    }

    /// Get the active experiment (if any).
    pub fn get_experiment(&self) -> Option<&ConsensusExperiment> {
        self.active_experiment.as_ref()
    }

    /// Get mutable reference to active experiment.
    pub fn get_experiment_mut(&mut self) -> Option<&mut ConsensusExperiment> {
        self.active_experiment.as_mut()
    }

    /// Update the experiment and check for crystallization.
    /// Returns Some(ConsensusOre) if crystallization occurred.
    pub fn update(&mut self, current_tick: u64) -> Option<ConsensusOre> {
        let experiment = self.active_experiment.as_mut()?;

        // Accumulate jitter
        let current_jitter = experiment.current_jitter();
        experiment.accumulated_jitter += current_jitter;
        experiment.peak_jitter = experiment.peak_jitter.max(current_jitter);

        // Check for crystallization
        if experiment.check_crystallization(current_tick) {
            let ore = experiment.crystallize(current_tick);
            self.ore_deposits.push(ore.clone());
            self.experiment_history.push(ore.clone());
            self.active_experiment = None;
            return Some(ore);
        }

        None
    }

    /// Get all foundational truths (C > 0.8).
    pub fn foundational_truths(&self) -> Vec<&ConsensusOre> {
        self.ore_deposits
            .iter()
            .filter(|o| o.is_foundational())
            .collect()
    }

    /// Get average certainty of all deposits.
    pub fn average_certainty(&self) -> f32 {
        if self.ore_deposits.is_empty() {
            return 0.0;
        }
        let sum: f32 = self.ore_deposits.iter().map(|o| o.certainty).sum();
        sum / self.ore_deposits.len() as f32
    }

    /// Clear all noise (C < 0.2) from deposits.
    pub fn clear_noise(&mut self) -> usize {
        let before = self.ore_deposits.len();
        self.ore_deposits.retain(|o| !o.is_noise());
        before - self.ore_deposits.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_certainty_calculation() {
        let mut exp = ConsensusExperiment::new(
            "Privacy is absolute".to_string(),
            1.0,
            "Transparency is mandatory".to_string(),
            1.0,
        );

        // No jitter → C = 1
        assert!((exp.certainty() - 1.0).abs() < 0.001);

        // Some jitter
        exp.accumulated_jitter = 1.0;
        assert!((exp.certainty() - 0.5).abs() < 0.001);

        // Lots of jitter
        exp.accumulated_jitter = 9.0;
        assert!((exp.certainty() - 0.1).abs() < 0.001);
    }

    #[test]
    fn test_ore_quality_classification() {
        let make_ore = |certainty: f32| ConsensusOre {
            id: Uuid::new_v4(),
            name: "test".to_string(),
            ore_type: ConsensusOreType::Synthesis,
            vent_a: "A".to_string(),
            vent_b: "B".to_string(),
            certainty,
            accumulated_jitter: 0.0,
            crystallization_time: 0,
            insight: None,
            integration_value: 0.0,
        };

        assert_eq!(make_ore(0.95).quality(), "foundational_truth");
        assert_eq!(make_ore(0.75).quality(), "strong_insight");
        assert_eq!(make_ore(0.55).quality(), "tentative_insight");
        assert_eq!(make_ore(0.35).quality(), "weak_signal");
        assert_eq!(make_ore(0.15).quality(), "noise");
    }

    #[test]
    fn test_thermal_collision() {
        let exp = ConsensusExperiment::new("A".to_string(), 1.0, "B".to_string(), 1.0);

        // At collision center (0.5), both vents exert force
        let (net, collision) = exp.thermal_collision_at(0.5);

        // Should be balanced (net ≈ 0) but high collision
        assert!(net.abs() < 0.1);
        assert!(collision > 0.0);
    }
}
