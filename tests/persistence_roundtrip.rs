use std::path::PathBuf;

fn test_config_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("cortex.toml")
}

fn test_state_path() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")).join("test_roundtrip.cx")
}

fn cleanup() {
    let _ = std::fs::remove_file(test_state_path());
}

#[test]
fn test_save_load_roundtrip() {
    cleanup();
    let config = cortex::config::CortexConfig::load(test_config_path().to_str().unwrap()).unwrap();
    let mut runtime = cortex::cortex::CortexRuntime::boot(config).unwrap();

    runtime.process("Test input for roundtrip").unwrap();
    let original_episode_count = runtime.state.metadata.episode_count;

    let path = test_state_path().to_str().unwrap().to_string();
    use cortex::persistence::PersistenceEngine;
    let persistence = cortex::persistence::PersistenceEngineImpl::new(&cortex::config::PersistenceConfig {
        state: path.clone(),
        checkpoint_interval: 1000,
    }).unwrap();
    persistence.save(&runtime.state).unwrap();

    let loaded_state = cortex::persistence::format::load_cx(&path).unwrap();
    assert_eq!(loaded_state.metadata.episode_count, original_episode_count);
    assert_eq!(loaded_state.metadata.state_id, runtime.state.metadata.state_id);

    cleanup();
}

#[test]
fn test_state_integrity_after_save() {
    cleanup();
    let config = cortex::config::CortexConfig::load(test_config_path().to_str().unwrap()).unwrap();
    let mut runtime = cortex::cortex::CortexRuntime::boot(config).unwrap();

    runtime.process("Integrity test input").unwrap();

    let path = test_state_path().to_str().unwrap().to_string();
    use cortex::persistence::PersistenceEngine;
    let persistence = cortex::persistence::PersistenceEngineImpl::new(&cortex::config::PersistenceConfig {
        state: path.clone(),
        checkpoint_interval: 1000,
    }).unwrap();
    persistence.save(&runtime.state).unwrap();

    assert!(cortex::persistence::format::verify_integrity(&path).unwrap());

    cleanup();
}

#[test]
fn test_state_corruption_detection() {
    cleanup();
    let config = cortex::config::CortexConfig::load(test_config_path().to_str().unwrap()).unwrap();
    let mut runtime = cortex::cortex::CortexRuntime::boot(config).unwrap();

    let path = test_state_path().to_str().unwrap().to_string();
    use cortex::persistence::PersistenceEngine;
    let persistence = cortex::persistence::PersistenceEngineImpl::new(&cortex::config::PersistenceConfig {
        state: path.clone(),
        checkpoint_interval: 1000,
    }).unwrap();
    persistence.save(&runtime.state).unwrap();

    let mut data = std::fs::read(&path).unwrap();
    if data.len() > 100 {
        data[100] = data[100].wrapping_add(1);
        std::fs::write(&path, &data).unwrap();
    }

    assert!(!cortex::persistence::format::verify_integrity(&path).unwrap());

    cleanup();
}

#[test]
fn test_checkpoint_save_load() {
    cleanup();
    let config = cortex::config::CortexConfig::load(test_config_path().to_str().unwrap()).unwrap();
    let mut runtime = cortex::cortex::CortexRuntime::boot(config).unwrap();

    runtime.process("Checkpoint test").unwrap();

    let path = test_state_path().to_str().unwrap().to_string();
    use cortex::persistence::PersistenceEngine;
    let persistence = cortex::persistence::PersistenceEngineImpl::new(&cortex::config::PersistenceConfig {
        state: path.clone(),
        checkpoint_interval: 1000,
    }).unwrap();
    let checkpoint_id = persistence.checkpoint(&runtime.state).unwrap();
    assert!(checkpoint_id.0 > 0);

    cleanup();
}
