"""Common types: timestamps, durations, enums, and shared containers.

This module provides the foundational types used across all CORTEX subsystems.
It contains no internal dependencies and serves as the base of the type hierarchy.
"""

from __future__ import annotations

import math
import time
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Any, ClassVar


# ---------------------------------------------------------------------------
# Enums
# ---------------------------------------------------------------------------

class ConfidenceLevel(Enum):
    """Discrete confidence classification."""
    VERY_LOW = "very_low"
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    VERY_HIGH = "very_high"


class EntityStateKind(Enum):
    """State classification for world-model entities."""
    ACTIVE = "active"
    INACTIVE = "inactive"
    TRANSIENT = "transient"
    HYPOTHETICAL = "hypothetical"
    CONFIRMED = "confirmed"
    DISPUTED = "disputed"


class MemoryType(Enum):
    """Classification of memory subsystems."""
    WORKING = "working"
    EPISODIC = "episodic"
    SEMANTIC = "semantic"
    PROCEDURAL = "procedural"
    ASSOCIATIVE = "associative"


class MemoryPressure(Enum):
    """Current memory pressure level."""
    LOW = "low"
    MODERATE = "moderate"
    HIGH = "high"
    CRITICAL = "critical"


class EvictionPolicy(Enum):
    """Memory eviction strategy."""
    LRU = "lru"
    LOWEST_IMPORTANCE = "lowest_importance"
    LOWEST_CONFIDENCE = "lowest_confidence"
    OLDEST = "oldest"
    COMPOSITE = "composite"


class VerificationStatus(Enum):
    """Progressive verification status for knowledge claims.

    Valid transitions: Unknown → Observed → Inferred → Supported → Provisional → Verified
    Can regress to Contradicted with contradicting evidence.
    """
    OBSERVED = "observed"
    INFERRED = "inferred"
    SUPPORTED = "supported"
    PROVISIONAL = "provisional"
    VERIFIED = "verified"
    UNKNOWN = "unknown"
    CONTRADICTED = "contradicted"


class ProvenanceCategory(Enum):
    """Category of knowledge origin."""
    OBSERVED = "observed"
    USER_PROVIDED = "user_provided"
    INTERNET = "internet"
    DERIVED = "derived"
    INFERRED = "inferred"
    REPLAYED = "replayed"
    VERIFIED = "verified"


class SourceKind(Enum):
    """Type of information source."""
    USER = "user"
    SYSTEM = "system"
    INTERNET = "internet"
    DERIVED = "derived"
    INTERNAL = "internal"


class RelationKind(Enum):
    """Semantic relation types between concepts/entities."""
    IS_A = "is_a"
    HAS_PROPERTY = "has_property"
    PART_OF = "part_of"
    CAUSES = "causes"
    REQUIRES = "requires"
    ENABLES = "enables"
    CONTRADICTS = "contradicts"
    SUPPORTS = "supports"
    RELATED_TO = "related_to"
    TEMPORAL_BEFORE = "temporal_before"
    TEMPORAL_AFTER = "temporal_after"
    SPATIAL_NEAR = "spatial_near"
    AGENT_OF = "agent_of"
    OBJECT_OF = "object_of"
    RECIPIENT_OF = "recipient_of"


class ActionKind(Enum):
    """Types of actions CORTEX can perform."""
    RESPOND = "respond"
    OBSERVE = "observe"
    QUERY = "query"
    LEARN = "learn"
    PLAN = "plan"
    VERIFY = "verify"
    FETCH = "fetch"
    STORE = "store"
    FORGET = "forget"
    CONSOLIDATE = "consolidate"
    CHECKPOINT = "checkpoint"
    NO_OP = "no_op"


class ObservationKind(Enum):
    """Classification of incoming observations."""
    USER_INPUT = "user_input"
    ENVIRONMENT = "environment"
    INTERNET = "internet"
    INTERNAL = "internal"
    FEEDBACK = "feedback"
    CORRECTION = "correction"


class EvidencePolarity(Enum):
    """Whether evidence supports or contradicts a claim."""
    SUPPORTS = "supports"
    CONTRADICTS = "contradicts"
    NEUTRAL = "neutral"


class RiskLevel(Enum):
    """Risk severity classification."""
    NONE = "none"
    LOW = "low"
    MODERATE = "moderate"
    HIGH = "high"
    CRITICAL = "critical"


class GoalStatus(Enum):
    """Lifecycle status of a planning goal."""
    ACTIVE = "active"
    ACHIEVED = "achieved"
    FAILED = "failed"
    ABANDONED = "abandoned"


class PlanStatus(Enum):
    """Lifecycle status of a plan."""
    CANDIDATE = "candidate"
    SELECTED = "selected"
    EXECUTING = "executing"
    COMPLETED = "completed"
    FAILED = "failed"
    ABANDONED = "abandoned"


class ReasoningType(Enum):
    """Type of reasoning being performed."""
    DEDUCTIVE = "deductive"
    INDUCTIVE = "inductive"
    ABDUCTIVE = "abductive"
    ANALOGICAL = "analogical"
    TEMPORAL = "temporal"
    CAUSAL = "causal"
    COUNTERFACTUAL = "counterfactual"
    CONSTRAINT = "constraint"
    CONSISTENCY = "consistency"


