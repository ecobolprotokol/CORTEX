use cortex::config::CortexConfig;
use cortex::cortex::CortexRuntime;
use cortex::runtime::Runtime;

#[test]
fn test_full_lifecycle() {
    let config_path = "/tmp/cortex_lifecycle_test.toml";
    let state_path = "/tmp/cortex_lifecycle_test.cx";
    std::fs::remove_file(config_path).ok();
    std::fs::remove_file(state_path).ok();
    let mut config = CortexConfig::default();
    config.persistence.state = state_path.into();
    config.persistence.checkpoint_interval = 5;
    config.learning.consolidation_interval = 10;

    let toml = r#"
[model]
cells = 256
columns = 16
dimension = 64
precision = "f32"
sparsity_ratio = 0.1

[language]
enabled = true
vocabulary_capacity = 1024
context_window = 256
generation_limit = 128
learning = true

[memory]
working_mb = 16
episodic_mb = 32
semantic_mb = 32
procedural_mb = 16
associative_mb = 16

[learning]
enabled = true
learning_rate = 0.001
plasticity = 0.01
replay = true
consolidation_interval = 10

[world]
enabled = true
prediction_horizon = 4

[reasoning]
enabled = true
max_steps = 16

[planning]
enabled = true
max_depth = 4
max_branches = 8

[verification]
enabled = true
minimum_confidence = 0.5

[internet]
enabled = true
timeout_seconds = 15
max_response_mb = 4

[policy]
learning = true
internet_learning = true
self_modification = false
policy_modification = false
runtime_modification = false

[api]
enabled = true
bind = "127.0.0.1:8080"
api_key_env = "CORTEX_API_KEY"

[persistence]
state = "/tmp/cortex_lifecycle_test.cx"
checkpoint_interval = 5
"#;
    std::fs::write(config_path, toml).unwrap();

    // Phase 1: Boot and process observations
    let cfg1 = CortexConfig::load(config_path).unwrap();
    let mut rt1 = CortexRuntime::new(cfg1).unwrap();
    rt1.boot().unwrap();

    assert!(rt1.ready());
    assert!(rt1.state.metadata.architecture_version >= 1);

    let inputs = vec![
        "What is gravity?",
        "How does photosynthesis work?",
        "Explain quantum computing",
        "What is the speed of light?",
        "How do computers store data?",
    ];
    for input in &inputs {
        let response = rt1.process(input).unwrap();
        assert!(!response.is_empty());
    }

    assert!(rt1.state.metadata.episode_count >= 5);
    assert!(rt1.language_vocabulary.size() > 0);

    let episodes_before = rt1.state.metadata.episode_count;

    // Phase 2: Save state to disk
    rt1.save_state().unwrap();
    assert!(std::path::Path::new(&rt1.config.persistence.state).exists());

    // Phase 3: Shutdown
    rt1.shutdown().unwrap();
    assert!(!rt1.ready());

    // Phase 4: Reboot from disk
    let cfg2 = CortexConfig::load(config_path).unwrap();
    let mut rt2 = CortexRuntime::new(cfg2).unwrap();
    rt2.boot().unwrap();

    assert!(rt2.ready());
    assert!(rt2.state.metadata.episode_count >= episodes_before);
    assert!(rt2.state_version > 0);

    // Phase 5: Process after reboot
    let response = rt2.process("Post-reboot observation").unwrap();
    assert!(!response.is_empty());
    assert!(rt2.state.metadata.episode_count > episodes_before);

    // Phase 6: Shutdown again
    rt2.shutdown().unwrap();

    // Cleanup
    std::fs::remove_file(config_path).ok();
    std::fs::remove_file(state_path).ok();
}

#[test]
fn test_cli_dispatch_observe() {
    let args = vec![
        "cortex".to_string(),
        "observe".to_string(),
        "test input".to_string(),
    ];
    let result = cortex::cli::commands::dispatch(&args);
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.is_empty());
}

#[test]
fn test_cli_dispatch_help() {
    let args = vec!["cortex".to_string(), "help".to_string()];
    let result = cortex::cli::commands::dispatch(&args);
    assert!(result.is_ok());
    assert!(result.unwrap().contains("CORTEX"));
}

#[test]
fn test_cli_dispatch_version() {
    let args = vec!["cortex".to_string(), "version".to_string()];
    let result = cortex::cli::commands::dispatch(&args);
    assert!(result.is_ok());
    assert!(result.unwrap().contains("CORTEX v"));
}

#[test]
fn test_api_manager_routing() {
    let mut api = cortex::api::ApiManager::new("test-key");
    let mut rt = CortexRuntime::new(CortexConfig::default()).unwrap();
    rt.boot().unwrap();
    let result = api.handle_request(&mut rt, "POST", "/v1/inference", None, Some("hello"));
    assert!(result.is_ok());
    assert!(!result.unwrap().is_empty());
    rt.shutdown().unwrap();
}

#[test]
fn test_api_manager_auth_rejection() {
    let mut api = cortex::api::ApiManager::new("secret-key");
    let mut rt = CortexRuntime::new(CortexConfig::default()).unwrap();
    rt.boot().unwrap();
    let result = api.handle_request(&mut rt, "POST", "/v1/inference", Some("wrong-key"), None);
    assert!(result.is_err());
    rt.shutdown().unwrap();
}

#[test]
fn test_api_manager_status_endpoint() {
    let mut api = cortex::api::ApiManager::new("test-key");
    let mut rt = CortexRuntime::new(CortexConfig::default()).unwrap();
    rt.boot().unwrap();
    let result = api.handle_request(&mut rt, "GET", "/v1/status", None, None);
    assert!(result.is_ok());
    assert!(!result.unwrap().is_empty());
    rt.shutdown().unwrap();
}
