use std::path::PathBuf;

fn test_config_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("cortex.toml")
}

fn test_state_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_cortex.cx")
}

fn cleanup() {
    let _ = std::fs::remove_file(test_state_path());
    let _ = std::fs::remove_dir_all(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("checkpoints"),
    );
}

#[test]
fn test_full_cognitive_pipeline() {
    cleanup();
    let config = cortex::config::CortexConfig::load(test_config_path().to_str().unwrap()).unwrap();
    let mut runtime = cortex::cortex::CortexRuntime::boot(config).unwrap();

    let response = runtime.process("Hello, what is gravity?").unwrap();
    assert!(!response.is_empty());

    let response2 = runtime.process("Tell me more about physics").unwrap();
    assert!(!response2.is_empty());

    assert!(runtime.state.metadata.episode_count >= 2);

    runtime.save().unwrap();
    cleanup();
}

#[test]
fn test_cognitive_pipeline_produces_language_state() {
    cleanup();
    let config = cortex::config::CortexConfig::load(test_config_path().to_str().unwrap()).unwrap();
    let mut runtime = cortex::cortex::CortexRuntime::boot(config).unwrap();

    let _response = runtime.process("The temperature is 25 degrees").unwrap();

    use cortex::language::LanguageCore;
    assert!(runtime.language.vocabulary_size() > 0);
    assert!(!runtime.language.state().symbols.is_empty());

    runtime.save().unwrap();
    cleanup();
}

#[test]
fn test_cognitive_pipeline_neural_processing() {
    cleanup();
    let config = cortex::config::CortexConfig::load(test_config_path().to_str().unwrap()).unwrap();
    let mut runtime = cortex::cortex::CortexRuntime::boot(config).unwrap();

    let _response = runtime.process("Water boils at 100 degrees Celsius").unwrap();

    use cortex::neural::NeuralCore;
    assert!(!runtime.neural.state().fields.is_empty());
    assert!(runtime.neural.state().active_cells.len() > 0 || runtime.neural.state().fields.iter().any(|f| f.average_activation > 0.0));

    runtime.save().unwrap();
    cleanup();
}

#[test]
fn test_multiple_interactions_accumulate_memory() {
    cleanup();
    let config = cortex::config::CortexConfig::load(test_config_path().to_str().unwrap()).unwrap();
    let mut runtime = cortex::cortex::CortexRuntime::boot(config).unwrap();

    runtime.process("The sky is blue").unwrap();
    runtime.process("Grass is green").unwrap();
    runtime.process("Fire is hot").unwrap();

    assert!(runtime.state.metadata.episode_count >= 3);

    use cortex::memory::MemorySystem;
    assert!(runtime.memory.working_memory().conversation_context.turn_count >= 3);

    runtime.save().unwrap();
    cleanup();
}

#[test]
fn test_cognitive_pipeline_with_question() {
    cleanup();
    let config = cortex::config::CortexConfig::load(test_config_path().to_str().unwrap()).unwrap();
    let mut runtime = cortex::cortex::CortexRuntime::boot(config).unwrap();

    let response = runtime.process("What is the meaning of life?").unwrap();
    assert!(!response.is_empty());

    runtime.save().unwrap();
    cleanup();
}
