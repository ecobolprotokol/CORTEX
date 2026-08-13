pub fn gate_decide(engine: &dyn crate::policy::PolicyEngine, operation: &crate::policy::ProposedOperation) -> crate::policy::PolicyDecision {
    engine.evaluate(operation)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy::*;

    #[test]
    fn test_gate_decide() {
        let config = crate::config::PolicyConfig {
            learning: true,
            internet_learning: true,
            self_modification: false,
            policy_modification: false,
            runtime_modification: false,
        };
        let engine = PolicyEngineImpl::new(&config).unwrap();
        let operation = ProposedOperation {
            classification: OperationClassification::CognitiveStateAdaptation,
            description: "test".into(),
            target: "test".into(),
            estimated_impact: 0.2,
            reversibility: 0.8,
        };
        assert_eq!(gate_decide(&engine, &operation), PolicyDecision::Allowed);
    }
}
