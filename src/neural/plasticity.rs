pub struct PlasticityRule {
    pub learning_rate: f32,
    pub plasticity_bound: f32,
}

impl PlasticityRule {
    pub fn new(learning_rate: f32, plasticity_bound: f32) -> Self {
        Self {
            learning_rate,
            plasticity_bound,
        }
    }

    /// Compute weight update: ΔW = η × A × C × E × V
    pub fn compute_update(
        &self,
        activation: f32,
        confidence: f32,
        error: f32,
        voltage: f32,
    ) -> f32 {
        let delta = self.learning_rate * activation * confidence * error * voltage;
        delta.clamp(-self.plasticity_bound, self.plasticity_bound)
    }

    pub fn apply_update(&self, weight: f32, activation: f32, confidence: f32, error: f32, voltage: f32) -> f32 {
        let delta = self.compute_update(activation, confidence, error, voltage);
        (weight + delta).clamp(-1.0, 1.0)
    }
}
