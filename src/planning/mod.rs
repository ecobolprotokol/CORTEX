pub mod plan;
pub mod risk;

pub use plan::{PlanBuilder, Plan, PlanStep};
pub use risk::{RiskEvaluator, RiskAssessment};

use crate::error::CortexError;
use crate::types::scalars::Scalar;

type TransitionFn<'a> = Option<&'a dyn Fn(&str, &str) -> (String, Scalar)>;

pub trait PlanningEngine {
    fn evaluate(&self, goal: &str) -> Result<plan::Plan, CortexError>;
    fn max_depth(&self) -> u32;
    fn max_branches(&self) -> u32;
}

pub fn simulate_plan(
    plan: &Plan,
    transition_fn: TransitionFn<'_>,
) -> PlanSimulationResult {
    let mut cumulative_cost = 0.0f32;
    let mut cumulative_risk = 0.0f32;
    let mut step_results = Vec::new();
    let mut current_state = "initial".to_string();

    for step in &plan.steps {
        let (new_state, confidence) = if let Some(f) = transition_fn {
            f(&current_state, &step.action)
        } else {
            (format!("After: {}", step.action), 0.5)
        };

        cumulative_cost += step.estimated_cost;
        cumulative_risk = cumulative_risk.max(step.estimated_cost * 0.5);

        step_results.push(StepSimulationResult {
            step_description: step.description.clone(),
            action: step.action.clone(),
            state_after: new_state.clone(),
            confidence,
            cost: step.estimated_cost,
        });

        current_state = new_state;
    }

    let avg_confidence = if step_results.is_empty() {
        0.0
    } else {
        step_results.iter().map(|r| r.confidence).sum::<Scalar>()
            / step_results.len() as Scalar
    };

    PlanSimulationResult {
        plan_id: plan.id,
        total_cost: cumulative_cost,
        total_risk: cumulative_risk,
        step_results,
        overall_confidence: avg_confidence,
        feasible: avg_confidence > 0.3 && cumulative_cost < 10.0,
    }
}

#[derive(Debug, Clone)]
pub struct StepSimulationResult {
    pub step_description: String,
    pub action: String,
    pub state_after: String,
    pub confidence: Scalar,
    pub cost: Scalar,
}

#[derive(Debug, Clone)]
pub struct PlanSimulationResult {
    pub plan_id: crate::types::ids::PlanId,
    pub total_cost: Scalar,
    pub total_risk: Scalar,
    pub step_results: Vec<StepSimulationResult>,
    pub overall_confidence: Scalar,
    pub feasible: bool,
}

pub fn evaluate_plan_quality(plan: &Plan) -> Scalar {
    let cost_factor = 1.0 - plan.estimated_cost.min(1.0);
    let risk_factor = 1.0 - plan.estimated_risk.min(1.0);
    let confidence_factor = plan.confidence;
    let step_factor = if plan.steps.is_empty() {
        0.0
    } else {
        (1.0 - (plan.steps.len() as Scalar / 20.0).min(1.0)) * 0.3
    };

    (cost_factor * 0.25 + risk_factor * 0.25 + confidence_factor * 0.3 + step_factor).min(1.0)
}
