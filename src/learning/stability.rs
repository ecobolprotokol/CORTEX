use crate::types::scalars::Scalar;

#[derive(Debug, Clone)]
pub struct StabilityMetrics {
    pub state_change_magnitude: Scalar,
    pub within_bounds: bool,
    pub clamped: bool,
    pub original_delta: Scalar,
    pub clamped_delta: Scalar,
}

pub struct StabilityGuard {
    pub max_change_per_observation: Scalar,
    pub plasticity_bound: Scalar,
    pub max_cumulative_change: Scalar,
    pub window_size: u32,
    pub recent_changes: Vec<Scalar>,
}

impl StabilityGuard {
    pub fn new(max_change: Scalar, plasticity_bound: Scalar) -> Self {
        Self {
            max_change_per_observation: max_change,
            plasticity_bound,
            max_cumulative_change: max_change * 10.0,
            window_size: 100,
            recent_changes: Vec::new(),
        }
    }

    pub fn check_stability(&self, state_change: Scalar) -> bool {
        state_change <= self.max_change_per_observation
    }

    pub fn clamp_update(&self, delta: Scalar) -> Scalar {
        delta.clamp(-self.plasticity_bound, self.plasticity_bound)
    }

    pub fn apply_with_stability(&mut self, delta: Scalar) -> StabilityMetrics {
        let original = delta;
        let clamped = self.clamp_update(delta);

        let window_avg = if self.recent_changes.is_empty() {
            0.0
        } else {
            self.recent_changes.iter().sum::<Scalar>() / self.recent_changes.len() as Scalar
        };

        let cumulative = window_avg + clamped.abs();
        let within_bounds = cumulative <= self.max_cumulative_change;

        let final_delta = if within_bounds {
            clamped
        } else {
            let scale =
                self.max_cumulative_change / cumulative.max(crate::types::scalars::SCALAR_EPSILON);
            clamped * scale
        };

        self.recent_changes.push(final_delta.abs());
        if self.recent_changes.len() > self.window_size as usize {
            self.recent_changes.remove(0);
        }

        StabilityMetrics {
            state_change_magnitude: final_delta.abs(),
            within_bounds,
            clamped: (original - final_delta).abs() > crate::types::scalars::SCALAR_EPSILON,
            original_delta: original,
            clamped_delta: final_delta,
        }
    }

    pub fn compute_effective_learning_rate(
        &self,
        base_rate: Scalar,
        recent_error: Scalar,
    ) -> Scalar {
        let error_factor = if recent_error > 0.5 {
            1.2
        } else if recent_error < 0.1 {
            0.8
        } else {
            1.0
        };

        let stability_factor = if self.recent_changes.len() > 10 {
            let avg =
                self.recent_changes.iter().sum::<Scalar>() / self.recent_changes.len() as Scalar;
            if avg > self.max_change_per_observation * 0.8 {
                0.7
            } else {
                1.0
            }
        } else {
            1.0
        };

        (base_rate * error_factor * stability_factor).clamp(0.0001, 0.1)
    }

    pub fn should_consolidate(&self, error_threshold: Scalar) -> bool {
        if self.recent_changes.is_empty() {
            return false;
        }

        let recent_avg = self.recent_changes.iter().rev().take(10).sum::<Scalar>()
            / self.recent_changes.len().min(10) as Scalar;

        recent_avg < error_threshold * 0.5
    }

    pub fn reset_window(&mut self) {
        self.recent_changes.clear();
    }
}
