pub mod entity;
pub mod transition;
pub mod causal;
pub mod simulation;

use crate::config::WorldConfig;
use crate::error::Result;
use crate::types::*;

pub trait WorldModelInterface {
    fn integrate(&mut self, representation: &crate::neural::NeuralRepresentation, memories: &MemoryRetrieval) -> Result<WorldState>;
    fn predict_next(&self) -> Result<Option<PredictedState>>;
    fn current_state(&self) -> &WorldState;
}

pub struct WorldModelImpl {
    config: WorldConfig,
    state: WorldState,
}

impl WorldModelImpl {
    pub fn new(config: &WorldConfig) -> Result<Self> {
        Ok(Self {
            config: config.clone(),
            state: WorldState {
                entities: Vec::new(),
                relations: Vec::new(),
                active_events: Vec::new(),
                temporal_context: TemporalContext::default(),
                uncertainty: UncertaintyState::initial(),
                next_entity_id: EntityId(1),
                next_relation_id: RelationId(1),
                next_event_id: EventId(1),
            },
        })
    }
}

impl WorldModelInterface for WorldModelImpl {
    fn integrate(&mut self, _representation: &crate::neural::NeuralRepresentation, _memories: &MemoryRetrieval) -> Result<WorldState> {
        Ok(self.state.clone())
    }

    fn predict_next(&self) -> Result<Option<PredictedState>> {
        if self.state.entities.is_empty() {
            return Ok(None);
        }
        Ok(Some(PredictedState {
            predicted_entities: self.state.entities.clone(),
            predicted_relations: self.state.relations.clone(),
            confidence: 0.5,
            uncertainty: 0.5,
            prediction_horizon: self.config.prediction_horizon,
        }))
    }

    fn current_state(&self) -> &WorldState {
        &self.state
    }
}
