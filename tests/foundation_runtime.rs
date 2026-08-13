use cortex::config::CortexConfig;
use cortex::cortex::CortexRuntime;
use cortex::runtime::{Runtime, RuntimeState};

#[test]
fn test_runtime_state_transitions() {
    let config = CortexConfig::default();
    let mut runtime = CortexRuntime::new(config).unwrap();
    assert_eq!(runtime.runtime_state, RuntimeState::Booting);
    runtime.boot().unwrap();
    assert!(runtime.ready());
    runtime.shutdown().unwrap();
    assert_eq!(runtime.runtime_state, RuntimeState::Stopped);
}

#[test]
fn test_invalid_transition_fails() {
    let result = RuntimeState::Ready.can_transition_to(&RuntimeState::Booting);
    assert!(!result);
}

#[test]
fn test_valid_transition_boot_sequence() {
    assert!(RuntimeState::Booting.can_transition_to(&RuntimeState::LoadingConfig));
    assert!(RuntimeState::LoadingConfig.can_transition_to(&RuntimeState::LoadingState));
    assert!(RuntimeState::LoadingState.can_transition_to(&RuntimeState::Validating));
    assert!(RuntimeState::Validating.can_transition_to(&RuntimeState::Initializing));
    assert!(RuntimeState::Initializing.can_transition_to(&RuntimeState::Ready));
}

#[test]
fn test_valid_transition_operational() {
    assert!(RuntimeState::Ready.can_transition_to(&RuntimeState::Processing));
    assert!(RuntimeState::Processing.can_transition_to(&RuntimeState::Ready));
    assert!(RuntimeState::Ready.can_transition_to(&RuntimeState::ShuttingDown));
}

#[test]
fn test_fault_transition() {
    let states = [
        RuntimeState::Ready,
        RuntimeState::Processing,
        RuntimeState::Learning,
    ];
    for state in &states {
        assert!(
            state.can_transition_to(&RuntimeState::Fault),
            "{:?} should transition to Fault",
            state
        );
    }
}

#[test]
fn test_recovery_from_fault() {
    assert!(RuntimeState::Fault.can_transition_to(&RuntimeState::Recovering));
    assert!(RuntimeState::Fault.can_transition_to(&RuntimeState::Stopped));
    assert!(RuntimeState::Recovering.can_transition_to(&RuntimeState::Ready));
}

#[test]
fn test_runtime_state_properties() {
    assert!(!RuntimeState::Booting.is_terminal());
    assert!(!RuntimeState::Booting.is_fault());
    assert!(RuntimeState::Booting.is_operational());
    assert!(RuntimeState::Ready.is_operational());
    assert!(!RuntimeState::Ready.is_fault());
    assert!(RuntimeState::Stopped.is_terminal());
    assert!(!RuntimeState::Stopped.is_operational());
    assert!(RuntimeState::Fault.is_fault());
    assert!(!RuntimeState::Fault.is_operational());
}

#[test]
fn test_cannot_boot_twice() {
    let config = CortexConfig::default();
    let mut runtime = CortexRuntime::new(config).unwrap();
    runtime.boot().unwrap();
    let result = runtime.boot();
    assert!(result.is_err());
}

#[test]
fn test_cannot_process_when_not_ready() {
    let config = CortexConfig::default();
    let mut runtime = CortexRuntime::new(config).unwrap();
    let result = runtime.process("hello");
    assert!(result.is_err());
}

#[test]
fn test_shutdown_stops_processing() {
    let config = CortexConfig::default();
    let mut runtime = CortexRuntime::new(config).unwrap();
    runtime.boot().unwrap();
    runtime.shutdown().unwrap();
    let result = runtime.process("hello");
    assert!(result.is_err());
}
