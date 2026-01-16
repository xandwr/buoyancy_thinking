use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A standing wave creates acoustic nodes at regular intervals.
/// Bubbles naturally settle into nodes when the system is divisible.
///
/// Enhanced with:
/// - Pauli Exclusion: Nodes have saturation limits
/// - Breathing: Time-varying amplitude keeps the system alive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandingWave {
    /// Unique identifier
    pub id: Uuid,
    /// The divisor - determines node spacing
    pub frequency: f32,
    /// Physical distance between nodes
    pub node_spacing: f32,
    /// Base amplitude of the wave (force strength)
    pub amplitude: f32,
    /// Positions of nodes (stable points)
    pub node_positions: Vec<f32>,
    /// Is this wave currently active?
    pub active: bool,

    // === Pauli Exclusion (Node Saturation) ===
    /// Maximum bubbles per node before repulsion kicks in
    pub saturation_limit: u32,
    /// Current occupancy of each node
    pub node_occupancy: Vec<u32>,

    // === Breathing Wave ===
    /// Enable time-varying amplitude (respiratory cycle)
    pub breathing_enabled: bool,
    /// Breathing frequency (radians per tick)
    pub breathing_omega: f32,
    /// Current phase of the breathing cycle
    pub breathing_phase: f32,
    /// Breathing depth (0.0 = no variation, 1.0 = full 0-2x amplitude swing)
    pub breathing_depth: f32,
}

impl StandingWave {
    /// Create a new standing wave with the given divisor.
    /// Nodes are created at regular intervals throughout the fluid depth.
    pub fn new(divisor: f32, amplitude: f32) -> Self {
        let node_spacing = 1.0 / divisor; // Normalize to 0-1 depth range
        let mut node_positions = Vec::new();

        // Create nodes from surface to floor
        let mut depth = node_spacing / 2.0; // Start at first node
        while depth < 1.0 {
            node_positions.push(depth);
            depth += node_spacing;
        }

        let node_count = node_positions.len();

        Self {
            id: Uuid::new_v4(),
            frequency: divisor,
            node_spacing,
            amplitude,
            node_positions,
            active: true,
            // Pauli Exclusion: each node can hold (dividend/divisor) bubbles
            // We'll set this dynamically when starting experiment
            saturation_limit: 2, // Default: 2 bubbles per node
            node_occupancy: vec![0; node_count],
            // Breathing: active respiratory cycle to prevent quick settlement
            breathing_enabled: true,
            breathing_omega: 0.15, // Faster cycle (~0.7 seconds) keeps system alive
            breathing_phase: 0.0,
            breathing_depth: 0.5, // 50% amplitude variation - more dramatic pulsing
        }
    }

    /// Create with specific saturation limit (for division experiments)
    pub fn new_with_saturation(divisor: f32, amplitude: f32, saturation_limit: u32) -> Self {
        let mut wave = Self::new(divisor, amplitude);
        wave.saturation_limit = saturation_limit;
        wave
    }

    /// Advance the breathing cycle by one tick
    pub fn tick(&mut self) {
        if self.breathing_enabled {
            self.breathing_phase += self.breathing_omega;
            if self.breathing_phase > std::f32::consts::TAU {
                self.breathing_phase -= std::f32::consts::TAU;
            }
        }
    }

    /// Get current effective amplitude (with breathing modulation)
    pub fn effective_amplitude(&self) -> f32 {
        if self.breathing_enabled {
            // A(t) = A_base × (1.0 + depth × sin(ωt))
            self.amplitude * (1.0 + self.breathing_depth * self.breathing_phase.sin())
        } else {
            self.amplitude
        }
    }

    /// Reset node occupancy counts
    pub fn reset_occupancy(&mut self) {
        for occ in &mut self.node_occupancy {
            *occ = 0;
        }
    }

    /// Update occupancy based on bubble positions
    pub fn update_occupancy(&mut self, bubble_depths: &[f32]) {
        self.reset_occupancy();
        let tolerance = self.node_spacing / 2.0;

        for &depth in bubble_depths {
            if let Some(node_idx) = self.nearest_node_index(depth) {
                let node_pos = self.node_positions[node_idx];
                if (depth - node_pos).abs() < tolerance {
                    self.node_occupancy[node_idx] += 1;
                }
            }
        }
    }

