use crate::types::scalars::Scalar;

pub struct StabilityGuard {
    pub max_change_per_observation: Scalar,
    pub plasticity_bound: Scalar,
}

impl StabilityGuard {
    pub fn new(max_change: Scalar, plasticity_bound: Scalar) -> Self {
        Self {
            max_change_per_observation: max_change,
            plasticity_bound,
        }
    }

    pub fn check_stability(&self, state_change: Scalar) -> bool {
        state_change <= self.max_change_per_observation
    }

    pub fn clamp_update(&self, delta: Scalar) -> Scalar {
        delta.clamp(-self.plasticity_bound, self.plasticity_bound)
    }
}
