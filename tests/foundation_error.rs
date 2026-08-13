use cortex::error::CortexError;
use std::error::Error;

#[test]
fn test_error_display_all_variants() {
    let errors = vec![
        CortexError::InputError("test".into()),
        CortexError::EncodingError("test".into()),
        CortexError::LanguageError("test".into()),
        CortexError::MemoryError("test".into()),
        CortexError::WorldModelError("test".into()),
        CortexError::ReasoningError("test".into()),
        CortexError::PlanningError("test".into()),
        CortexError::VerificationError("test".into()),
        CortexError::LearningError("test".into()),
        CortexError::PersistenceError("test".into()),
        CortexError::PolicyError("test".into()),
        CortexError::ResourceError("test".into()),
        CortexError::NetworkError("test".into()),
        CortexError::RuntimeError("test".into()),
        CortexError::ConfigError("test".into()),
        CortexError::StateError("test".into()),
        CortexError::SerializationError("test".into()),
        CortexError::SubsystemDisabled("test".into()),
    ];
    for e in &errors {
        let msg = e.to_string();
        assert!(!msg.is_empty());
        let _ = e.source();
    }
}

#[test]
fn test_error_is_recoverable() {
    assert!(CortexError::NetworkError("test".into()).is_recoverable());
    assert!(CortexError::ResourceError("test".into()).is_recoverable());
    assert!(CortexError::SubsystemDisabled("test".into()).is_recoverable());
    assert!(!CortexError::InputError("test".into()).is_recoverable());
    assert!(!CortexError::ConfigError("test".into()).is_recoverable());
    assert!(!CortexError::PersistenceError("test".into()).is_recoverable());
    assert!(!CortexError::RuntimeError("test".into()).is_recoverable());
    assert!(!CortexError::PolicyError("test".into()).is_recoverable());
}

#[test]
fn test_error_clone() {
    let e = CortexError::MemoryError("test".into());
    let cloned = e.clone();
    assert_eq!(e.to_string(), cloned.to_string());
}

#[test]
fn test_error_debug() {
    let e = CortexError::ConfigError("bad value".into());
    let debug = format!("{:?}", e);
    assert!(debug.contains("ConfigError"));
    assert!(debug.contains("bad value"));
}
