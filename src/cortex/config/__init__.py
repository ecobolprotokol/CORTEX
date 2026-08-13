"""CORTEX configuration - parsing, validation, distribution.

This package handles configuration parsing, validation,
and distribution to all subsystems.

Key components:
- TOML config file parsing
- Configuration validation
- Configuration distribution

Governing Doc: DOC-10
"""

from cortex.config.loader import load_config
from cortex.config.schema import (
    ApiConfig,
    Config,
    ConfigError,
    InternetConfig,
    LanguageConfig,
    LearningConfig,
    MemoryConfig,
    ModelConfig,
    PersistenceConfig,
    PlanningConfig,
    PolicyConfig,
    Precision,
    ReasoningConfig,
    VerificationConfig,
    WorldConfig,
)

__all__ = [
    "ApiConfig",
    "Config",
    "ConfigError",
    "InternetConfig",
    "LanguageConfig",
    "LearningConfig",
    "load_config",
    "MemoryConfig",
    "ModelConfig",
    "PersistenceConfig",
    "PlanningConfig",
    "PolicyConfig",
    "Precision",
    "ReasoningConfig",
    "VerificationConfig",
    "WorldConfig",
]
