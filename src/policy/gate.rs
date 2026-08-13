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
    pub runtime_modification_allowed: bool,
    pub rate_limit_per_minute: u32,
    pub blocked_operations: Vec<String>,
}

impl PolicyGate {
    pub fn new() -> Self {
        Self {
            learning_enabled: true,
            internet_learning_enabled: true,
            self_modification_allowed: false,
            policy_modification_allowed: false,
            runtime_modification_allowed: false,
            rate_limit_per_minute: 60,
            blocked_operations: Vec::new(),
        }
    }

    pub fn with_defaults() -> Self {
        Self::new()
    }

    pub fn evaluate(&self, operation: &str) -> GateResult {
        if self.blocked_operations.iter().any(|b| b == operation) {
            return GateResult {
                decision: PolicyDecision::Deny,
                reason: format!("Operation '{}' is explicitly blocked", operation),
                risk_score: 1.0,
            };
        }

        let (decision, reason, risk_score) = match operation {
            "learning" if !self.learning_enabled => (
                PolicyDecision::Deny,
                "Learning is disabled by policy".into(),
                0.0,
            ),
            "internet_learning" if !self.internet_learning_enabled => (
                PolicyDecision::Deny,
                "Internet learning is disabled by policy".into(),
                0.0,
            ),
            "self_modification" if !self.self_modification_allowed => (
                PolicyDecision::Deny,
                "Self modification is not allowed".into(),
                0.9,
            ),
            "policy_modification" if !self.policy_modification_allowed => (
                PolicyDecision::Deny,
                "Policy modification is not allowed".into(),
                0.95,
            ),
            "runtime_modification" if !self.runtime_modification_allowed => (
                PolicyDecision::Deny,
                "Runtime modification is not allowed".into(),
                0.85,
            ),
            "checkpoint" => (
                PolicyDecision::Allow,
                "Checkpoint operation allowed".into(),
                0.1,
            ),
            "observe" => (PolicyDecision::Allow, "Observation allowed".into(), 0.05),
            "query" => (
                PolicyDecision::Allow,
                "Query operation allowed".into(),
                0.05,
            ),
            "fetch" => (
                PolicyDecision::Limit,
                "Internet fetch limited by rate".into(),
                0.3,
            ),
            "consolidate" => (
                PolicyDecision::Allow,
                "Memory consolidation allowed".into(),
                0.2,
            ),
            "neural_process" => (PolicyDecision::Allow, "Neural process allowed".into(), 0.1),
            "world_integrate" => (
                PolicyDecision::Allow,
                "World integration allowed".into(),
                0.1,
            ),
            "reasoning_evaluate" => (
                PolicyDecision::Allow,
                "Reasoning evaluation allowed".into(),
                0.1,
            ),
            "planning_evaluate" => (
                PolicyDecision::Allow,
                "Planning evaluation allowed".into(),
                0.1,
            ),
            "verification_evaluate" => (
                PolicyDecision::Allow,
                "Verification evaluation allowed".into(),
                0.1,
            ),
            "memory_store" => (PolicyDecision::Allow, "Memory store allowed".into(), 0.1),
            "memory_evict" => (PolicyDecision::Allow, "Memory eviction allowed".into(), 0.2),
            _ => (
                PolicyDecision::Deny,
                format!(
                    "Unknown operation '{}' is not allowed by default",
                    operation
                ),
                1.0,
            ),
        };

        GateResult {
            decision,
            reason,
            risk_score,
        }
    }

    pub fn evaluate_with_context(&self, operation: &str, _context: &str) -> GateResult {
        self.evaluate(operation)
    }

    pub fn is_allowed(&self, operation: &str) -> bool {
        self.evaluate(operation).decision == PolicyDecision::Allow
    }

    pub fn block_operation(&mut self, operation: &str) {
        if !self.blocked_operations.contains(&operation.to_string()) {
            self.blocked_operations.push(operation.to_string());
        }
    }

    pub fn unblock_operation(&mut self, operation: &str) {
        self.blocked_operations.retain(|b| b != operation);
    }
}

impl Default for PolicyGate {
    fn default() -> Self {
        Self::new()
    }
}
