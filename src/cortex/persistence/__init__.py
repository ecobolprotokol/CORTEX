"""CORTEX persistence engine - .cx format, checkpoints.

This package implements the persistence engine, including
the binary .cx format, checkpoint lifecycle, and
state migration.

Key components:
- .cx format handling (binary layout, serialization)
- Checkpoint lifecycle (creation, validation, recovery)
- State migration (version upgrades, schema evolution)

Governing Doc: DOC-02 sections 26-28
"""
