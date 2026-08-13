pub mod cell;
pub mod column;
pub mod field;
pub mod temporal;
pub mod plasticity;

use crate::error::CortexError;
use crate::types::scalars::Scalar;

pub trait NeuralCore {
    fn process(&mut self, input: &[Scalar]) -> Result<Vec<Scalar>, CortexError>;
    fn active_cells(&self) -> usize;
    fn sparsity_ratio(&self) -> Scalar;
}
