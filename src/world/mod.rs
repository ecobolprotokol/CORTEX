pub mod causal;
pub mod entity;
pub mod simulation;
pub mod transition;

pub use causal::CausalModel;
pub use entity::EntityManager;
pub use simulation::WorldSimulator;
pub use transition::TransitionModel;

use crate::error::CortexError;
use crate::types::scalars::Scalar;

pub trait WorldModelInterface {
    fn integrate(&mut self, observation: &str) -> Result<(), CortexError>;
    fn predict_transition(&self, action: &str) -> Result<transition::PredictedState, CortexError>;
    fn entity_count(&self) -> usize;
}

impl WorldModelInterface for crate::types::state::WorldState {
    fn integrate(&mut self, observation: &str) -> Result<(), CortexError> {
        let words: Vec<&str> = observation.split_whitespace().collect();
        for word in words.iter().take(5) {
            if word.len() > 2 {
                let id = crate::types::ids::EntityId::next();
                self.entities.push(crate::types::state::Entity {
                    id,
                    name: word.to_string(),
                    confidence: 0.5,
                    created_at: crate::types::common::Timestamp::now(),
                    updated_at: crate::types::common::Timestamp::now(),
                });
            }
        }
        Ok(())
    }

    fn predict_transition(&self, _action: &str) -> Result<transition::PredictedState, CortexError> {
        Ok(transition::PredictedState {
            confidence: 0.5,
            uncertainty: 0.5,
            description: "Transition predicted".into(),
            state_changes: Vec::new(),
        })
    }

    fn entity_count(&self) -> usize {
        self.entities.len()
    }
}

pub fn compute_state_distance(state_a: &[Scalar], state_b: &[Scalar]) -> Scalar {
    state_a
        .iter()
        .zip(state_b.iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum::<Scalar>()
        .sqrt()
}

pub fn normalize_state(state: &mut [Scalar]) {
    let norm: Scalar = state.iter().map(|x| x * x).sum::<Scalar>().sqrt();
    if norm > crate::types::scalars::SCALAR_EPSILON {
        for x in state.iter_mut() {
            *x /= norm;
        }
    }
}
