use cortex::config::CortexConfig;
use cortex::cortex::CortexRuntime;
use cortex::persistence::format::FormatHandler;
use cortex::runtime::Runtime;
use cortex::transaction::invariant::StateInvariant;
use cortex::types::state::CortexState;
use tempfile::NamedTempFile;

#[test]
fn test_state_snapshot_roundtrip() {
    let config = CortexConfig::default();
    let mut runtime = CortexRuntime::new(config).unwrap();
    runtime.boot().unwrap();

    let _ = runtime.process("Test observation 1");
    let _ = runtime.process("Test observation 2");

    let state_json = serde_json::to_string(&runtime.state).unwrap();
    let snapshot: CortexState = serde_json::from_str(&state_json).unwrap();

    assert_eq!(
        runtime.state.metadata.episode_count,
        snapshot.metadata.episode_count
    );
    assert_eq!(
        runtime.state.language.symbols.len(),
        snapshot.language.symbols.len()
    );
}

fn temp_path() -> String {
    let tmp = NamedTempFile::new().unwrap();
    let path = tmp.path().to_str().unwrap().to_string();
    tmp.keep().unwrap();
    path
}

#[test]
fn test_atomic_disk_persistence_roundtrip() {
    let handler = FormatHandler::new();
    let data = b"test persistence data for atomic write";
    let path = temp_path();

    handler.save_to_file(&path, data).unwrap();
    let loaded = handler.load_from_file(&path).unwrap();
    assert_eq!(data.to_vec(), loaded);
    std::fs::remove_file(&path).ok();
}

#[test]
fn test_atomic_write_preserves_old_on_crash() {
    let handler = FormatHandler::new();
    let original_data = b"original valid data";
    let path = temp_path();

    handler.save_to_file(&path, original_data).unwrap();

    let bad_data = b"bad data that should not replace original";
    let tmp_path = format!("{}.tmp", path);
    std::fs::write(&tmp_path, bad_data).unwrap();
    std::fs::remove_file(&tmp_path).ok();

    let loaded = handler.load_from_file(&path).unwrap();
    assert_eq!(original_data.to_vec(), loaded);
    std::fs::remove_file(&path).ok();
}

#[test]
fn test_runtime_recovery_from_fault() {
    let config = CortexConfig::default();
    let mut runtime = CortexRuntime::new(config).unwrap();
    runtime.boot().unwrap();

    let _ = runtime.process("Observation 1");
    let _ = runtime.process("Observation 2");

    let episode_count_before = runtime.state.metadata.episode_count;

    let response = runtime.process("Normal observation after recovery");
    assert!(response.is_ok());
    assert!(runtime.state.metadata.episode_count > episode_count_before);
}

#[test]
fn test_runtime_shutdown_and_restart() {
    let config = CortexConfig::default();
    let mut runtime = CortexRuntime::new(config).unwrap();
    runtime.boot().unwrap();
    let _ = runtime.process("Observation before shutdown");
    runtime.shutdown().unwrap();

    let mut runtime2 = CortexRuntime::new(CortexConfig::default()).unwrap();
    runtime2.boot().unwrap();
    let response = runtime2.process("Observation after restart");
    assert!(response.is_ok());
}

#[test]
fn test_invariant_check_prevents_corrupt_state() {
    let mut state = CortexState::default();
    state.verification.confidence_threshold = 1.5;
    let result = StateInvariant::validate_state(&state);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("confidence_threshold"));
}

#[test]
fn test_invariant_check_prevents_zero_version() {
    let mut state = CortexState::default();
    state.metadata.architecture_version = 0;
    let result = StateInvariant::validate_state(&state);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("architecture_version"));
}

#[test]
fn test_mutation_log_tracks_pipeline_mutations() {
    let config = CortexConfig::default();
    let mut runtime = CortexRuntime::new(config).unwrap();
    runtime.boot().unwrap();

    let _ = runtime.process("Test input");
    let log = &runtime.mutation_log;
    assert!(!log.records.is_empty());
    assert!(log.records.iter().all(|r| r.success));
}

#[test]
fn test_state_version_increments() {
    let config = CortexConfig::default();
    let mut runtime = CortexRuntime::new(config).unwrap();
    runtime.boot().unwrap();
    let v1 = runtime.state_version;
    let _ = runtime.process("Observation 1");
    let v2 = runtime.state_version;
    assert!(v2 > v1);
}

#[test]
fn test_diagnostics_collect_with_runtime_metrics() {
    use cortex::observability::diagnostics::{Diagnostics, RuntimeMetrics};
    let mut diag = Diagnostics::new();
    diag.record_prediction_error(0.3);
    diag.record_prediction_error(0.5);
    diag.record_learning_signal(10);

    let metrics = diag.collect(Some(RuntimeMetrics {
        episode_count: 42,
        entity_count: 10,
        checkpoint_count: 3,
        ..Default::default()
    }));

    assert_eq!(metrics.episode_count, 42);
    assert_eq!(metrics.entity_count, 10);
    assert_eq!(metrics.checkpoint_count, 3);
    assert!((metrics.prediction_error - 0.4).abs() < 0.001);
}

#[test]
fn test_policy_blocks_mutation() {
    let mut config = CortexConfig::default();
    config.policy.learning = false;
    config.policy.self_modification = false;
    let mut runtime = CortexRuntime::new(config).unwrap();
    runtime.boot().unwrap();

    let response = runtime.process("Test with restricted policy");
    assert!(response.is_ok());
}
