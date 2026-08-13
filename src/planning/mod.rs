pub mod plan;
pub mod risk;

use crate::error::CortexError;

pub trait PlanningEngine {
    fn evaluate(&self, goal: &str) -> Result<plan::Plan, CortexError>;
    fn max_depth(&self) -> u32;
    fn max_branches(&self) -> u32;
}
