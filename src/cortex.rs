//! Global orchestration and CortexRuntime.

use crate::config::CortexConfig;
use crate::error::CortexError;
use crate::runtime::{Runtime, RuntimeState};
use crate::types::state::{
    AlgorithmVersions, CortexState, LanguageState, LearningState, MemoryState, NeuralState,
    PlanningState, ProvenanceState, ReasoningState, SelfModel, StateMetadata, VerificationState,
    WorldState,
};
use crate::types::common::Timestamp;
use crate::types::ids::SessionId;

pub struct CortexRuntime {
    pub state: CortexState,
    pub config: CortexConfig,
    pub runtime_state: RuntimeState,
}

impl Default for CortexState {
    fn default() -> Self {
        Self {
            metadata: StateMetadata {
                version: env!("CARGO_PKG_VERSION").into(),
                created_at: Timestamp::now(),
                updated_at: Timestamp::now(),
                session_id: SessionId::next(),
                algorithm_versions: AlgorithmVersions {
                    attention: "1.0.0".into(),
                    consolidation: "1.0.0".into(),
                    inference: "1.0.0".into(),
                    planning: "1.0.0".into(),
                    learning: "1.0.0".into(),
                },
            },
            language: LanguageState::default(),
            neural: NeuralState::default(),
            memory: MemoryState::default(),
            world: WorldState::default(),
            reasoning: ReasoningState::default(),
            planning: PlanningState::default(),
            verification: VerificationState::default(),
            learning: LearningState::default(),
            self_model: SelfModel::default(),
            provenance: ProvenanceState::default(),
        }
    }
}

impl CortexRuntime {
    pub fn new(config: CortexConfig) -> Result<Self, CortexError> {
        let state = CortexState::default();
        Ok(Self {
            state,
            config,
            runtime_state: RuntimeState::Booting,
        })
    }
}

impl Runtime for CortexRuntime {
    fn boot(&mut self) -> Result<(), CortexError> {
        self.runtime_state = RuntimeState::LoadingConfig;

        self.runtime_state = RuntimeState::LoadingState;

        self.runtime_state = RuntimeState::Validating;

        self.runtime_state = RuntimeState::Initializing;

        self.runtime_state = RuntimeState::Ready;
        Ok(())
    }

    fn ready(&self) -> bool {
        matches!(self.runtime_state, RuntimeState::Ready)
    }

    fn run(&mut self) -> Result<(), CortexError> {
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), CortexError> {
        self.runtime_state = RuntimeState::ShuttingDown;
        self.runtime_state = RuntimeState::Stopped;
        Ok(())
    }
}
