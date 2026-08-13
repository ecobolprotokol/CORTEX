"""CORTEX runtime lifecycle - state machine, boot, shutdown.

This package implements the runtime lifecycle, including
the state machine, boot sequence, and shutdown procedures.

Key components:
- Runtime state machine (14 states, transitions)
- Boot sequence (7 phases)
- Graceful shutdown
- Health monitoring

Architectural Layer: Runtime
Governing Doc: DOC-02 section 8
"""
