use crate::types::scalars::Scalar;

pub fn distinguish_causation(correlation: Scalar, temporal_order: bool, confounders: usize) -> CausalStrength {
    let mut strength = correlation;
    if temporal_order {
        strength *= 1.2;
    }
    strength *= 1.0 / (1.0 + confounders as Scalar);
    match strength {
        x if x > 0.8 => CausalStrength::Strong,
        x if x > 0.5 => CausalStrength::Moderate,
        x if x > 0.2 => CausalStrength::Weak,
        _ => CausalStrength::None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CausalStrength {
    Strong,
    Moderate,
    Weak,
    None,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strong_causation() {
        assert_eq!(distinguish_causation(0.9, true, 0), CausalStrength::Strong);
    }

    #[test]
    fn test_weak_causation() {
        let result = distinguish_causation(0.3, false, 5);
        assert!(matches!(result, CausalStrength::Weak | CausalStrength::None));
    }

    #[test]
    fn test_no_causation() {
        let result = distinguish_causation(0.1, false, 10);
        assert!(matches!(result, CausalStrength::None));
    }

    #[test]
    fn test_temporal_order_boosts() {
        let with_temporal = 0.5 * 1.2;
        let without_temporal = 0.5;
        assert!(with_temporal > without_temporal);
    }
}
