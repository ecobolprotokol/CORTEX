use crate::types::*;

pub fn estimate_risk(impact: Scalar, reversibility: Scalar) -> RiskAssessment {
    let score = impact * (1.0 - reversibility);
    let level = match score {
        x if x > 0.8 => RiskLevel::Critical,
        x if x > 0.6 => RiskLevel::High,
        x if x > 0.3 => RiskLevel::Moderate,
        x if x > 0.0 => RiskLevel::Low,
        _ => RiskLevel::None,
    };
    RiskAssessment {
        score,
        level,
        factors: Vec::new(),
        reversibility,
    }
}
