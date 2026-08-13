pub mod risk;
pub mod gate;
pub mod resource_limits;

pub use risk::{RiskEstimator, RiskEstimate};
pub use gate::{PolicyGate, PolicyDecision, GateResult};
pub use resource_limits::ResourceLimits;

use crate::error::CortexError;
use crate::types::scalars::Scalar;

pub trait PolicyEngine {
    fn evaluate(&self, operation: &str) -> Result<GateResult, CortexError>;
    fn is_learning_allowed(&self) -> bool;
}

pub struct PolicyManager {
    pub gate: PolicyGate,
    pub risk_estimator: RiskEstimator,
    pub audit_log: Vec<AuditEntry>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AuditEntry {
    pub operation: String,
    pub decision: PolicyDecision,
    pub risk_score: f32,
    pub timestamp: u64,
}

impl PolicyManager {
    pub fn new() -> Self {
        Self {
            gate: PolicyGate::new(),
            risk_estimator: RiskEstimator::new(),
            audit_log: Vec::new(),
        }
    }

    pub fn evaluate_operation(&mut self, operation: &str) -> GateResult {
        let gate_result = self.gate.evaluate(operation);

        self.audit_log.push(AuditEntry {
            operation: operation.to_string(),
            decision: gate_result.decision.clone(),
            risk_score: gate_result.risk_score,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        });

        if self.audit_log.len() > 1000 {
            self.audit_log.remove(0);
        }

        gate_result
    }

    pub fn audit_summary(&self) -> (u64, u64, u64) {
        let mut allow_count = 0u64;
        let mut limit_count = 0u64;
        let mut deny_count = 0u64;

        for entry in &self.audit_log {
            match entry.decision {
                PolicyDecision::Allow => allow_count += 1,
                PolicyDecision::Limit => limit_count += 1,
                PolicyDecision::Deny => deny_count += 1,
            }
        }

        (allow_count, limit_count, deny_count)
    }

    pub fn query_by_operation(&self, operation: &str) -> Vec<&AuditEntry> {
        self.audit_log
            .iter()
            .filter(|e| e.operation == operation)
            .collect()
    }

    pub fn query_by_decision(&self, decision: &PolicyDecision) -> Vec<&AuditEntry> {
        self.audit_log
            .iter()
            .filter(|e| e.decision == *decision)
            .collect()
    }

    pub fn query_by_min_risk(&self, min_risk: Scalar) -> Vec<&AuditEntry> {
        self.audit_log
            .iter()
            .filter(|e| e.risk_score >= min_risk)
            .collect()
    }

    pub fn recent(&self, n: usize) -> &[AuditEntry] {
        let start = self.audit_log.len().saturating_sub(n);
        &self.audit_log[start..]
    }
}

impl Default for PolicyManager {
    fn default() -> Self {
        Self::new()
    }
}
