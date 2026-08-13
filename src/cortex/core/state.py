"""CORTEX state types: complete state hierarchy for all subsystems.

Contains the top-level :class:`CortexState` and all sub-state containers:
memory, world model, reasoning, planning, verification, learning, and
self-model.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Any

from .common import (
    AssociationKind,
    ConsolidationTarget,
    Duration,
    EntityKind,
    EntityStateKind,
    ErrorAttribution,
    EvictionPolicy,
    GoalStatus,
    HttpMethod,
    MemoryPressure,
    MemoryType,
    PlanStatus,
    Precision,
    ProvenanceCategory,
    ReasoningType,
    RelationKind,
    RiskLevel,
    SourceKind,
    TemporalContext,
    Timestamp,
    VerificationStatus,
)
from .evidence import (
    ConfidenceState,
    EvidenceSet,
    Provenance,
    Source,
    SourceIdentity,
    UncertaintyState,
)
from .ids import (
    ActionId,
    AssociationId,
    ClaimId,
    CheckpointId,
    ConceptId,
    EntityId,
    EpisodeId,
    EventId,
    EvidenceId,
    GoalId,
    HypothesisId,
    KnowledgeId,
    PlanId,
    ProcedureId,
    RelationId,
    SessionId,
    SourceId,
)
from .observation import Observation, Prediction, PredictionError
from .scalars import Scalar

from dataclasses import field as _field


# ---------------------------------------------------------------------------
# Bounded Vec (ring buffer)
# ---------------------------------------------------------------------------

class BoundedVec:
    """Fixed-capacity ring buffer backed by a plain list."""

    __slots__ = ("_items", "_capacity")

    def __init__(self, capacity: int) -> None:
        self._items: list[Any] = []
        self._capacity = capacity

    def push(self, item: Any) -> None:
        if len(self._items) >= self._capacity:
            self._items.pop(0)
        self._items.append(item)

    def __iter__(self):
        return iter(self._items)

    def __len__(self) -> int:
        return len(self._items)

    def __bool__(self) -> bool:
        return len(self._items) > 0

    def __getitem__(self, idx: int) -> Any:
        return self._items[idx]

    def is_empty(self) -> bool:
        return len(self._items) == 0

    def to_dict(self) -> dict[str, Any]:
        return {"capacity": self._capacity, "items": [
            item.to_dict() if hasattr(item, "to_dict") else item
            for item in self._items
        ]}


# ---------------------------------------------------------------------------
# Action types
# ---------------------------------------------------------------------------

@dataclass(slots=True)
class ActionParameter:
    """Typed parameter for an action."""
    kind: str = "text"
    text_value: str = ""
    number_value: float = 0.0
    integer_value: int = 0
    boolean_value: bool = False
    list_value: list[ActionParameter] = field(default_factory=list)

    @classmethod
    def text(cls, value: str) -> ActionParameter:
        return cls(kind="text", text_value=value)

    @classmethod
    def number(cls, value: float) -> ActionParameter:
        return cls(kind="number", number_value=value)

    @classmethod
    def integer(cls, value: int) -> ActionParameter:
        return cls(kind="integer", integer_value=value)

    @classmethod
    def boolean(cls, value: bool) -> ActionParameter:
        return cls(kind="boolean", boolean_value=value)

    def to_dict(self) -> dict[str, Any]:
        d: dict[str, Any] = {"kind": self.kind}
        if self.kind == "text":
            d["text_value"] = self.text_value
        elif self.kind == "number":
            d["number_value"] = self.number_value
        elif self.kind == "integer":
            d["integer_value"] = self.integer_value
        elif self.kind == "boolean":
            d["boolean_value"] = self.boolean_value
        elif self.kind == "list":
            d["list_value"] = [p.to_dict() for p in self.list_value]
        return d

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ActionParameter:
        kind = data["kind"]
        if kind == "text":
            return cls.text(data["text_value"])
        if kind == "number":
            return cls.number(data["number_value"])
        if kind == "integer":
            return cls.integer(data["integer_value"])
        if kind == "boolean":
            return cls.boolean(data["boolean_value"])
        if kind == "list":
            return cls(kind="list", list_value=[cls.from_dict(p) for p in data.get("list_value", [])])
        return cls()


@dataclass(slots=True)
class Outcome:
    """Result of an action."""
    success: bool = True
    description: str = ""
    result: str | None = None
    timestamp: Timestamp = field(default_factory=Timestamp.now)
    confidence: Scalar = Scalar(0.5)

    def to_dict(self) -> dict[str, Any]:
        return {
            "success": self.success,
            "description": self.description,
            "result": self.result,
            "timestamp": self.timestamp.to_dict(),
            "confidence": float(self.confidence),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Outcome:
        return cls(
            success=data["success"],
            description=data["description"],
            result=data.get("result"),
            timestamp=Timestamp.from_dict(data["timestamp"]),
            confidence=Scalar(data["confidence"]),
        )


@dataclass(slots=True)
class RiskFactor:
    """Individual risk factor."""
    description: str = ""
    severity: Scalar = Scalar(0.0)
    likelihood: Scalar = Scalar(0.0)

    def to_dict(self) -> dict[str, Any]:
        return {
            "description": self.description,
            "severity": float(self.severity),
            "likelihood": float(self.likelihood),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> RiskFactor:
        return cls(
            description=data["description"],
            severity=Scalar(data["severity"]),
            likelihood=Scalar(data["likelihood"]),
        )


@dataclass(slots=True)
class RiskAssessment:
    """Risk assessment for an action or procedure."""
    score: Scalar = Scalar(0.0)
    level: RiskLevel = RiskLevel.NONE
    factors: list[RiskFactor] = field(default_factory=list)
    reversibility: Scalar = Scalar(1.0)

    def to_dict(self) -> dict[str, Any]:
        return {
            "score": float(self.score),
            "level": self.level.value,
            "factors": [f.to_dict() for f in self.factors],
            "reversibility": float(self.reversibility),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> RiskAssessment:
        return cls(
            score=Scalar(data["score"]),
            level=RiskLevel(data["level"]),
            factors=[RiskFactor.from_dict(f) for f in data["factors"]],
            reversibility=Scalar(data["reversibility"]),
        )


@dataclass(slots=True)
class Action:
    """An action that CORTEX may take or has taken."""
    id: ActionId = field(default_factory=ActionId.generate)
    kind: str = "respond"
    parameters: dict[str, ActionParameter] = field(default_factory=dict)
    expected_outcome: Outcome | None = None
    risk: RiskAssessment = field(default_factory=RiskAssessment)
    timestamp: Timestamp = field(default_factory=Timestamp.now)
    provenance: Provenance = field(default_factory=Provenance.user_provided)

    def to_dict(self) -> dict[str, Any]:
        return {
            "id": self.id.to_dict(),
            "kind": self.kind,
            "parameters": {k: v.to_dict() for k, v in self.parameters.items()},
            "expected_outcome": self.expected_outcome.to_dict() if self.expected_outcome else None,
            "risk": self.risk.to_dict(),
            "timestamp": self.timestamp.to_dict(),
            "provenance": self.provenance.to_dict(),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Action:
        eo = None
        if data.get("expected_outcome"):
            eo = Outcome.from_dict(data["expected_outcome"])
        return cls(
            id=ActionId.from_dict(data["id"]),
            kind=data["kind"],
            parameters={k: ActionParameter.from_dict(v) for k, v in data.get("parameters", {}).items()},
            expected_outcome=eo,
            risk=RiskAssessment.from_dict(data["risk"]),
            timestamp=Timestamp.from_dict(data["timestamp"]),
            provenance=Provenance.from_dict(data["provenance"]),
        )


# ---------------------------------------------------------------------------
# Property & Relation (used by semantic memory and world model)
# ---------------------------------------------------------------------------

@dataclass(slots=True)
class PropertyValue:
    """Typed property value."""
    kind: str = "text"
    text_value: str = ""
    number_value: float = 0.0
    boolean_value: bool = False
    concept_ref: str | None = None
    entity_ref: str | None = None
    list_value: list[PropertyValue] = field(default_factory=list)

    @classmethod
    def text(cls, value: str) -> PropertyValue:
        return cls(kind="text", text_value=value)

    @classmethod
    def number(cls, value: float) -> PropertyValue:
        return cls(kind="number", number_value=value)

    @classmethod
    def boolean(cls, value: bool) -> PropertyValue:
        return cls(kind="boolean", boolean_value=value)

    def to_dict(self) -> dict[str, Any]:
        d: dict[str, Any] = {"kind": self.kind}
        if self.kind == "text":
            d["text_value"] = self.text_value
        elif self.kind == "number":
            d["number_value"] = self.number_value
        elif self.kind == "boolean":
            d["boolean_value"] = self.boolean_value
        elif self.kind == "list":
            d["list_value"] = [v.to_dict() for v in self.list_value]
        return d

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> PropertyValue:
        kind = data["kind"]
        if kind == "text":
            return cls.text(data["text_value"])
        if kind == "number":
            return cls.number(data["number_value"])
        if kind == "boolean":
            return cls.boolean(data["boolean_value"])
        if kind == "list":
            return cls(kind="list", list_value=[cls.from_dict(v) for v in data.get("list_value", [])])
        return cls()


@dataclass(slots=True)
class Property:
    """A named property of a concept or entity."""
    name: str = ""
    value: PropertyValue = field(default_factory=PropertyValue)
    confidence: Scalar = Scalar(0.5)
    provenance: Provenance = field(default_factory=Provenance.user_provided)

    def to_dict(self) -> dict[str, Any]:
        return {
            "name": self.name,
            "value": self.value.to_dict(),
            "confidence": float(self.confidence),
            "provenance": self.provenance.to_dict(),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Property:
        return cls(
            name=data["name"],
            value=PropertyValue.from_dict(data["value"]),
            confidence=Scalar(data["confidence"]),
            provenance=Provenance.from_dict(data["provenance"]),
        )


@dataclass(slots=True)
class Relation:
    """A typed relation between two internal IDs."""
    id: RelationId = field(default_factory=RelationId.generate)
    kind: RelationKind = RelationKind.RELATED_TO
    source: str = ""
    target: str = ""
    confidence: Scalar = Scalar(0.5)
    provenance: Provenance = field(default_factory=Provenance.user_provided)

    def to_dict(self) -> dict[str, Any]:
        return {
            "id": self.id.to_dict(),
            "kind": self.kind.value,
            "source": self.source,
            "target": self.target,
            "confidence": float(self.confidence),
            "provenance": self.provenance.to_dict(),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Relation:
        return cls(
            id=RelationId.from_dict(data["id"]),
            kind=RelationKind(data["kind"]),
            source=data["source"],
            target=data["target"],
            confidence=Scalar(data["confidence"]),
            provenance=Provenance.from_dict(data["provenance"]),
        )


# ---------------------------------------------------------------------------
# Memory types
# ---------------------------------------------------------------------------

@dataclass(slots=True)
class CurrentInput:
    """The input currently being processed."""
    text: str = ""
    timestamp: Timestamp = field(default_factory=Timestamp.now)
    kind: str = "user_input"

    def to_dict(self) -> dict[str, Any]:
        return {
            "text": self.text,
            "timestamp": self.timestamp.to_dict(),
            "kind": self.kind,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> CurrentInput:
        return cls(
            text=data["text"],
            timestamp=Timestamp.from_dict(data["timestamp"]),
            kind=data["kind"],
        )


@dataclass(slots=True)
class ConversationContext:
    """Tracks conversation-level state."""
    session_id: str = ""
    turn_count: int = 0
    recent_inputs: list[str] = field(default_factory=list)
    recent_outputs: list[str] = field(default_factory=list)
    started_at: Timestamp = field(default_factory=Timestamp.now)

    def to_dict(self) -> dict[str, Any]:
        return {
            "session_id": self.session_id,
            "turn_count": self.turn_count,
            "recent_inputs": list(self.recent_inputs),
            "recent_outputs": list(self.recent_outputs),
            "started_at": self.started_at.to_dict(),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ConversationContext:
        return cls(
            session_id=data["session_id"],
            turn_count=data["turn_count"],
            recent_inputs=data.get("recent_inputs", []),
            recent_outputs=data.get("recent_outputs", []),
            started_at=Timestamp.from_dict(data["started_at"]),
        )


@dataclass(slots=True)
class ReasoningSnapshot:
    """Snapshot of reasoning state for working memory."""
    active_hypotheses: list[str] = field(default_factory=list)
    current_step: int = 0
    budget_remaining: int = 0

    def to_dict(self) -> dict[str, Any]:
        return {
            "active_hypotheses": list(self.active_hypotheses),
            "current_step": self.current_step,
            "budget_remaining": self.budget_remaining,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ReasoningSnapshot:
        return cls(
            active_hypotheses=data.get("active_hypotheses", []),
            current_step=data.get("current_step", 0),
            budget_remaining=data.get("budget_remaining", 0),
        )


@dataclass(slots=True)
class CandidateContinuation:
    """A candidate token continuation during generation."""
    text: str = ""
    score: Scalar = Scalar(0.0)

    def to_dict(self) -> dict[str, Any]:
        return {"text": self.text, "score": float(self.score)}

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> CandidateContinuation:
        return cls(text=data["text"], score=Scalar(data["score"]))


@dataclass(slots=True)
class GenerationState:
    """Current state of text generation."""
    tokens_generated: int = 0
    max_tokens: int = 0
    current_candidates: list[CandidateContinuation] = field(default_factory=list)

    def to_dict(self) -> dict[str, Any]:
        return {
            "tokens_generated": self.tokens_generated,
            "max_tokens": self.max_tokens,
            "current_candidates": [c.to_dict() for c in self.current_candidates],
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> GenerationState:
        return cls(
            tokens_generated=data["tokens_generated"],
            max_tokens=data["max_tokens"],
            current_candidates=[CandidateContinuation.from_dict(c) for c in data.get("current_candidates", [])],
        )


@dataclass(slots=True)
class WorkingMemory:
    """Active cognitive state. Bounded by memory.working_mb."""
    input: CurrentInput | None = None
    conversation_context: ConversationContext = field(default_factory=ConversationContext)
    active_concepts: list[str] = field(default_factory=list)
    active_hypotheses: list[str] = field(default_factory=list)
    goals: list[str] = field(default_factory=list)
    reasoning_state: ReasoningSnapshot | None = None
    world_assumptions: list[str] = field(default_factory=list)
    generation_state: GenerationState | None = None

    def to_dict(self) -> dict[str, Any]:
        return {
            "input": self.input.to_dict() if self.input else None,
            "conversation_context": self.conversation_context.to_dict(),
            "active_concepts": list(self.active_concepts),
            "active_hypotheses": list(self.active_hypotheses),
            "goals": list(self.goals),
            "reasoning_state": self.reasoning_state.to_dict() if self.reasoning_state else None,
            "world_assumptions": list(self.world_assumptions),
            "generation_state": self.generation_state.to_dict() if self.generation_state else None,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> WorkingMemory:
        inp = CurrentInput.from_dict(data["input"]) if data.get("input") else None
        rs = ReasoningSnapshot.from_dict(data["reasoning_state"]) if data.get("reasoning_state") else None
        gs = GenerationState.from_dict(data["generation_state"]) if data.get("generation_state") else None
        return cls(
            input=inp,
            conversation_context=ConversationContext.from_dict(data["conversation_context"]),
            active_concepts=data.get("active_concepts", []),
            active_hypotheses=data.get("active_hypotheses", []),
            goals=data.get("goals", []),
            reasoning_state=rs,
            world_assumptions=data.get("world_assumptions", []),
            generation_state=gs,
        )


@dataclass(slots=True)
class Episode:
    """A single experience episode in episodic memory."""
    id: EpisodeId = field(default_factory=EpisodeId.generate)
    observation: Observation = field(default_factory=Observation)
    context: Any = None  # ContextState – forward ref resolved at runtime
    action: Action | None = None
    outcome: Outcome | None = None
    timestamp: Timestamp = field(default_factory=Timestamp.now)
    prediction: Prediction | None = None
    prediction_error: PredictionError = field(default_factory=PredictionError)
    confidence: ConfidenceState = field(default_factory=ConfidenceState.default)
    source: Provenance = field(default_factory=Provenance.user_provided)
    importance: Scalar = Scalar(0.5)
    retrieval_count: int = 0
    last_retrieved: Timestamp | None = None
    consolidated: bool = False

    def to_dict(self) -> dict[str, Any]:
        return {
            "id": self.id.to_dict(),
            "observation": self.observation.to_dict(),
            "action": self.action.to_dict() if self.action else None,
            "outcome": self.outcome.to_dict() if self.outcome else None,
            "timestamp": self.timestamp.to_dict(),
            "prediction": self.prediction.to_dict() if self.prediction else None,
            "prediction_error": self.prediction_error.to_dict(),
            "confidence": self.confidence.to_dict(),
            "source": self.source.to_dict(),
            "importance": float(self.importance),
            "retrieval_count": self.retrieval_count,
            "last_retrieved": self.last_retrieved.to_dict() if self.last_retrieved else None,
            "consolidated": self.consolidated,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Episode:
        action = Action.from_dict(data["action"]) if data.get("action") else None
        outcome = Outcome.from_dict(data["outcome"]) if data.get("outcome") else None
        pred = Prediction.from_dict(data["prediction"]) if data.get("prediction") else None
        lr = Timestamp.from_dict(data["last_retrieved"]) if data.get("last_retrieved") else None
        return cls(
            id=EpisodeId.from_dict(data["id"]),
            observation=Observation.from_dict(data["observation"]),
            action=action,
            outcome=outcome,
            timestamp=Timestamp.from_dict(data["timestamp"]),
            prediction=pred,
            prediction_error=PredictionError.from_dict(data["prediction_error"]),
            confidence=ConfidenceState.from_dict(data["confidence"]),
            source=Provenance.from_dict(data["source"]),
            importance=Scalar(data["importance"]),
            retrieval_count=data.get("retrieval_count", 0),
            last_retrieved=lr,
            consolidated=data.get("consolidated", False),
        )


@dataclass(slots=True)
class EpisodicMemory:
    """Stores experience episodes. Bounded by memory.episodic_mb."""
    episodes: list[Episode] = field(default_factory=list)
    capacity_bytes: int = 0
    current_usage_bytes: int = 0
    eviction_policy: EvictionPolicy = EvictionPolicy.LRU

    def to_dict(self) -> dict[str, Any]:
        return {
            "episodes": [e.to_dict() for e in self.episodes],
            "capacity_bytes": self.capacity_bytes,
            "current_usage_bytes": self.current_usage_bytes,
            "eviction_policy": self.eviction_policy.value,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> EpisodicMemory:
        return cls(
            episodes=[Episode.from_dict(e) for e in data.get("episodes", [])],
            capacity_bytes=data.get("capacity_bytes", 0),
            current_usage_bytes=data.get("current_usage_bytes", 0),
            eviction_policy=EvictionPolicy(data["eviction_policy"]),
        )


@dataclass(slots=True)
class Knowledge:
    """A single knowledge item in semantic memory."""
    id: KnowledgeId = field(default_factory=KnowledgeId.generate)
    concept: str = ""
    properties: list[Property] = field(default_factory=list)
    relations: list[Relation] = field(default_factory=list)
    evidence: EvidenceSet = field(default_factory=EvidenceSet)
    confidence: ConfidenceState = field(default_factory=ConfidenceState.default)
    provenance: list[Provenance] = field(default_factory=list)
    verification_status: VerificationStatus = VerificationStatus.OBSERVED
    created_at: Timestamp = field(default_factory=Timestamp.now)
    updated_at: Timestamp = field(default_factory=Timestamp.now)
    confirmation_count: int = 0
    contradiction_count: int = 0

    def to_dict(self) -> dict[str, Any]:
        return {
            "id": self.id.to_dict(),
            "concept": self.concept,
            "properties": [p.to_dict() for p in self.properties],
            "relations": [r.to_dict() for r in self.relations],
            "evidence": self.evidence.to_dict(),
            "confidence": self.confidence.to_dict(),
            "provenance": [p.to_dict() for p in self.provenance],
            "verification_status": self.verification_status.value,
            "created_at": self.created_at.to_dict(),
            "updated_at": self.updated_at.to_dict(),
            "confirmation_count": self.confirmation_count,
            "contradiction_count": self.contradiction_count,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Knowledge:
        return cls(
            id=KnowledgeId.from_dict(data["id"]),
            concept=data["concept"],
            properties=[Property.from_dict(p) for p in data.get("properties", [])],
            relations=[Relation.from_dict(r) for r in data.get("relations", [])],
            evidence=EvidenceSet.from_dict(data.get("evidence", {"items": []})),
            confidence=ConfidenceState.from_dict(data["confidence"]),
            provenance=[Provenance.from_dict(p) for p in data.get("provenance", [])],
            verification_status=VerificationStatus(data["verification_status"]),
            created_at=Timestamp.from_dict(data["created_at"]),
            updated_at=Timestamp.from_dict(data["updated_at"]),
            confirmation_count=data.get("confirmation_count", 0),
            contradiction_count=data.get("contradiction_count", 0),
        )


@dataclass(slots=True)
class SemanticMemory:
    """Stores knowledge items. Bounded by memory.semantic_mb."""
    knowledge: list[Knowledge] = field(default_factory=list)
    capacity_bytes: int = 0
    current_usage_bytes: int = 0
    eviction_policy: EvictionPolicy = EvictionPolicy.LOWEST_CONFIDENCE

    def to_dict(self) -> dict[str, Any]:
        return {
            "knowledge": [k.to_dict() for k in self.knowledge],
            "capacity_bytes": self.capacity_bytes,
            "current_usage_bytes": self.current_usage_bytes,
            "eviction_policy": self.eviction_policy.value,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> SemanticMemory:
        return cls(
            knowledge=[Knowledge.from_dict(k) for k in data.get("knowledge", [])],
            capacity_bytes=data.get("capacity_bytes", 0),
            current_usage_bytes=data.get("current_usage_bytes", 0),
            eviction_policy=EvictionPolicy(data["eviction_policy"]),
        )


@dataclass(slots=True)
class Condition:
    """Condition under which a procedure applies."""
    description: str = ""
    required_concepts: list[str] = field(default_factory=list)
    required_entities: list[str] = field(default_factory=list)

    def to_dict(self) -> dict[str, Any]:
        return {
            "description": self.description,
            "required_concepts": list(self.required_concepts),
            "required_entities": list(self.required_entities),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Condition:
        return cls(
            description=data["description"],
            required_concepts=data.get("required_concepts", []),
            required_entities=data.get("required_entities", []),
        )


@dataclass(slots=True)
class ContextRequirements:
    """Resource requirements for a procedure."""
    requires_world_model: bool = False
    requires_memory: bool = False
    requires_reasoning: bool = False
    max_context_tokens: int = 0

    def to_dict(self) -> dict[str, Any]:
        return {
            "requires_world_model": self.requires_world_model,
            "requires_memory": self.requires_memory,
            "requires_reasoning": self.requires_reasoning,
            "max_context_tokens": self.max_context_tokens,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ContextRequirements:
        return cls(**{k: data[k] for k in data})


@dataclass(slots=True)
class Procedure:
    """A stored procedure in procedural memory."""
    id: ProcedureId = field(default_factory=ProcedureId.generate)
    condition: Condition = field(default_factory=Condition)
    steps: list[Action] = field(default_factory=list)
    expected_outcome: Outcome = field(default_factory=Outcome)
    success_count: int = 0
    failure_count: int = 0
    confidence: Scalar = Scalar(0.5)
    context_requirements: ContextRequirements = field(default_factory=ContextRequirements)
    risk: RiskAssessment = field(default_factory=RiskAssessment)
    provenance: Provenance = field(default_factory=Provenance.user_provided)
    created_at: Timestamp = field(default_factory=Timestamp.now)
    last_used: Timestamp | None = None

    def to_dict(self) -> dict[str, Any]:
        return {
            "id": self.id.to_dict(),
            "condition": self.condition.to_dict(),
            "steps": [s.to_dict() for s in self.steps],
            "expected_outcome": self.expected_outcome.to_dict(),
            "success_count": self.success_count,
            "failure_count": self.failure_count,
            "confidence": float(self.confidence),
            "context_requirements": self.context_requirements.to_dict(),
            "risk": self.risk.to_dict(),
            "provenance": self.provenance.to_dict(),
            "created_at": self.created_at.to_dict(),
            "last_used": self.last_used.to_dict() if self.last_used else None,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Procedure:
        lu = Timestamp.from_dict(data["last_used"]) if data.get("last_used") else None
        return cls(
            id=ProcedureId.from_dict(data["id"]),
            condition=Condition.from_dict(data["condition"]),
            steps=[Action.from_dict(s) for s in data.get("steps", [])],
            expected_outcome=Outcome.from_dict(data["expected_outcome"]),
            success_count=data.get("success_count", 0),
            failure_count=data.get("failure_count", 0),
            confidence=Scalar(data["confidence"]),
            context_requirements=ContextRequirements.from_dict(data["context_requirements"]),
            risk=RiskAssessment.from_dict(data["risk"]),
            provenance=Provenance.from_dict(data["provenance"]),
            created_at=Timestamp.from_dict(data["created_at"]),
            last_used=lu,
        )


@dataclass(slots=True)
class ProceduralMemory:
    """Stores procedures/skills. Bounded by memory.procedural_mb."""
    procedures: list[Procedure] = field(default_factory=list)
    capacity_bytes: int = 0
    current_usage_bytes: int = 0

    def to_dict(self) -> dict[str, Any]:
        return {
            "procedures": [p.to_dict() for p in self.procedures],
            "capacity_bytes": self.capacity_bytes,
            "current_usage_bytes": self.current_usage_bytes,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ProceduralMemory:
        return cls(
            procedures=[Procedure.from_dict(p) for p in data.get("procedures", [])],
            capacity_bytes=data.get("capacity_bytes", 0),
            current_usage_bytes=data.get("current_usage_bytes", 0),
        )


@dataclass(slots=True)
class Association:
    """A typed association between two internal structures."""
    id: AssociationId = field(default_factory=AssociationId.generate)
    source: str = ""
    target: str = ""
    kind: AssociationKind = AssociationKind.SEMANTIC
    strength: Scalar = Scalar(0.0)
    confidence: Scalar = Scalar(0.5)
    provenance: Provenance = field(default_factory=Provenance.user_provided)
    created_at: Timestamp = field(default_factory=Timestamp.now)
    last_strengthened: Timestamp = field(default_factory=Timestamp.now)
    activation_count: int = 0

    def to_dict(self) -> dict[str, Any]:
        return {
            "id": self.id.to_dict(),
            "source": self.source,
            "target": self.target,
            "kind": self.kind.value,
            "strength": float(self.strength),
            "confidence": float(self.confidence),
            "provenance": self.provenance.to_dict(),
            "created_at": self.created_at.to_dict(),
            "last_strengthened": self.last_strengthened.to_dict(),
            "activation_count": self.activation_count,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Association:
        return cls(
            id=AssociationId.from_dict(data["id"]),
            source=data["source"],
            target=data["target"],
            kind=AssociationKind(data["kind"]),
            strength=Scalar(data["strength"]),
            confidence=Scalar(data["confidence"]),
            provenance=Provenance.from_dict(data["provenance"]),
            created_at=Timestamp.from_dict(data["created_at"]),
            last_strengthened=Timestamp.from_dict(data["last_strengthened"]),
            activation_count=data.get("activation_count", 0),
        )


@dataclass(slots=True)
class AssociativeMemory:
    """Stores typed associations. Bounded by memory.associative_mb."""
    associations: list[Association] = field(default_factory=list)
    capacity_bytes: int = 0
    current_usage_bytes: int = 0

    def to_dict(self) -> dict[str, Any]:
        return {
            "associations": [a.to_dict() for a in self.associations],
            "capacity_bytes": self.capacity_bytes,
            "current_usage_bytes": self.current_usage_bytes,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> AssociativeMemory:
        return cls(
            associations=[Association.from_dict(a) for a in data.get("associations", [])],
            capacity_bytes=data.get("capacity_bytes", 0),
            current_usage_bytes=data.get("current_usage_bytes", 0),
        )


@dataclass(slots=True)
class MemoryState:
    """Complete memory state."""
    working: WorkingMemory = field(default_factory=WorkingMemory)
    episodic: EpisodicMemory = field(default_factory=EpisodicMemory)
    semantic: SemanticMemory = field(default_factory=SemanticMemory)
    procedural: ProceduralMemory = field(default_factory=ProceduralMemory)
    associative: AssociativeMemory = field(default_factory=AssociativeMemory)

    def to_dict(self) -> dict[str, Any]:
        return {
            "working": self.working.to_dict(),
            "episodic": self.episodic.to_dict(),
            "semantic": self.semantic.to_dict(),
            "procedural": self.procedural.to_dict(),
            "associative": self.associative.to_dict(),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> MemoryState:
        return cls(
            working=WorkingMemory.from_dict(data["working"]),
            episodic=EpisodicMemory.from_dict(data["episodic"]),
            semantic=SemanticMemory.from_dict(data["semantic"]),
            procedural=ProceduralMemory.from_dict(data["procedural"]),
            associative=AssociativeMemory.from_dict(data["associative"]),
        )


@dataclass(slots=True)
class MemoryUsage:
    """Memory usage report."""
    working_bytes: int = 0
    episodic_bytes: int = 0
    semantic_bytes: int = 0
    procedural_bytes: int = 0
    associative_bytes: int = 0
    total_bytes: int = 0
    pressure: MemoryPressure = MemoryPressure.LOW

    def to_dict(self) -> dict[str, Any]:
        return {
            "working_bytes": self.working_bytes,
            "episodic_bytes": self.episodic_bytes,
            "semantic_bytes": self.semantic_bytes,
            "procedural_bytes": self.procedural_bytes,
            "associative_bytes": self.associative_bytes,
            "total_bytes": self.total_bytes,
            "pressure": self.pressure.value,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> MemoryUsage:
        return cls(**{
            k: (MemoryPressure(v) if k == "pressure" else v)
            for k, v in data.items()
        })


# ---------------------------------------------------------------------------
# World Model types
# ---------------------------------------------------------------------------

@dataclass(slots=True)
class IdentityState:
    """Entity identity metadata."""
    name: str = ""
    aliases: list[str] = field(default_factory=list)
    unique_identifier: str | None = None
    identity_confidence: Scalar = Scalar(0.5)

    def to_dict(self) -> dict[str, Any]:
        return {
            "name": self.name,
            "aliases": list(self.aliases),
            "unique_identifier": self.unique_identifier,
            "identity_confidence": float(self.identity_confidence),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> IdentityState:
        return cls(
            name=data["name"],
            aliases=data.get("aliases", []),
            unique_identifier=data.get("unique_identifier"),
            identity_confidence=Scalar(data["identity_confidence"]),
        )


@dataclass(slots=True)
class EntityState:
    """Entity mutable state."""
    state_description: str = ""
    state_properties: list[Property] = field(default_factory=list)
    state_timestamp: Timestamp = field(default_factory=Timestamp.now)
    state_confidence: Scalar = Scalar(0.5)

    def to_dict(self) -> dict[str, Any]:
        return {
            "state_description": self.state_description,
            "state_properties": [p.to_dict() for p in self.state_properties],
            "state_timestamp": self.state_timestamp.to_dict(),
            "state_confidence": float(self.state_confidence),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> EntityState:
        return cls(
            state_description=data["state_description"],
            state_properties=[Property.from_dict(p) for p in data.get("state_properties", [])],
            state_timestamp=Timestamp.from_dict(data["state_timestamp"]),
            state_confidence=Scalar(data["state_confidence"]),
        )


@dataclass(slots=True)
class Entity:
    """An entity in the world model."""
    id: EntityId = field(default_factory=EntityId.generate)
    kind: EntityKind = EntityKind.OBJECT
    identity: IdentityState = field(default_factory=IdentityState)
    properties: list[Property] = field(default_factory=list)
    state: EntityState = field(default_factory=EntityState)
    relations: list[str] = field(default_factory=list)
    confidence: Scalar = Scalar(0.5)
    provenance: list[Provenance] = field(default_factory=list)
    created_at: Timestamp = field(default_factory=Timestamp.now)
    updated_at: Timestamp = field(default_factory=Timestamp.now)

    def to_dict(self) -> dict[str, Any]:
        return {
            "id": self.id.to_dict(),
            "kind": self.kind.value,
            "identity": self.identity.to_dict(),
            "properties": [p.to_dict() for p in self.properties],
            "state": self.state.to_dict(),
            "relations": list(self.relations),
            "confidence": float(self.confidence),
            "provenance": [p.to_dict() for p in self.provenance],
            "created_at": self.created_at.to_dict(),
            "updated_at": self.updated_at.to_dict(),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Entity:
        return cls(
            id=EntityId.from_dict(data["id"]),
            kind=EntityKind(data["kind"]),
            identity=IdentityState.from_dict(data["identity"]),
            properties=[Property.from_dict(p) for p in data.get("properties", [])],
            state=EntityState.from_dict(data["state"]),
            relations=data.get("relations", []),
            confidence=Scalar(data["confidence"]),
            provenance=[Provenance.from_dict(p) for p in data.get("provenance", [])],
            created_at=Timestamp.from_dict(data["created_at"]),
            updated_at=Timestamp.from_dict(data["updated_at"]),
        )


@dataclass(slots=True)
class Event:
    """An event in the world model."""
    id: EventId = field(default_factory=EventId.generate)
    description: str = ""
    participants: list[str] = field(default_factory=list)
    timestamp: Timestamp = field(default_factory=Timestamp.now)
    duration: Duration | None = None
    outcome: Outcome | None = None
    provenance: Provenance = field(default_factory=Provenance.user_provided)

    def to_dict(self) -> dict[str, Any]:
        return {
            "id": self.id.to_dict(),
            "description": self.description,
            "participants": list(self.participants),
            "timestamp": self.timestamp.to_dict(),
            "duration": self.duration.to_dict() if self.duration else None,
            "outcome": self.outcome.to_dict() if self.outcome else None,
            "provenance": self.provenance.to_dict(),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Event:
        dur = Duration.from_dict(data["duration"]) if data.get("duration") else None
        out = Outcome.from_dict(data["outcome"]) if data.get("outcome") else None
        return cls(
            id=EventId.from_dict(data["id"]),
            description=data["description"],
            participants=data.get("participants", []),
            timestamp=Timestamp.from_dict(data["timestamp"]),
            duration=dur,
            outcome=out,
            provenance=Provenance.from_dict(data["provenance"]),
        )


@dataclass(slots=True)
class WorldState:
    """Persistent world-model snapshot."""
    entities: list[Entity] = field(default_factory=list)
    relations: list[Relation] = field(default_factory=list)
    active_events: list[Event] = field(default_factory=list)
    temporal_context: TemporalContext = field(default_factory=TemporalContext)
    uncertainty: UncertaintyState = field(default_factory=UncertaintyState.initial)

    def to_dict(self) -> dict[str, Any]:
        return {
            "entities": [e.to_dict() for e in self.entities],
            "relations": [r.to_dict() for r in self.relations],
            "active_events": [ev.to_dict() for ev in self.active_events],
            "temporal_context": self.temporal_context.to_dict(),
            "uncertainty": self.uncertainty.to_dict(),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> WorldState:
        return cls(
            entities=[Entity.from_dict(e) for e in data.get("entities", [])],
            relations=[Relation.from_dict(r) for r in data.get("relations", [])],
            active_events=[Event.from_dict(e) for e in data.get("active_events", [])],
            temporal_context=TemporalContext.from_dict(data["temporal_context"]),
            uncertainty=UncertaintyState.from_dict(data["uncertainty"]),
        )


# ---------------------------------------------------------------------------
# Reasoning types
# ---------------------------------------------------------------------------

@dataclass(slots=True)
class Proposition:
    """A logical proposition used in reasoning."""
    subject: str = ""
    predicate: str = ""
    object: str | None = None
    modifiers: list[str] = field(default_factory=list)
    negated: bool = False

    def to_dict(self) -> dict[str, Any]:
        return {
            "subject": self.subject,
            "predicate": self.predicate,
            "object": self.object,
            "modifiers": list(self.modifiers),
            "negated": self.negated,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Proposition:
        return cls(
            subject=data["subject"],
            predicate=data["predicate"],
            object=data.get("object"),
            modifiers=data.get("modifiers", []),
            negated=data.get("negated", False),
        )


@dataclass(slots=True)
class Contradiction:
    """A detected contradiction between two hypotheses."""
    claim_a: str = ""
    claim_b: str = ""
    description: str = ""
    severity: Scalar = Scalar(0.0)
    detected_at: Timestamp = field(default_factory=Timestamp.now)
    resolved: bool = False

    def to_dict(self) -> dict[str, Any]:
        return {
            "claim_a": self.claim_a,
            "claim_b": self.claim_b,
            "description": self.description,
            "severity": float(self.severity),
            "detected_at": self.detected_at.to_dict(),
            "resolved": self.resolved,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Contradiction:
        return cls(
            claim_a=data["claim_a"],
            claim_b=data["claim_b"],
            description=data["description"],
            severity=Scalar(data["severity"]),
            detected_at=Timestamp.from_dict(data["detected_at"]),
            resolved=data.get("resolved", False),
        )


@dataclass(slots=True)
class Hypothesis:
    """A hypothesis in the reasoning workspace."""
    id: HypothesisId = field(default_factory=HypothesisId.generate)
    proposition: Proposition = field(default_factory=Proposition)
    evidence: EvidenceSet = field(default_factory=EvidenceSet)
    counter_evidence: EvidenceSet = field(default_factory=EvidenceSet)
    confidence: Scalar = Scalar(0.0)
    dependencies: list[str] = field(default_factory=list)
    contradictions: list[Contradiction] = field(default_factory=list)
    provenance: list[Provenance] = field(default_factory=list)
    reasoning_type: ReasoningType = ReasoningType.DEDUCTIVE
    created_at: Timestamp = field(default_factory=Timestamp.now)

    def to_dict(self) -> dict[str, Any]:
        return {
            "id": self.id.to_dict(),
            "proposition": self.proposition.to_dict(),
            "evidence": self.evidence.to_dict(),
            "counter_evidence": self.counter_evidence.to_dict(),
            "confidence": float(self.confidence),
            "dependencies": list(self.dependencies),
            "contradictions": [c.to_dict() for c in self.contradictions],
            "provenance": [p.to_dict() for p in self.provenance],
            "reasoning_type": self.reasoning_type.value,
            "created_at": self.created_at.to_dict(),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Hypothesis:
        return cls(
            id=HypothesisId.from_dict(data["id"]),
            proposition=Proposition.from_dict(data["proposition"]),
            evidence=EvidenceSet.from_dict(data["evidence"]),
            counter_evidence=EvidenceSet.from_dict(data["counter_evidence"]),
            confidence=Scalar(data["confidence"]),
            dependencies=data.get("dependencies", []),
            contradictions=[Contradiction.from_dict(c) for c in data.get("contradictions", [])],
            provenance=[Provenance.from_dict(p) for p in data.get("provenance", [])],
            reasoning_type=ReasoningType(data["reasoning_type"]),
            created_at=Timestamp.from_dict(data["created_at"]),
        )


@dataclass(slots=True)
class Conclusion:
    """A reasoning conclusion."""
    hypothesis_id: str = ""
    proposition: Proposition = field(default_factory=Proposition)
    confidence: Scalar = Scalar(0.0)
    evidence_strength: Scalar = Scalar(0.0)
    reasoning_steps: int = 0
    bounded: bool = False

    def to_dict(self) -> dict[str, Any]:
        return {
            "hypothesis_id": self.hypothesis_id,
            "proposition": self.proposition.to_dict(),
            "confidence": float(self.confidence),
            "evidence_strength": float(self.evidence_strength),
            "reasoning_steps": self.reasoning_steps,
            "bounded": self.bounded,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Conclusion:
        return cls(
            hypothesis_id=data["hypothesis_id"],
            proposition=Proposition.from_dict(data["proposition"]),
            confidence=Scalar(data["confidence"]),
            evidence_strength=Scalar(data["evidence_strength"]),
            reasoning_steps=data["reasoning_steps"],
            bounded=data.get("bounded", False),
        )


@dataclass(slots=True)
class ReasoningState:
    """Tracks active reasoning process."""
    active_hypotheses: list[Hypothesis] = field(default_factory=list)
    conclusion: Conclusion | None = None
    premises: list[Proposition] = field(default_factory=list)
    evidence_index: dict[str, list[str]] = field(default_factory=dict)
    contradiction_log: list[Contradiction] = field(default_factory=list)
    budget_remaining: int = 100

    def to_dict(self) -> dict[str, Any]:
        return {
            "active_hypotheses": [h.to_dict() for h in self.active_hypotheses],
            "conclusion": self.conclusion.to_dict() if self.conclusion else None,
            "premises": [p.to_dict() for p in self.premises],
            "evidence_index": dict(self.evidence_index),
            "contradiction_log": [c.to_dict() for c in self.contradiction_log],
            "budget_remaining": self.budget_remaining,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ReasoningState:
        conc = Conclusion.from_dict(data["conclusion"]) if data.get("conclusion") else None
        return cls(
            active_hypotheses=[Hypothesis.from_dict(h) for h in data.get("active_hypotheses", [])],
            conclusion=conc,
            premises=[Proposition.from_dict(p) for p in data.get("premises", [])],
            evidence_index=data.get("evidence_index", {}),
            contradiction_log=[Contradiction.from_dict(c) for c in data.get("contradiction_log", [])],
            budget_remaining=data.get("budget_remaining", 100),
        )


# ---------------------------------------------------------------------------
# Planning types
# ---------------------------------------------------------------------------

@dataclass(slots=True)
class Goal:
    """A goal for planning."""
    id: GoalId = field(default_factory=GoalId.generate)
    description: str = ""
    priority: Scalar = Scalar(0.5)
    status: GoalStatus = GoalStatus.ACTIVE

    def to_dict(self) -> dict[str, Any]:
        return {
            "id": self.id.to_dict(),
            "description": self.description,
            "priority": float(self.priority),
            "status": self.status.value,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Goal:
        return cls(
            id=GoalId.from_dict(data["id"]),
            description=data["description"],
            priority=Scalar(data["priority"]),
            status=GoalStatus(data["status"]),
        )


@dataclass(slots=True)
class Plan:
    """A plan of actions to achieve a goal."""
    id: PlanId = field(default_factory=PlanId.generate)
    goal: str = ""
    steps: list[Action] = field(default_factory=list)
    predicted_outcomes: list[Outcome] = field(default_factory=list)
    estimated_cost: Scalar = Scalar(0.0)
    estimated_risk: Scalar = Scalar(0.0)
    uncertainty: Scalar = Scalar(0.5)
    confidence: Scalar = Scalar(0.0)
    created_at: Timestamp = field(default_factory=Timestamp.now)
    status: PlanStatus = PlanStatus.CANDIDATE

    def to_dict(self) -> dict[str, Any]:
        return {
            "id": self.id.to_dict(),
            "goal": self.goal,
            "steps": [s.to_dict() for s in self.steps],
            "predicted_outcomes": [o.to_dict() for o in self.predicted_outcomes],
            "estimated_cost": float(self.estimated_cost),
            "estimated_risk": float(self.estimated_risk),
            "uncertainty": float(self.uncertainty),
            "confidence": float(self.confidence),
            "created_at": self.created_at.to_dict(),
            "status": self.status.value,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Plan:
        return cls(
            id=PlanId.from_dict(data["id"]),
            goal=data["goal"],
            steps=[Action.from_dict(s) for s in data.get("steps", [])],
            predicted_outcomes=[Outcome.from_dict(o) for o in data.get("predicted_outcomes", [])],
            estimated_cost=Scalar(data["estimated_cost"]),
            estimated_risk=Scalar(data["estimated_risk"]),
            uncertainty=Scalar(data["uncertainty"]),
            confidence=Scalar(data["confidence"]),
            created_at=Timestamp.from_dict(data["created_at"]),
            status=PlanStatus(data["status"]),
        )


@dataclass(slots=True)
class PlanningState:
    """Tracks active planning process."""
    active_goals: list[Goal] = field(default_factory=list)
    candidate_plans: list[Plan] = field(default_factory=list)
    selected_plan: Plan | None = None
    budget_remaining: int = 100
    simulation_count: int = 0

    def to_dict(self) -> dict[str, Any]:
        return {
            "active_goals": [g.to_dict() for g in self.active_goals],
            "candidate_plans": [p.to_dict() for p in self.candidate_plans],
            "selected_plan": self.selected_plan.to_dict() if self.selected_plan else None,
            "budget_remaining": self.budget_remaining,
            "simulation_count": self.simulation_count,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> PlanningState:
        sp = Plan.from_dict(data["selected_plan"]) if data.get("selected_plan") else None
        return cls(
            active_goals=[Goal.from_dict(g) for g in data.get("active_goals", [])],
            candidate_plans=[Plan.from_dict(p) for p in data.get("candidate_plans", [])],
            selected_plan=sp,
            budget_remaining=data.get("budget_remaining", 100),
            simulation_count=data.get("simulation_count", 0),
        )


# ---------------------------------------------------------------------------
# Verification types
# ---------------------------------------------------------------------------

@dataclass(slots=True)
class EvidenceRequirements:
    """Minimum requirements for evidence-based verification."""
    min_independent_sources: int = 1
    min_evidence_strength: Scalar = Scalar(0.5)
    min_source_quality: Scalar = Scalar(0.5)
    require_no_contradictions: bool = True

    def to_dict(self) -> dict[str, Any]:
        return {
            "min_independent_sources": self.min_independent_sources,
            "min_evidence_strength": float(self.min_evidence_strength),
            "min_source_quality": float(self.min_source_quality),
            "require_no_contradictions": self.require_no_contradictions,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> EvidenceRequirements:
        return cls(
            min_independent_sources=data["min_independent_sources"],
            min_evidence_strength=Scalar(data["min_evidence_strength"]),
            min_source_quality=Scalar(data["min_source_quality"]),
            require_no_contradictions=data["require_no_contradictions"],
        )


@dataclass(slots=True)
class KnowledgeClaim:
    """A claim subject to verification."""
    id: ClaimId = field(default_factory=ClaimId.generate)
    proposition: Proposition = field(default_factory=Proposition)
    evidence: EvidenceSet = field(default_factory=EvidenceSet)
    counter_evidence: EvidenceSet = field(default_factory=EvidenceSet)
    status: VerificationStatus = VerificationStatus.UNKNOWN
    confidence: ConfidenceState = field(default_factory=ConfidenceState.default)
    provenance: Provenance = field(default_factory=Provenance.user_provided)
    claimed_at: Timestamp = field(default_factory=Timestamp.now)
    last_verified: Timestamp | None = None
    verification_attempts: int = 0

    def to_dict(self) -> dict[str, Any]:
        return {
            "id": self.id.to_dict(),
            "proposition": self.proposition.to_dict(),
            "evidence": self.evidence.to_dict(),
            "counter_evidence": self.counter_evidence.to_dict(),
            "status": self.status.value,
            "confidence": self.confidence.to_dict(),
            "provenance": self.provenance.to_dict(),
            "claimed_at": self.claimed_at.to_dict(),
            "last_verified": self.last_verified.to_dict() if self.last_verified else None,
            "verification_attempts": self.verification_attempts,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> KnowledgeClaim:
        lv = Timestamp.from_dict(data["last_verified"]) if data.get("last_verified") else None
        return cls(
            id=ClaimId.from_dict(data["id"]),
            proposition=Proposition.from_dict(data["proposition"]),
            evidence=EvidenceSet.from_dict(data["evidence"]),
            counter_evidence=EvidenceSet.from_dict(data["counter_evidence"]),
            status=VerificationStatus(data["status"]),
            confidence=ConfidenceState.from_dict(data["confidence"]),
            provenance=Provenance.from_dict(data["provenance"]),
            claimed_at=Timestamp.from_dict(data["claimed_at"]),
            last_verified=lv,
            verification_attempts=data.get("verification_attempts", 0),
        )


@dataclass(slots=True)
class VerificationState:
    """Tracks verification process for knowledge claims."""
    pending_claims: list[KnowledgeClaim] = field(default_factory=list)
    verified_claims: list[KnowledgeClaim] = field(default_factory=list)
    contradicted_claims: list[KnowledgeClaim] = field(default_factory=list)
    confidence_threshold: Scalar = Scalar(0.7)
    evidence_requirements: EvidenceRequirements = field(default_factory=EvidenceRequirements)

    def to_dict(self) -> dict[str, Any]:
        return {
            "pending_claims": [c.to_dict() for c in self.pending_claims],
            "verified_claims": [c.to_dict() for c in self.verified_claims],
            "contradicted_claims": [c.to_dict() for c in self.contradicted_claims],
            "confidence_threshold": float(self.confidence_threshold),
            "evidence_requirements": self.evidence_requirements.to_dict(),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> VerificationState:
        return cls(
            pending_claims=[KnowledgeClaim.from_dict(c) for c in data.get("pending_claims", [])],
            verified_claims=[KnowledgeClaim.from_dict(c) for c in data.get("verified_claims", [])],
            contradicted_claims=[KnowledgeClaim.from_dict(c) for c in data.get("contradicted_claims", [])],
            confidence_threshold=Scalar(data["confidence_threshold"]),
            evidence_requirements=EvidenceRequirements.from_dict(data["evidence_requirements"]),
        )


# ---------------------------------------------------------------------------
# Learning types
# ---------------------------------------------------------------------------

@dataclass(slots=True)
class LearningEvent:
    """A single learning event record."""
    timestamp: Timestamp = field(default_factory=Timestamp.now)
    signal_magnitude: Scalar = Scalar(0.0)
    attribution: str = "input_error"
    subsystem: str = ""
    applied: bool = False

    def to_dict(self) -> dict[str, Any]:
        return {
            "timestamp": self.timestamp.to_dict(),
            "signal_magnitude": float(self.signal_magnitude),
            "attribution": self.attribution,
            "subsystem": self.subsystem,
            "applied": self.applied,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> LearningEvent:
        return cls(
            timestamp=Timestamp.from_dict(data["timestamp"]),
            signal_magnitude=Scalar(data["signal_magnitude"]),
            attribution=data["attribution"],
            subsystem=data["subsystem"],
            applied=data["applied"],
        )


@dataclass(slots=True)
class LearningState:
    """Tracks learning statistics and parameters."""
    total_learning_events: int = 0
    total_replay_events: int = 0
    total_consolidation_events: int = 0
    average_prediction_error: Scalar = Scalar(0.0)
    learning_rate: Scalar = Scalar(0.001)
    plasticity_rate: Scalar = Scalar(0.01)
    consolidation_threshold: Scalar = Scalar(0.5)
    replay_buffer: list[str] = field(default_factory=list)
    replay_buffer_capacity: int = 100
    next_consolidation_at: int = 100

    def to_dict(self) -> dict[str, Any]:
        return {
            "total_learning_events": self.total_learning_events,
            "total_replay_events": self.total_replay_events,
            "total_consolidation_events": self.total_consolidation_events,
            "average_prediction_error": float(self.average_prediction_error),
            "learning_rate": float(self.learning_rate),
            "plasticity_rate": float(self.plasticity_rate),
            "consolidation_threshold": float(self.consolidation_threshold),
            "replay_buffer": list(self.replay_buffer),
            "replay_buffer_capacity": self.replay_buffer_capacity,
            "next_consolidation_at": self.next_consolidation_at,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> LearningState:
        return cls(
            total_learning_events=data.get("total_learning_events", 0),
            total_replay_events=data.get("total_replay_events", 0),
            total_consolidation_events=data.get("total_consolidation_events", 0),
            average_prediction_error=Scalar(data.get("average_prediction_error", 0.0)),
            learning_rate=Scalar(data.get("learning_rate", 0.001)),
            plasticity_rate=Scalar(data.get("plasticity_rate", 0.01)),
            consolidation_threshold=Scalar(data.get("consolidation_threshold", 0.5)),
            replay_buffer=data.get("replay_buffer", []),
            replay_buffer_capacity=data.get("replay_buffer_capacity", 100),
            next_consolidation_at=data.get("next_consolidation_at", 100),
        )


@dataclass(slots=True)
class PlasticityState:
    """Tracks neural adaptation parameters."""
    learning_rate: Scalar = Scalar(0.001)
    plasticity_bound: Scalar = Scalar(0.01)
    total_updates: int = 0
    average_update_magnitude: Scalar = Scalar(0.0)
    max_update_magnitude: Scalar = Scalar(0.0)
    enabled: bool = True
    last_applied: Timestamp | None = None

    def compute_update(
        self,
        activation_relationship: float,
        context_factor: float,
        prediction_error: float,
        evidence_confidence: float,
    ) -> Scalar:
        """Compute bounded weight update: ΔW = η × A × C × E × V."""
        delta = (
            float(self.learning_rate)
            * activation_relationship
            * context_factor
            * prediction_error
            * evidence_confidence
        )
        clamped = max(-float(self.plasticity_bound), min(float(self.plasticity_bound), delta))
        return Scalar(clamped)

    def to_dict(self) -> dict[str, Any]:
        return {
            "learning_rate": float(self.learning_rate),
            "plasticity_bound": float(self.plasticity_bound),
            "total_updates": self.total_updates,
            "average_update_magnitude": float(self.average_update_magnitude),
            "max_update_magnitude": float(self.max_update_magnitude),
            "enabled": self.enabled,
            "last_applied": self.last_applied.to_dict() if self.last_applied else None,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> PlasticityState:
        la = Timestamp.from_dict(data["last_applied"]) if data.get("last_applied") else None
        return cls(
            learning_rate=Scalar(data["learning_rate"]),
            plasticity_bound=Scalar(data["plasticity_bound"]),
            total_updates=data.get("total_updates", 0),
            average_update_magnitude=Scalar(data["average_update_magnitude"]),
            max_update_magnitude=Scalar(data["max_update_magnitude"]),
            enabled=data.get("enabled", True),
            last_applied=la,
        )


# ---------------------------------------------------------------------------
# Consolidation types
# ---------------------------------------------------------------------------

@dataclass(slots=True)
class ConsolidationCandidate:
    """A candidate item for memory consolidation."""
    target: ConsolidationTarget = ConsolidationTarget.SEMANTIC
    knowledge: Knowledge | None = None
    procedure: Procedure | None = None
    supporting_episodes: list[str] = field(default_factory=list)
    episode_count: int = 0
    pattern_strength: Scalar = Scalar(0.0)
    evidence_strength: Scalar = Scalar(0.0)
    contradiction_risk: Scalar = Scalar(0.0)
    identified_at: Timestamp = field(default_factory=Timestamp.now)

    def to_dict(self) -> dict[str, Any]:
        return {
            "target": self.target.value,
            "knowledge": self.knowledge.to_dict() if self.knowledge else None,
            "procedure": self.procedure.to_dict() if self.procedure else None,
            "supporting_episodes": list(self.supporting_episodes),
            "episode_count": self.episode_count,
            "pattern_strength": float(self.pattern_strength),
            "evidence_strength": float(self.evidence_strength),
            "contradiction_risk": float(self.contradiction_risk),
            "identified_at": self.identified_at.to_dict(),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ConsolidationCandidate:
        k = Knowledge.from_dict(data["knowledge"]) if data.get("knowledge") else None
        p = Procedure.from_dict(data["procedure"]) if data.get("procedure") else None
        return cls(
            target=ConsolidationTarget(data["target"]),
            knowledge=k,
            procedure=p,
            supporting_episodes=data.get("supporting_episodes", []),
            episode_count=data.get("episode_count", 0),
            pattern_strength=Scalar(data["pattern_strength"]),
            evidence_strength=Scalar(data["evidence_strength"]),
            contradiction_risk=Scalar(data["contradiction_risk"]),
            identified_at=Timestamp.from_dict(data["identified_at"]),
        )


@dataclass(slots=True)
class ConsolidationStats:
    """Consolidation statistics."""
    semantic_integrations: int = 0
    procedural_integrations: int = 0
    associative_integrations: int = 0
    merges: int = 0
    compressions: int = 0
    generalizations: int = 0
    rejections: int = 0

    def to_dict(self) -> dict[str, Any]:
        return {k: getattr(self, k) for k in [
            "semantic_integrations", "procedural_integrations",
            "associative_integrations", "merges", "compressions",
            "generalizations", "rejections",
        ]}

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ConsolidationStats:
        return cls(**{k: data.get(k, 0) for k in [
            "semantic_integrations", "procedural_integrations",
            "associative_integrations", "merges", "compressions",
            "generalizations", "rejections",
        ]})


@dataclass(slots=True)
class ConsolidationState:
    """Tracks long-term memory formation."""
    pending_candidates: list[ConsolidationCandidate] = field(default_factory=list)
    total_consolidations: int = 0
    last_consolidation: Timestamp | None = None
    next_trigger: int = 100
    stats: ConsolidationStats = field(default_factory=ConsolidationStats)

    def to_dict(self) -> dict[str, Any]:
        return {
            "pending_candidates": [c.to_dict() for c in self.pending_candidates],
            "total_consolidations": self.total_consolidations,
            "last_consolidation": self.last_consolidation.to_dict() if self.last_consolidation else None,
            "next_trigger": self.next_trigger,
            "stats": self.stats.to_dict(),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ConsolidationState:
        lc = Timestamp.from_dict(data["last_consolidation"]) if data.get("last_consolidation") else None
        return cls(
            pending_candidates=[ConsolidationCandidate.from_dict(c) for c in data.get("pending_candidates", [])],
            total_consolidations=data.get("total_consolidations", 0),
            last_consolidation=lc,
            next_trigger=data.get("next_trigger", 100),
            stats=ConsolidationStats.from_dict(data.get("stats", {})),
        )


@dataclass(slots=True)
class ForgettingPolicy:
    """Parameters controlling memory forgetting."""
    min_importance: Scalar = Scalar(0.1)
    min_confidence: Scalar = Scalar(0.1)
    max_age: Duration | None = None
    min_retrieval_count: int = 0
    allow_contradicted: bool = False
    aggressive: bool = False

    def to_dict(self) -> dict[str, Any]:
        return {
            "min_importance": float(self.min_importance),
            "min_confidence": float(self.min_confidence),
            "max_age": self.max_age.to_dict() if self.max_age else None,
            "min_retrieval_count": self.min_retrieval_count,
            "allow_contradicted": self.allow_contradicted,
            "aggressive": self.aggressive,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ForgettingPolicy:
        ma = Duration.from_dict(data["max_age"]) if data.get("max_age") else None
        return cls(
            min_importance=Scalar(data["min_importance"]),
            min_confidence=Scalar(data["min_confidence"]),
            max_age=ma,
            min_retrieval_count=data.get("min_retrieval_count", 0),
            allow_contradicted=data.get("allow_contradicted", False),
            aggressive=data.get("aggressive", False),
        )


# ---------------------------------------------------------------------------
# Self Model types
# ---------------------------------------------------------------------------

@dataclass(slots=True)
class CapabilitySet:
    """Estimated system capabilities."""
    language_accuracy: Scalar = Scalar(0.5)
    prediction_accuracy: Scalar = Scalar(0.5)
    verification_reliability: Scalar = Scalar(0.5)
    planning_success: Scalar = Scalar(0.5)
    memory_retrieval_success: Scalar = Scalar(0.5)
    reasoning_consistency: Scalar = Scalar(0.5)
    resource_availability: Scalar = Scalar(1.0)

    def to_dict(self) -> dict[str, Any]:
        return {
            "language_accuracy": float(self.language_accuracy),
            "prediction_accuracy": float(self.prediction_accuracy),
            "verification_reliability": float(self.verification_reliability),
            "planning_success": float(self.planning_success),
            "memory_retrieval_success": float(self.memory_retrieval_success),
            "reasoning_consistency": float(self.reasoning_consistency),
            "resource_availability": float(self.resource_availability),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> CapabilitySet:
        return cls(**{k: Scalar(data[k]) for k in [
            "language_accuracy", "prediction_accuracy", "verification_reliability",
            "planning_success", "memory_retrieval_success", "reasoning_consistency",
            "resource_availability",
        ]})


@dataclass(slots=True)
class LimitationSet:
    """Known system limitations."""
    known_limitations: list[str] = field(default_factory=list)
    resource_constraints: list[str] = field(default_factory=list)
    capability_gaps: list[str] = field(default_factory=list)

    def to_dict(self) -> dict[str, Any]:
        return {
            "known_limitations": list(self.known_limitations),
            "resource_constraints": list(self.resource_constraints),
            "capability_gaps": list(self.capability_gaps),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> LimitationSet:
        return cls(
            known_limitations=data.get("known_limitations", []),
            resource_constraints=data.get("resource_constraints", []),
            capability_gaps=data.get("capability_gaps", []),
        )


@dataclass(slots=True)
class MemoryHealth:
    """Memory subsystem health assessment."""
    pressure: MemoryPressure = MemoryPressure.LOW
    fragmentation: Scalar = Scalar(0.0)
    consolidation_backlog: int = 0
    eviction_rate: Scalar = Scalar(0.0)

    def to_dict(self) -> dict[str, Any]:
        return {
            "pressure": self.pressure.value,
            "fragmentation": float(self.fragmentation),
            "consolidation_backlog": self.consolidation_backlog,
            "eviction_rate": float(self.eviction_rate),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> MemoryHealth:
        return cls(
            pressure=MemoryPressure(data["pressure"]),
            fragmentation=Scalar(data["fragmentation"]),
            consolidation_backlog=data["consolidation_backlog"],
            eviction_rate=Scalar(data["eviction_rate"]),
        )


@dataclass(slots=True)
class LanguageCapability:
    """Language subsystem capability assessment."""
    vocabulary_size: int = 0
    accuracy: Scalar = Scalar(0.5)
    confidence: Scalar = Scalar(0.5)
    unknown_word_rate: Scalar = Scalar(0.0)

    def to_dict(self) -> dict[str, Any]:
        return {
            "vocabulary_size": self.vocabulary_size,
            "accuracy": float(self.accuracy),
            "confidence": float(self.confidence),
            "unknown_word_rate": float(self.unknown_word_rate),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> LanguageCapability:
        return cls(
            vocabulary_size=data["vocabulary_size"],
            accuracy=Scalar(data["accuracy"]),
            confidence=Scalar(data["confidence"]),
            unknown_word_rate=Scalar(data["unknown_word_rate"]),
        )


@dataclass(slots=True)
class ReasoningPerformance:
    """Reasoning subsystem performance assessment."""
    consistency: Scalar = Scalar(0.5)
    confidence: Scalar = Scalar(0.5)
    average_steps: Scalar = Scalar(0.0)
    contradiction_rate: Scalar = Scalar(0.0)

    def to_dict(self) -> dict[str, Any]:
        return {
            "consistency": float(self.consistency),
            "confidence": float(self.confidence),
            "average_steps": float(self.average_steps),
            "contradiction_rate": float(self.contradiction_rate),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ReasoningPerformance:
        return cls(**{k: Scalar(data[k]) for k in [
            "consistency", "confidence", "average_steps", "contradiction_rate",
        ]})


@dataclass(slots=True)
class ResourceState:
    """System resource state."""
    memory_available_bytes: int = 0
    memory_total_bytes: int = 0
    compute_available: bool = True
    network_available: bool = True

    def to_dict(self) -> dict[str, Any]:
        return {
            "memory_available_bytes": self.memory_available_bytes,
            "memory_total_bytes": self.memory_total_bytes,
            "compute_available": self.compute_available,
            "network_available": self.network_available,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ResourceState:
        return cls(**{k: data[k] for k in [
            "memory_available_bytes", "memory_total_bytes",
            "compute_available", "network_available",
        ]})


@dataclass(slots=True)
class LearningStatistics:
    """Learning system statistics."""
    total_events: int = 0
    average_error: Scalar = Scalar(0.0)
    learning_rate_effective: Scalar = Scalar(0.001)
    consolidation_rate: Scalar = Scalar(0.0)
    forgetting_rate: Scalar = Scalar(0.0)

    def to_dict(self) -> dict[str, Any]:
        return {
            "total_events": self.total_events,
            "average_error": float(self.average_error),
            "learning_rate_effective": float(self.learning_rate_effective),
            "consolidation_rate": float(self.consolidation_rate),
            "forgetting_rate": float(self.forgetting_rate),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> LearningStatistics:
        return cls(
            total_events=data.get("total_events", 0),
            average_error=Scalar(data["average_error"]),
            learning_rate_effective=Scalar(data["learning_rate_effective"]),
            consolidation_rate=Scalar(data["consolidation_rate"]),
            forgetting_rate=Scalar(data["forgetting_rate"]),
        )


@dataclass(slots=True)
class PerformanceSnapshot:
    """Point-in-time performance measurement."""
    timestamp: Timestamp = field(default_factory=Timestamp.now)
    prediction_accuracy: Scalar = Scalar(0.0)
    memory_pressure: MemoryPressure = MemoryPressure.LOW
    learning_events: int = 0
    reasoning_steps: int = 0

    def to_dict(self) -> dict[str, Any]:
        return {
            "timestamp": self.timestamp.to_dict(),
            "prediction_accuracy": float(self.prediction_accuracy),
            "memory_pressure": self.memory_pressure.value,
            "learning_events": self.learning_events,
            "reasoning_steps": self.reasoning_steps,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> PerformanceSnapshot:
        return cls(
            timestamp=Timestamp.from_dict(data["timestamp"]),
            prediction_accuracy=Scalar(data["prediction_accuracy"]),
            memory_pressure=MemoryPressure(data["memory_pressure"]),
            learning_events=data.get("learning_events", 0),
            reasoning_steps=data.get("reasoning_steps", 0),
        )


@dataclass(slots=True)
class SelfModel:
    """CORTEX's computational self-assessment. NOT a conscious state."""
    capabilities: CapabilitySet = field(default_factory=CapabilitySet)
    limitations: LimitationSet = field(default_factory=LimitationSet)
    prediction_accuracy: Scalar = Scalar(0.5)
    uncertainty: UncertaintyState = field(default_factory=UncertaintyState.initial)
    memory_health: MemoryHealth = field(default_factory=MemoryHealth)
    language_capability: LanguageCapability = field(default_factory=LanguageCapability)
    reasoning_performance: ReasoningPerformance = field(default_factory=ReasoningPerformance)
    resource_state: ResourceState = field(default_factory=ResourceState)
    learning_statistics: LearningStatistics = field(default_factory=LearningStatistics)
    historical_performance: BoundedVec = field(default_factory=lambda: BoundedVec(100))
    last_updated: Timestamp = field(default_factory=Timestamp.now)

    def to_dict(self) -> dict[str, Any]:
        return {
            "capabilities": self.capabilities.to_dict(),
            "limitations": self.limitations.to_dict(),
            "prediction_accuracy": float(self.prediction_accuracy),
            "uncertainty": self.uncertainty.to_dict(),
            "memory_health": self.memory_health.to_dict(),
            "language_capability": self.language_capability.to_dict(),
            "reasoning_performance": self.reasoning_performance.to_dict(),
            "resource_state": self.resource_state.to_dict(),
            "learning_statistics": self.learning_statistics.to_dict(),
            "historical_performance": self.historical_performance.to_dict(),
            "last_updated": self.last_updated.to_dict(),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> SelfModel:
        hp = BoundedVec(100)
        for item in data.get("historical_performance", {}).get("items", []):
            hp.push(PerformanceSnapshot.from_dict(item))
        return cls(
            capabilities=CapabilitySet.from_dict(data["capabilities"]),
            limitations=LimitationSet.from_dict(data["limitations"]),
            prediction_accuracy=Scalar(data["prediction_accuracy"]),
            uncertainty=UncertaintyState.from_dict(data["uncertainty"]),
            memory_health=MemoryHealth.from_dict(data["memory_health"]),
            language_capability=LanguageCapability.from_dict(data["language_capability"]),
            reasoning_performance=ReasoningPerformance.from_dict(data["reasoning_performance"]),
            resource_state=ResourceState.from_dict(data["resource_state"]),
            learning_statistics=LearningStatistics.from_dict(data["learning_statistics"]),
            historical_performance=hp,
            last_updated=Timestamp.from_dict(data["last_updated"]),
        )


# ---------------------------------------------------------------------------
# Policy types
# ---------------------------------------------------------------------------

@dataclass(slots=True)
class RiskThresholds:
    """Risk thresholds for policy evaluation."""
    auto_approve_below: Scalar = Scalar(0.3)
    limit_above: Scalar = Scalar(0.6)
    deny_above: Scalar = Scalar(0.8)

    def to_dict(self) -> dict[str, Any]:
        return {
            "auto_approve_below": float(self.auto_approve_below),
            "limit_above": float(self.limit_above),
            "deny_above": float(self.deny_above),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> RiskThresholds:
        return cls(**{k: Scalar(data[k]) for k in [
            "auto_approve_below", "limit_above", "deny_above",
        ]})


@dataclass(slots=True)
class OperationConstraints:
    """Constraints applied to a limited operation."""
    max_magnitude: Scalar = Scalar(1.0)
    max_scope: int = 1
    requires_confirmation: bool = False
    timeout: Duration = field(default_factory=lambda: Duration.from_secs(60))

    def to_dict(self) -> dict[str, Any]:
        return {
            "max_magnitude": float(self.max_magnitude),
            "max_scope": self.max_scope,
            "requires_confirmation": self.requires_confirmation,
            "timeout": self.timeout.to_dict(),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> OperationConstraints:
        return cls(
            max_magnitude=Scalar(data["max_magnitude"]),
            max_scope=data["max_scope"],
            requires_confirmation=data["requires_confirmation"],
            timeout=Duration.from_dict(data["timeout"]),
        )


@dataclass(slots=True)
class PolicyState:
    """Non-learned boundary state. ONLY modifiable via admin operation."""
    learning_enabled: bool = True
    internet_learning_enabled: bool = True
    self_modification_allowed: bool = False
    policy_modification_allowed: bool = False
    runtime_modification_allowed: bool = False
    risk_thresholds: RiskThresholds = field(default_factory=RiskThresholds)
    operation_constraints: dict[str, OperationConstraints] = field(default_factory=dict)

    def to_dict(self) -> dict[str, Any]:
        return {
            "learning_enabled": self.learning_enabled,
            "internet_learning_enabled": self.internet_learning_enabled,
            "self_modification_allowed": self.self_modification_allowed,
            "policy_modification_allowed": self.policy_modification_allowed,
            "runtime_modification_allowed": self.runtime_modification_allowed,
            "risk_thresholds": self.risk_thresholds.to_dict(),
            "operation_constraints": {k: v.to_dict() for k, v in self.operation_constraints.items()},
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> PolicyState:
        return cls(
            learning_enabled=data.get("learning_enabled", True),
            internet_learning_enabled=data.get("internet_learning_enabled", True),
            self_modification_allowed=data.get("self_modification_allowed", False),
            policy_modification_allowed=data.get("policy_modification_allowed", False),
            runtime_modification_allowed=data.get("runtime_modification_allowed", False),
            risk_thresholds=RiskThresholds.from_dict(data["risk_thresholds"]),
            operation_constraints={
                k: OperationConstraints.from_dict(v)
                for k, v in data.get("operation_constraints", {}).items()
            },
        )


# ---------------------------------------------------------------------------
# Internet types
# ---------------------------------------------------------------------------

@dataclass(slots=True)
class NetworkObservation:
    """Result of a network fetch."""
    content: str = ""
    status: int = 0
    timestamp: Timestamp = field(default_factory=Timestamp.now)
    source_url: str = ""
    size_bytes: int = 0

    def to_dict(self) -> dict[str, Any]:
        return {
            "content": self.content,
            "status": self.status,
            "timestamp": self.timestamp.to_dict(),
            "source_url": self.source_url,
            "size_bytes": self.size_bytes,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> NetworkObservation:
        return cls(
            content=data["content"],
            status=data["status"],
            timestamp=Timestamp.from_dict(data["timestamp"]),
            source_url=data["source_url"],
            size_bytes=data.get("size_bytes", 0),
        )


@dataclass(slots=True)
class InternetState:
    """Internet interface state."""
    enabled: bool = False
    total_requests: int = 0
    successful_requests: int = 0
    failed_requests: int = 0
    total_bytes_received: int = 0
    last_request: Timestamp | None = None
    last_result: NetworkObservation | None = None
    pending_observations: list[Observation] = field(default_factory=list)

    def to_dict(self) -> dict[str, Any]:
        return {
            "enabled": self.enabled,
            "total_requests": self.total_requests,
            "successful_requests": self.successful_requests,
            "failed_requests": self.failed_requests,
            "total_bytes_received": self.total_bytes_received,
            "last_request": self.last_request.to_dict() if self.last_request else None,
            "last_result": self.last_result.to_dict() if self.last_result else None,
            "pending_observations": [o.to_dict() for o in self.pending_observations],
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> InternetState:
        lr = Timestamp.from_dict(data["last_request"]) if data.get("last_request") else None
        lres = NetworkObservation.from_dict(data["last_result"]) if data.get("last_result") else None
        return cls(
            enabled=data.get("enabled", False),
            total_requests=data.get("total_requests", 0),
            successful_requests=data.get("successful_requests", 0),
            failed_requests=data.get("failed_requests", 0),
            total_bytes_received=data.get("total_bytes_received", 0),
            last_request=lr,
            last_result=lres,
            pending_observations=[Observation.from_dict(o) for o in data.get("pending_observations", [])],
        )


# ---------------------------------------------------------------------------
# Persistence types
# ---------------------------------------------------------------------------

@dataclass(slots=True)
class SaveResult:
    """Result of a state save operation."""
    bytes_written: int = 0
    checksum: int = 0
    duration_ms: int = 0
    timestamp: Timestamp = field(default_factory=Timestamp.now)

    def to_dict(self) -> dict[str, Any]:
        return {
            "bytes_written": self.bytes_written,
            "checksum": self.checksum,
            "duration_ms": self.duration_ms,
            "timestamp": self.timestamp.to_dict(),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> SaveResult:
        return cls(
            bytes_written=data.get("bytes_written", 0),
            checksum=data.get("checksum", 0),
            duration_ms=data.get("duration_ms", 0),
            timestamp=Timestamp.from_dict(data["timestamp"]),
        )


@dataclass(slots=True)
class CheckpointMetadata:
    """Metadata for a state checkpoint."""
    id: CheckpointId = field(default_factory=CheckpointId.generate)
    state_version: int = 1
    algorithm_version: int = 1
    timestamp: Timestamp = field(default_factory=Timestamp.now)
    episode_count: int = 0
    file_path: str = ""
    file_size_bytes: int = 0
    integrity_checksum: int = 0

    def to_dict(self) -> dict[str, Any]:
        return {
            "id": self.id.to_dict(),
            "state_version": self.state_version,
            "algorithm_version": self.algorithm_version,
            "timestamp": self.timestamp.to_dict(),
            "episode_count": self.episode_count,
            "file_path": self.file_path,
            "file_size_bytes": self.file_size_bytes,
            "integrity_checksum": self.integrity_checksum,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> CheckpointMetadata:
        return cls(
            id=CheckpointId.from_dict(data["id"]),
            state_version=data.get("state_version", 1),
            algorithm_version=data.get("algorithm_version", 1),
            timestamp=Timestamp.from_dict(data["timestamp"]),
            episode_count=data.get("episode_count", 0),
            file_path=data.get("file_path", ""),
            file_size_bytes=data.get("file_size_bytes", 0),
            integrity_checksum=data.get("integrity_checksum", 0),
        )


@dataclass(slots=True)
class PersistenceState:
    """Persistence engine metadata."""
    state_path: str = ""
    checkpoint_dir: str = ""
    last_save: Timestamp | None = None
    last_save_result: SaveResult | None = None
    checkpoints: list[CheckpointMetadata] = field(default_factory=list)
    max_checkpoints: int = 10

    def to_dict(self) -> dict[str, Any]:
        return {
            "state_path": self.state_path,
            "checkpoint_dir": self.checkpoint_dir,
            "last_save": self.last_save.to_dict() if self.last_save else None,
            "last_save_result": self.last_save_result.to_dict() if self.last_save_result else None,
            "checkpoints": [c.to_dict() for c in self.checkpoints],
            "max_checkpoints": self.max_checkpoints,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> PersistenceState:
        ls = Timestamp.from_dict(data["last_save"]) if data.get("last_save") else None
        lsr = SaveResult.from_dict(data["last_save_result"]) if data.get("last_save_result") else None
        return cls(
            state_path=data.get("state_path", ""),
            checkpoint_dir=data.get("checkpoint_dir", ""),
            last_save=ls,
            last_save_result=lsr,
            checkpoints=[CheckpointMetadata.from_dict(c) for c in data.get("checkpoints", [])],
            max_checkpoints=data.get("max_checkpoints", 10),
        )


# ---------------------------------------------------------------------------
# Runtime status
# ---------------------------------------------------------------------------

@dataclass(slots=True)
class RuntimeStatus:
    """Runtime status report."""
    state: str = "boot"
    uptime_seconds: int = 0
    memory_usage: MemoryUsage = field(default_factory=MemoryUsage)
    episode_count: int = 0
    prediction_error: Scalar = Scalar(0.0)
    learning_enabled: bool = True
    world_model_size: int = 0
    language_vocabulary_size: int = 0
    checkpoint_count: int = 0
    last_checkpoint: Timestamp | None = None

    def to_dict(self) -> dict[str, Any]:
        return {
            "state": self.state,
            "uptime_seconds": self.uptime_seconds,
            "memory_usage": self.memory_usage.to_dict(),
            "episode_count": self.episode_count,
            "prediction_error": float(self.prediction_error),
            "learning_enabled": self.learning_enabled,
            "world_model_size": self.world_model_size,
            "language_vocabulary_size": self.language_vocabulary_size,
            "checkpoint_count": self.checkpoint_count,
            "last_checkpoint": self.last_checkpoint.to_dict() if self.last_checkpoint else None,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> RuntimeStatus:
        lc = Timestamp.from_dict(data["last_checkpoint"]) if data.get("last_checkpoint") else None
        return cls(
            state=data.get("state", "boot"),
            uptime_seconds=data.get("uptime_seconds", 0),
            memory_usage=MemoryUsage.from_dict(data["memory_usage"]),
            episode_count=data.get("episode_count", 0),
            prediction_error=Scalar(data.get("prediction_error", 0.0)),
            learning_enabled=data.get("learning_enabled", True),
            world_model_size=data.get("world_model_size", 0),
            language_vocabulary_size=data.get("language_vocabulary_size", 0),
            checkpoint_count=data.get("checkpoint_count", 0),
            last_checkpoint=lc,
        )


# ---------------------------------------------------------------------------
# Cortex State (top-level)
# ---------------------------------------------------------------------------

@dataclass(slots=True)
class StateMetadata:
    """Metadata about the state instance."""
    state_id: str = ""
    format_version: int = 1
    architecture_version: int = 1
    algorithm_version: int = 1
    created_at: Timestamp = field(default_factory=Timestamp.now)
    last_updated: Timestamp = field(default_factory=Timestamp.now)
    config_hash: bytes = field(default=b"\x00" * 32)

    def to_dict(self) -> dict[str, Any]:
        return {
            "state_id": self.state_id,
            "format_version": self.format_version,
            "architecture_version": self.architecture_version,
            "algorithm_version": self.algorithm_version,
            "created_at": self.created_at.to_dict(),
            "last_updated": self.last_updated.to_dict(),
            "config_hash": list(self.config_hash),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> StateMetadata:
        return cls(
            state_id=data.get("state_id", ""),
            format_version=data.get("format_version", 1),
            architecture_version=data.get("architecture_version", 1),
            algorithm_version=data.get("algorithm_version", 1),
            created_at=Timestamp.from_dict(data["created_at"]),
            last_updated=Timestamp.from_dict(data["last_updated"]),
            config_hash=bytes(data.get("config_hash", [0] * 32)),
        )


@dataclass(slots=True)
class CortexState:
    """Top-level state container for the entire CORTEX system.

    Owned by ``CortexRuntime``. All sub-states are accessed through this
    root container.
    """

    metadata: StateMetadata = field(default_factory=StateMetadata)
    context: Any = None  # ContextState – imported from common
    memory: MemoryState = field(default_factory=MemoryState)
    world: WorldState = field(default_factory=WorldState)
    reasoning: ReasoningState = field(default_factory=ReasoningState)
    planning: PlanningState = field(default_factory=PlanningState)
    verification: VerificationState = field(default_factory=VerificationState)
    learning: LearningState = field(default_factory=LearningState)
    plasticity: PlasticityState = field(default_factory=PlasticityState)
    consolidation: ConsolidationState = field(default_factory=ConsolidationState)
    self_model: SelfModel = field(default_factory=SelfModel)
    policy: PolicyState = field(default_factory=PolicyState)
    internet: InternetState = field(default_factory=InternetState)
    persistence: PersistenceState = field(default_factory=PersistenceState)
    runtime_status: RuntimeStatus = field(default_factory=RuntimeStatus)

    def to_dict(self) -> dict[str, Any]:
        from .common import ContextState
        ctx = None
        if self.context is not None:
            ctx = self.context.to_dict() if hasattr(self.context, "to_dict") else self.context
        elif hasattr(self, "_context_dict"):
            ctx = self._context_dict
        return {
            "metadata": self.metadata.to_dict(),
            "context": ctx,
            "memory": self.memory.to_dict(),
            "world": self.world.to_dict(),
            "reasoning": self.reasoning.to_dict(),
            "planning": self.planning.to_dict(),
            "verification": self.verification.to_dict(),
            "learning": self.learning.to_dict(),
            "plasticity": self.plasticity.to_dict(),
            "consolidation": self.consolidation.to_dict(),
            "self_model": self.self_model.to_dict(),
            "policy": self.policy.to_dict(),
            "internet": self.internet.to_dict(),
            "persistence": self.persistence.to_dict(),
            "runtime_status": self.runtime_status.to_dict(),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> CortexState:
        from .common import ContextState
        ctx = None
        if data.get("context"):
            ctx = ContextState.from_dict(data["context"])
        return cls(
            metadata=StateMetadata.from_dict(data["metadata"]),
            context=ctx,
            memory=MemoryState.from_dict(data["memory"]),
            world=WorldState.from_dict(data["world"]),
            reasoning=ReasoningState.from_dict(data["reasoning"]),
            planning=PlanningState.from_dict(data["planning"]),
            verification=VerificationState.from_dict(data["verification"]),
            learning=LearningState.from_dict(data["learning"]),
            plasticity=PlasticityState.from_dict(data["plasticity"]),
            consolidation=ConsolidationState.from_dict(data["consolidation"]),
            self_model=SelfModel.from_dict(data["self_model"]),
            policy=PolicyState.from_dict(data["policy"]),
            internet=InternetState.from_dict(data["internet"]),
            persistence=PersistenceState.from_dict(data["persistence"]),
            runtime_status=RuntimeStatus.from_dict(data["runtime_status"]),
        )

    def validate_scalars(self) -> list[str]:
        """Validate that all Scalar fields contain finite values.

        Returns a list of error descriptions (empty if all valid).
        """
        errors: list[str] = []

        def _check(obj: Any, path: str) -> None:
            if isinstance(obj, Scalar) and not obj.is_valid_cognitive_value():
                errors.append(f"{path}: non-finite Scalar value {obj!r}")
            if isinstance(obj, float) and not __import__("math").isfinite(obj):
                errors.append(f"{path}: non-finite float value {obj!r}")

        # Walk known scalar-bearing structures
        for name in ("learning", "plasticity", "self_model", "verification", "reasoning", "planning"):
            sub = getattr(self, name, None)
            if sub is None:
                continue
            if hasattr(sub, "to_dict"):
                d = sub.to_dict()
                _walk_dict(d, f"state.{name}", _check, errors)

        return errors


def _walk_dict(d: Any, prefix: str, check_fn: Any, errors: list[str]) -> None:
    """Recursively walk a dict looking for scalar issues."""
    if isinstance(d, dict):
        for k, v in d.items():
            _walk_dict(v, f"{prefix}.{k}", check_fn, errors)
    elif isinstance(d, list):
        for i, v in enumerate(d):
            _walk_dict(v, f"{prefix}[{i}]", check_fn, errors)
    elif isinstance(d, float):
        check_fn(d, prefix)
