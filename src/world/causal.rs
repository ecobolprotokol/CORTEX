use crate::types::*;

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
