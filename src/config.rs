use serde::{Deserialize, Serialize};
use std::env;

use crate::error::CortexError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CortexConfig {
    pub model: ModelConfig,
    pub language: LanguageConfig,
    pub memory: MemoryConfig,
    pub learning: LearningConfig,
    pub world: WorldConfig,
    pub reasoning: ReasoningConfig,
    pub planning: PlanningConfig,
    pub verification: VerificationConfig,
    pub internet: InternetConfig,
    pub policy: PolicyConfig,
    pub api: ApiConfig,
    pub persistence: PersistenceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub cells: u32,
    pub columns: u32,
    pub dimension: u32,
    pub precision: String,
    pub sparsity_ratio: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageConfig {
    pub enabled: bool,
    pub vocabulary_capacity: u32,
    pub context_window: u32,
    pub generation_limit: u32,
    pub learning: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    pub working_mb: u32,
    pub episodic_mb: u32,
    pub semantic_mb: u32,
    pub procedural_mb: u32,
    pub associative_mb: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningConfig {
    pub enabled: bool,
    pub learning_rate: f32,
    pub plasticity: f32,
    pub replay: bool,
    pub consolidation_interval: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldConfig {
    pub enabled: bool,
    pub prediction_horizon: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningConfig {
    pub enabled: bool,
    pub max_steps: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanningConfig {
    pub enabled: bool,
    pub max_depth: u32,
    pub max_branches: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationConfig {
    pub enabled: bool,
    pub minimum_confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternetConfig {
    pub enabled: bool,
    pub timeout_seconds: u32,
    pub max_response_mb: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyConfig {
    pub learning: bool,
    pub internet_learning: bool,
    pub self_modification: bool,
    pub policy_modification: bool,
    pub runtime_modification: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub enabled: bool,
    pub bind: String,
    pub api_key_env: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistenceConfig {
    pub state: String,
    pub checkpoint_interval: u64,
}

impl Default for CortexConfig {
    fn default() -> Self {
        Self {
            model: ModelConfig {
                cells: 4096,
                columns: 64,
                dimension: 256,
                precision: "f32".into(),
                sparsity_ratio: 0.05,
            },
            language: LanguageConfig {
                enabled: true,
                vocabulary_capacity: 65536,
                context_window: 4096,
                generation_limit: 1024,
                learning: true,
            },
            memory: MemoryConfig {
                working_mb: 128,
                episodic_mb: 512,
                semantic_mb: 512,
                procedural_mb: 256,
                associative_mb: 256,
            },
            learning: LearningConfig {
                enabled: true,
                learning_rate: 0.001,
                plasticity: 0.01,
                replay: true,
                consolidation_interval: 1000,
            },
            world: WorldConfig {
                enabled: true,
                prediction_horizon: 8,
            },
            reasoning: ReasoningConfig {
                enabled: true,
                max_steps: 32,
            },
            planning: PlanningConfig {
                enabled: true,
                max_depth: 8,
                max_branches: 16,
            },
            verification: VerificationConfig {
                enabled: true,
                minimum_confidence: 0.80,
            },
            internet: InternetConfig {
                enabled: true,
                timeout_seconds: 15,
                max_response_mb: 4,
            },
            policy: PolicyConfig {
                learning: true,
                internet_learning: true,
                self_modification: false,
                policy_modification: false,
                runtime_modification: false,
            },
            api: ApiConfig {
                enabled: true,
                bind: "127.0.0.1:8080".into(),
                api_key_env: "CORTEX_API_KEY".into(),
            },
            persistence: PersistenceConfig {
                state: "cortex.cx".into(),
                checkpoint_interval: 1000,
            },
        }
    }
}

impl CortexConfig {
    pub fn load(path: &str) -> Result<Self, CortexError> {
        let config = if std::path::Path::new(path).exists() {
            let content = std::fs::read_to_string(path).map_err(|e| {
                CortexError::ConfigError(format!("Failed to read config '{}': {}", path, e))
            })?;
            let config: Self = toml::from_str(&content).map_err(|e| {
                CortexError::ConfigError(format!("Failed to parse config '{}': {}", path, e))
            })?;
            config
        } else {
            tracing::debug!(path = %path, "Config file not found, using defaults");
            Self::default()
        };
        let config = config.apply_env_overrides();
        config.validate()?;
        tracing::debug!("Configuration loaded and validated");
        Ok(config)
    }

    pub fn validate(&self) -> Result<(), CortexError> {
        if self.model.cells < 256 {
            return Err(CortexError::ConfigError(
                "model.cells must be >= 256".into(),
            ));
        }
        if self.model.columns < 16 {
            return Err(CortexError::ConfigError(
                "model.columns must be >= 16".into(),
            ));
        }
        if self.model.dimension < 64 {
            return Err(CortexError::ConfigError(
                "model.dimension must be >= 64".into(),
            ));
        }
        if self.model.sparsity_ratio <= 0.0 || self.model.sparsity_ratio > 1.0 {
            return Err(CortexError::ConfigError(
                "model.sparsity_ratio must be in (0, 1]".into(),
            ));
        }
        if self.language.vocabulary_capacity < 256 {
            return Err(CortexError::ConfigError(
                "language.vocabulary_capacity must be >= 256".into(),
            ));
        }
        if self.language.context_window < 64 {
            return Err(CortexError::ConfigError(
                "language.context_window must be >= 64".into(),
            ));
        }
        if self.language.generation_limit < 32 {
            return Err(CortexError::ConfigError(
                "language.generation_limit must be >= 32".into(),
            ));
        }
        if self.memory.working_mb < 16 {
            return Err(CortexError::ConfigError(
                "memory.working_mb must be >= 16".into(),
            ));
        }
        if self.memory.episodic_mb < 32 {
            return Err(CortexError::ConfigError(
                "memory.episodic_mb must be >= 32".into(),
            ));
        }
        if self.memory.semantic_mb < 32 {
            return Err(CortexError::ConfigError(
                "memory.semantic_mb must be >= 32".into(),
            ));
        }
        if self.learning.learning_rate <= 0.0 || self.learning.learning_rate > 1.0 {
            return Err(CortexError::ConfigError(
                "learning.learning_rate must be in (0, 1]".into(),
            ));
        }
        if self.learning.plasticity < 0.0 || self.learning.plasticity > 1.0 {
            return Err(CortexError::ConfigError(
                "learning.plasticity must be in [0, 1]".into(),
            ));
        }
        if self.verification.minimum_confidence < 0.0 || self.verification.minimum_confidence > 1.0
        {
            return Err(CortexError::ConfigError(
                "verification.minimum_confidence must be in [0, 1]".into(),
            ));
        }
        if self.persistence.checkpoint_interval == 0 {
            return Err(CortexError::ConfigError(
                "persistence.checkpoint_interval must be > 0".into(),
            ));
        }
        if self.model.cells % self.model.columns != 0 {
            return Err(CortexError::ConfigError(
                "model.cells must be evenly divisible by model.columns".into(),
            ));
        }
        if self.language.context_window < self.language.generation_limit {
            return Err(CortexError::ConfigError(
                "language.context_window must be >= language.generation_limit".into(),
            ));
        }
        if self.memory.procedural_mb < 16 {
            return Err(CortexError::ConfigError(
                "memory.procedural_mb must be >= 16".into(),
            ));
        }
        if self.memory.associative_mb < 16 {
            return Err(CortexError::ConfigError(
                "memory.associative_mb must be >= 16".into(),
            ));
        }
        Ok(())
    }

    pub fn apply_env_overrides(mut self) -> Self {
        if let Ok(v) = env::var("CORTEX_MODEL_CELLS") {
            if let Ok(n) = v.parse() {
                self.model.cells = n;
            }
        }
        if let Ok(v) = env::var("CORTEX_MODEL_COLUMNS") {
            if let Ok(n) = v.parse() {
                self.model.columns = n;
            }
        }
        if let Ok(v) = env::var("CORTEX_MODEL_DIMENSION") {
            if let Ok(n) = v.parse() {
                self.model.dimension = n;
            }
        }
        if let Ok(v) = env::var("CORTEX_MODEL_SPARSITY_RATIO") {
            if let Ok(n) = v.parse() {
                self.model.sparsity_ratio = n;
            }
        }
        if let Ok(v) = env::var("CORTEX_LANGUAGE_ENABLED") {
            if let Ok(n) = v.parse() {
                self.language.enabled = n;
            }
        }
        if let Ok(v) = env::var("CORTEX_LANGUAGE_VOCABULARY_CAPACITY") {
            if let Ok(n) = v.parse() {
                self.language.vocabulary_capacity = n;
            }
        }
        if let Ok(v) = env::var("CORTEX_LANGUAGE_CONTEXT_WINDOW") {
            if let Ok(n) = v.parse() {
                self.language.context_window = n;
            }
        }
        if let Ok(v) = env::var("CORTEX_LANGUAGE_GENERATION_LIMIT") {
            if let Ok(n) = v.parse() {
                self.language.generation_limit = n;
            }
        }
        if let Ok(v) = env::var("CORTEX_MEMORY_WORKING_MB") {
            if let Ok(n) = v.parse() {
                self.memory.working_mb = n;
            }
        }
        if let Ok(v) = env::var("CORTEX_MEMORY_EPISODIC_MB") {
            if let Ok(n) = v.parse() {
                self.memory.episodic_mb = n;
            }
        }
        if let Ok(v) = env::var("CORTEX_MEMORY_SEMANTIC_MB") {
            if let Ok(n) = v.parse() {
                self.memory.semantic_mb = n;
            }
        }
        if let Ok(v) = env::var("CORTEX_LEARNING_ENABLED") {
            if let Ok(n) = v.parse() {
                self.learning.enabled = n;
            }
        }
        if let Ok(v) = env::var("CORTEX_LEARNING_RATE") {
            if let Ok(n) = v.parse() {
                self.learning.learning_rate = n;
            }
        }
        if let Ok(v) = env::var("CORTEX_LEARNING_PLASTICITY") {
            if let Ok(n) = v.parse() {
                self.learning.plasticity = n;
            }
        }
        if let Ok(v) = env::var("CORTEX_WORLD_ENABLED") {
            if let Ok(n) = v.parse() {
                self.world.enabled = n;
            }
        }
        if let Ok(v) = env::var("CORTEX_REASONING_ENABLED") {
            if let Ok(n) = v.parse() {
                self.reasoning.enabled = n;
            }
        }
        if let Ok(v) = env::var("CORTEX_REASONING_MAX_STEPS") {
            if let Ok(n) = v.parse() {
                self.reasoning.max_steps = n;
            }
        }
        if let Ok(v) = env::var("CORTEX_PLANNING_ENABLED") {
            if let Ok(n) = v.parse() {
                self.planning.enabled = n;
            }
        }
        if let Ok(v) = env::var("CORTEX_VERIFICATION_ENABLED") {
            if let Ok(n) = v.parse() {
                self.verification.enabled = n;
            }
        }
        if let Ok(v) = env::var("CORTEX_VERIFICATION_MINIMUM_CONFIDENCE") {
            if let Ok(n) = v.parse() {
                self.verification.minimum_confidence = n;
            }
        }
        if let Ok(v) = env::var("CORTEX_INTERNET_ENABLED") {
            if let Ok(n) = v.parse() {
                self.internet.enabled = n;
            }
        }
        if let Ok(v) = env::var("CORTEX_API_ENABLED") {
            if let Ok(n) = v.parse() {
                self.api.enabled = n;
            }
        }
        if let Ok(v) = env::var("CORTEX_API_BIND") {
            self.api.bind = v;
        }
        if let Ok(v) = env::var("CORTEX_PERSISTENCE_STATE") {
            self.persistence.state = v;
        }
        if let Ok(v) = env::var("CORTEX_PERSISTENCE_CHECKPOINT_INTERVAL") {
            if let Ok(n) = v.parse() {
                self.persistence.checkpoint_interval = n;
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_valid() {
        let config = CortexConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_load_nonexistent_file_uses_defaults() {
        let config = CortexConfig::load("/nonexistent/path/cortex.toml").unwrap();
        assert_eq!(config.model.cells, 4096);
        assert_eq!(config.language.vocabulary_capacity, 65536);
    }

    #[test]
    fn test_load_valid_toml() {
        let toml_content = r#"
[model]
cells = 1024
columns = 32
dimension = 128
precision = "f32"
sparsity_ratio = 0.1

[language]
enabled = false
vocabulary_capacity = 1024
context_window = 256
generation_limit = 128
learning = false

[memory]
working_mb = 64
episodic_mb = 128
semantic_mb = 128
procedural_mb = 64
associative_mb = 64

[learning]
enabled = false
learning_rate = 0.01
plasticity = 0.05
replay = false
consolidation_interval = 500

[world]
enabled = false
prediction_horizon = 4

[reasoning]
enabled = false
max_steps = 16

[planning]
enabled = false
max_depth = 4
max_branches = 8

[verification]
enabled = false
minimum_confidence = 0.9

[internet]
enabled = false
timeout_seconds = 30
max_response_mb = 8

[policy]
learning = false
internet_learning = false
self_modification = false
policy_modification = false
runtime_modification = false

[api]
enabled = false
bind = "0.0.0.0:9090"
api_key_env = "MY_API_KEY"

[persistence]
state = "/tmp/test.cx"
checkpoint_interval = 500
"#;
        let path = "/tmp/cortex_test_config.toml";
        std::fs::write(path, toml_content).unwrap();
        let config = CortexConfig::load(path).unwrap();
        assert_eq!(config.model.cells, 1024);
        assert_eq!(config.model.columns, 32);
        assert!(!config.language.enabled);
        assert_eq!(config.api.bind, "0.0.0.0:9090");
        assert_eq!(config.persistence.state, "/tmp/test.cx");
        std::fs::remove_file(path).ok();
    }

    #[test]
    fn test_validation_rejects_invalid_cells() {
        let mut config = CortexConfig::default();
        config.model.cells = 100;
        assert!(config.validate().is_err());
        let err = config.validate().unwrap_err().to_string();
        assert!(err.contains("cells"));
    }

    #[test]
    fn test_validation_rejects_invalid_sparsity() {
        let mut config = CortexConfig::default();
        config.model.sparsity_ratio = 0.0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validation_rejects_invalid_learning_rate() {
        let mut config = CortexConfig::default();
        config.learning.learning_rate = 1.5;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validation_rejects_zero_checkpoint_interval() {
        let mut config = CortexConfig::default();
        config.persistence.checkpoint_interval = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_env_override_model_cells() {
        env::set_var("CORTEX_MODEL_CELLS", "2048");
        let config = CortexConfig::default().apply_env_overrides();
        assert_eq!(config.model.cells, 2048);
        env::remove_var("CORTEX_MODEL_CELLS");
    }

    #[test]
    fn test_env_override_language_enabled() {
        env::set_var("CORTEX_LANGUAGE_ENABLED", "false");
        let config = CortexConfig::default().apply_env_overrides();
        assert!(!config.language.enabled);
        env::remove_var("CORTEX_LANGUAGE_ENABLED");
    }

    #[test]
    fn test_env_override_api_bind() {
        env::set_var("CORTEX_API_BIND", "0.0.0.0:9999");
        let config = CortexConfig::default().apply_env_overrides();
        assert_eq!(config.api.bind, "0.0.0.0:9999");
        env::remove_var("CORTEX_API_BIND");
    }

    #[test]
    fn test_env_override_learning_rate() {
        env::set_var("CORTEX_LEARNING_RATE", "0.05");
        let config = CortexConfig::default().apply_env_overrides();
        assert!((config.learning.learning_rate - 0.05).abs() < f32::EPSILON);
        env::remove_var("CORTEX_LEARNING_RATE");
    }

    #[test]
    fn test_env_override_invalid_value_ignored() {
        env::set_var("CORTEX_MODEL_CELLS", "not_a_number");
        let config = CortexConfig::default().apply_env_overrides();
        assert_eq!(config.model.cells, 4096);
        env::remove_var("CORTEX_MODEL_CELLS");
    }

    #[test]
    fn test_invalid_toml() {
        let path = "/tmp/cortex_invalid.toml";
        std::fs::write(path, "this is not valid toml {{{").unwrap();
        let result = CortexConfig::load(path);
        assert!(result.is_err());
        std::fs::remove_file(path).ok();
    }
}