class PredictionTarget(Enum):
    """What a prediction is targeting."""
    NEXT_TOKEN = "next_token"
    NEXT_STATE = "next_state"
    NEXT_ACTION = "next_action"
    OUTCOME = "outcome"
    TRANSITION = "transition"
    INTENT = "intent"


class EntityKind(Enum):
    """World-model entity classification."""
    PERSON = "person"
    OBJECT = "object"
    PLACE = "place"
    ORGANIZATION = "organization"
    CONCEPTUAL_OBJECT = "conceptual_object"
    EVENT = "event"
    SYSTEM = "system"
    PROCESS = "process"


class AssociationKind(Enum):
    """Type of associative memory link."""
    SEMANTIC = "semantic"
    TEMPORAL = "temporal"
    CONTEXTUAL = "contextual"
    CAUSAL = "causal"
    EPISODIC = "episodic"
    PROCEDURAL = "procedural"


class ConsolidationTarget(Enum):
    """Target memory type for consolidation."""
    SEMANTIC = "semantic"
    PROCEDURAL = "procedural"
    ASSOCIATIVE = "associative"


class ErrorAttribution(Enum):
    """Source attribution for prediction errors."""
    INPUT_ERROR = "input_error"
    MEMORY_ERROR = "memory_error"
    WORLD_ERROR = "world_error"
    REASONING_ERROR = "reasoning_error"
    PROCEDURE_ERROR = "procedure_error"
    ENVIRONMENT_ERROR = "environment_error"


class Precision(Enum):
    """Numeric precision configuration."""
    F32 = "f32"
    F16 = "f16"
    BF16 = "bf16"


class OperationClass(Enum):
    """Classification of policy-evaluated operations."""
    COGNITIVE_STATE_ADAPTATION = "cognitive_state_adaptation"
    ALGORITHM_ADAPTATION = "algorithm_adaptation"
    SECURITY_POLICY_MODIFICATION = "security_policy_modification"
    RUNTIME_MODIFICATION = "runtime_modification"


class DenialReason(Enum):
    """Reason for policy denial."""
    LEARNING_DISABLED = "learning_disabled"
    SELF_MODIFICATION_DISABLED = "self_modification_disabled"
    POLICY_MODIFICATION_DISABLED = "policy_modification_disabled"
    RUNTIME_MODIFICATION_DISABLED = "runtime_modification_disabled"
    CRITICAL_RISK = "critical_risk"
    INSUFFICIENT_CONFIDENCE = "insufficient_confidence"
    POLICY_VIOLATION = "policy_violation"
    RESOURCE_EXHAUSTION = "resource_exhaustion"


class HttpMethod(Enum):
    """HTTP request methods."""
    GET = "GET"
    POST = "POST"


class RuntimeState(Enum):
    """Runtime state machine states."""
    BOOT = "boot"
    LOAD_CONFIGURATION = "load_configuration"
    LOAD_STATE = "load_state"
    VALIDATE = "validate"
    INITIALIZE = "initialize"
    READY = "ready"
    PROCESSING = "processing"
    LEARNING = "learning"
    CONSOLIDATING = "consolidating"
    CHECKPOINTING = "checkpointing"
    FAULT = "fault"
    RECOVERY = "recovery"
    SAFE_STOP = "safe_stop"
    SHUTDOWN = "shutdown"


# ---------------------------------------------------------------------------
# Timestamp
# ---------------------------------------------------------------------------

@dataclass(frozen=True, slots=True, order=True, eq=True)
class Timestamp:
    """Unix timestamp in milliseconds since epoch (UTC).

    Used for all temporal ordering in CORTEX. Immutable and orderable.
    ``ZERO`` indicates an absent or unknown timestamp.
    """

    millis: int = field(compare=True, repr=True)

    ZERO: ClassVar[Timestamp]

    @classmethod
    def now(cls) -> Timestamp:
        """Create a timestamp representing the current instant."""
        return cls(int(time.time() * 1000))

    @classmethod
    def from_secs(cls, secs: int) -> Timestamp:
        """Create a timestamp from whole seconds since epoch."""
        return cls(secs * 1000)

    def as_secs(self) -> int:
        """Return the timestamp as whole seconds since epoch."""
        return self.millis // 1000

    def as_millis(self) -> int:
        """Return the raw millisecond value."""
        return self.millis

    def elapsed_since(self, earlier: Timestamp) -> Duration:
        """Return the duration between *earlier* and this timestamp.

        Uses saturating subtraction: if *earlier* is after this timestamp,
        the result is ``Duration.ZERO``.
        """
        diff = max(0, self.millis - earlier.millis)
        return Duration(diff)

    def is_before(self, other: Timestamp) -> bool:
        """Return ``True`` if this timestamp is strictly before *other*."""
        return self.millis < other.millis

    def is_after(self, other: Timestamp) -> bool:
        """Return ``True`` if this timestamp is strictly after *other*."""
        return self.millis > other.millis

    def to_dict(self) -> dict[str, Any]:
        """Serialise to a plain dictionary."""
        return {"millis": self.millis}

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Timestamp:
        """Deserialise from a plain dictionary."""
        return cls(millis=int(data["millis"]))


