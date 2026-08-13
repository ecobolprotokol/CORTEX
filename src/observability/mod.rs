pub mod diagnostics;

use crate::error::CortexError;

pub trait ObservabilityInterface {
    fn metrics(&self) -> Result<diagnostics::Metrics, CortexError>;
    fn health_check(&self) -> Result<bool, CortexError>;
}
