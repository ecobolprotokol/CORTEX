use crate::types::*;

pub fn construct_plan(goal: GoalId, steps: Vec<crate::types::Action>, confidence: Scalar) -> Plan {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_plan() {
        let plan = construct_plan(GoalId(1), Vec::new(), 0.8);
        assert_eq!(plan.goal, GoalId(1));
        assert_eq!(plan.confidence, 0.8);
        assert!((plan.uncertainty - 0.2).abs() < 0.001);
    }
}
