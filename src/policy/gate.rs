use crate::types::scalars::Scalar;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyDecision {
    Allow,
    Limit,
    Deny,
}

#[derive(Debug, Clone)]
pub struct GateResult {
    pub decision: PolicyDecision,
    pub reason: String,
    pub risk_score: Scalar,
}

pub struct PolicyGate {
    pub learning_enabled: bool,
    pub internet_learning_enabled: bool,
    pub self_modification_allowed: bool,
    pub policy_modification_allowed: bool,
}

impl PolicyGate {
    pub fn new() -> Self {
        Self {
            learning_enabled: true,
            internet_learning_enabled: true,
            self_modification_allowed: false,
            policy_modification_allowed: false,
        }
    }

    pub fn evaluate(&self, operation: &str) -> GateResult {
        let decision = match operation {
            "learning" if !self.learning_enabled => PolicyDecision::Deny,
            "internet_learning" if !self.internet_learning_enabled => PolicyDecision::Deny,
            "self_modification" if !self.self_modification_allowed => PolicyDecision::Deny,
            "policy_modification" if !self.policy_modification_allowed => PolicyDecision::Deny,
            _ => PolicyDecision::Allow,
        };

        GateResult {
            decision,
            reason: format!("Operation '{}' evaluated", operation),
            risk_score: 0.0,
        }
    }
}
