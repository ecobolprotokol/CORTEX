pub mod risk;
pub mod gate;

use crate::config::PolicyConfig;
use crate::error::Result;
use crate::types::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyDecision {
    Allowed,
    Limited,
    Denied,
}

pub trait PolicyEngine {
    fn evaluate(&self, operation: &ProposedOperation) -> PolicyDecision;
    fn config(&self) -> &PolicyConfig;
}

pub struct ProposedOperation {
    pub classification: OperationClassification,
    pub description: String,
    pub target: String,
    pub estimated_impact: Scalar,
    pub reversibility: Scalar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationClassification {
    CognitiveStateAdaptation,
    AlgorithmAdaptation,
    SecurityPolicyModification,
    ReadOperation,
    NetworkOperation,
}

pub struct PolicyEngineImpl {
    config: PolicyConfig,
}

impl PolicyEngineImpl {
    pub fn new(config: &PolicyConfig) -> Result<Self> {
        Ok(Self { config: config.clone() })
    }
}

impl PolicyEngine for PolicyEngineImpl {
    fn evaluate(&self, operation: &ProposedOperation) -> PolicyDecision {
        match operation.classification {
            OperationClassification::SecurityPolicyModification => {
                if self.config.policy_modification {
                    PolicyDecision::Allowed
                } else {
                    PolicyDecision::Denied
                }
            }
            OperationClassification::AlgorithmAdaptation => {
                if self.config.self_modification {
                    PolicyDecision::Allowed
                } else {
                    PolicyDecision::Denied
                }
            }
            OperationClassification::NetworkOperation => {
                if self.config.internet_learning {
                    PolicyDecision::Allowed
                } else {
                    PolicyDecision::Denied
                }
            }
            OperationClassification::CognitiveStateAdaptation => {
                if self.config.learning {
                    PolicyDecision::Allowed
                } else {
                    PolicyDecision::Denied
                }
            }
            OperationClassification::ReadOperation => PolicyDecision::Allowed,
        }
    }

    fn config(&self) -> &PolicyConfig {
        &self.config
    }
}
