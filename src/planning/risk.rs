use crate::types::scalars::Scalar;

#[derive(Debug, Clone)]
pub struct RiskAssessment {
    pub score: Scalar,
    pub level: String,
    pub factors: Vec<String>,
}

pub struct RiskEvaluator;

impl RiskEvaluator {
    pub fn new() -> Self { Self }

    pub fn evaluate(&self, plan_cost: Scalar, uncertainty: Scalar) -> RiskAssessment {
        let score = (plan_cost * 0.5 + uncertainty * 0.5).min(1.0);
        let level = if score < 0.3 { "Low" }
            else if score < 0.6 { "Moderate" }
            else if score < 0.8 { "High" }
            else { "Critical" };

        RiskAssessment {
            score,
            level: level.to_string(),
            factors: vec![
                format!("Cost: {:.2}", plan_cost),
                format!("Uncertainty: {:.2}", uncertainty),
            ],
        }
    }
}
