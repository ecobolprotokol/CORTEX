"""CORTEX exception hierarchy.

All exceptions inherit from CortexError and carry:
- code: machine-readable error code (e.g. "STATE_CORRUPTED")
- details: arbitrary context dict for diagnostics
- human-readable __str__ via __init__ message formatting
- standard Python exception chaining via __cause__

Governing Doc: DOC-02 section 30 (Error Architecture)
"""

from __future__ import annotations


class CortexError(Exception):
    """Base exception for all CORTEX errors.

    Attributes:
        code: Machine-readable error code.
        details: Additional context dictionary.
    """

    def __init__(
        self,
        message: str,
        *,
        code: str = "CORTEX_ERROR",
        details: dict | None = None,
    ) -> None:
        self.code = code
        self.details = details or {}
        super().__init__(message)

    def __str__(self) -> str:
        base = super().__str__()
        if self.code and self.code != "CORTEX_ERROR":
            return f"[{self.code}] {base}"
        return base

    def to_dict(self) -> dict:
        """Serialize error for structured logging / API responses."""
        result: dict = {
            "error_code": self.code,
            "message": str(self),
            "details": self.details,
        }
        if self.__cause__ is not None:
            result["cause"] = {
                "type": type(self.__cause__).__name__,
                "message": str(self.__cause__),
            }
        return result


# ---------------------------------------------------------------------------
# State errors — DOC-02 §14 (Runtime State Machine), state corruption
# ---------------------------------------------------------------------------

class StateError(CortexError):
    """State corruption, invalid transition, or invariant violation."""

    def __init__(
        self,
        message: str,
        *,
        code: str = "STATE_ERROR",
        details: dict | None = None,
    ) -> None:
        super().__init__(message, code=code, details=details)


# ---------------------------------------------------------------------------
# Persistence errors — DOC-02 §26-28 (Persistence Architecture)
# ---------------------------------------------------------------------------

class PersistenceError(CortexError):
    """File I/O, serialization, format, or corruption errors."""

    def __init__(
        self,
        message: str,
        *,
        code: str = "PERSISTENCE_ERROR",
        details: dict | None = None,
    ) -> None:
        super().__init__(message, code=code, details=details)


# ---------------------------------------------------------------------------
# Configuration errors — DOC-02 §11, §44.2 (Config Validation)
# ---------------------------------------------------------------------------

class ConfigError(CortexError):
    """Invalid or missing configuration."""

    def __init__(
        self,
        message: str,
        *,
        code: str = "CONFIG_ERROR",
        details: dict | None = None,
    ) -> None:
        super().__init__(message, code=code, details=details)


# ---------------------------------------------------------------------------
# Security errors — DOC-02 §36 (Security Architecture)
# ---------------------------------------------------------------------------

class SecurityError(CortexError):
    """Authentication failure, authorization denial, or policy violation."""

    def __init__(
        self,
        message: str,
        *,
        code: str = "SECURITY_ERROR",
        details: dict | None = None,
    ) -> None:
        super().__init__(message, code=code, details=details)


# ---------------------------------------------------------------------------
# Validation errors — DOC-02 §36.1 Layer 1 (Input Validation)
# ---------------------------------------------------------------------------

class ValidationError(CortexError):
    """Input validation failure (schema, range, type)."""

    def __init__(
        self,
        message: str,
        *,
        code: str = "VALIDATION_ERROR",
        details: dict | None = None,
    ) -> None:
        super().__init__(message, code=code, details=details)


# ---------------------------------------------------------------------------
# Resource errors — DOC-02 §31 (Resource Management)
# ---------------------------------------------------------------------------

class ResourceError(CortexError):
    """Memory pressure, compute budget exhaustion, or resource limits."""

    def __init__(
        self,
        message: str,
        *,
        code: str = "RESOURCE_ERROR",
        details: dict | None = None,
    ) -> None:
        super().__init__(message, code=code, details=details)


# ---------------------------------------------------------------------------
# Integrity errors — DOC-02 §27 (.cx format checksums), DOC-02 §36.1 Layer 5
# ---------------------------------------------------------------------------

class IntegrityError(CortexError):
    """Hash/checksum mismatch, data corruption detected."""

    def __init__(
        self,
        message: str,
        *,
        code: str = "INTEGRITY_ERROR",
        details: dict | None = None,
    ) -> None:
        super().__init__(message, code=code, details=details)


# ---------------------------------------------------------------------------
# Migration errors — DOC-02 §28 (State Versioning)
# ---------------------------------------------------------------------------

class MigrationError(CortexError):
    """Version incompatibility, missing migration path, or migration failure."""

    def __init__(
        self,
        message: str,
        *,
        code: str = "MIGRATION_ERROR",
        details: dict | None = None,
    ) -> None:
        super().__init__(message, code=code, details=details)


# ---------------------------------------------------------------------------
# API errors — DOC-02 §33 (API Architecture)
# ---------------------------------------------------------------------------

class APIError(CortexError):
    """HTTP-level errors from the API server."""

    def __init__(
        self,
        message: str,
        *,
        code: str = "API_ERROR",
        status_code: int | None = None,
        details: dict | None = None,
    ) -> None:
        self.status_code = status_code
        merged = dict(details) if details else {}
        if status_code is not None:
            merged["status_code"] = status_code
        super().__init__(message, code=code, details=merged)


# ---------------------------------------------------------------------------
# CLI errors — DOC-02 §34 (CLI Architecture)
# ---------------------------------------------------------------------------

class CLIError(CortexError):
    """Command dispatch, argument parsing, or CLI execution errors."""

    def __init__(
        self,
        message: str,
        *,
        code: str = "CLI_ERROR",
        details: dict | None = None,
    ) -> None:
        super().__init__(message, code=code, details=details)
