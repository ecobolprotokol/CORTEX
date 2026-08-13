"""Scalar type: validated float wrapper for all cognitive values.

Every floating-point cognitive value in CORTEX uses the ``Scalar`` type.
Construction rejects NaN and Infinity per DOC-03 rules NUM-002/NUM-003.
"""

from __future__ import annotations

import math
from typing import Any

SCALAR_EPSILON: float = 1e-6
"""Default comparison epsilon for f32 scalar equality."""


def scalar_eq(a: float, b: float) -> bool:
    """Approximate equality for scalar values using ``SCALAR_EPSILON``."""
    return abs(a - b) < SCALAR_EPSILON


class Scalar(float):
    """Float wrapper that rejects NaN and Infinity on construction.

    ``Scalar`` is a ``float`` subclass, so it participates naturally in
    arithmetic, comparison, hashing, and JSON serialisation.  Every
    ``Scalar`` value is guaranteed to be finite (``math.isfinite``).

    Examples::

        >>> Scalar(0.5)
        Scalar(0.5)
        >>> Scalar(float("nan"))
        Traceback (most recent call last): ...
        ValueError: Scalar rejected: NaN and Infinity are not valid. Got nan
        >>> Scalar(1.0) + Scalar(2.0)
        3.0
    """

    __slots__ = ()

    def __new__(cls, value: float = 0.0) -> Scalar:
        if not math.isfinite(value):
            raise ValueError(
                f"Scalar rejected: NaN and Infinity are not valid. Got {value!r}"
            )
        return super().__new__(cls, value)

    def __repr__(self) -> str:
        return f"Scalar({float(self)})"

    # -- Cognitive validation helpers ----------------------------------------

    def is_valid_cognitive_value(self) -> bool:
        """Return ``True`` if this scalar is a valid cognitive value (finite)."""
        return math.isfinite(self)

    def validate_range(self, min_val: float, max_val: float) -> None:
        """Raise ``ValueError`` if the scalar is outside ``[min_val, max_val]``.

        Also raises if the value is non-finite (defensive check).
        """
        if not math.isfinite(self):
            raise ValueError("NonFiniteValue")
        if self < min_val or self > max_val:
            raise ValueError(
                f"OutOfRange: value={self}, min={min_val}, max={max_val}"
            )

    def clamp(self, min_val: float, max_val: float) -> Scalar:
        """Return a new ``Scalar`` clamped to ``[min_val, max_val]``."""
        clamped = max(min_val, min(max_val, float(self)))
        return Scalar(clamped)

    # -- Serialisation -------------------------------------------------------

    def to_dict(self) -> dict[str, Any]:
        """Serialise to a plain dictionary."""
        return {"value": float(self)}

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Scalar:
        """Deserialise from a plain dictionary."""
        return cls(float(data["value"]))


# -- Convenience type alias -------------------------------------------------

# Public alias for readability in type annotations.
ScalarType = Scalar
