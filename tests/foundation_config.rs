use cortex::config::CortexConfig;

#[test]
fn test_config_default_valid() {
    let config = CortexConfig::default();
    assert!(config.validate().is_ok());
}

#[test]
fn test_config_load_file_not_found() {
    let result = CortexConfig::load("/nonexistent/config.toml");
    assert!(result.is_ok());
}

#[test]
fn test_config_validation_min_bounds() {
    let mut config = CortexConfig::default();
    config.model.cells = 255;
    assert!(config.validate().is_err());
    assert!(config.validate().unwrap_err().to_string().contains("cells"));
}

#[test]
fn test_config_validation_sparsity_zero() {
    let mut config = CortexConfig::default();
    config.model.sparsity_ratio = 0.0;
    assert!(config.validate().is_err());
}

#[test]
fn test_config_validation_learning_rate_bounds() {
    let mut config = CortexConfig::default();
    config.learning.learning_rate = 0.0;
    assert!(config.validate().is_err());
    config.learning.learning_rate = 1.1;
    assert!(config.validate().is_err());
    config.learning.learning_rate = 0.5;
    assert!(config.validate().is_ok());
}

#[test]
fn test_config_validation_checkpoint_interval_zero() {
    let mut config = CortexConfig::default();
    config.persistence.checkpoint_interval = 0;
    assert!(config.validate().is_err());
}

#[test]
fn test_config_env_override() {
    std::env::set_var("CORTEX_MODEL_CELLS", "512");
    std::env::set_var("CORTEX_API_BIND", "0.0.0.0:1234");
    let config = CortexConfig::default().apply_env_overrides();
    assert_eq!(config.model.cells, 512);
    assert_eq!(config.api.bind, "0.0.0.0:1234");
    std::env::remove_var("CORTEX_MODEL_CELLS");
    std::env::remove_var("CORTEX_API_BIND");
}

#[test]
fn test_config_env_override_invalid_ignored() {
    std::env::set_var("CORTEX_MODEL_CELLS", "abc");
    let config = CortexConfig::default().apply_env_overrides();
    assert_eq!(config.model.cells, 4096);
    std::env::remove_var("CORTEX_MODEL_CELLS");
}

#[test]
fn test_config_toml_parse() {
    let toml = r#"
[model]
cells = 2048
columns = 32
dimension = 128
precision = "f32"
sparsity_ratio = 0.1

[language]
enabled = true
vocabulary_capacity = 1024
context_window = 256
generation_limit = 128
learning = true

[memory]
working_mb = 64
episodic_mb = 128
semantic_mb = 128
procedural_mb = 64
associative_mb = 64

[learning]
enabled = true
learning_rate = 0.01
plasticity = 0.05
replay = true
consolidation_interval = 500

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
minimum_confidence = 0.9

[internet]
enabled = true
timeout_seconds = 30
max_response_mb = 8

[policy]
learning = true
internet_learning = true
self_modification = false
policy_modification = false
runtime_modification = false

[api]
enabled = true
bind = "0.0.0.0:9090"
api_key_env = "MY_KEY"

[persistence]
state = "test.cx"
checkpoint_interval = 200
"#;
    let path = "/tmp/cortex_test_foundation.toml";
    std::fs::write(path, toml).unwrap();
    let config = CortexConfig::load(path).unwrap();
    assert_eq!(config.model.cells, 2048);
    assert_eq!(config.language.vocabulary_capacity, 1024);
    assert_eq!(config.api.bind, "0.0.0.0:9090");
    assert_eq!(config.persistence.checkpoint_interval, 200);
    std::fs::remove_file(path).ok();
}
