"""CORTEX configuration schema - frozen dataclasses for all subsystems.

Defines the complete configuration contract per DOC-10:
35 parameters across 12 sections with explicit defaults, types, and validation.

Governing Doc: DOC-10 Configuration Reference
"""

from __future__ import annotations

import warnings
from dataclasses import dataclass, field
from enum import StrEnum


class Precision(StrEnum):
    """Floating-point precision for neural computation."""

    F32 = "f32"
    F16 = "f16"
    BF16 = "bf16"


class ConfigError(Exception):
    """Raised when configuration validation fails.

    Attributes:
        rule: The validation rule ID that failed (e.g. "MDL-001").
        message: Human-readable description of the failure.
    """

    def __init__(self, rule: str, message: str) -> None:
        self.rule = rule
        self.message = message
        super().__init__(f"[{rule}] {message}")


@dataclass(frozen=True)
class ModelConfig:
    """Neural architecture configuration.

    Governing Doc: DOC-10 §2
    """

    cells: int = 4096
    columns: int = 64
    dimension: int = 256
    precision: Precision = Precision.F32
    sparsity_ratio: float = 0.05

    def validate(self) -> None:
        """Validate model configuration per DOC-10 §2.2."""
        if self.cells < 256:
            raise ConfigError("MDL-001", f"cells too small: {self.cells} < 256")
        if self.columns < 16:
            raise ConfigError("MDL-002", f"columns too small: {self.columns} < 16")
        if self.dimension < 64:
            raise ConfigError("MDL-003", f"dimension too small: {self.dimension} < 64")
        if not (0.0 < self.sparsity_ratio <= 1.0):
            raise ConfigError(
                "MDL-004",
                f"invalid sparsity_ratio: {self.sparsity_ratio} not in (0.0, 1.0]",
            )
        if self.cells % self.columns != 0:
            raise ConfigError(
                "MDL-005",
                f"cells must be divisible by columns: {self.cells} % {self.columns} != 0",
            )

    @property
    def cells_per_column(self) -> int:
        return self.cells // self.columns

    @property
    def max_active_cells(self) -> int:
        return int(self.cells * self.sparsity_ratio)

    @property
    def field_count(self) -> int:
        return self.cells // self.columns


@dataclass(frozen=True)
class LanguageConfig:
    """Language core configuration.

    Governing Doc: DOC-10 §3
    """

    enabled: bool = True
    vocabulary_capacity: int = 65536
    context_window: int = 4096
    generation_limit: int = 1024
    learning: bool = True

    def validate(self) -> None:
        """Validate language configuration per DOC-10 §3.2."""
        if self.vocabulary_capacity < 256:
            raise ConfigError(
                "LNG-001", f"vocabulary too small: {self.vocabulary_capacity} < 256"
            )
        if self.context_window < 64:
            raise ConfigError(
                "LNG-002", f"context window too small: {self.context_window} < 64"
            )
        if self.generation_limit < 32:
            raise ConfigError(
                "LNG-003", f"generation limit too small: {self.generation_limit} < 32"
            )
        if self.context_window < self.generation_limit:
            raise ConfigError(
                "LNG-004",
                f"context_window must be >= generation_limit: "
                f"{self.context_window} < {self.generation_limit}",
            )


@dataclass(frozen=True)
class MemoryConfig:
    """Memory budget configuration.

    Governing Doc: DOC-10 §4
    """

    working_mb: int = 128
    episodic_mb: int = 512
    semantic_mb: int = 512
    procedural_mb: int = 256
    associative_mb: int = 256

    def validate(self) -> None:
        """Validate memory configuration per DOC-10 §4.2."""
        if self.working_mb < 16:
            raise ConfigError(
                "MEM-001", f"working memory too small: {self.working_mb} < 16"
            )
        if self.episodic_mb < 32:
            raise ConfigError(
                "MEM-002", f"episodic memory too small: {self.episodic_mb} < 32"
            )
        if self.semantic_mb < 32:
            raise ConfigError(
                "MEM-003", f"semantic memory too small: {self.semantic_mb} < 32"
            )
        if self.procedural_mb < 16:
            raise ConfigError(
                "MEM-004", f"procedural memory too small: {self.procedural_mb} < 16"
            )
        if self.associative_mb < 16:
            raise ConfigError(
                "MEM-005", f"associative memory too small: {self.associative_mb} < 16"
            )

    @property
    def total_mb(self) -> int:
        return (
            self.working_mb
            + self.episodic_mb
            + self.semantic_mb
            + self.procedural_mb
            + self.associative_mb
        )


