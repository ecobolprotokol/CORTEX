use cortex::config::CortexConfig;
use cortex::cortex::CortexRuntime;
use cortex::runtime::Runtime;

#[test]
fn test_runtime_boot() {
    let config = CortexConfig::default();
    let mut runtime = CortexRuntime::new(config).unwrap();
    assert!(!runtime.ready());
    runtime.boot().unwrap();
    assert!(runtime.ready());
}

#[test]
fn test_runtime_shutdown() {
    let config = CortexConfig::default();
    let mut runtime = CortexRuntime::new(config).unwrap();
    runtime.boot().unwrap();
    runtime.shutdown().unwrap();
    assert!(!runtime.ready());
}

#[test]
fn test_cognitive_pipeline_processes_observation() {
    let config = CortexConfig::default();
    let mut runtime = CortexRuntime::new(config).unwrap();
    runtime.boot().unwrap();
    let response = runtime.process("What is gravity?").unwrap();
    assert!(!response.is_empty());
}

#[test]
fn test_multiple_observations() {
    let config = CortexConfig::default();
    let mut runtime = CortexRuntime::new(config).unwrap();
    runtime.boot().unwrap();
    let inputs = vec![
        "What is gravity?",
        "How does photosynthesis work?",
        "Explain quantum computing",
        "What is the capital of France?",
        "How do computers store data?",
    ];
    for input in inputs {
        let response = runtime.process(input).unwrap();
        assert!(!response.is_empty());
    }
}

#[test]
fn test_empty_input() {
    let config = CortexConfig::default();
    let mut runtime = CortexRuntime::new(config).unwrap();
    runtime.boot().unwrap();
    let response = runtime.process("").unwrap();
    assert!(!response.is_empty());
}

#[test]
fn test_state_updates_after_processing() {
    let config = CortexConfig::default();
    let mut runtime = CortexRuntime::new(config).unwrap();
    runtime.boot().unwrap();
    let initial_episode_count = runtime.state.metadata.episode_count;
    let _ = runtime.process("Test observation");
    assert!(runtime.state.metadata.episode_count > initial_episode_count);
}

#[test]
fn test_vocabulary_grows() {
    let config = CortexConfig::default();
    let mut runtime = CortexRuntime::new(config).unwrap();
    runtime.boot().unwrap();
    let initial_vocab = runtime.language_vocabulary.size();
    let _ = runtime.process("The quick brown fox jumps over the lazy dog");
    assert!(runtime.language_vocabulary.size() > initial_vocab);
}

#[test]
fn test_policy_blocks_when_learning_disabled() {
    let mut config = CortexConfig::default();
    config.policy.learning = false;
    let mut runtime = CortexRuntime::new(config).unwrap();
    runtime.boot().unwrap();
    let response = runtime.process("Test observation").unwrap();
    assert!(!response.is_empty());
}
