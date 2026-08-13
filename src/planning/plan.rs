use serde::{Deserialize, Serialize};
use crate::types::ids::{PlanId, GoalId};
use crate::types::scalars::Scalar;
use crate::types::common::Timestamp;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanStep {
    pub description: String,
    pub action: String,
    pub estimated_cost: Scalar,
    pub risk: Scalar,
    pub preconditions: Vec<String>,
    pub postconditions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub id: PlanId,
    pub goal: String,
    pub goal_id: Option<GoalId>,
    pub steps: Vec<PlanStep>,
    pub estimated_cost: Scalar,
    pub estimated_risk: Scalar,
    pub confidence: Scalar,
    pub created_at: Timestamp,
}

pub struct PlanBuilder {
    next_id: u64,
}

impl PlanBuilder {
    pub fn new() -> Self {
        Self { next_id: 1 }
    }

    pub fn build(&mut self, goal: &str) -> Plan {
        let id = PlanId::from(self.next_id);
        self.next_id += 1;

        let steps = self.decompose_goal(goal);

        let total_cost: Scalar = steps.iter().map(|s| s.estimated_cost).sum();
        let max_risk: Scalar = steps
            .iter()
            .map(|s| s.risk)
            .fold(0.0f32, Scalar::max);

        Plan {
            id,
            goal: goal.to_string(),
            goal_id: None,
            steps,
            estimated_cost: total_cost,
            estimated_risk: max_risk,
            confidence: 0.6,
            created_at: Timestamp::now(),
        }
    }

    pub fn build_detailed(
        &mut self,
        goal: &str,
        steps: Vec<PlanStep>,
    ) -> Plan {
        let id = PlanId::from(self.next_id);
        self.next_id += 1;

        let total_cost: Scalar = steps.iter().map(|s| s.estimated_cost).sum();
        let max_risk: Scalar = steps
            .iter()
            .map(|s| s.risk)
            .fold(0.0f32, Scalar::max);
        let avg_confidence: Scalar = if steps.is_empty() {
            0.0
        } else {
            steps.iter().map(|s| 1.0 - s.risk).sum::<Scalar>() / steps.len() as Scalar
        };

        Plan {
            id,
            goal: goal.to_string(),
            goal_id: None,
            steps,
            estimated_cost: total_cost,
            estimated_risk: max_risk,
            confidence: avg_confidence,
            created_at: Timestamp::now(),
        }
    }

    fn decompose_goal(&self, goal: &str) -> Vec<PlanStep> {
        let mut steps = Vec::new();
        let word_count = goal.split_whitespace().count();

        steps.push(PlanStep {
            description: "Analyze goal requirements".into(),
            action: "reason".into(),
            estimated_cost: 0.1,
            risk: 0.05,
            preconditions: vec![],
            postconditions: vec!["Goal analyzed".into()],
        });

        steps.push(PlanStep {
            description: "Gather relevant information".into(),
            action: "query".into(),
            estimated_cost: 0.15,
            risk: 0.1,
            preconditions: vec!["Goal analyzed".into()],
            postconditions: vec!["Information gathered".into()],
        });

        steps.push(PlanStep {
            description: format!("Process: {}", goal),
            action: "act".into(),
            estimated_cost: 0.2 + (word_count as Scalar * 0.01),
            risk: 0.15,
            preconditions: vec!["Information gathered".into()],
            postconditions: vec!["Goal processed".into()],
        });

        steps.push(PlanStep {
            description: "Verify outcome".into(),
            action: "verify".into(),
            estimated_cost: 0.05,
            risk: 0.05,
            preconditions: vec!["Goal processed".into()],
            postconditions: vec!["Outcome verified".into()],
        });

        steps
    }

    pub fn add_step(&self, plan: &mut Plan, step: PlanStep) {
        plan.estimated_cost += step.estimated_cost;
        plan.estimated_risk = plan.estimated_risk.max(step.risk);
        plan.steps.push(step);
    }

    pub fn optimize_steps(plan: &mut Plan) {
        plan.steps.retain(|s| s.estimated_cost > 0.001);

        for step in &mut plan.steps {
            if step.risk > 0.8 {
                step.risk = 0.8;
            }
        }

        plan.estimated_cost = plan.steps.iter().map(|s| s.estimated_cost).sum();
        plan.estimated_risk = plan
            .steps
            .iter()
            .map(|s| s.risk)
            .fold(0.0f32, Scalar::max);
    }
}

impl Default for PlanBuilder {
    fn default() -> Self {
        Self::new()
    }
}
