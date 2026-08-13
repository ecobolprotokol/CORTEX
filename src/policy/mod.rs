pub mod risk;
pub mod gate;

use crate::error::CortexError;

pub trait PolicyEngine {
    fn evaluate(&self, operation: &str) -> Result<gate::PolicyDecision, CortexError>;
    fn is_learning_allowed(&self) -> bool;
}
