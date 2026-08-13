use crate::types::*;

pub fn construct_plan(goal: GoalId, steps: Vec<super::observation::Action>, confidence: Scalar) -> Plan {
    Plan {
        id: PlanId(0),
        goal,
        steps,
        estimated_cost: 0.1,
        estimated_risk: 0.1,
        uncertainty: 1.0 - confidence,
        confidence,
        predicted_outcomes: Vec::new(),
    }
}
