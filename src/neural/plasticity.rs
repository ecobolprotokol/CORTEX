use crate::types::scalars::Scalar;

pub fn compute_weight_update(
    current_weight: Scalar,
    activation: Scalar,
    confidence: Scalar,
    error: Scalar,
    learning_rate: Scalar,
    plasticity: Scalar,
) -> Scalar {
    let delta = learning_rate * activation * confidence * error * plasticity;
    (current_weight + delta).clamp(-1.0, 1.0)
}

pub fn enforce_stability(delta: Scalar, max_change: Scalar) -> Scalar {
    delta.clamp(-max_change, max_change)
}
