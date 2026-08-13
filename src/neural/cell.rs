use crate::types::ids::CellId;
use serde::{Deserialize, Serialize};

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
    pub activation: f32,
    pub prediction: f32,
    pub weight: f32,
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

    pub fn set_activation(&mut self, value: f32) {
        self.activation = value.clamp(0.0, 1.0);
    }

    pub fn activate(&mut self, threshold: f32) {
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

    pub fn predict(&self) -> f32 {
        self.prediction
    }

    pub fn set_predicting(&mut self, value: f32) {
        self.state = CellState::Predicting;
        self.prediction = value;
    }

    pub fn start_learning(&mut self) {
        if self.state == CellState::Active {
            self.state = CellState::Learning;
        }
    }

    pub fn adapt(&mut self, error: f32) {
        self.weight += self.activation * error;
        self.weight = self.weight.clamp(-1.0, 1.0);
        if self.state == CellState::Learning || self.state == CellState::Active {
            self.state = CellState::Active;
        }
    }

    pub fn is_active(&self) -> bool {
        self.state == CellState::Active
    }

    pub fn reset(&mut self) {
        self.state = CellState::Resting;
        self.activation = 0.0;
        self.prediction = 0.0;
    }
}
