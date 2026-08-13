use crate::config::CortexConfig;
use crate::error::Result;
use crate::types::*;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeState {
    Boot,
    LoadingConfig,
    LoadingState,
    Validating,
    Initializing,
    Ready,
    Processing,
    Learning,
    Consolidating,
    Checkpointing,
    Fault,
    Recovering,
    ShuttingDown,
    Stopped,
}

impl RuntimeState {
    pub fn can_transition_to(&self, next: &RuntimeState) -> bool {
        matches!(
            (self, next),
            (RuntimeState::Boot, RuntimeState::LoadingConfig)
                | (RuntimeState::LoadingConfig, RuntimeState::LoadingState)
                | (RuntimeState::LoadingConfig, RuntimeState::Initializing)
                | (RuntimeState::LoadingState, RuntimeState::Validating)
                | (RuntimeState::Initializing, RuntimeState::Validating)
                | (RuntimeState::Validating, RuntimeState::Ready)
                | (RuntimeState::Ready, RuntimeState::Processing)
                | (RuntimeState::Ready, RuntimeState::Learning)
                | (RuntimeState::Ready, RuntimeState::Consolidating)
                | (RuntimeState::Ready, RuntimeState::Checkpointing)
                | (RuntimeState::Processing, RuntimeState::Learning)
                | (RuntimeState::Processing, RuntimeState::Ready)
                | (RuntimeState::Learning, RuntimeState::Consolidating)
                | (RuntimeState::Learning, RuntimeState::Ready)
                | (RuntimeState::Consolidating, RuntimeState::Ready)
                | (RuntimeState::Checkpointing, RuntimeState::Ready)
                | (_, RuntimeState::Fault)
                | (RuntimeState::Fault, RuntimeState::Recovering)
                | (RuntimeState::Recovering, RuntimeState::Ready)
                | (RuntimeState::Recovering, RuntimeState::Stopped)
                | (_, RuntimeState::ShuttingDown)
                | (RuntimeState::ShuttingDown, RuntimeState::Stopped)
        )
    }

    pub fn name(&self) -> &'static str {
        match self {
            RuntimeState::Boot => "Boot",
            RuntimeState::LoadingConfig => "LoadingConfig",
            RuntimeState::LoadingState => "LoadingState",
            RuntimeState::Validating => "Validating",
            RuntimeState::Initializing => "Initializing",
            RuntimeState::Ready => "Ready",
            RuntimeState::Processing => "Processing",
            RuntimeState::Learning => "Learning",
            RuntimeState::Consolidating => "Consolidating",
            RuntimeState::Checkpointing => "Checkpointing",
            RuntimeState::Fault => "Fault",
            RuntimeState::Recovering => "Recovering",
            RuntimeState::ShuttingDown => "ShuttingDown",
            RuntimeState::Stopped => "Stopped",
        }
    }
}

