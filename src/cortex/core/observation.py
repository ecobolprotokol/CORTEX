"""Observation and Experience types for the cognitive input pipeline.

An :class:`Observation` is a single unit of input entering the cognitive
pipeline.  An :class:`Experience` bundles an observation with the
prediction made before it and the resulting prediction error.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any

from .common import ContextState, ObservationKind, Timestamp
from .evidence import ConfidenceState, EvidenceSet, Provenance
from .ids import ObservationId
from .scalars import Scalar


@dataclass(slots=True)
class Observation:
    """A single observation entering the cognitive pipeline."""

    id: ObservationId = field(default_factory=ObservationId.generate)
    text: str = ""
    source: Provenance = field(default_factory=Provenance.user_provided)
    timestamp: Timestamp = field(default_factory=Timestamp.now)
    context: ContextState = field(default_factory=ContextState.initial)
    kind: ObservationKind = ObservationKind.USER_INPUT
    importance: Scalar = Scalar(0.5)

    @classmethod
    def user_provided(cls, text: str) -> Observation:
        """Create an observation from direct user input."""
        return cls(
            text=text,
            source=Provenance.user_provided(),
            kind=ObservationKind.USER_INPUT,
            importance=Scalar(0.5),
        )

    @classmethod
    def from_internet(cls, text: str, url: str) -> Observation:
        """Create an observation sourced from the internet."""
        return cls(
            text=text,
            source=Provenance.internet(url),
            kind=ObservationKind.INTERNET,
            importance=Scalar(0.3),
        )

    @classmethod
    def from_environment(cls, text: str) -> Observation:
        """Create an observation from the environment."""
        return cls(
            text=text,
            kind=ObservationKind.ENVIRONMENT,
            importance=Scalar(0.4),
        )

    @classmethod
    def feedback(cls, text: str) -> Observation:
        """Create a feedback observation."""
        return cls(
            text=text,
            kind=ObservationKind.FEEDBACK,
            importance=Scalar(0.6),
        )

    @classmethod
    def correction(cls, text: str) -> Observation:
        """Create a correction observation."""
        return cls(
            text=text,
            kind=ObservationKind.CORRECTION,
            importance=Scalar(0.8),
        )

    def to_dict(self) -> dict[str, Any]:
        return {
            "id": self.id.to_dict(),
            "text": self.text,
            "source": self.source.to_dict(),
            "timestamp": self.timestamp.to_dict(),
            "context": self.context.to_dict(),
            "kind": self.kind.value,
            "importance": float(self.importance),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Observation:
        return cls(
            id=ObservationId.from_dict(data["id"]),
            text=data["text"],
            source=Provenance.from_dict(data["source"]),
            timestamp=Timestamp.from_dict(data["timestamp"]),
            context=ContextState.from_dict(data["context"]),
            kind=ObservationKind(data["kind"]),
            importance=Scalar(data["importance"]),
        )


@dataclass(slots=True)
class Prediction:
    """A prediction made by the neural core or world model."""

    target: str = "next_token"
    predicted_state: list[Scalar] = field(default_factory=list)
    confidence: Scalar = Scalar(0.0)
    timestamp: Timestamp = field(default_factory=Timestamp.now)
    context: ContextState = field(default_factory=ContextState.initial)
    resolved: bool = False
    actual: list[Scalar] | None = None
    error_magnitude: Scalar | None = None

    def resolve(self, actual: list[float]) -> None:
        """Resolve the prediction against actual outcomes and compute error."""
        self.resolved = True
        self.actual = [Scalar(v) for v in actual]
        if self.predicted_state and self.actual:
            mse = Scalar(
                sum(
                    (float(p) - float(a)) ** 2
                    for p, a in zip(self.predicted_state, self.actual)
                )
                / max(len(self.predicted_state), 1)
            )
            self.error_magnitude = Scalar(mse ** 0.5)

    def is_zero_error(self) -> bool:
        """Return ``True`` if the prediction error is effectively zero."""
        if self.error_magnitude is None:
            return not self.resolved
        from .scalars import SCALAR_EPSILON
        return float(self.error_magnitude) < SCALAR_EPSILON

    def to_dict(self) -> dict[str, Any]:
        d: dict[str, Any] = {
            "target": self.target,
            "predicted_state": [float(v) for v in self.predicted_state],
            "confidence": float(self.confidence),
            "timestamp": self.timestamp.to_dict(),
            "context": self.context.to_dict(),
            "resolved": self.resolved,
        }
        if self.actual is not None:
            d["actual"] = [float(v) for v in self.actual]
        if self.error_magnitude is not None:
            d["error_magnitude"] = float(self.error_magnitude)
        return d

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Prediction:
        actual = None
        if data.get("actual") is not None:
            actual = [Scalar(v) for v in data["actual"]]
        err = None
        if data.get("error_magnitude") is not None:
            err = Scalar(data["error_magnitude"])
        return cls(
            target=data["target"],
            predicted_state=[Scalar(v) for v in data["predicted_state"]],
            confidence=Scalar(data["confidence"]),
            timestamp=Timestamp.from_dict(data["timestamp"]),
            context=ContextState.from_dict(data["context"]),
            resolved=data["resolved"],
            actual=actual,
            error_magnitude=err,
        )


@dataclass(slots=True)
class PredictionError:
    """Prediction error: difference between predicted and actual."""

    magnitude: Scalar = Scalar(0.0)
    dimensions: dict[str, Scalar] = field(default_factory=dict)
    timestamp: Timestamp = field(default_factory=Timestamp.now)
    prediction_id: str | None = None

    @classmethod
    def compute(cls, predicted: list[float], actual: list[float]) -> PredictionError:
        """Compute the root-mean-square error between predicted and actual."""
        if not predicted or not actual:
            return cls(magnitude=Scalar(0.0))
        min_len = min(len(predicted), len(actual))
        mse = sum(
            (predicted[i] - actual[i]) ** 2 for i in range(min_len)
        ) / max(min_len, 1)
        return cls(magnitude=Scalar(mse ** 0.0 if mse == 0 else mse ** 0.5))

    def is_zero(self) -> bool:
        from .scalars import SCALAR_EPSILON
        return float(self.magnitude) < SCALAR_EPSILON

    def to_dict(self) -> dict[str, Any]:
        return {
            "magnitude": float(self.magnitude),
            "dimensions": {k: float(v) for k, v in self.dimensions.items()},
            "timestamp": self.timestamp.to_dict(),
            "prediction_id": self.prediction_id,
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> PredictionError:
        return cls(
            magnitude=Scalar(data["magnitude"]),
            dimensions={k: Scalar(v) for k, v in data["dimensions"].items()},
            timestamp=Timestamp.from_dict(data["timestamp"]),
            prediction_id=data.get("prediction_id"),
        )


@dataclass(slots=True)
class Experience:
    """An observation bundled with prediction and error for learning.

    This is the unit of experience passed to the learning system.
    """

    observation: Observation = field(default_factory=Observation)
    prediction: Prediction | None = None
    prediction_error: PredictionError = field(default_factory=PredictionError)
    outcome_description: str = ""
    success: bool = True
    confidence: ConfidenceState = field(default_factory=ConfidenceState.default)
    evidence: EvidenceSet = field(default_factory=EvidenceSet)

    def to_dict(self) -> dict[str, Any]:
        return {
            "observation": self.observation.to_dict(),
            "prediction": self.prediction.to_dict() if self.prediction else None,
            "prediction_error": self.prediction_error.to_dict(),
            "outcome_description": self.outcome_description,
            "success": self.success,
            "confidence": self.confidence.to_dict(),
            "evidence": self.evidence.to_dict(),
        }

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Experience:
        pred = None
        if data.get("prediction"):
            pred = Prediction.from_dict(data["prediction"])
        return cls(
            observation=Observation.from_dict(data["observation"]),
            prediction=pred,
            prediction_error=PredictionError.from_dict(data["prediction_error"]),
            outcome_description=data.get("outcome_description", ""),
            success=data.get("success", True),
            confidence=ConfidenceState.from_dict(data.get("confidence", ConfidenceState.default().to_dict())),
            evidence=EvidenceSet.from_dict(data.get("evidence", {"items": []})),
        )
