use crate::types::scalars::Scalar;

pub struct PlasticityRule {
    pub learning_rate: Scalar,
    pub plasticity_bound: Scalar,
}

impl PlasticityRule {
    pub fn new(learning_rate: Scalar, plasticity_bound: Scalar) -> Self {
        Self {
            learning_rate,
            plasticity_bound,
        }
    }

    /// Compute weight update: ΔW = η × A × C × E × V
    pub fn compute_update(
        &self,
        activation: Scalar,
        confidence: Scalar,
        error: Scalar,
        voltage: Scalar,
    ) -> Scalar {
        let delta = self.learning_rate * activation * confidence * error * voltage;
        delta.clamp(-self.plasticity_bound, self.plasticity_bound)
    }
}
