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

    fn extract_entities_from_memories(&mut self, memories: &MemoryRetrieval) {
        for sk in &memories.semantic {
            let name = sk.knowledge.properties.iter()
                .find(|p| p.name == "type")
                .map(|p| match &p.value {
                    PropertyValue::Text(t) => t.clone(),
                    _ => "unknown".to_string(),
                })
                .unwrap_or_else(|| "unknown".to_string());

            let already_exists = self.state.entities.iter()
                .any(|e| e.identity.name == name && e.kind == EntityKind::ConceptualObject);

            if !already_exists {
                let _ = entity::create_entity(&mut self.state, EntityKind::ConceptualObject, &name);
            }
        }
    }

    fn update_event_from_neural(&mut self, representation: &crate::neural::NeuralRepresentation) {
        if representation.active_cells.is_empty() {
            return;
        }
        let event_id = self.state.next_event_id;
        self.state.next_event_id = event_id.next();
        let event = WorldEvent {
            id: event_id,
            description: format!(
                "Neural activity: {} active cells across {} columns",
                representation.active_cells.len(),
                representation.active_columns.len()
            ),
            participants: self.state.entities.iter().take(3).map(|e| e.id).collect(),
            timestamp: Timestamp::now(),
            duration: None,
            outcome: None,
            provenance: Provenance::system("cortex"),
        };
        self.state.active_events.push(event);
        if self.state.active_events.len() > 100 {
            self.state.active_events.drain(0..50);
        }
    }

    fn update_uncertainty(&mut self, representation: &crate::neural::NeuralRepresentation) {
        let activation_level = if representation.field_activations.is_empty() {
            0.0
        } else {
            representation.field_activations.iter().sum::<f32>() / representation.field_activations.len() as f32
        };
        self.state.uncertainty.level = (1.0 - activation_level).max(0.0);
        self.state.uncertainty.updated_at = Timestamp::now();
    }
}

impl WorldModelInterface for WorldModelImpl {
    fn integrate(&mut self, representation: &crate::neural::NeuralRepresentation, memories: &MemoryRetrieval) -> Result<WorldState> {
        if !self.config.enabled {
            return Ok(self.state.clone());
        }
        self.extract_entities_from_memories(memories);
        self.update_event_from_neural(representation);
        self.update_uncertainty(representation);

        for entity in &mut self.state.entities {
            entity.updated_at = Timestamp::now();
            entity.confidence = (entity.confidence * 0.95 + 0.05).min(1.0);
        }

        self.state.temporal_context.current_time = Timestamp::now();
        self.state.temporal_context.sequence_position += 1;

        Ok(self.state.clone())
    }

    fn predict_next(&self) -> Result<Option<PredictedState>> {
        if self.state.entities.is_empty() {
            return Ok(None);
        }
        let confidence = if self.state.uncertainty.level < 0.5 { 0.7 } else { 0.4 };
        Ok(Some(PredictedState {
            predicted_entities: self.state.entities.clone(),
            predicted_relations: self.state.relations.clone(),
            confidence,
            uncertainty: self.state.uncertainty.level,
            prediction_horizon: self.config.prediction_horizon,
        }))
    }

    fn current_state(&self) -> &WorldState {
        &self.state
    }
}
