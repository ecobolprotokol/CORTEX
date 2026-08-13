use serde::{Deserialize, Serialize};
use crate::types::ids::CellId;
use crate::types::scalars::Scalar;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CellState {
    Resting,
    Active,
    Inhibited,
    Learning,
    Predicting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cell {
    pub id: CellId,
    pub state: CellState,
    pub activation: Scalar,
    pub prediction: Scalar,
    pub weight: Scalar,
}

impl Cell {
    pub fn new(id: CellId) -> Self {
        Self {
            id,
            state: CellState::Resting,
            activation: 0.0,
            prediction: 0.0,
            weight: 0.0,
        }
    }

    pub fn activate(&mut self, threshold: Scalar) {
        if self.activation >= threshold {
            self.state = CellState::Active;
        } else {
            self.state = CellState::Resting;
        }
    }

    pub fn inhibit(&mut self) {
        self.state = CellState::Inhibited;
        self.activation = 0.0;
    }

    pub fn predict(&self) -> Scalar {
        self.prediction
    }
}
