pub mod hypothesis;
pub mod evidence;
pub mod contradiction;

use crate::error::CortexError;

pub trait ReasoningEngine {
    fn evaluate(&mut self, input: &str) -> Result<hypothesis::ReasoningResult, CortexError>;
    fn max_steps(&self) -> u32;
}
