//! Runtime lifecycle and state machine.

use crate::error::CortexError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeState {
    Booting,
    LoadingConfig,
    LoadingState,
    Validating,
    Initializing,
    Ready,
    Processing,
    Learning,
    Consolidating,
    Checkpointing,
    ShuttingDown,
    Fault,
    Recovering,
    Stopped,
}

impl RuntimeState {
    pub fn is_terminal(&self) -> bool {
        matches!(self, RuntimeState::Stopped)
    }

    pub fn is_fault(&self) -> bool {
        matches!(self, RuntimeState::Fault)
    }

    pub fn is_operational(&self) -> bool {
        !self.is_terminal() && !self.is_fault()
    }

    pub fn can_transition_to(&self, target: &RuntimeState) -> bool {
        use RuntimeState::*;
        if matches!(target, Fault) {
            return self.is_operational();
        }
        matches!(
            (self, target),
            (Booting, LoadingConfig)
                | (LoadingConfig, LoadingState)
                | (LoadingState, Validating)
                | (Validating, Initializing)
                | (Initializing, Ready)
                | (Ready, Processing)
                | (Ready, Learning)
                | (Ready, ShuttingDown)
                | (Processing, Ready)
                | (Processing, Fault)
                | (Learning, Consolidating)
                | (Learning, Fault)
                | (Consolidating, Checkpointing)
                | (Consolidating, Fault)
                | (Checkpointing, Ready)
                | (Checkpointing, Fault)
                | (ShuttingDown, Stopped)
                | (Fault, Recovering)
                | (Fault, Stopped)
                | (Recovering, Ready)
                | (Recovering, Fault)
                | (Recovering, Stopped)
        )
    }
}

pub trait Runtime {
    fn boot(&mut self) -> Result<(), CortexError>;
    fn ready(&self) -> bool;
    fn run(&mut self) -> Result<(), CortexError>;
    fn shutdown(&mut self) -> Result<(), CortexError>;
}
