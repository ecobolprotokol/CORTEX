pub mod signal;
pub mod attribution;
pub mod replay;
pub mod stability;

use crate::error::CortexError;

pub trait LearningSystem {
    fn record_experience(&mut self, experience: &str) -> Result<(), CortexError>;
    fn apply_signal(&mut self, signal: &signal::LearningSignal) -> Result<(), CortexError>;
    fn learning_rate(&self) -> f32;
}
