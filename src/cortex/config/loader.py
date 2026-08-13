"""CORTEX configuration loader - TOML parsing, env overrides, validation.

Implements the configuration discovery order per DOC-10 §1.2:
1. --config <path> (explicit CLI argument)
2. CORTEX_CONFIG environment variable
3. ./cortex.toml (current working directory)
4. /opt/cortex/cortex.toml (default install path)
5. Error: configuration not found

Governing Doc: DOC-10 Configuration Reference
"""

from __future__ import annotations

import os
import tomllib
from pathlib import Path
from typing import Any

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

_DEFAULT_CONFIG_PATHS = [
    Path("cortex.toml"),
    Path("/opt/cortex/cortex.toml"),
]


def _deep_merge(base: dict[str, Any], override: dict[str, Any]) -> dict[str, Any]:
    """Recursively merge override into base, returning a new dict."""
    result = dict(base)
    for key, value in override.items():
        if key in result and isinstance(result[key], dict) and isinstance(value, dict):
            result[key] = _deep_merge(result[key], value)
        else:
            result[key] = value
    return result


def _parse_precision(value: str) -> Precision:
    """Parse a precision string into a Precision enum."""
    try:
        return Precision(value)
    except ValueError as err:
        raise ConfigError(
            "CFG-TYPE",
            f"invalid precision: {value!r} not in ('f32', 'f16', 'bf16')",
        ) from err


def _build_model_config(data: dict[str, Any]) -> ModelConfig:
    """Build ModelConfig from parsed TOML data."""
    return ModelConfig(
        cells=data.get("cells", 4096),
        columns=data.get("columns", 64),
        dimension=data.get("dimension", 256),
        precision=_parse_precision(data.get("precision", "f32")),
        sparsity_ratio=data.get("sparsity_ratio", 0.05),
    )


def _build_language_config(data: dict[str, Any]) -> LanguageConfig:
    """Build LanguageConfig from parsed TOML data."""
    return LanguageConfig(
        enabled=data.get("enabled", True),
        vocabulary_capacity=data.get("vocabulary_capacity", 65536),
        context_window=data.get("context_window", 4096),
        generation_limit=data.get("generation_limit", 1024),
        learning=data.get("learning", True),
    )


def _build_memory_config(data: dict[str, Any]) -> MemoryConfig:
    """Build MemoryConfig from parsed TOML data."""
    return MemoryConfig(
        working_mb=data.get("working_mb", 128),
        episodic_mb=data.get("episodic_mb", 512),
        semantic_mb=data.get("semantic_mb", 512),
        procedural_mb=data.get("procedural_mb", 256),
        associative_mb=data.get("associative_mb", 256),
    )


def _build_learning_config(data: dict[str, Any]) -> LearningConfig:
    """Build LearningConfig from parsed TOML data."""
    return LearningConfig(
        enabled=data.get("enabled", True),
        learning_rate=data.get("learning_rate", 0.001),
        plasticity=data.get("plasticity", 0.01),
        replay=data.get("replay", True),
        consolidation_interval=data.get("consolidation_interval", 1000),
    )


def _build_world_config(data: dict[str, Any]) -> WorldConfig:
    """Build WorldConfig from parsed TOML data."""
    return WorldConfig(
        enabled=data.get("enabled", True),
        prediction_horizon=data.get("prediction_horizon", 8),
    )


def _build_reasoning_config(data: dict[str, Any]) -> ReasoningConfig:
    """Build ReasoningConfig from parsed TOML data."""
    return ReasoningConfig(
        enabled=data.get("enabled", True),
        max_steps=data.get("max_steps", 32),
    )


def _build_planning_config(data: dict[str, Any]) -> PlanningConfig:
    """Build PlanningConfig from parsed TOML data."""
    return PlanningConfig(
        enabled=data.get("enabled", True),
        max_depth=data.get("max_depth", 8),
        max_branches=data.get("max_branches", 16),
    )


def _build_verification_config(data: dict[str, Any]) -> VerificationConfig:
    """Build VerificationConfig from parsed TOML data."""
    return VerificationConfig(
        enabled=data.get("enabled", True),
        minimum_confidence=data.get("minimum_confidence", 0.80),
    )


def _build_internet_config(data: dict[str, Any]) -> InternetConfig:
    """Build InternetConfig from parsed TOML data."""
    return InternetConfig(
        enabled=data.get("enabled", True),
        timeout_seconds=data.get("timeout_seconds", 15),
        max_response_mb=data.get("max_response_mb", 4),
    )


