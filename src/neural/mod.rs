pub mod cell;
pub mod column;
pub mod field;
pub mod temporal;
pub mod plasticity;

use crate::error::CortexError;
use crate::types::common::ContextState;
use crate::types::ids::{CellId, ColumnId};
use crate::types::observation::Prediction;
use crate::types::evidence::ConfidenceState;

pub trait NeuralCore {
    fn process(
        &mut self,
        input: &[f32],
        context: &ContextState,
    ) -> Result<NeuralRepresentation, CortexError>;
    fn active_cells(&self) -> usize;
    fn active_columns(&self) -> usize;
    fn sparsity_ratio(&self) -> f32;
}

#[derive(Debug, Clone)]
pub struct NeuralRepresentation {
    pub active_cells: Vec<CellId>,
    pub active_columns: Vec<ColumnId>,
    pub field_activations: Vec<f32>,
    pub temporal_encoding: Vec<f32>,
    pub prediction: Prediction,
    pub confidence: ConfidenceState,
}
