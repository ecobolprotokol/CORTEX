"""ID types: UUID4-based identifiers for all CORTEX entities.

Each entity type has its own distinct ID class (DDP-009).  IDs use
``uuid.uuid4()`` generation, are immutable, hashable, and comparable.
ID 0 / ``NULL`` is the sentinel for "no ID".
"""

from __future__ import annotations

import uuid
from dataclasses import dataclass, field
from typing import Any, ClassVar


def _make_id_class(name: str) -> type:
    """Factory that creates a UUID4-based ID dataclass.

    Each generated class provides:
    - ``NULL`` class constant (all-zero UUID)
    - ``generate()`` class method for new IDs
    - ``is_null()`` predicate
    - ``__str__`` / ``__repr__`` for display
    - Equality, hashing, and ordering
    """

    @dataclass(frozen=True, slots=True, order=True, eq=True)
    class _IdType:
        """Generic UUID-based identifier."""

        value: uuid.UUID = field(compare=True, repr=True)

        NULL: ClassVar[uuid.UUID] = uuid.UUID(int=0)

        @classmethod
        def generate(cls) -> _IdType:
            """Create a new random ID (UUID v4)."""
            return cls(uuid.uuid4())

        @classmethod
        def from_str(cls, s: str) -> _IdType:
            """Parse an ID from a hex string."""
            return cls(uuid.UUID(s))

        @classmethod
        def null(cls) -> _IdType:
            """Return the null sentinel ID."""
            return cls(cls.NULL)

        def is_null(self) -> bool:
            """Return ``True`` if this is the null sentinel."""
            return self.value == self.NULL

        def __str__(self) -> str:
            return f"{name}({self.value})"

        def __repr__(self) -> str:
            return f"{name}(value={self.value!r})"

        def to_dict(self) -> dict[str, str]:
            return {"value": str(self.value)}

        @classmethod
        def from_dict(cls, data: dict[str, str]) -> _IdType:
            return cls(uuid.UUID(data["value"]))

    _IdType.__name__ = name
    _IdType.__qualname__ = name
    return _IdType


# ---------------------------------------------------------------------------
# Neural IDs
# ---------------------------------------------------------------------------

CellId = _make_id_class("CellId")
ColumnId = _make_id_class("ColumnId")
FieldId = _make_id_class("FieldId")

# ---------------------------------------------------------------------------
# Memory IDs
# ---------------------------------------------------------------------------

EpisodeId = _make_id_class("EpisodeId")
KnowledgeId = _make_id_class("KnowledgeId")
ProcedureId = _make_id_class("ProcedureId")
AssociationId = _make_id_class("AssociationId")
MemoryId = _make_id_class("MemoryId")

# ---------------------------------------------------------------------------
# Language IDs
# ---------------------------------------------------------------------------

SymbolId = _make_id_class("SymbolId")
TokenId = _make_id_class("TokenId")
ConceptId = _make_id_class("ConceptId")

# ---------------------------------------------------------------------------
# World IDs
# ---------------------------------------------------------------------------

EntityId = _make_id_class("EntityId")
RelationId = _make_id_class("RelationId")
EventId = _make_id_class("EventId")
TransitionId = _make_id_class("TransitionId")

# ---------------------------------------------------------------------------
# Reasoning IDs
# ---------------------------------------------------------------------------

HypothesisId = _make_id_class("HypothesisId")
EvidenceId = _make_id_class("EvidenceId")

# ---------------------------------------------------------------------------
# Planning IDs
# ---------------------------------------------------------------------------

PlanId = _make_id_class("PlanId")
GoalId = _make_id_class("GoalId")
ActionId = _make_id_class("ActionId")

# ---------------------------------------------------------------------------
# Verification IDs
# ---------------------------------------------------------------------------

ClaimId = _make_id_class("ClaimId")

# ---------------------------------------------------------------------------
# Provenance IDs
# ---------------------------------------------------------------------------

SourceId = _make_id_class("SourceId")
ProvenanceId = _make_id_class("ProvenanceId")

# ---------------------------------------------------------------------------
# Runtime IDs
# ---------------------------------------------------------------------------

CheckpointId = _make_id_class("CheckpointId")
SessionId = _make_id_class("SessionId")

# ---------------------------------------------------------------------------
# Observation ID (requested by spec)
# ---------------------------------------------------------------------------

ObservationId = _make_id_class("ObservationId")


# ---------------------------------------------------------------------------
# InternalId union (for associative memory references)
# ---------------------------------------------------------------------------

@dataclass(frozen=True, slots=True, eq=True, order=True)
class InternalId:
    """Discriminated union of ID types for associative memory.

    Allows an association's source or target to reference any typed entity.
    """

    kind: str = field(compare=True, repr=True)
    _cell: uuid.UUID | None = field(default=None, compare=False, repr=False)
    _column: uuid.UUID | None = field(default=None, compare=False, repr=False)
    _episode: uuid.UUID | None = field(default=None, compare=False, repr=False)
    _concept: uuid.UUID | None = field(default=None, compare=False, repr=False)
    _entity: uuid.UUID | None = field(default=None, compare=False, repr=False)
    _procedure: uuid.UUID | None = field(default=None, compare=False, repr=False)
    _association: uuid.UUID | None = field(default=None, compare=False, repr=False)
    _hypothesis: uuid.UUID | None = field(default=None, compare=False, repr=False)
    _symbol: uuid.UUID | None = field(default=None, compare=False, repr=False)

    @classmethod
    def cell(cls, id: CellId) -> InternalId:
        return cls(kind="cell", _cell=id.value)

    @classmethod
    def column(cls, id: ColumnId) -> InternalId:
        return cls(kind="column", _column=id.value)

    @classmethod
    def episode(cls, id: EpisodeId) -> InternalId:
        return cls(kind="episode", _episode=id.value)

    @classmethod
    def concept(cls, id: ConceptId) -> InternalId:
        return cls(kind="concept", _concept=id.value)

    @classmethod
    def entity(cls, id: EntityId) -> InternalId:
        return cls(kind="entity", _entity=id.value)

    @classmethod
    def procedure(cls, id: ProcedureId) -> InternalId:
        return cls(kind="procedure", _procedure=id.value)

    @classmethod
    def association(cls, id: AssociationId) -> InternalId:
        return cls(kind="association", _association=id.value)

    @classmethod
    def hypothesis(cls, id: HypothesisId) -> InternalId:
        return cls(kind="hypothesis", _hypothesis=id.value)

    @classmethod
    def symbol(cls, id: SymbolId) -> InternalId:
        return cls(kind="symbol", _symbol=id.value)

    def to_dict(self) -> dict[str, Any]:
        return {"kind": self.kind, "value": str(self._value_for_kind())}

    def _value_for_kind(self) -> uuid.UUID:
        mapping: dict[str, uuid.UUID | None] = {
            "cell": self._cell,
            "column": self._column,
            "episode": self._episode,
            "concept": self._concept,
            "entity": self._entity,
            "procedure": self._procedure,
            "association": self._association,
            "hypothesis": self._hypothesis,
            "symbol": self._symbol,
        }
        val = mapping.get(self.kind)
        if val is None:
            raise ValueError(f"InternalId kind={self.kind!r} has no value")
        return val

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> InternalId:
        kind = data["kind"]
        val = uuid.UUID(data["value"])
        kwargs: dict[str, Any] = {"kind": kind}
        kwargs[f"_{kind}"] = val
        return cls(**kwargs)
