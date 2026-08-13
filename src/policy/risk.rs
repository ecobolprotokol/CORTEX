use crate::types::scalars::Scalar;

#[derive(Debug, Clone)]
pub struct RiskEstimate {
    pub score: Scalar,
    pub level: String,
    pub factors: Vec<String>,
}

pub struct RiskEstimator;

impl RiskEstimator {
    pub fn new() -> Self { Self }

    pub fn estimate(&self, operation: &str, impact: Scalar, reversibility: Scalar) -> RiskEstimate {
        let score = impact * (1.0 - reversibility);
        let level = if score < 0.3 { "Low" }
            else if score < 0.6 { "Moderate" }
            else if score < 0.8 { "High" }
            else { "Critical" };

        RiskEstimate {
            score,
            level: level.to_string(),
            factors: vec![
                format!("Operation: {}", operation),
                format!("Impact: {:.2}", impact),
                format!("Reversibility: {:.2}", reversibility),
            ],
        }
    }
}
