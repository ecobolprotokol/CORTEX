use cortex::policy::gate::{PolicyDecision, PolicyGate};
use cortex::policy::resource_limits::ResourceLimits;
use cortex::policy::risk::RiskEstimator;

#[test]
fn test_policy_gate_allows_known_operation() {
    let gate = PolicyGate::new();
    let result = gate.evaluate("observe");
    assert_eq!(result.decision, PolicyDecision::Allow);
}

#[test]
fn test_policy_gate_denies_unknown_operation() {
    let gate = PolicyGate::new();
    let result = gate.evaluate("unknown_operation_xyz");
    assert_eq!(result.decision, PolicyDecision::Deny);
}

#[test]
fn test_policy_gate_blocks_learning() {
    let mut gate = PolicyGate::new();
    gate.learning_enabled = false;
    let result = gate.evaluate("learning");
    assert_eq!(result.decision, PolicyDecision::Deny);
}

#[test]
fn test_policy_gate_allows_all_cognitive_operations() {
    let gate = PolicyGate::new();
    let allowed = vec![
        "observe",
        "query",
        "checkpoint",
        "consolidate",
        "neural_process",
        "world_integrate",
        "reasoning_evaluate",
        "planning_evaluate",
        "verification_evaluate",
        "memory_store",
        "memory_evict",
    ];
    for op in allowed {
        let result = gate.evaluate(op);
        assert_eq!(
            result.decision,
            PolicyDecision::Allow,
            "Operation '{}' should be Allow",
            op
        );
    }
}

#[test]
fn test_policy_gate_emergency_override_removed() {
    let gate = PolicyGate::new();
    let result_normal = gate.evaluate("self_modification");
    let result_emergency = gate.evaluate_with_context("self_modification", "emergency override");
    assert_eq!(result_normal.decision, result_emergency.decision);
}

#[test]
fn test_risk_estimator() {
    let estimator = RiskEstimator::new();
    let estimate = estimator.estimate("write_file", 0.5, 0.8);
    assert!(estimate.score > 0.0);
    assert!(!estimate.overall_assessment.is_empty());
}

#[test]
fn test_resource_limits_memory() {
    let limits = ResourceLimits::default();
    assert!(limits.check_memory_usage(100).is_ok());
    assert!(limits.check_memory_usage(limits.max_memory_bytes).is_err());
}

#[test]
fn test_resource_limits_episodes() {
    let limits = ResourceLimits::default();
    assert!(limits.check_episode_count(100).is_ok());
    assert!(limits.check_episode_count(limits.max_episodes).is_err());
}

#[test]
fn test_resource_limits_entities() {
    let limits = ResourceLimits::default();
    assert!(limits.check_entity_count(100).is_ok());
    assert!(limits.check_entity_count(limits.max_entities).is_err());
}

#[test]
fn test_resource_limits_operation_rate() {
    let limits = ResourceLimits::default();
    assert!(limits.check_operation_rate(10).is_ok());
    assert!(limits
        .check_operation_rate(limits.max_operations_per_minute)
        .is_err());
}

#[test]
fn test_policy_gate_block_operation() {
    let mut gate = PolicyGate::new();
    assert_eq!(gate.evaluate("observe").decision, PolicyDecision::Allow);
    gate.block_operation("observe");
    assert_eq!(gate.evaluate("observe").decision, PolicyDecision::Deny);
    gate.unblock_operation("observe");
    assert_eq!(gate.evaluate("observe").decision, PolicyDecision::Allow);
}
