use serde::{Deserialize, Serialize};
use crate::types::ids::PlanId;
use crate::types::scalars::Scalar;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanStep {
    pub description: String,
    pub action: String,
    pub estimated_cost: Scalar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub id: PlanId,
    pub goal: String,
    pub steps: Vec<PlanStep>,
    pub estimated_cost: Scalar,
    pub estimated_risk: Scalar,
    pub confidence: Scalar,
}

pub struct PlanBuilder;

impl PlanBuilder {
    pub fn new() -> Self { Self }

    pub fn build(&self, goal: &str) -> Plan {
        Plan {
            id: PlanId::from(1),
            goal: goal.to_string(),
            steps: vec![
                PlanStep { description: "Analyze goal".into(), action: "reason".into(), estimated_cost: 0.1 },
                PlanStep { description: "Execute plan".into(), action: "act".into(), estimated_cost: 0.3 },
            ],
            estimated_cost: 0.4,
            estimated_risk: 0.2,
            confidence: 0.6,
        }
    }
}
