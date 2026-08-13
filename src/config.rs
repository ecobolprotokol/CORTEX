use crate::error::{CortexError, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

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

impl CortexConfig {
    pub fn load(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| CortexError::ConfigError(format!("Failed to read config: {}", e)))?;
        let config: CortexConfig = toml::from_str(&content)
            .map_err(|e| CortexError::ConfigError(format!("Failed to parse config: {}", e)))?;
        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<()> {
        if self.model.cells < 256 {
            return Err(CortexError::ConfigError("cells must be >= 256".into()));
        }
        if self.model.columns < 16 {
            return Err(CortexError::ConfigError("columns must be >= 16".into()));
        }
        if self.model.dimension < 64 {
            return Err(CortexError::ConfigError("dimension must be >= 64".into()));
        }
        if self.model.sparsity_ratio <= 0.0 || self.model.sparsity_ratio > 1.0 {
            return Err(CortexError::ConfigError(
                "sparsity_ratio must be in (0.0, 1.0]".into(),
            ));
        }
        if self.model.cells % self.model.columns != 0 {
            return Err(CortexError::ConfigError(
                "cells must be divisible by columns".into(),
            ));
        }
        if self.language.vocabulary_capacity < 256 {
            return Err(CortexError::ConfigError(
                "vocabulary_capacity must be >= 256".into(),
            ));
        }
        if self.language.context_window < 64 {
            return Err(CortexError::ConfigError(
                "context_window must be >= 64".into(),
            ));
        }
        if self.language.generation_limit < 32 {
            return Err(CortexError::ConfigError(
                "generation_limit must be >= 32".into(),
            ));
        }
        if self.memory.working_mb < 16 {
            return Err(CortexError::ConfigError(
                "working_mb must be >= 16".into(),
            ));
        }
        if self.memory.episodic_mb < 32 {
            return Err(CortexError::ConfigError(
                "episodic_mb must be >= 32".into(),
            ));
        }
        if self.learning.learning_rate <= 0.0 || self.learning.learning_rate > 1.0 {
            return Err(CortexError::ConfigError(
                "learning_rate must be in (0.0, 1.0]".into(),
            ));
        }
        if self.learning.plasticity < 0.0 || self.learning.plasticity > 1.0 {
            return Err(CortexError::ConfigError(
                "plasticity must be in [0.0, 1.0]".into(),
            ));
        }
        if self.verification.minimum_confidence < 0.0 || self.verification.minimum_confidence > 1.0
        {
            return Err(CortexError::ConfigError(
                "minimum_confidence must be in [0.0, 1.0]".into(),
            ));
        }
        if self.reasoning.max_steps < 1 {
            return Err(CortexError::ConfigError(
                "reasoning.max_steps must be >= 1".into(),
            ));
        }
        if self.planning.max_depth < 1 {
            return Err(CortexError::ConfigError(
                "planning.max_depth must be >= 1".into(),
            ));
        }
        if self.planning.max_branches < 1 {
            return Err(CortexError::ConfigError(
                "planning.max_branches must be >= 1".into(),
            ));
        }
        Ok(())
    }

    pub fn compute_budget(&self) -> crate::types::ComputeBudget {
        crate::types::ComputeBudget {
            max_reasoning_steps: self.reasoning.max_steps,
            max_planning_depth: self.planning.max_depth,
            max_planning_branches: self.planning.max_branches,
            max_simulation_steps: self.world.prediction_horizon,
            max_generation_length: self.language.generation_limit,
            max_memory_retrieval: 10,
            max_replay_count: std::cmp::max(1, self.learning.consolidation_interval / 10) as u32,
        }
    }

    pub fn config_hash(&self) -> [u8; 32] {
        let serialized = toml::to_string(self).unwrap_or_default();
        *blake3::hash(serialized.as_bytes()).as_bytes()
    }

    pub fn find_config() -> Option<String> {
        let candidates = vec![
            "./cortex.toml".to_string(),
            "/opt/cortex/cortex.toml".to_string(),
        ];
        for path in candidates {
            if Path::new(&path).exists() {
                return Some(path);
            }
        }
        None
    }
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
