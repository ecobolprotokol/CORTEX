use std::path::PathBuf;

fn test_config_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("cortex.toml")
}

fn cleanup() {
    let _ = std::fs::remove_file("test_policy_cx");
}

#[test]
fn test_policy_engine_allows_learning() {
    cleanup();
    let config = cortex::config::CortexConfig::load(test_config_path().to_str().unwrap()).unwrap();
    let policy = cortex::policy::PolicyEngineImpl::new(&config.policy).unwrap();

    let operation = cortex::policy::ProposedOperation {
        classification: cortex::policy::OperationClassification::CognitiveStateAdaptation,
        description: "Update semantic memory".into(),
        target: "semantic_memory".into(),
        estimated_impact: 0.2,
        reversibility: 0.8,
    };

    use cortex::policy::PolicyEngine;
    let decision = policy.evaluate(&operation);
    assert_eq!(decision, cortex::policy::PolicyDecision::Allowed);

    cleanup();
}

#[test]
fn test_policy_engine_denies_self_modification() {
    cleanup();
    let mut config = cortex::config::CortexConfig::load(test_config_path().to_str().unwrap()).unwrap();
    config.policy.self_modification = false;

    let policy = cortex::policy::PolicyEngineImpl::new(&config.policy).unwrap();

    let operation = cortex::policy::ProposedOperation {
        classification: cortex::policy::OperationClassification::AlgorithmAdaptation,
        description: "Modify learning algorithm".into(),
        target: "learning".into(),
        estimated_impact: 0.5,
        reversibility: 0.3,
    };

    use cortex::policy::PolicyEngine;
    let decision = policy.evaluate(&operation);
    assert_eq!(decision, cortex::policy::PolicyDecision::Denied);

    cleanup();
}

#[test]
fn test_policy_engine_denies_policy_modification() {
    cleanup();
    let mut config = cortex::config::CortexConfig::load(test_config_path().to_str().unwrap()).unwrap();
    config.policy.policy_modification = false;

    let policy = cortex::policy::PolicyEngineImpl::new(&config.policy).unwrap();

    let operation = cortex::policy::ProposedOperation {
        classification: cortex::policy::OperationClassification::SecurityPolicyModification,
        description: "Modify risk thresholds".into(),
        target: "policy".into(),
        estimated_impact: 0.9,
        reversibility: 0.1,
    };

    use cortex::policy::PolicyEngine;
    let decision = policy.evaluate(&operation);
    assert_eq!(decision, cortex::policy::PolicyDecision::Denied);

    cleanup();
}

#[test]
fn test_risk_estimation() {
    let risk = cortex::policy::risk::estimate_risk(0.5, 0.8);
    assert!(risk.score > 0.0);
    assert!(risk.score < 1.0);

    let risk_high = cortex::policy::risk::estimate_risk(0.9, 0.1);
    assert!(risk_high.score > risk.score);

    cleanup();
}
