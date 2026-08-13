use crate::types::scalars::Scalar;

#[derive(Debug, Clone)]
pub struct PredictedState {
    pub confidence: Scalar,
    pub uncertainty: Scalar,
    pub description: String,
}

pub struct TransitionModel;

impl TransitionModel {
    pub fn new() -> Self {
        Self
    }

    pub fn predict(&self, current_state: &str, action: &str) -> PredictedState {
        PredictedState {
            confidence: 0.5,
            uncertainty: 0.5,
            description: format!("After '{}', state changes due to '{}'", current_state, action),
        }
    }
}