    /// Find the index of the nearest node to a given depth
    pub fn nearest_node_index(&self, depth: f32) -> Option<usize> {
        self.node_positions
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                (depth - *a)
                    .abs()
                    .partial_cmp(&(depth - *b).abs())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(idx, _)| idx)
    }

    /// Calculate the force on a bubble at a given depth.
    /// Now includes:
    /// - Breathing modulation (time-varying amplitude)
    /// - Pauli Exclusion (saturated nodes repel)
    /// - Depth-Compensated Attraction (shallow nodes boosted to overcome buoyancy)
    pub fn force_at_depth(&self, depth: f32) -> f32 {
        if !self.active || self.node_positions.is_empty() {
            return 0.0;
        }

        let effective_amp = self.effective_amplitude();

        // Find nearest node and its index
        let (nearest_idx, nearest_node) = self
            .node_positions
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                (depth - *a)
                    .abs()
                    .partial_cmp(&(depth - *b).abs())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(idx, &pos)| (idx, pos))
            .unwrap_or((0, depth));

        let displacement = nearest_node - depth;

        // === Depth-Compensated Attraction ===
        // Shallow nodes need slight boost to overcome buoyancy, but not too much
        // or they'll over-attract. Let LJ repulsion handle distribution.
        // At depth 0.1, boost = 1.6; at depth 0.9, boost = 1.1
        let depth_compensation = 1.0 + 0.6 * (1.0 - nearest_node).max(0.0);

        // Check if nearest node is saturated (Pauli Exclusion)
        let node_occ = self.node_occupancy.get(nearest_idx).copied().unwrap_or(0);

        if node_occ >= self.saturation_limit {
            // Node is FULL - flip to repulsion!
            // The harder you try to enter, the harder you're pushed out
            let repulsion_strength = 10.0; // Very strong repulsion from full nodes
            -displacement * effective_amp * repulsion_strength * depth_compensation
        } else {
            // Node has room - attract with depth compensation
            displacement * effective_amp * depth_compensation
        }
    }

    /// Check if a depth is at a node (within tolerance).
    pub fn is_at_node(&self, depth: f32, tolerance: f32) -> bool {
        self.node_positions
            .iter()
            .any(|node| (depth - node).abs() < tolerance)
    }

    /// Get the number of nodes.
    pub fn node_count(&self) -> usize {
        self.node_positions.len()
    }

    /// Check if any node is over-saturated (remainder condition)
    pub fn has_overflow(&self) -> bool {
        self.node_occupancy
            .iter()
            .any(|&occ| occ > self.saturation_limit)
    }

    /// Count bubbles that couldn't find a home (remainder detection)
    pub fn homeless_count(&self) -> u32 {
        self.node_occupancy
            .iter()
            .filter(|&&occ| occ > self.saturation_limit)
            .map(|&occ| occ - self.saturation_limit)
            .sum()
    }
}

/// A division problem encoded as fluid dynamics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DivisionProblem {
    /// The dividend - number of bubbles to inject
    pub dividend: f32,
    /// The divisor - acoustic frequency creating nodes
    pub divisor: f32,
    /// Unique identifier for this problem
    pub id: Uuid,
}

impl DivisionProblem {
    pub fn new(dividend: f32, divisor: f32) -> Self {
        Self {
            dividend,
            divisor,
            id: Uuid::new_v4(),
        }
    }
}

/// Result of a division computation via fluid dynamics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DivisionResult {
    /// The original problem
    pub dividend: f32,
    pub divisor: f32,
    /// Was the division clean (no remainder)?
    pub is_divisible: bool,
    /// The quotient (bubbles per node when stable)
    pub quotient: f32,
    /// The remainder (derived from turbulence energy)
    pub remainder: f32,
    /// Reynolds number at settlement
    pub reynolds_number: f32,
    /// Total turbulence energy (chaos indicator)
    pub turbulence_energy: f32,
    /// How many ticks until settlement
    pub ticks_to_settle: u64,
    /// Distribution of bubbles across nodes
    pub node_occupancy: Vec<u32>,
    /// Salinity boost used for Laminar Streamlining (0.0 = none)
    pub salinity_boost: f32,
    /// Velocity standard deviation - "arrival jitter" / micro-cavitation detector
    pub velocity_sigma: f32,
    /// Mean velocity over measurement window
    pub velocity_mean: f32,
    /// Peak jitter observed during settling (captures transient micro-cavitation)
    /// This is the key remainder detection metric!
    pub peak_jitter: f32,
}

/// Tracks the state of an active division experiment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DivisionExperiment {
    /// Problem being solved
    pub problem: DivisionProblem,
    /// Standing wave for this experiment
    pub wave: StandingWave,
    /// IDs of bubbles injected for this experiment
    pub bubble_ids: Vec<Uuid>,
    /// Has the experiment settled?
    pub settled: bool,
    /// Tick count when experiment started
    pub start_tick: u64,
    /// Maximum ticks before forced settlement
    pub max_ticks: u64,
    /// Accumulated turbulence over the experiment
    pub accumulated_turbulence: f32,
    /// Peak Reynolds number observed
    pub peak_reynolds: f32,
    /// Original salinity before experiment (for restoration)
    pub original_salinity: f32,
    /// Salinity boost applied for Laminar Streamlining
    pub salinity_boost: f32,
    /// Velocity history for jitter detection (last N ticks of avg velocity)
    /// Used to calculate velocity standard deviation (vσ)
    pub velocity_history: Vec<f32>,
    /// Window size for jitter measurement
    pub jitter_window: usize,
    /// Peak jitter observed during experiment (captures transient micro-cavitation)
    pub peak_jitter: f32,
    /// Running sum for incremental std dev calculation
    pub velocity_sum: f32,
    pub velocity_sum_sq: f32,
    pub velocity_samples: u32,
}

