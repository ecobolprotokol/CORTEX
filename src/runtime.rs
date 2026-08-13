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

pub trait Runtime {
    fn boot(&mut self) -> Result<(), CortexError>;
    fn ready(&self) -> bool;
    fn run(&mut self) -> Result<(), CortexError>;
    fn shutdown(&mut self) -> Result<(), CortexError>;
}