pub struct Runtime {
    state: RuntimeState,
    config: Option<CortexConfig>,
    start_time: std::time::Instant,
    error_log: Vec<RuntimeError>,
}

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub kind: String,
    pub message: String,
    pub timestamp: Timestamp,
    pub recoverable: bool,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            state: RuntimeState::Boot,
            config: None,
            start_time: std::time::Instant::now(),
            error_log: Vec::new(),
        }
    }

    pub fn state(&self) -> RuntimeState {
        self.state
    }

    pub fn uptime_secs(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }

    pub fn transition(&mut self, next: RuntimeState) -> Result<()> {
        if !self.state.can_transition_to(&next) {
            let msg = format!("Invalid transition: {:?} -> {:?}", self.state, next);
            self.error_log.push(RuntimeError {
                kind: "StateError".into(),
                message: msg.clone(),
                timestamp: Timestamp::now(),
                recoverable: false,
            });
            return Err(crate::error::CortexError::StateError(msg));
        }
        self.state = next;
        Ok(())
    }

    pub fn load_config(&mut self, path: &str) -> Result<&CortexConfig> {
        self.transition(RuntimeState::LoadingConfig)?;
        let config = CortexConfig::load(path)?;
        self.config = Some(config);
        self.transition(RuntimeState::LoadingState)?;
        Ok(self.config.as_ref().unwrap())
    }

    pub fn config(&self) -> Option<&CortexConfig> {
        self.config.as_ref()
    }

    pub fn state_file_exists(&self) -> bool {
        self.config
            .as_ref()
            .map(|c| Path::new(&c.persistence.state).exists())
            .unwrap_or(false)
    }

    pub fn record_error(&mut self, kind: &str, message: &str, recoverable: bool) {
        self.error_log.push(RuntimeError {
            kind: kind.to_string(),
            message: message.to_string(),
            timestamp: Timestamp::now(),
            recoverable,
        });
    }

    pub fn recent_errors(&self, count: usize) -> Vec<&RuntimeError> {
        self.error_log.iter().rev().take(count).collect()
    }

    pub fn error_count(&self) -> usize {
        self.error_log.len()
    }

    pub fn health_check(&self) -> RuntimeHealth {
        RuntimeHealth {
            state: self.state,
            uptime_secs: self.uptime_secs(),
            error_count: self.error_count(),
            recoverable_errors: self.error_log.iter().filter(|e| e.recoverable).count(),
            critical_errors: self.error_log.iter().filter(|e| !e.recoverable).count(),
            healthy: self.state == RuntimeState::Ready || self.state == RuntimeState::Processing,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeHealth {
    pub state: RuntimeState,
    pub uptime_secs: u64,
    pub error_count: usize,
    pub recoverable_errors: usize,
    pub critical_errors: usize,
    pub healthy: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_state_transitions() {
        assert!(RuntimeState::Boot.can_transition_to(&RuntimeState::LoadingConfig));
        assert!(RuntimeState::LoadingConfig.can_transition_to(&RuntimeState::LoadingState));
        assert!(RuntimeState::LoadingState.can_transition_to(&RuntimeState::Validating));
        assert!(RuntimeState::Validating.can_transition_to(&RuntimeState::Ready));
        assert!(RuntimeState::Ready.can_transition_to(&RuntimeState::Processing));
        assert!(RuntimeState::Processing.can_transition_to(&RuntimeState::Ready));
    }

    #[test]
    fn test_invalid_transition() {
        assert!(!RuntimeState::Ready.can_transition_to(&RuntimeState::Boot));
        assert!(!RuntimeState::Processing.can_transition_to(&RuntimeState::LoadingConfig));
    }

    #[test]
    fn test_fault_transition() {
        assert!(RuntimeState::Ready.can_transition_to(&RuntimeState::Fault));
        assert!(RuntimeState::Processing.can_transition_to(&RuntimeState::Fault));
        assert!(RuntimeState::Fault.can_transition_to(&RuntimeState::Recovering));
    }

    #[test]
    fn test_shutdown_transition() {
        assert!(RuntimeState::Ready.can_transition_to(&RuntimeState::ShuttingDown));
        assert!(RuntimeState::ShuttingDown.can_transition_to(&RuntimeState::Stopped));
    }

    #[test]
    fn test_runtime_new() {
        let runtime = Runtime::new();
        assert_eq!(runtime.state(), RuntimeState::Boot);
        assert_eq!(runtime.error_count(), 0);
    }

    #[test]
    fn test_health_check() {
        let mut runtime = Runtime::new();
        runtime.state = RuntimeState::Ready;
        let health = runtime.health_check();
        assert!(health.healthy);
        assert_eq!(health.error_count, 0);
    }

    #[test]
    fn test_record_error() {
        let mut runtime = Runtime::new();
        runtime.record_error("TestError", "test message", true);
        assert_eq!(runtime.error_count(), 1);
        assert!(runtime.recent_errors(1)[0].recoverable);
    }
}