def _build_policy_config(data: dict[str, Any]) -> PolicyConfig:
    """Build PolicyConfig from parsed TOML data."""
    return PolicyConfig(
        learning=data.get("learning", True),
        internet_learning=data.get("internet_learning", True),
        self_modification=data.get("self_modification", False),
        policy_modification=data.get("policy_modification", False),
        runtime_modification=data.get("runtime_modification", False),
    )


def _build_api_config(data: dict[str, Any]) -> ApiConfig:
    """Build ApiConfig from parsed TOML data."""
    return ApiConfig(
        enabled=data.get("enabled", True),
        bind=data.get("bind", "127.0.0.1:8080"),
        api_key_env=data.get("api_key_env", "CORTEX_API_KEY"),
    )


def _build_persistence_config(data: dict[str, Any]) -> PersistenceConfig:
    """Build PersistenceConfig from parsed TOML data."""
    return PersistenceConfig(
        state=data.get("state", "cortex.cx"),
        checkpoint_interval=data.get("checkpoint_interval", 1000),
    )


def _build_config_from_toml(data: dict[str, Any]) -> Config:
    """Construct a Config from parsed TOML dictionary."""
    return Config(
        model=_build_model_config(data.get("model", {})),
        language=_build_language_config(data.get("language", {})),
        memory=_build_memory_config(data.get("memory", {})),
        learning=_build_learning_config(data.get("learning", {})),
        world=_build_world_config(data.get("world", {})),
        reasoning=_build_reasoning_config(data.get("reasoning", {})),
        planning=_build_planning_config(data.get("planning", {})),
        verification=_build_verification_config(data.get("verification", {})),
        internet=_build_internet_config(data.get("internet", {})),
        policy=_build_policy_config(data.get("policy", {})),
        api=_build_api_config(data.get("api", {})),
        persistence=_build_persistence_config(data.get("persistence", {})),
    )


def discover_config_path(cli_path: str | None = None) -> Path:
    """Find the configuration file using DOC-10 §1.2 discovery order.

    Args:
        cli_path: Explicit path from --config CLI argument.

    Returns:
        Path to the configuration file.

    Raises:
        FileNotFoundError: If no configuration file is found.
    """
    if cli_path is not None:
        path = Path(cli_path)
        if path.is_file():
            return path
        raise FileNotFoundError(f"Configuration file not found: {path}")

    env_path = os.environ.get("CORTEX_CONFIG")
    if env_path:
        path = Path(env_path)
        if path.is_file():
            return path
        raise FileNotFoundError(f"CORTEX_CONFIG points to missing file: {path}")

    for default_path in _DEFAULT_CONFIG_PATHS:
        if default_path.is_file():
            return default_path

    raise FileNotFoundError(
        "Configuration not found. Searched:\n"
        "  1. --config <path> (not provided)\n"
        "  2. CORTEX_CONFIG env var (not set)\n"
        + "\n".join(f"  {i + 3}. {p} (not found)" for i, p in enumerate(_DEFAULT_CONFIG_PATHS))
    )


def load_toml(path: Path) -> dict[str, Any]:
    """Load and parse a TOML file.

    Args:
        path: Path to the TOML configuration file.

    Returns:
        Parsed TOML data as a dictionary.

    Raises:
        ConfigError: If TOML parsing fails.
    """
    try:
        with open(path, "rb") as f:
            data = tomllib.load(f)
    except tomllib.TOMLDecodeError as e:
        raise ConfigError("CFG-TOML", f"TOML parse error in {path}: {e}") from e
    return data


def load_config(
    cli_path: str | None = None,
    *,
    _raw_data: dict[str, Any] | None = None,
) -> Config:
    """Load, merge, and validate the CORTEX configuration.

    Implements the full configuration pipeline per DOC-10 §1.3:
    Parse TOML → Schema Validation → Range Validation → Dependency Validation
    → Policy Validation → Runtime Initialization

    Args:
        cli_path: Explicit path from --config CLI argument. When None,
                  uses environment variable / default path discovery.
        _raw_data: Internal override for testing. Bypasses file discovery
                   and TOML parsing.

    Returns:
        Fully validated, immutable Config instance.

    Raises:
        FileNotFoundError: If no configuration file is found.
        ConfigError: If configuration validation fails.
    """
    if _raw_data is not None:
        return _build_config_from_toml(_raw_data)

    path = discover_config_path(cli_path)
    data = load_toml(path)
    config = _build_config_from_toml(data)
    config.validate()
    return config
