use cortex::config::CortexConfig;
use cortex::cortex::CortexRuntime;
use cortex::persistence::format::FormatHandler;
use cortex::runtime::Runtime;
use cortex::types::state::CortexState;

#[test]
fn test_state_persists_across_restarts() {
    let config = CortexConfig::default();
    let mut runtime = CortexRuntime::new(config).unwrap();
    runtime.boot().unwrap();

    let _ = runtime.process("First observation");
    let _ = runtime.process("Second observation");
    let episodes_after = runtime.state.metadata.episode_count;
    runtime.shutdown().unwrap();

    let mut runtime2 = CortexRuntime::new(CortexConfig::default()).unwrap();
    runtime2.boot().unwrap();
    assert!(runtime2.state.metadata.episode_count >= episodes_after);
    runtime2.shutdown().unwrap();
}

#[test]
fn test_state_serialize_roundtrip() {
    let state = CortexState::default();
    let bincode_data = bincode::serialize(&state).unwrap();
    let deserialized: CortexState = bincode::deserialize(&bincode_data).unwrap();
    assert_eq!(
        state.metadata.architecture_version,
        deserialized.metadata.architecture_version
    );
    assert_eq!(
        state.metadata.schema_version,
        deserialized.metadata.schema_version
    );
}

#[test]
fn test_format_handler_full_roundtrip() {
    let handler = FormatHandler::new();
    let state = CortexState::default();
    let bincode_data = bincode::serialize(&state).unwrap();
    let compressed = handler.serialize(&bincode_data).unwrap();
    let decompressed = handler.deserialize(&compressed).unwrap();
    assert_eq!(bincode_data, decompressed);
}

#[test]
fn test_checkpoint_creates_disk_state() {
    let config = CortexConfig::default();
    let mut runtime = CortexRuntime::new(config).unwrap();
    runtime.boot().unwrap();
    let _ = runtime.process("Checkpoint test");

    let result = runtime.save_state();
    assert!(result.is_ok());

    let path = &runtime.config.persistence.state;
    assert!(std::path::Path::new(path).exists());
    std::fs::remove_file(path).ok();
}

#[test]
fn test_corrupt_state_file_falls_back_to_default() {
    let path = "/tmp/cortex_corrupt_test.cx";
    std::fs::write(path, b"this is not valid state data").ok();

    let mut config = CortexConfig::default();
    config.persistence.state = path.to_string();
    let mut runtime = CortexRuntime::new(config).unwrap();
    runtime.boot().unwrap();

    assert_eq!(runtime.state.metadata.episode_count, 0);
    runtime.shutdown().unwrap();
    std::fs::remove_file(path).ok();
}

#[test]
fn test_pipeline_records_mutations_in_log() {
    let config = CortexConfig::default();
    let mut runtime = CortexRuntime::new(config).unwrap();
    runtime.boot().unwrap();
    let _ = runtime.process("Test input");

    let log = &runtime.mutation_log;
    let kinds: Vec<_> = log.records.iter().map(|r| r.kind).collect();
    assert!(kinds.contains(&cortex::transaction::mutation::MutationKind::LanguageEncode));
    assert!(kinds.contains(&cortex::transaction::mutation::MutationKind::NeuralProcess));
    assert!(kinds.contains(&cortex::transaction::mutation::MutationKind::MemoryStore));
    assert!(kinds.contains(&cortex::transaction::mutation::MutationKind::WorldIntegrate));
    assert!(kinds.contains(&cortex::transaction::mutation::MutationKind::ReasoningEvaluate));
    assert!(log.records.iter().all(|r| r.success));
}
