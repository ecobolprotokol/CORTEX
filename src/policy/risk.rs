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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate_risk_low() {
        let risk = estimate_risk(0.2, 0.9);
        assert!(risk.score < 0.1);
    }

    #[test]
    fn test_estimate_risk_high() {
        let risk = estimate_risk(0.9, 0.1);
        assert!(risk.score > 0.5);
    }
}
