use crate::types::*;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weight_update_basic() {
        let w = compute_weight_update(0.5, 1.0, 1.0, 0.1, 0.01, 1.0);
        assert!(w > 0.5);
        assert!(w <= 1.0);
    }

    #[test]
    fn test_weight_update_bounds() {
        let w = compute_weight_update(0.99, 1.0, 1.0, 1.0, 1.0, 1.0);
        assert!(w <= 1.0);
        let w2 = compute_weight_update(-0.99, 1.0, 1.0, -1.0, 1.0, 1.0);
        assert!(w2 >= -1.0);
    }

    #[test]
    fn test_enforce_stability() {
        assert_eq!(enforce_stability(0.5, 0.3), 0.3);
        assert_eq!(enforce_stability(-0.5, 0.3), -0.3);
        assert_eq!(enforce_stability(0.1, 0.3), 0.1);
    }
}
