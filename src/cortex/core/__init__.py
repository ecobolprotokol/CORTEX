"""CORTEX core types, IDs, scalars, common definitions.

This package contains the foundational type system for all CORTEX modules.
It is a leaf package with zero internal dependencies.

Key types:
- ID types (22 types via macro in DOC-03)
- Scalar (f32 wrapper with NaN/Infinity guard)
- CortexState (top-level state container)
- Observation, Experience (input types)
- Evidence, Provenance (knowledge provenance types)
- Timestamp, Duration, common enums
"""
