use cortex::planning::plan::PlanBuilder;
use cortex::planning::risk::RiskEvaluator;

#[test]
fn test_plan_builder() {
    let mut builder = PlanBuilder::new();
    let plan = builder.build("achieve goal");
    assert!(!plan.goal.is_empty());
    assert!(!plan.steps.is_empty());
}

#[test]
fn test_risk_evaluator() {
    let evaluator = RiskEvaluator::new();
    let assessment = evaluator.evaluate(0.5, 0.3, 0.4, 0.6, 0.2);
    assert!(assessment.score >= 0.0);
    assert!(assessment.score <= 1.0);
}

#[test]
fn test_plan_step_costs() {
    let mut builder = PlanBuilder::new();
    let plan = builder.build("complex task");
    let total_cost: f32 = plan.steps.iter().map(|s| s.estimated_cost).sum();
    assert!(total_cost > 0.0);
}
