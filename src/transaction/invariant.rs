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
        Ok(())
    }

    fn validate_metadata(state: &CortexState) -> Result<(), CortexError> {
        if state.metadata.architecture_version == 0 {
            return Err(CortexError::StateError(
                "architecture_version must be > 0".into(),
            ));
        }
        if state.metadata.schema_version == 0 {
            return Err(CortexError::StateError(
                "schema_version must be > 0".into(),
            ));
        }
        Ok(())
    }

    fn validate_confidence_bounds(state: &CortexState) -> Result<(), CortexError> {
        if state.verification.confidence_threshold > 1.0 || state.verification.confidence_threshold < 0.0 {
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

    pub fn pre_mutation_check(state: &CortexState, version: u64) -> Result<(), CortexError> {
        if state.metadata.last_updated.as_millis() == 0 && version > 0 {
            return Err(CortexError::StateError(
                "State appears uninitialized but version > 0".into(),
            ));
        }
        Self::validate_state(state)
    }

    pub fn post_mutation_check(state: &CortexState, _expected_version: u64) -> Result<(), CortexError> {
        Self::validate_state(state)
    }
}
