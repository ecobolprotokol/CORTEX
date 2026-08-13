pub mod confidence;

use crate::error::CortexError;

pub trait VerificationEngine {
    fn verify(&self, claim: &str) -> Result<confidence::VerificationResult, CortexError>;
    fn minimum_confidence(&self) -> f32;
}