@dataclass(frozen=True)
class LearningConfig:
    """Learning parameters configuration.

    Governing Doc: DOC-10 §5
    """

    enabled: bool = True
    learning_rate: float = 0.001
    plasticity: float = 0.01
    replay: bool = True
    consolidation_interval: int = 1000

    def validate(self) -> None:
        """Validate learning configuration per DOC-10 §5.2."""
        if not (0.0 < self.learning_rate <= 1.0):
            raise ConfigError(
                "LRN-001",
                f"invalid learning_rate: {self.learning_rate} not in (0.0, 1.0]",
            )
        if not (0.0 <= self.plasticity <= 1.0):
            raise ConfigError(
                "LRN-002",
                f"invalid plasticity: {self.plasticity} not in [0.0, 1.0]",
            )
        if self.consolidation_interval < 1:
            raise ConfigError(
                "LRN-003",
                f"consolidation interval too small: {self.consolidation_interval} < 1",
            )

    @property
    def max_replay_count(self) -> int:
        return max(1, self.consolidation_interval // 10)

    @property
    def effective_update_bound(self) -> float:
        return self.learning_rate * self.plasticity


@dataclass(frozen=True)
class WorldConfig:
    """World model configuration.

    Governing Doc: DOC-10 §6
    """

    enabled: bool = True
    prediction_horizon: int = 8

    def validate(self) -> None:
        """Validate world configuration per DOC-10 §6.2."""
        if self.prediction_horizon < 1:
            raise ConfigError(
                "WLD-001",
                f"prediction horizon too small: {self.prediction_horizon} < 1",
            )


@dataclass(frozen=True)
class ReasoningConfig:
    """Reasoning engine configuration.

    Governing Doc: DOC-10 §7
    """

    enabled: bool = True
    max_steps: int = 32

    def validate(self) -> None:
        """Validate reasoning configuration per DOC-10 §7.2."""
        if self.max_steps < 1:
            raise ConfigError(
                "RSN-001", f"max steps too small: {self.max_steps} < 1"
            )


@dataclass(frozen=True)
class PlanningConfig:
    """Planning engine configuration.

    Governing Doc: DOC-10 §8
    """

    enabled: bool = True
    max_depth: int = 8
    max_branches: int = 16

    def validate(self) -> None:
        """Validate planning configuration per DOC-10 §8.2."""
        if self.max_depth < 1:
            raise ConfigError(
                "PLN-001", f"max depth too small: {self.max_depth} < 1"
            )
        if self.max_branches < 1:
            raise ConfigError(
                "PLN-002", f"max branches too small: {self.max_branches} < 1"
            )


@dataclass(frozen=True)
class VerificationConfig:
    """Verification engine configuration.

    Governing Doc: DOC-10 §9
    """

    enabled: bool = True
    minimum_confidence: float = 0.80

    def validate(self) -> None:
        """Validate verification configuration per DOC-10 §9.2."""
        if not (0.0 <= self.minimum_confidence <= 1.0):
            raise ConfigError(
                "VER-001",
                f"invalid confidence threshold: "
                f"{self.minimum_confidence} not in [0.0, 1.0]",
            )


@dataclass(frozen=True)
class InternetConfig:
    """Internet interface configuration.

    Governing Doc: DOC-10 §10
    """

    enabled: bool = True
    timeout_seconds: int = 15
    max_response_mb: int = 4

    def validate(self) -> None:
        """Validate internet configuration per DOC-10 §10.2."""
        if self.timeout_seconds < 1:
            raise ConfigError(
                "INT-001",
                f"timeout too small: {self.timeout_seconds} < 1",
            )
        if self.max_response_mb < 1:
            raise ConfigError(
                "INT-002",
                f"max response too small: {self.max_response_mb} < 1",
            )


@dataclass(frozen=True)
class PolicyConfig:
    """Policy configuration.

    Governing Doc: DOC-10 §11
    """

    learning: bool = True
    internet_learning: bool = True
    self_modification: bool = False
    policy_modification: bool = False
    runtime_modification: bool = False

    def validate(self) -> None:
        """Validate policy configuration per DOC-10 §11.2.

        Emits warnings for risky policy choices per DOC-10 §11.3.
        """
        if self.self_modification:
            warnings.warn(
                "Self-modification Level 2 enabled; algorithm adaptation allowed",
                stacklevel=2,
            )
        if self.policy_modification:
            warnings.warn(
                "Self-modification Level 3 enabled; policy modification allowed",
                stacklevel=2,
            )
        if self.runtime_modification:
            warnings.warn(
                "Runtime modification enabled",
                stacklevel=2,
            )


@dataclass(frozen=True)
class ApiConfig:
    """API server configuration.

    Governing Doc: DOC-10 §12
    """

    enabled: bool = True
    bind: str = "127.0.0.1:8080"
    api_key_env: str = "CORTEX_API_KEY"

    def validate(self) -> None:
        """Validate API configuration per DOC-10 §12.2."""
        if not self._is_valid_bind(self.bind):
            raise ConfigError(
                "API-001", f"invalid bind address: {self.bind!r}"
            )
        if not self.api_key_env:
            raise ConfigError(
                "API-002", "invalid API key env var: empty string"
            )

    @staticmethod
    def _is_valid_bind(address: str) -> bool:
        """Check that address is host:port with valid components."""
        parts = address.rsplit(":", 1)
        if len(parts) != 2:
            return False
        host, port_str = parts
        if not host:
            return False
        try:
            port = int(port_str)
        except ValueError:
            return False
        return 1 <= port <= 65535


@dataclass(frozen=True)
class PersistenceConfig:
    """Persistence configuration.

    Governing Doc: DOC-10 §13
    """

    state: str = "cortex.cx"
    checkpoint_interval: int = 1000

    def validate(self) -> None:
        """Validate persistence configuration per DOC-10 §13.2."""
        if not self.state:
            raise ConfigError("PRS-001", "invalid state path: empty string")
        if self.checkpoint_interval < 1:
            raise ConfigError(
                "PRS-002",
                f"checkpoint interval too small: {self.checkpoint_interval} < 1",
            )


@dataclass(frozen=True)
class Config:
    """Top-level CORTEX configuration.

    Contains all subsystem configurations. Immutable after boot per DOC-10 §16 CFG-005.
    """

    model: ModelConfig = field(default_factory=ModelConfig)
    language: LanguageConfig = field(default_factory=LanguageConfig)
    memory: MemoryConfig = field(default_factory=MemoryConfig)
    learning: LearningConfig = field(default_factory=LearningConfig)
    world: WorldConfig = field(default_factory=WorldConfig)
    reasoning: ReasoningConfig = field(default_factory=ReasoningConfig)
    planning: PlanningConfig = field(default_factory=PlanningConfig)
    verification: VerificationConfig = field(default_factory=VerificationConfig)
    internet: InternetConfig = field(default_factory=InternetConfig)
    policy: PolicyConfig = field(default_factory=PolicyConfig)
    api: ApiConfig = field(default_factory=ApiConfig)
    persistence: PersistenceConfig = field(default_factory=PersistenceConfig)

    def validate(self) -> None:
        """Run full validation pipeline per DOC-10 §1.3.

        Pipeline: Schema Validation → Range Validation → Dependency Validation
        → Policy Validation → Runtime Initialization
        """
        self.model.validate()
        self.language.validate()
        self.memory.validate()
        self.learning.validate()
        self.world.validate()
        self.reasoning.validate()
        self.planning.validate()
        self.verification.validate()
        self.internet.validate()
        self.policy.validate()
        self.api.validate()
        self.persistence.validate()
