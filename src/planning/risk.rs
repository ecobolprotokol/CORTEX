use crate::types::*;

pub fn evaluate_risk(plan: &Plan, _world: &WorldState) -> RiskAssessment {
    let mut factors = Vec::new();
    let reversibility = 0.8;
    factors.push(RiskFactor {
        description: "Reversibility".into(),
        severity: 1.0 - reversibility,
        likelihood: 0.5,
    });
    factors.push(RiskFactor {
        description: "Uncertainty".into(),
        severity: plan.uncertainty,
        likelihood: 1.0,
    });
    let score = (1.0 - reversibility) * 0.3 + plan.uncertainty * 0.5;
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
        factors,
        reversibility,
    }
}
