use crate::error::CortexError;
use crate::types::state::CortexState;

pub struct StateInvariant;

pub trait InvariantCheck {
    fn validate(&self) -> Result<(), CortexError>;
}

impl InvariantCheck for CortexState {
    fn validate(&self) -> Result<(), CortexError> {
        StateInvariant::validate_state(self)
    }
}

impl StateInvariant {
    pub fn validate_state(state: &CortexState) -> Result<(), CortexError> {
        Self::validate_metadata(state)?;
        Self::validate_confidence_bounds(state)?;
        Self::validate_memory_consistency(state)?;
        Self::validate_neural_state(state)?;
        Self::validate_reasoning_state(state)?;
        Self::validate_world_state(state)?;
        Self::validate_learning_state(state)?;
        Self::validate_verification_state(state)?;
        Ok(())
    }

    fn validate_metadata(state: &CortexState) -> Result<(), CortexError> {
        if state.metadata.architecture_version == 0 {
            return Err(CortexError::StateError(
                "architecture_version must be > 0".into(),
            ));
        }
        if state.metadata.schema_version == 0 {
            return Err(CortexError::StateError("schema_version must be > 0".into()));
        }
        Ok(())
    }

    fn validate_confidence_bounds(state: &CortexState) -> Result<(), CortexError> {
        if state.verification.confidence_threshold > 1.0
            || state.verification.confidence_threshold < 0.0
        {
            return Err(CortexError::StateError(
                "verification.confidence_threshold must be in [0, 1]".into(),
            ));
        }
        Ok(())
    }

    fn validate_memory_consistency(state: &CortexState) -> Result<(), CortexError> {
        if state.memory.episodic.capacity_bytes > 0
            && state.memory.episodic.current_usage_bytes > state.memory.episodic.capacity_bytes
        {
            return Err(CortexError::StateError(
                "episodic memory usage exceeds capacity".into(),
            ));
        }
        Ok(())
    }

    fn validate_neural_state(state: &CortexState) -> Result<(), CortexError> {
        if state.neural.active_cells.len() > 10_000 {
            return Err(CortexError::StateError(
                "active_cells count exceeds reasonable bound".into(),
            ));
        }
        if state.neural.active_columns.len() > 1_000 {
            return Err(CortexError::StateError(
                "active_columns count exceeds reasonable bound".into(),
            ));
        }
        Ok(())
    }

    fn validate_reasoning_state(state: &CortexState) -> Result<(), CortexError> {
        if state.reasoning.active_hypotheses.len() > 100 {
            return Err(CortexError::StateError(
                "active_hypotheses count exceeds reasonable bound".into(),
            ));
        }
        for hyp in &state.reasoning.active_hypotheses {
            if hyp.confidence < 0.0 || hyp.confidence > 1.0 {
                return Err(CortexError::StateError(
                    "hypothesis confidence out of [0,1] bounds".into(),
                ));
            }
        }
        Ok(())
    }

    fn validate_world_state(state: &CortexState) -> Result<(), CortexError> {
        if state.world.entities.len() > 10_000 {
            return Err(CortexError::StateError(
                "entity count exceeds reasonable bound".into(),
            ));
        }
        for entity in &state.world.entities {
            if entity.confidence < 0.0 || entity.confidence > 1.0 {
                return Err(CortexError::StateError(
                    "entity confidence out of [0,1] bounds".into(),
                ));
            }
        }
        Ok(())
    }

    fn validate_learning_state(state: &CortexState) -> Result<(), CortexError> {
        if state.learning.learning_rate < 0.0 || state.learning.learning_rate > 1.0 {
            return Err(CortexError::StateError(
                "learning_rate must be in [0, 1]".into(),
            ));
        }
        if state.learning.plasticity_rate < 0.0 || state.learning.plasticity_rate > 1.0 {
            return Err(CortexError::StateError(
                "plasticity_rate must be in [0, 1]".into(),
            ));
        }
        Ok(())
    }

    fn validate_verification_state(state: &CortexState) -> Result<(), CortexError> {
        if state.self_model.prediction_accuracy < 0.0
            || state.self_model.prediction_accuracy > 1.0
        {
            return Err(CortexError::StateError(
                "prediction_accuracy must be in [0, 1]".into(),
            ));
        }
        if state.self_model.uncertainty_level < 0.0 || state.self_model.uncertainty_level > 1.0 {
            return Err(CortexError::StateError(
                "uncertainty_level must be in [0, 1]".into(),
            ));
        }
        Ok(())
    }

    pub fn pre_mutation_check(state: &CortexState, version: u64) -> Result<(), CortexError> {
        if state.metadata.last_updated.as_millis() == 0 && version > 0 {
            return Err(CortexError::StateError(
                "State appears uninitialized but version > 0".into(),
            ));
        }
        Self::validate_state(state)
    }

    pub fn post_mutation_check(
        state: &CortexState,
        expected_version: u64,
    ) -> Result<(), CortexError> {
        Self::validate_state(state)?;
        if expected_version > 0 && state.metadata.last_updated.as_millis() == 0 {
            return Err(CortexError::StateError(
                "State timestamp invalid after mutation".into(),
            ));
        }
        Ok(())
    }
}
