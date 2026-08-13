"""Evidence and Provenance types for knowledge tracking.

Every knowledge item in CORTEX carries provenance (DDP-011: provenance is
NEVER optional).  Evidence items support or contradict claims with explicit
source attribution.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any

from .common import (
    EvidencePolarity,
    ProvenanceCategory,
    SourceKind,
    Timestamp,
    VerificationStatus,
)
from .ids import EvidenceId, EpisodeId, KnowledgeId, SourceId
from .scalars import Scalar


# ---------------------------------------------------------------------------
# Confidence State
# ---------------------------------------------------------------------------

@dataclass(slots=True)
class ConfidenceState:
    """Multi-dimensional confidence representation.

    Combines belief strength, evidence quality, source reliability,
    internal consistency, uncertainty, and prediction reliability into
    a single composite score via :meth:`overall`.
    """

    belief: float = 0.5
    evidence_strength: float = 0.0
    source_quality: float = 0.5
    consistency: float = 0.5
    uncertainty: float = 0.5
    prediction_reliability: float = 0.0
    verification_status: VerificationStatus = VerificationStatus.UNKNOWN

    @classmethod
    def default(cls) -> ConfidenceState:
        """Return a neutral default confidence state."""
        return cls()

    @classmethod
    def low(cls) -> ConfidenceState:
        """Return a low-confidence state (high uncertainty, weak belief)."""
        return cls(
            belief=0.1,
            evidence_strength=0.0,
            source_quality=0.1,
            consistency=0.1,
            uncertainty=0.9,
            prediction_reliability=0.0,
            verification_status=VerificationStatus.UNKNOWN,
        )

    @classmethod
    def high(cls) -> ConfidenceState:
        """Return a high-confidence state."""
        return cls(
            belief=0.9,
            evidence_strength=0.8,
            source_quality=0.9,
            consistency=0.9,
            uncertainty=0.1,
            prediction_reliability=0.8,
            verification_status=VerificationStatus.SUPPORTED,
        )

    def overall(self) -> Scalar:
        """Compute weighted overall confidence score.

        Weights: belief 0.3, evidence 0.25, source 0.15,
        consistency 0.2, 1-uncertainty 0.1.
        """
        return Scalar(
            (self.belief * 0.3)
            + (self.evidence_strength * 0.25)
            + (self.source_quality * 0.15)
            + (self.consistency * 0.2)
            + ((1.0 - self.uncertainty) * 0.1)
        )

    def is_verified(self) -> bool:
        """Return ``True`` if verification status is ``VERIFIED``."""
        return self.verification_status == VerificationStatus.VERIFIED

    def to_dict(self) -> dict[str, Any]:
        return {
            "belief": self.belief,
            "evidence_strength": self.evidence_strength,
            "source_quality": self.source_quality,
            "consistency": self.consistency,
            "uncertainty": self.uncertainty,
            "prediction_reliability": self.prediction_reliability,
            "verification_status": self.verification_status.value,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ConfidenceState:
        return cls(
            belief=data["belief"],
            evidence_strength=data["evidence_strength"],
            source_quality=data["source_quality"],
            consistency=data["consistency"],
            uncertainty=data["uncertainty"],
            prediction_reliability=data["prediction_reliability"],
            verification_status=VerificationStatus(data["verification_status"]),
        )


# ---------------------------------------------------------------------------
# Uncertainty State
# ---------------------------------------------------------------------------

@dataclass(slots=True)
class UncertaintyState:
    """Per-dimension uncertainty tracking for world model and predictions."""

    level: float = 1.0
    dimensions: dict[str, float] = field(default_factory=dict)
    reducible: bool = True
    updated_at: Timestamp = field(default_factory=Timestamp.now)

    @classmethod
    def initial(cls) -> UncertaintyState:
        """Create a maximally uncertain state."""
        return cls()

    def to_dict(self) -> dict[str, Any]:
        return {
            "level": self.level,
            "dimensions": dict(self.dimensions),
            "reducible": self.reducible,
            "updated_at": self.updated_at.to_dict(),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> UncertaintyState:
        return cls(
            level=data["level"],
            dimensions=data.get("dimensions", {}),
            reducible=data.get("reducible", True),
            updated_at=Timestamp.from_dict(data["updated_at"]),
        )


# ---------------------------------------------------------------------------
# Source & Source Identity
# ---------------------------------------------------------------------------

@dataclass(slots=True)
class Source:
    """Identity of an information source."""

    id: SourceId = field(default_factory=SourceId.null)
    name: str = ""
    kind: SourceKind = SourceKind.INTERNAL

    def to_dict(self) -> dict[str, Any]:
        return {
            "id": self.id.to_dict(),
            "name": self.name,
            "kind": self.kind.value,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Source:
        return cls(
            id=SourceId.from_dict(data["id"]),
            name=data["name"],
            kind=SourceKind(data["kind"]),
        )


@dataclass(slots=True)
class SourceIdentity:
    """Reliability metadata for a source."""

    identifier: str = ""
    reliability: float = 0.5
    verification_count: int = 0

    def to_dict(self) -> dict[str, Any]:
        return {
            "identifier": self.identifier,
            "reliability": self.reliability,
            "verification_count": self.verification_count,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> SourceIdentity:
        return cls(
            identifier=data["identifier"],
            reliability=data["reliability"],
            verification_count=data["verification_count"],
        )


@dataclass(slots=True)
class RetrievalContext:
    """Context under which a knowledge item was retrieved."""

    query: str = ""
    timestamp: Timestamp = field(default_factory=Timestamp.now)
    session_id: str = ""

    def to_dict(self) -> dict[str, Any]:
        return {
            "query": self.query,
            "timestamp": self.timestamp.to_dict(),
            "session_id": self.session_id,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> RetrievalContext:
        return cls(
            query=data["query"],
            timestamp=Timestamp.from_dict(data["timestamp"]),
            session_id=data.get("session_id", ""),
        )


# ---------------------------------------------------------------------------
# Evidence
# ---------------------------------------------------------------------------

@dataclass(slots=True)
class Evidence:
    """A single piece of evidence supporting or contradicting a claim."""

    id: EvidenceId = field(default_factory=EvidenceId.null)
    source: Provenance | None = None
    content: EvidenceContent | None = None
    strength: float = 0.0
    polarity: EvidencePolarity = EvidencePolarity.NEUTRAL
    timestamp: Timestamp = field(default_factory=Timestamp.now)
    related: list[EvidenceId] = field(default_factory=list)

    def to_dict(self) -> dict[str, Any]:
        result: dict[str, Any] = {
            "id": self.id.to_dict(),
            "source": self.source.to_dict() if self.source else None,
            "content": self.content.to_dict() if self.content else None,
            "strength": self.strength,
            "polarity": self.polarity.value,
            "timestamp": self.timestamp.to_dict(),
            "related": [e.to_dict() for e in self.related],
        }
        return result

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Evidence:
        source = None
        if data.get("source"):
            source = Provenance.from_dict(data["source"])
        content = None
        if data.get("content"):
            content = EvidenceContent.from_dict(data["content"])
        return cls(
            id=EvidenceId.from_dict(data["id"]),
            source=source,
            content=content,
            strength=data["strength"],
            polarity=EvidencePolarity(data["polarity"]),
            timestamp=Timestamp.from_dict(data["timestamp"]),
            related=[EvidenceId.from_dict(e) for e in data.get("related", [])],
        )


@dataclass(slots=True)
class EvidenceContent:
    """Discriminated union for evidence payload."""

    kind: str = "text"
    text: str = ""
    knowledge_ref: KnowledgeId | None = None
    episode_ref: EpisodeId | None = None
    numeric_value: float = 0.0
    composite: list[EvidenceContent] = field(default_factory=list)

    @classmethod
    def text_content(cls, text: str) -> EvidenceContent:
        return cls(kind="text", text=text)

    @classmethod
    def numeric(cls, value: float) -> EvidenceContent:
        return cls(kind="numeric", numeric_value=value)

    @classmethod
    def knowledge_ref_content(cls, kid: KnowledgeId) -> EvidenceContent:
        return cls(kind="knowledge_ref", knowledge_ref=kid)

    @classmethod
    def episode_ref_content(cls, eid: EpisodeId) -> EvidenceContent:
        return cls(kind="episode_ref", episode_ref=eid)

    @classmethod
    def composite_content(cls, items: list[EvidenceContent]) -> EvidenceContent:
        return cls(kind="composite", composite=items)

    def to_dict(self) -> dict[str, Any]:
        d: dict[str, Any] = {"kind": self.kind}
        if self.kind == "text":
            d["text"] = self.text
        elif self.kind == "numeric":
            d["numeric_value"] = self.numeric_value
        elif self.kind == "knowledge_ref":
            d["knowledge_ref"] = self.knowledge_ref.to_dict() if self.knowledge_ref else None
        elif self.kind == "episode_ref":
            d["episode_ref"] = self.episode_ref.to_dict() if self.episode_ref else None
        elif self.kind == "composite":
            d["composite"] = [c.to_dict() for c in self.composite]
        return d

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> EvidenceContent:
        kind = data["kind"]
        if kind == "text":
            return cls.text_content(data["text"])
        if kind == "numeric":
            return cls.numeric(data["numeric_value"])
        if kind == "knowledge_ref":
            ref = KnowledgeId.from_dict(data["knowledge_ref"]) if data.get("knowledge_ref") else None
            return cls.knowledge_ref_content(ref) if ref else cls()
        if kind == "episode_ref":
            ref = EpisodeId.from_dict(data["episode_ref"]) if data.get("episode_ref") else None
            return cls.episode_ref_content(ref) if ref else cls()
        if kind == "composite":
            return cls.composite_content([cls.from_dict(c) for c in data.get("composite", [])])
        return cls()


# ---------------------------------------------------------------------------
# Evidence Set
# ---------------------------------------------------------------------------

@dataclass(slots=True)
class EvidenceSet:
    """Ordered collection of evidence items with aggregate queries."""

    items: list[Evidence] = field(default_factory=list)

    def add(self, evidence: Evidence) -> None:
        """Append an evidence item."""
        self.items.append(evidence)

    def total_strength(self) -> Scalar:
        """Average strength across all items (0 if empty)."""
        if not self.items:
            return Scalar(0.0)
        return Scalar(sum(e.strength for e in self.items) / len(self.items))

    def supporting(self) -> list[Evidence]:
        """Return all evidence that supports the claim."""
        return [e for e in self.items if e.polarity == EvidencePolarity.SUPPORTS]

    def contradicting(self) -> list[Evidence]:
        """Return all evidence that contradicts the claim."""
        return [e for e in self.items if e.polarity == EvidencePolarity.CONTRADICTS]

    def is_empty(self) -> bool:
        return len(self.items) == 0

    def __len__(self) -> int:
        return len(self.items)

    def merge(self, other: EvidenceSet) -> EvidenceSet:
        """Return a new set combining items from both sets."""
        return EvidenceSet(items=list(self.items) + list(other.items))

    def to_dict(self) -> dict[str, Any]:
        return {"items": [e.to_dict() for e in self.items]}

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> EvidenceSet:
        return cls(items=[Evidence.from_dict(e) for e in data.get("items", [])])


# ---------------------------------------------------------------------------
# Provenance
# ---------------------------------------------------------------------------

@dataclass(slots=True)
class Provenance:
    """Complete provenance information for any knowledge item.

    Provenance is **NEVER optional** (DDP-011).
    """

    category: ProvenanceCategory = ProvenanceCategory.OBSERVED
    source: Source = field(default_factory=Source)
    source_identity: SourceIdentity = field(default_factory=SourceIdentity)
    timestamp: Timestamp = field(default_factory=Timestamp.now)
    retrieval_context: RetrievalContext | None = None
    content_hash: bytes = field(default=b"\x00" * 32)
    evidence: EvidenceSet = field(default_factory=EvidenceSet)
    verification_status: VerificationStatus = VerificationStatus.OBSERVED
    confidence: ConfidenceState = field(default_factory=ConfidenceState.default)

    @classmethod
    def user_provided(cls) -> Provenance:
        """Create provenance for user-provided content."""
        return cls(
            category=ProvenanceCategory.USER_PROVIDED,
            source=Source(
                id=SourceId.generate(),
                name="user",
                kind=SourceKind.USER,
            ),
            source_identity=SourceIdentity(
                identifier="user", reliability=0.8, verification_count=0
            ),
            verification_status=VerificationStatus.OBSERVED,
        )

    @classmethod
    def internet(cls, url: str) -> Provenance:
        """Create provenance for internet-sourced content."""
        return cls(
            category=ProvenanceCategory.INTERNET,
            source=Source(
                id=SourceId(uuid=__import__("uuid").uuid4()),
                name=url,
                kind=SourceKind.INTERNET,
            ),
            source_identity=SourceIdentity(
                identifier=url, reliability=0.3, verification_count=0
            ),
            verification_status=VerificationStatus.UNKNOWN,
            confidence=ConfidenceState.low(),
        )

    @classmethod
    def derived(cls, parents: list[Provenance]) -> Provenance:
        """Create provenance for derived/inferred content."""
        merged_evidence = EvidenceSet()
        for parent in parents:
            merged_evidence = merged_evidence.merge(parent.evidence)
        return cls(
            category=ProvenanceCategory.DERIVED,
            source=Source(
                id=SourceId(uuid=__import__("uuid").uuid4()),
                name="derived",
                kind=SourceKind.DERIVED,
            ),
            source_identity=SourceIdentity(
                identifier="derived", reliability=0.5, verification_count=0
            ),
            evidence=merged_evidence,
            verification_status=VerificationStatus.INFERRED,
        )

    def to_dict(self) -> dict[str, Any]:
        return {
            "category": self.category.value,
            "source": self.source.to_dict(),
            "source_identity": self.source_identity.to_dict(),
            "timestamp": self.timestamp.to_dict(),
            "retrieval_context": self.retrieval_context.to_dict() if self.retrieval_context else None,
            "content_hash": list(self.content_hash),
            "evidence": self.evidence.to_dict(),
            "verification_status": self.verification_status.value,
            "confidence": self.confidence.to_dict(),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Provenance:
        retrieval_ctx = None
        if data.get("retrieval_context"):
            retrieval_ctx = RetrievalContext.from_dict(data["retrieval_context"])
        return cls(
            category=ProvenanceCategory(data["category"]),
            source=Source.from_dict(data["source"]),
            source_identity=SourceIdentity.from_dict(data["source_identity"]),
            timestamp=Timestamp.from_dict(data["timestamp"]),
            retrieval_context=retrieval_ctx,
            content_hash=bytes(data.get("content_hash", [0] * 32)),
            evidence=EvidenceSet.from_dict(data.get("evidence", {"items": []})),
            verification_status=VerificationStatus(data["verification_status"]),
            confidence=ConfidenceState.from_dict(data["confidence"]),
        )
