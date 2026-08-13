use std::path::PathBuf;

fn test_config_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("cortex.toml")
}

fn cleanup() {
    let _ = std::fs::remove_file("test_learning_cx");
}

#[test]
fn test_learning_stability_guard() {
    cleanup();
    let config = cortex::config::CortexConfig::load(test_config_path().to_str().unwrap()).unwrap();
    let mut runtime = cortex::cortex::CortexRuntime::boot(config).unwrap();

    use cortex::learning::LearningSystem;
    let initial_error = runtime.learning.state().average_prediction_error;
    for i in 0..10 {
        runtime.process(&format!("Test input {}", i)).unwrap();
    }

    let final_error = runtime.learning.state().average_prediction_error;
    assert!(final_error < 1.0, "Prediction error should be bounded");
    assert!(runtime.state.metadata.episode_count >= 10);

    cleanup();
}

#[test]
fn test_single_observation_no_destabilization() {
    cleanup();
    let config = cortex::config::CortexConfig::load(test_config_path().to_str().unwrap()).unwrap();
    let mut runtime = cortex::cortex::CortexRuntime::boot(config).unwrap();

    runtime.process("Normal input").unwrap();

    let state_before = runtime.state.clone();
    runtime.process("A very long and complex input with many words that might destabilize the system if learning is not properly bounded and guarded").unwrap();

    let episode_diff = runtime.state.metadata.episode_count - state_before.metadata.episode_count;
    assert_eq!(episode_diff, 1, "Only one episode should be added");

    cleanup();
}

#[test]
fn test_learning_disabled_no_state_change() {
    cleanup();
    let mut config = cortex::config::CortexConfig::load(test_config_path().to_str().unwrap()).unwrap();
    config.learning.enabled = false;

    let mut runtime = cortex::cortex::CortexRuntime::boot(config).unwrap();
    runtime.process("Test with learning disabled").unwrap();

    use cortex::learning::LearningSystem;
    assert_eq!(runtime.learning.state().total_learning_events, 0);

    cleanup();
}
