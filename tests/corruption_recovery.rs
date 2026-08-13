use std::path::PathBuf;

fn test_config_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("cortex.toml")
}

fn test_state_path() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")).join("test_corruption.cx")
}

fn cleanup() {
    let _ = std::fs::remove_file(test_state_path());
}

#[test]
fn test_corruption_detection() {
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

    assert!(cortex::persistence::format::verify_integrity(&path).unwrap());

    let mut data = std::fs::read(&path).unwrap();
    if data.len() > 100 {
        data[100] = data[100].wrapping_add(1);
        std::fs::write(&path, &data).unwrap();
    }

    assert!(!cortex::persistence::format::verify_integrity(&path).unwrap());

    let result = cortex::persistence::format::load_cx(&path);
    assert!(result.is_err());

    cleanup();
}

#[test]
fn test_empty_file_handling() {
    cleanup();
    let path = test_state_path().to_str().unwrap().to_string();
    std::fs::write(&path, "").unwrap();

    let result = cortex::persistence::format::load_cx(&path);
    assert!(result.is_err());

    cleanup();
}

#[test]
fn test_truncated_file_handling() {
    cleanup();
    let path = test_state_path().to_str().unwrap().to_string();
    std::fs::write(&path, b"short").unwrap();

    let result = cortex::persistence::format::load_cx(&path);
    assert!(result.is_err());

    cleanup();
}

#[test]
fn test_valid_state_loads_after_save() {
    cleanup();
    let config = cortex::config::CortexConfig::load(test_config_path().to_str().unwrap()).unwrap();
    let mut runtime = cortex::cortex::CortexRuntime::boot(config).unwrap();

    runtime.process("Corruption recovery test").unwrap();

    let path = test_state_path().to_str().unwrap().to_string();

    use cortex::persistence::PersistenceEngine;
    let persistence = cortex::persistence::PersistenceEngineImpl::new(&cortex::config::PersistenceConfig {
        state: path.clone(),
        checkpoint_interval: 1000,
    }).unwrap();
    persistence.save(&runtime.state).unwrap();

    let loaded = cortex::persistence::format::load_cx(&path).unwrap();
    assert_eq!(loaded.metadata.state_id, runtime.state.metadata.state_id);
    assert!(loaded.metadata.episode_count >= 1);

    cleanup();
}