impl DivisionExperiment {
    pub fn new(problem: DivisionProblem, start_tick: u64) -> Self {
        let wave = StandingWave::new(problem.divisor, 5.0); // Strong wave amplitude

        Self {
            problem,
            wave,
            bubble_ids: Vec::new(),
            settled: false,
            start_tick,
            max_ticks: 300, // 5 seconds at 60Hz
            accumulated_turbulence: 0.0,
            peak_reynolds: 0.0,
            original_salinity: 0.0,
            salinity_boost: 0.0,
            velocity_history: Vec::with_capacity(50),
            jitter_window: 50, // Last 50 ticks for jitter measurement
            peak_jitter: 0.0,
            velocity_sum: 0.0,
            velocity_sum_sq: 0.0,
            velocity_samples: 0,
        }
    }

    /// Record velocity sample for jitter analysis.
    /// Maintains a rolling window of the last `jitter_window` samples.
    /// Also tracks peak jitter for detecting transient micro-cavitation.
    pub fn record_velocity(&mut self, avg_velocity: f32) {
        // Rolling window for final jitter calculation
        if self.velocity_history.len() >= self.jitter_window {
            self.velocity_history.remove(0);
        }
        self.velocity_history.push(avg_velocity);

        // Running statistics for peak jitter detection
        self.velocity_sum += avg_velocity;
        self.velocity_sum_sq += avg_velocity * avg_velocity;
        self.velocity_samples += 1;

        // Calculate current window jitter and track peak
        if self.velocity_history.len() >= 10 {
            let (_, current_sigma) = self.calculate_velocity_sigma();
            if current_sigma > self.peak_jitter {
                self.peak_jitter = current_sigma;
            }
        }
    }

    /// Calculate velocity standard deviation (vσ) - the "jitter" metric.
    /// High vσ indicates micro-cavitation / stuttering from remainder bubbles.
    /// Low vσ indicates laminar, predictable flow (divisible case).
    pub fn calculate_velocity_sigma(&self) -> (f32, f32) {
        if self.velocity_history.is_empty() {
            return (0.0, 0.0);
        }

        let n = self.velocity_history.len() as f32;
        let mean = self.velocity_history.iter().sum::<f32>() / n;

        let variance = self
            .velocity_history
            .iter()
            .map(|v| (v - mean).powi(2))
            .sum::<f32>()
            / n;

        let sigma = variance.sqrt();
        (mean, sigma)
    }

    /// Check if experiment has timed out.
    pub fn is_timed_out(&self, current_tick: u64) -> bool {
        current_tick - self.start_tick >= self.max_ticks
    }

    /// Calculate the remainder from accumulated turbulence.
    /// The key insight: turbulence energy correlates with the remainder!
    pub fn calculate_remainder(&self) -> f32 {
        let expected_remainder = self.problem.dividend % self.problem.divisor;

        // Turbulence-based remainder estimation
        // When bubbles can't fit evenly into nodes, they jostle → turbulence
        // More leftover bubbles = more turbulence
        let turbulence_remainder = self.accumulated_turbulence / 10.0; // Scale factor

        // The actual remainder should emerge from the physics
        // But we can cross-reference with mathematical remainder
        turbulence_remainder.round().min(self.problem.divisor - 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standing_wave_nodes() {
        let wave = StandingWave::new(3.0, 1.0);
        // With divisor 3, expect ~3 nodes in 0-1 range
        assert!(wave.node_count() >= 2 && wave.node_count() <= 4);
    }

    #[test]
    fn test_force_toward_node() {
        let wave = StandingWave::new(2.0, 1.0);
        // At a node, force should be near zero
        if let Some(&node) = wave.node_positions.first() {
            let force = wave.force_at_depth(node);
            assert!(force.abs() < 0.01);
        }
    }

    #[test]
    fn test_pauli_exclusion() {
        // Create wave with saturation limit of 2 (like 6÷3=2)
        let mut wave = StandingWave::new_with_saturation(3.0, 1.0, 2);

        // Initially, nodes should attract
        let node = wave.node_positions[0];
        let force_before = wave.force_at_depth(node + 0.05);
        assert!(force_before > 0.0, "Should attract toward node");

        // Saturate the first node
        wave.node_occupancy[0] = 2;

        // Now the same position should be repelled
        let force_after = wave.force_at_depth(node + 0.05);
        assert!(force_after < 0.0, "Should repel from saturated node");
    }

    #[test]
    fn test_breathing_wave() {
        let mut wave = StandingWave::new(2.0, 1.0);
        wave.breathing_enabled = true;
        wave.breathing_depth = 0.5;

        let amp1 = wave.effective_amplitude();

        // Advance phase by π/2 (quarter cycle)
        wave.breathing_phase = std::f32::consts::FRAC_PI_2;
        let amp2 = wave.effective_amplitude();

        // At phase π/2, sin = 1, so amplitude should be higher
        assert!(amp2 > amp1, "Amplitude should vary with breathing");
    }
}
