pub mod entity;
pub mod transition;
pub mod causal;
pub mod simulation;

use crate::error::CortexError;

pub trait WorldModelInterface {
    fn integrate(&mut self, observation: &str) -> Result<(), CortexError>;
    fn predict_transition(&self, action: &str) -> Result<transition::PredictedState, CortexError>;
    fn entity_count(&self) -> usize;
}
