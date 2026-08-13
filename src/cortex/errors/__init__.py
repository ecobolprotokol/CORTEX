"""CORTEX error taxonomy - exception hierarchy.

This package defines the error taxonomy and exception
hierarchy for all CORTEX modules.

Governing Doc: DOC-02 section 30 (Error Architecture)
"""

from .exceptions import (
    APIError,
    CLIError,
    ConfigError,
    CortexError,
    IntegrityError,
    MigrationError,
    PersistenceError,
    ResourceError,
    SecurityError,
    StateError,
    ValidationError,
)

__all__ = [
    "APIError",
    "CLIError",
    "ConfigError",
    "CortexError",
    "IntegrityError",
    "MigrationError",
    "PersistenceError",
    "ResourceError",
    "SecurityError",
    "StateError",
    "ValidationError",
]
