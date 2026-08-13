pub mod capability;

use crate::error::CortexError;

pub trait SelfModelInterface {
    fn assess_capabilities(&self) -> Result<capability::CapabilityAssessment, CortexError>;
    fn prediction_accuracy(&self) -> f32;
}