# Set the class-level constant after the class is defined.
Timestamp.ZERO = Timestamp(0)  # type: ignore[attr-defined]


# ---------------------------------------------------------------------------
# Duration
# ---------------------------------------------------------------------------

@dataclass(frozen=True, slots=True, order=True, eq=True)
class Duration:
    """Duration in milliseconds. Always non-negative."""

    millis: int = field(compare=True, repr=True)

    ZERO: ClassVar[Duration]

    @classmethod
    def from_secs(cls, secs: int) -> Duration:
        """Create a duration from whole seconds."""
        return cls(secs * 1000)

    @classmethod
    def from_millis(cls, ms: int) -> Duration:
        """Create a duration from milliseconds."""
        return cls(ms)

    def as_secs(self) -> int:
        """Return the duration as whole seconds."""
        return self.millis // 1000

    def as_millis(self) -> int:
        """Return the raw millisecond value."""
        return self.millis

    def to_dict(self) -> dict[str, Any]:
        """Serialise to a plain dictionary."""
        return {"millis": self.millis}

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Duration:
        """Deserialise from a plain dictionary."""
        return cls(millis=int(data["millis"]))


Duration.ZERO = Duration(0)  # type: ignore[attr-defined]


# ---------------------------------------------------------------------------
# Temporal Context
# ---------------------------------------------------------------------------

@dataclass(slots=True)
class TemporalContext:
    """Temporal positioning information for cognitive operations."""

    current_time: Timestamp = field(default_factory=Timestamp.now)
    sequence_position: int = 0
    prior_states: list[Timestamp] = field(default_factory=list)
    temporal_horizon: Duration = field(default_factory=lambda: Duration.from_secs(3600))

    def to_dict(self) -> dict[str, Any]:
        return {
            "current_time": self.current_time.to_dict(),
            "sequence_position": self.sequence_position,
            "prior_states": [ts.to_dict() for ts in self.prior_states],
            "temporal_horizon": self.temporal_horizon.to_dict(),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> TemporalContext:
        return cls(
            current_time=Timestamp.from_dict(data["current_time"]),
            sequence_position=data["sequence_position"],
            prior_states=[Timestamp.from_dict(ts) for ts in data["prior_states"]],
            temporal_horizon=Duration.from_dict(data["temporal_horizon"]),
        )


# ---------------------------------------------------------------------------
# Intent Hypothesis (lightweight, used by ContextState)
# ---------------------------------------------------------------------------

@dataclass(slots=True)
class IntentHypothesis:
    """Hypothesis about user intent, held in working context."""

    intent_description: str = ""
    confidence: float = 0.0
    supporting_evidence: int = 0

    def to_dict(self) -> dict[str, Any]:
        return {
            "intent_description": self.intent_description,
            "confidence": self.confidence,
            "supporting_evidence": self.supporting_evidence,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> IntentHypothesis:
        return cls(
            intent_description=data["intent_description"],
            confidence=data["confidence"],
            supporting_evidence=data["supporting_evidence"],
        )


# ---------------------------------------------------------------------------
# Context State (placed here to avoid circular imports with observation/state)
# ---------------------------------------------------------------------------

@dataclass(slots=True)
class ContextState:
    """Hierarchical context state influencing all cognitive operations.

    Owned by the memory system. Mutable during the cognitive loop.
    """

    conversation_id: int | None = None
    episode_context: list[int] = field(default_factory=list)
    active_concepts: list[int] = field(default_factory=list)
    world_assumptions: list[int] = field(default_factory=list)
    temporal_context: TemporalContext = field(default_factory=TemporalContext)
    active_intents: list[IntentHypothesis] = field(default_factory=list)
    window_position: int = 0
    tokens_used: int = 0

    @classmethod
    def initial(cls) -> ContextState:
        """Create a fresh, empty context state."""
        return cls()

    def advance_time(self) -> None:
        """Advance the temporal context to the current instant."""
        self.temporal_context.current_time = Timestamp.now()
        self.temporal_context.sequence_position += 1

    def to_dict(self) -> dict[str, Any]:
        return {
            "conversation_id": self.conversation_id,
            "episode_context": list(self.episode_context),
            "active_concepts": list(self.active_concepts),
            "world_assumptions": list(self.world_assumptions),
            "temporal_context": self.temporal_context.to_dict(),
            "active_intents": [i.to_dict() for i in self.active_intents],
            "window_position": self.window_position,
            "tokens_used": self.tokens_used,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ContextState:
        return cls(
            conversation_id=data.get("conversation_id"),
            episode_context=data.get("episode_context", []),
            active_concepts=data.get("active_concepts", []),
            world_assumptions=data.get("world_assumptions", []),
            temporal_context=TemporalContext.from_dict(data["temporal_context"]),
            active_intents=[IntentHypothesis.from_dict(i) for i in data.get("active_intents", [])],
            window_position=data.get("window_position", 0),
            tokens_used=data.get("tokens_used", 0),
        )
