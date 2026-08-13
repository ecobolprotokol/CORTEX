use cortex::policy::gate::PolicyGate;
use cortex::policy::risk::RiskEstimator;

#[test]
fn test_policy_gate_allows() {
    let gate = PolicyGate::new();
    let result = gate.evaluate("normal_operation");
    assert_eq!(result.decision, cortex::policy::gate::PolicyDecision::Allow);
}

#[test]
fn test_policy_gate_blocks_learning() {
    let mut gate = PolicyGate::new();
    gate.learning_enabled = false;
    let result = gate.evaluate("learning");
    assert_eq!(result.decision, cortex::policy::gate::PolicyDecision::Deny);
}

#[test]
fn test_risk_estimator() {
    let estimator = RiskEstimator::new();
    let estimate = estimator.estimate("write_file", 0.5, 0.8);
    assert!(estimate.score > 0.0);
    assert!(!estimate.overall_assessment.is_empty());
}
