# CORTEX — Final Architectural Baseline

---

## Document Control

| Property | Value |
|---|---|
| **Document ID** | CORTEX-FINAL-BASELINE |
| **Title** | Final Architectural Baseline |
| **Version** | 1.0.0 |
| **Status** | Target Architecture |
| **Classification** | Repository Contract — Target State |
| **Scope** | Target repository tree, Python package architecture, documentation structure, test architecture, schema architecture, configuration, scripts, deployment, CI/CD, migration matrix, invariants, definition of done |
| **Parent Document** | CORTEX-DOC-02 Software Design Specification |
| **Current State Document** | CORTEX-DOC-11 Current Repository Architecture & Structure |
| **Effective Date** | 2026-08-13 |
| **Review Cycle** | Per architecture version transition |

### Revision History

| Version | Date | Author | Description |
|---|---|---|---|
| 1.0.0 | 2026-08-13 | CORTEX Architecture | Initial Final Architectural Baseline |

### Approval

| Role | Signature | Date |
|---|---|---|
| System Architect | _____________ | _____________ |
| Repository Maintainer | _____________ | _____________ |

### Document Purpose

This document defines **the target repository architecture of CORTEX** — the structure that the repository SHALL have when the Final Architectural Baseline is fully realized. It answers: *"What does the CORTEX repository look like when the target architecture is fully implemented?"*

This document is NOT the current state. The current repository structure is documented in **CORTEX-DOC-11 Current Repository Architecture & Structure**. The relationship between current state and this target state is documented in the Migration Matrix (§8).

### Document Scope

This specification covers:

- Complete target repository tree with all directories and files.
- Python package architecture and source organization.
- Documentation structure and contracts.
- Test architecture with category definitions.
- Schema architecture for `.cx`, API, and configuration.
- Configuration architecture with environment separation.
- Build, test, audit, and release script structure.
- Deployment architecture (Docker, Kubernetes, systemd, reverse-proxy).
- Migration architecture for schema/state evolution.
- Artifact architecture for generated outputs.
- CI/CD architecture with workflow definitions.
- Current → Target migration matrix.
- Repository invariants (FAB-R-xxx namespace).
- Definition of Done for baseline realization.

This specification does NOT cover:

- Internal module design or algorithm semantics (governed by DOC-02, DOC-03, DOC-04).
- Build pipeline stage semantics (governed by DOC-06).
- Configuration parameter semantics (governed by DOC-10).
- Security architecture beyond repository boundaries (governed by DOC-09).
- Deployment procedures or runbooks (governed by DOC-08).
- Testing strategy or test case specifications (governed by DOC-07).

---

## 1. Target Repository Identity

### 1.1 Target Repository Properties

| Property | Target Value |
|---|---|
| Repository name | `CORTEX` |
| Primary language | **Python** |
| Build system | **pyproject.toml** |
| Package name | `cortex` |
| Package version | 1.0.0 |
| Package structure | `src/cortex/` (src layout) |
| State file | `cortex.cx` (BLAKE3-integrity, binary format) |
| Configuration file | `cortex.toml` (TOML format) |

### 1.2 Target Repository Classification

| Attribute | Classification |
|---|---|
| Language ecosystem | Python / pyproject.toml |
| License | Proprietary (all rights reserved) |
| Version control | Git |
| Branching model | Mainline development |
| Commit convention | Conventional commits (`feat:`, `fix:`, `improve:`, `BREAKING CHANGE:`) |

---

## 2. Target Repository Tree

### 2.1 Authoritative Target Layout

The following tree represents the **target repository structure** that SHALL exist when the Final Architectural Baseline is fully realized.

```
CORTEX/
│
├── README.md                             # Project overview, quickstart, links
├── LICENSE                               # License file
├── CHANGELOG.md                          # Version changelog (conventional commits)
├── VERSION                               # Single-source version string
├── pyproject.toml                        # Package manifest — dependencies, build, metadata
├── Makefile                              # Common development commands
├── .gitignore                            # Git ignore rules
├── .editorconfig                         # Editor configuration — indentation, charset
│
├── docs/                                 # Documentation root
│   ├── DOC-01-Requirements.md            # Technical Specification
│   ├── DOC-02-Architecture.md            # Software Design Specification
│   ├── DOC-03-Data-Architecture.md       # Data & State Specification
│   ├── DOC-04-Algorithms.md              # Algorithm Specification
│   ├── DOC-05-API-CLI.md                 # API & CLI Specification
│   ├── DOC-06-Build-Release.md           # Build & Release Specification
│   ├── DOC-07-Testing-Validation.md      # Testing & Validation Specification
│   ├── DOC-08-Deployment-Operations.md   # Deployment & Operations Specification
│   ├── DOC-09-Security-Privacy.md        # Security & Privacy Specification
│   ├── DOC-10-Configuration-Reference.md # Configuration Reference
│   ├── DOC-11-Repository-Architecture.md # Current Repository Architecture (DOC-11)
│   │
│   ├── architecture/                     # Architecture-specific documentation
│   │   ├── final-architectural-baseline.md   # This document (mirror)
│   │   ├── consistency-audit.md              # Consistency audit results
│   │   ├── decision-records/                 # Architecture Decision Records (ADRs)
│   │   └── diagrams/                         # Architecture diagrams
│   │
│   ├── contracts/                        # Interface contracts
│   │   ├── api/                          # API contract definitions
│   │   ├── cli/                          # CLI contract definitions
│   │   ├── persistence/                  # Persistence format contracts
│   │   └── configuration/                # Configuration contracts
│   │
│   └── traceability/                     # Traceability documentation
│       ├── requirements-to-design.md     # Requirement → Design mapping
│       ├── requirements-to-tests.md      # Requirement → Test mapping
│       └── cross-document-matrix.md      # Cross-document traceability matrix
│
├── src/                                  # Source code root
│   └── cortex/                           # Python package root
│       ├── __init__.py                   # Package initialization, version export
│       │
│       ├── core/                         # Core types, IDs, scalars, common definitions
│       ├── cognitive/                    # Cognitive pipeline orchestration
│       ├── world/                        # World model — entities, transitions, simulation
│       ├── memory/                       # Memory system — working, episodic, semantic, procedural
│       ├── learning/                     # Continual learning — signals, attribution, replay
│       ├── inference/                    # Inference engine
│       ├── prediction/                   # Prediction engine
│       ├── hypothesis/                   # Hypothesis generation and evaluation
│       ├── self_model/                   # Self model — capability estimation
│       ├── policy/                       # Policy / risk gate — security boundary
│       ├── runtime/                      # Runtime lifecycle — state machine, boot, shutdown
│       ├── state/                        # State management — serialization, mutations
│       ├── persistence/                  # Persistence engine — .cx format, checkpoints
│       ├── serialization/                # Serialization layer — bincode, JSON, TOML
│       ├── provenance/                   # Provenance tracking — origin, lineage
│       ├── security/                     # Security implementation
│       │   ├── __init__.py
│       │   ├── hashing/                  # BLAKE3 hashing — integrity operations
│       │   ├── integrity/                # Integrity verification — state validation
│       │   └── key_management/           # Key management — token handling
│       ├── config/                       # Configuration — parsing, validation, distribution
│       ├── api/                          # Embedded API — HTTP server, routes, handlers
│       ├── cli/                          # CLI — command parsing, dispatch
│       └── errors/                       # Error taxonomy — exception hierarchy
│
├── tests/                                # Test root
│   ├── unit/                             # Unit tests — per-module, per-class
│   ├── integration/                      # Integration tests — cross-module scenarios
│   ├── system/                           # System tests — end-to-end pipeline
│   ├── acceptance/                       # Acceptance tests — DOC-01 criteria
│   ├── regression/                       # Regression tests — backward compatibility
│   ├── property/                         # Property-based tests — invariant verification
│   ├── performance/                      # Performance tests — latency, throughput
│   ├── security/                         # Security tests — policy, auth, injection
│   └── fixtures/                         # Test fixtures — shared data, mocks
│
├── schemas/                              # Schema definitions
│   ├── cx/                               # .cx file format schemas
│   │   ├── format.md                     # Binary format specification
│   │   ├── sections/                     # Per-section schema definitions
│   │   └── schema.json                   # Machine-readable schema
│   ├── api/                              # API schemas — request/response
│   └── configuration/                    # Configuration schemas — validation rules
│
├── config/                               # Configuration profiles
│   ├── defaults/                         # Default configuration values
│   ├── development/                      # Development environment config
│   ├── testing/                          # Testing environment config
│   └── production/                       # Production environment config
│
├── scripts/                              # Development and operations scripts
│   ├── build/                            # Build scripts — compilation, packaging
│   ├── test/                             # Test scripts — runner orchestration
│   ├── audit/                            # Audit scripts — security, license, dependency
│   ├── migration/                        # Migration scripts — schema/state evolution
│   └── release/                          # Release scripts — tagging, packaging, publishing
│
├── deployment/                           # Deployment configurations
│   ├── docker/                           # Docker — Dockerfile, docker-compose
│   ├── kubernetes/                       # Kubernetes — manifests, Helm charts
│   ├── systemd/                          # systemd — service files
│   └── reverse-proxy/                    # Reverse proxy — nginx, caddy configs
│
├── benchmarks/                           # Performance benchmarks
│   ├── cognitive/                        # Cognitive pipeline benchmarks
│   ├── memory/                           # Memory retrieval benchmarks
│   ├── learning/                         # Learning system benchmarks
│   ├── inference/                        # Inference benchmarks
│   └── persistence/                      # Persistence I/O benchmarks
│
├── examples/                             # Usage examples
│   ├── basic/                            # Basic usage examples
│   ├── api/                              # API usage examples
│   ├── cli/                              # CLI usage examples
│   └── persistence/                      # Persistence examples
│
├── migrations/                           # Schema/state migration artifacts
│   └── v1/                               # Version 1 migrations
│
├── artifacts/                            # Generated artifacts (gitignored)
│   ├── builds/                           # Build outputs
│   ├── test-reports/                     # Test report outputs
│   └── audit-reports/                    # Audit report outputs
│
└── .github/                              # GitHub configuration
    ├── workflows/
    │   ├── ci.yml                        # CI pipeline — lint, type-check, test
    │   ├── test.yml                      # Test pipeline — full test suite
    │   ├── security.yml                  # Security scanning — audit, dependency check
    │   └── release.yml                   # Release pipeline — build, package, publish
    ├── ISSUE_TEMPLATE/                   # Issue templates
    └── pull_request_template.md          # PR template
```

---

## 3. Target Source Architecture

### 3.1 Python Package Structure

The target uses a **src layout** with `src/cortex/` as the package root. Each subdirectory is a Python sub-package.

| Package | Responsibility | Governing Doc |
|---|---|---|
| `core/` | Core types, IDs, scalars, common definitions | DOC-03 |
| `cognitive/` | Cognitive pipeline orchestration | DOC-02 §9 |
| `world/` | World model — entities, transitions, simulation | DOC-02 §18 |
| `memory/` | Memory system — working, episodic, semantic, procedural, associative | DOC-02 §17 |
| `learning/` | Continual learning — signals, attribution, replay, stability | DOC-02 §22 |
| `inference/` | Inference engine — reasoning, deduction | DOC-02 §19 |
| `prediction/` | Prediction engine — future state estimation | DOC-02 §20 |
| `hypothesis/` | Hypothesis generation and evaluation | DOC-02 §19 |
| `self_model/` | Self model — capability estimation, health | DOC-02 §23 |
| `policy/` | Policy / risk gate — security boundary | DOC-02 §24 |
| `runtime/` | Runtime lifecycle — state machine, boot, shutdown | DOC-02 §8 |
| `state/` | State management — serialization, mutations | DOC-03 |
| `persistence/` | Persistence engine — `.cx` format, checkpoints | DOC-02 §26-28 |
| `serialization/` | Serialization layer — bincode, JSON, TOML | DOC-03 §32 |
| `provenance/` | Provenance tracking — origin, lineage | DOC-03 §36 |
| `security/` | Security implementation | DOC-09 |
| `security/hashing/` | BLAKE3 hashing — integrity operations | DOC-09, DOC-03 §35 |
| `security/integrity/` | Integrity verification — state validation | DOC-09 |
| `security/key_management/` | Key management — token handling | DOC-09 |
| `config/` | Configuration — parsing, validation, distribution | DOC-10 |
| `api/` | Embedded API — HTTP server, routes, handlers | DOC-05 |
| `cli/` | CLI — command parsing, dispatch | DOC-05 |
| `errors/` | Error taxonomy — exception hierarchy | DOC-02 §30 |

### 3.2 Security Package Boundary

The `security/` package is the designated boundary for all cryptographic operations:

| Sub-package | Function | Algorithm |
|---|---|---|
| `security/hashing/` | Integrity hashing | BLAKE3-256 |
| `security/integrity/` | State verification | BLAKE3-256 |
| `security/key_management/` | Token handling | HMAC-SHA256 (API tokens only) |

> **Security Boundary Rule:** All hashing and integrity operations MUST reside within `src/cortex/security/`. No other package SHALL perform cryptographic operations directly.

> **BLAKE3 Contract:** BLAKE3 is used exclusively for integrity hashing. It is NOT used for encryption, key derivation, or password hashing. There is no AES-256 or symmetric encryption in the architecture.

### 3.3 Dependency Flow Rules

Target dependency flow follows the rules defined in DOC-02 §6, translated to Python package boundaries:

| Allowed Direction | From | To |
|---|---|---|
| 1 | `core/` | (no internal deps — leaf package) |
| 2 | Any cognitive package | `core/` |
| 3 | `policy/` | `core/`, `security/` |
| 4 | `persistence/` | `core/`, `serialization/`, `security/` |
| 5 | `api/` | `core/`, `cli/` |
| 6 | `cli/` | `core/` |
| 7 | `runtime/` | All packages (orchestration only) |

| Forbidden Direction | Reason |
|---|---|
| `core/` → any package | Leaf package — no upward dependencies |
| `learning/` → `policy/` | Learning must not bypass policy gate |
| `memory/` → `api/` | Memory has no awareness of API layer |
| `security/hashing/` → any cognitive package | Security boundary isolation |
| Any cognitive package → `runtime/` | No circular dependency to orchestrator |

---

## 4. Target Documentation Architecture

### 4.1 Documentation Root

All documentation resides under `docs/`. The DOC-01 through DOC-11 series is relocated from repository root to `docs/`.

| Current Location | Target Location | Action |
|---|---|---|
| `CORTEX-DOC-01.md` | `docs/DOC-01-Requirements.md` | MIGRATE |
| `CORTEX-DOC-02.md` | `docs/DOC-02-Architecture.md` | MIGRATE |
| `CORTEX-DOC-03.md` | `docs/DOC-03-Data-Architecture.md` | MIGRATE |
| `CORTEX-DOC-04.md` | `docs/DOC-04-Algorithms.md` | MIGRATE |
| `CORTEX-DOC-05.md` | `docs/DOC-05-API-CLI.md` | MIGRATE |
| `CORTEX-DOC-06.md` | `docs/DOC-06-Build-Release.md` | MIGRATE |
| `CORTEX-DOC-07.md` | `docs/DOC-07-Testing-Validation.md` | MIGRATE |
| `CORTEX-DOC-08.md` | `docs/DOC-08-Deployment-Operations.md` | MIGRATE |
| `CORTEX-DOC-09.md` | `docs/DOC-09-Security-Privacy.md` | MIGRATE |
| `CORTEX-DOC-10.md` | `docs/DOC-10-Configuration-Reference.md` | MIGRATE |
| `CORTEX-DOC-11.md` | `docs/DOC-11-Repository-Architecture.md` | MIGRATE |

### 4.2 Supplementary Documentation

| Directory | Purpose | Content |
|---|---|---|
| `docs/architecture/` | Architecture decisions and baselines | ADRs, consistency audits, diagrams |
| `docs/contracts/` | Interface contracts | API, CLI, persistence, configuration contracts |
| `docs/traceability/` | Traceability documentation | Requirement → design → implementation → test |

### 4.3 Documentation Boundaries

| Boundary | Rule | Governing Doc |
|---|---|---|
| DOC series | DOC-01 through DOC-11 are versioned specifications | DOC-11 §5 |
| Architecture decisions | ADRs record significant architectural choices | DOC-FINAL |
| Contracts | Machine-readable interface definitions | DOC-05 |
| Traceability | Requirement-to-test mapping | DOC-07 |

---

## 5. Target Test Architecture

### 5.1 Test Directory Structure

| Directory | Category | Scope | Governing Doc |
|---|---|---|---|
| `tests/unit/` | Unit tests | Per-module, per-class isolation | DOC-07 §2.1 |
| `tests/integration/` | Integration tests | Cross-module scenarios | DOC-07 §2.2 |
| `tests/system/` | System tests | End-to-end pipeline | DOC-07 §2.3 |
| `tests/acceptance/` | Acceptance tests | DOC-01 acceptance criteria | DOC-07 §2.4 |
| `tests/regression/` | Regression tests | Backward compatibility | DOC-07 §3 |
| `tests/property/` | Property-based tests | Invariant verification | DOC-07 §4 |
| `tests/performance/` | Performance tests | Latency, throughput | DOC-07 §6 |
| `tests/security/` | Security tests | Policy, auth, injection | DOC-07 §5 |
| `tests/fixtures/` | Test fixtures | Shared data, mocks | DOC-07 |

### 5.2 Test Traceability

```
DOC-01 requirement
    ↓
DOC-02 design
    ↓
implementation (src/cortex/<package>/)
    ↓
test category (tests/<category>/)
    ↓
acceptance criterion
```

Every test category SHALL be traceable to DOC-01 requirements through DOC-02 design specifications.

---

## 6. Target Schema Architecture

### 6.1 Schema Directory Structure

| Directory | Purpose | Content |
|---|---|---|
| `schemas/cx/` | `.cx` file format schemas | Binary format spec, section definitions, machine-readable schema |
| `schemas/api/` | API schemas | Request/response schemas, endpoint definitions |
| `schemas/configuration/` | Configuration schemas | `cortex.toml` schema, validation rules |

### 6.2 Schema Boundaries

| Boundary | Rule |
|---|---|
| Format specification | `schemas/cx/format.md` — human-readable binary layout |
| Section definitions | `schemas/cx/sections/` — per-section schemas |
| Machine-readable | `schemas/cx/schema.json` — JSON Schema for validation |
| Separation | Schemas are NOT mixed with implementation source |

---

## 7. Target Configuration Architecture

### 7.1 Configuration Directory Structure

| Directory | Purpose | Environment |
|---|---|---|
| `config/defaults/` | Default configuration values | All |
| `config/development/` | Development overrides | Development |
| `config/testing/` | Testing overrides | Testing |
| `config/production/` | Production overrides | Production |

### 7.2 Configuration Boundaries

| Boundary | Rule | Governing Doc |
|---|---|---|
| Parameter semantics | Defined by DOC-10 | DOC-10 |
| File location | `config/` directory | DOC-FINAL §7 |
| Loading boundary | Application loads from `config/` + `cortex.toml` | DOC-10 §1.2 |
| Environment separation | Defaults + environment overlay | DOC-FINAL §7 |

---

## 8. Current → Target Migration Matrix

### 8.1 Migration Overview

The current repository (Rust/Cargo) SHALL be migrated to the target repository (Python/pyproject.toml). The following matrix documents every path transformation.

### 8.2 Root-Level Migration

| Current Path | Target Path | Action | Status |
|---|---|---|---|
| `Cargo.toml` | `pyproject.toml` | RESTRUCTURE | PENDING |
| `Cargo.lock` | (removed — Python uses lockfiles differently) | REMOVE | PENDING |
| `.gitignore` | `.gitignore` | MODIFY | PENDING |
| (none) | `README.md` | CREATE | PENDING |
| (none) | `LICENSE` | CREATE | PENDING |
| (none) | `CHANGELOG.md` | CREATE | PENDING |
| (none) | `VERSION` | CREATE | PENDING |
| (none) | `Makefile` | CREATE | PENDING |
| (none) | `.editorconfig` | CREATE | PENDING |

### 8.3 Documentation Migration

| Current Path | Target Path | Action | Status |
|---|---|---|---|
| `CORTEX-DOC-01.md` | `docs/DOC-01-Requirements.md` | MIGRATE | PENDING |
| `CORTEX-DOC-02.md` | `docs/DOC-02-Architecture.md` | MIGRATE | PENDING |
| `CORTEX-DOC-03.md` | `docs/DOC-03-Data-Architecture.md` | MIGRATE | PENDING |
| `CORTEX-DOC-04.md` | `docs/DOC-04-Algorithms.md` | MIGRATE | PENDING |
| `CORTEX-DOC-05.md` | `docs/DOC-05-API-CLI.md` | MIGRATE | PENDING |
| `CORTEX-DOC-06.md` | `docs/DOC-06-Build-Release.md` | MIGRATE | PENDING |
| `CORTEX-DOC-07.md` | `docs/DOC-07-Testing-Validation.md` | MIGRATE | PENDING |
| `CORTEX-DOC-08.md` | `docs/DOC-08-Deployment-Operations.md` | MIGRATE | PENDING |
| `CORTEX-DOC-09.md` | `docs/DOC-09-Security-Privacy.md` | MIGRATE | PENDING |
| `CORTEX-DOC-10.md` | `docs/DOC-10-Configuration-Reference.md` | MIGRATE | PENDING |
| `CORTEX-DOC-11.md` | `docs/DOC-11-Repository-Architecture.md` | MIGRATE | PENDING |
| (none) | `docs/architecture/` | CREATE | PENDING |
| (none) | `docs/architecture/final-architectural-baseline.md` | CREATE | PENDING |
| (none) | `docs/architecture/consistency-audit.md` | CREATE | PENDING |
| (none) | `docs/architecture/decision-records/` | CREATE | PENDING |
| (none) | `docs/architecture/diagrams/` | CREATE | PENDING |
| (none) | `docs/contracts/api/` | CREATE | PENDING |
| (none) | `docs/contracts/cli/` | CREATE | PENDING |
| (none) | `docs/contracts/persistence/` | CREATE | PENDING |
| (none) | `docs/contracts/configuration/` | CREATE | PENDING |
| (none) | `docs/traceability/` | CREATE | PENDING |
| (none) | `docs/traceability/requirements-to-design.md` | CREATE | PENDING |
| (none) | `docs/traceability/requirements-to-tests.md` | CREATE | PENDING |
| (none) | `docs/traceability/cross-document-matrix.md` | CREATE | PENDING |

### 8.4 Source Code Migration

| Current Path | Target Path | Action | Status |
|---|---|---|---|
| `src/main.rs` | `src/cortex/__init__.py` | RESTRUCTURE | PENDING |
| `src/cortex.rs` | `src/cortex/runtime/__init__.py` | RESTRUCTURE | PENDING |
| `src/config.rs` | `src/cortex/config/__init__.py` | RESTRUCTURE | PENDING |
| `src/error.rs` | `src/cortex/errors/__init__.py` | RESTRUCTURE | PENDING |
| `src/runtime.rs` | `src/cortex/runtime/lifecycle.py` | RESTRUCTURE | PENDING |
| `src/types/` | `src/cortex/core/` | RESTRUCTURE | PENDING |
| `src/language/` | `src/cortex/cognitive/` (absorbed) | RESTRUCTURE | PENDING |
| `src/neural/` | `src/cortex/cognitive/` (absorbed) | RESTRUCTURE | PENDING |
| `src/memory/` | `src/cortex/memory/` | RESTRUCTURE | PENDING |
| `src/world/` | `src/cortex/world/` | RESTRUCTURE | PENDING |
| `src/reasoning/` | `src/cortex/inference/` | RESTRUCTURE | PENDING |
| `src/planning/` | `src/cortex/inference/` (absorbed) | RESTRUCTURE | PENDING |
| `src/verification/` | `src/cortex/inference/` (absorbed) | RESTRUCTURE | PENDING |
| `src/learning/` | `src/cortex/learning/` | RESTRUCTURE | PENDING |
| `src/self_model/` | `src/cortex/self_model/` | RESTRUCTURE | PENDING |
| `src/policy/` | `src/cortex/policy/` | RESTRUCTURE | PENDING |
| `src/internet/` | `src/cortex/cognitive/` (absorbed) | RESTRUCTURE | PENDING |
| `src/persistence/` | `src/cortex/persistence/` | RESTRUCTURE | PENDING |
| `src/api/` | `src/cortex/api/` | RESTRUCTURE | PENDING |
| `src/cli/` | `src/cortex/cli/` | RESTRUCTURE | PENDING |
| `src/observability/` | `src/cortex/runtime/` (absorbed) | RESTRUCTURE | PENDING |
| (none) | `src/cortex/state/` | CREATE | PENDING |
| (none) | `src/cortex/serialization/` | CREATE | PENDING |
| (none) | `src/cortex/provenance/` | CREATE | PENDING |
| (none) | `src/cortex/security/` | CREATE | PENDING |
| (none) | `src/cortex/security/hashing/` | CREATE | PENDING |
| (none) | `src/cortex/security/integrity/` | CREATE | PENDING |
| (none) | `src/cortex/security/key_management/` | CREATE | PENDING |
| (none) | `src/cortex/errors/` | CREATE | PENDING |

### 8.5 Test Migration

| Current Path | Target Path | Action | Status |
|---|---|---|---|
| `tests/cognitive_pipeline.rs` | `tests/integration/test_cognitive_pipeline.py` | RESTRUCTURE | PENDING |
| `tests/persistence_roundtrip.rs` | `tests/integration/test_persistence_roundtrip.py` | RESTRUCTURE | PENDING |
| `tests/learning_stability.rs` | `tests/integration/test_learning_stability.py` | RESTRUCTURE | PENDING |
| `tests/security_policy.rs` | `tests/security/test_policy_enforcement.py` | RESTRUCTURE | PENDING |
| `tests/api_endpoints.rs` | `tests/integration/test_api_endpoints.py` | RESTRUCTURE | PENDING |
| `tests/corruption_recovery.rs` | `tests/integration/test_corruption_recovery.py` | RESTRUCTURE | PENDING |
| (none) | `tests/unit/` | CREATE | PENDING |
| (none) | `tests/system/` | CREATE | PENDING |
| (none) | `tests/acceptance/` | CREATE | PENDING |
| (none) | `tests/regression/` | CREATE | PENDING |
| (none) | `tests/property/` | CREATE | PENDING |
| (none) | `tests/performance/` | CREATE | PENDING |
| (none) | `tests/fixtures/` | CREATE | PENDING |

### 8.6 Benchmark Migration

| Current Path | Target Path | Action | Status |
|---|---|---|---|
| `benches/cognitive_loop.rs` | `benchmarks/cognitive/test_cognitive_loop.py` | RESTRUCTURE | PENDING |
| `benches/memory_retrieval.rs` | `benchmarks/memory/test_memory_retrieval.py` | RESTRUCTURE | PENDING |
| `benches/persistence.rs` | `benchmarks/persistence/test_persistence.py` | RESTRUCTURE | PENDING |
| (none) | `benchmarks/learning/` | CREATE | PENDING |
| (none) | `benchmarks/inference/` | CREATE | PENDING |

### 8.7 New Directories (TARGET-ONLY)

| Target Path | Action | Status |
|---|---|---|
| `schemas/` | CREATE | PENDING |
| `schemas/cx/` | CREATE | PENDING |
| `schemas/cx/sections/` | CREATE | PENDING |
| `schemas/api/` | CREATE | PENDING |
| `schemas/configuration/` | CREATE | PENDING |
| `config/` | CREATE | PENDING |
| `config/defaults/` | CREATE | PENDING |
| `config/development/` | CREATE | PENDING |
| `config/testing/` | CREATE | PENDING |
| `config/production/` | CREATE | PENDING |
| `scripts/` | CREATE | PENDING |
| `scripts/build/` | CREATE | PENDING |
| `scripts/test/` | CREATE | PENDING |
| `scripts/audit/` | CREATE | PENDING |
| `scripts/migration/` | CREATE | PENDING |
| `scripts/release/` | CREATE | PENDING |
| `deployment/` | CREATE | PENDING |
| `deployment/docker/` | CREATE | PENDING |
| `deployment/kubernetes/` | CREATE | PENDING |
| `deployment/systemd/` | CREATE | PENDING |
| `deployment/reverse-proxy/` | CREATE | PENDING |
| `examples/` | CREATE | PENDING |
| `examples/basic/` | CREATE | PENDING |
| `examples/api/` | CREATE | PENDING |
| `examples/cli/` | CREATE | PENDING |
| `examples/persistence/` | CREATE | PENDING |
| `migrations/` | CREATE | PENDING |
| `migrations/v1/` | CREATE | PENDING |
| `artifacts/` | CREATE | PENDING |
| `artifacts/builds/` | CREATE | PENDING |
| `artifacts/test-reports/` | CREATE | PENDING |
| `artifacts/audit-reports/` | CREATE | PENDING |
| `.github/` | CREATE | PENDING |
| `.github/workflows/` | CREATE | PENDING |
| `.github/ISSUE_TEMPLATE/` | CREATE | PENDING |

### 8.8 Removed from Target

| Current Path | Action | Reason | Status |
|---|---|---|---|
| `Cargo.toml` | REMOVE | Replaced by `pyproject.toml` | PENDING |
| `Cargo.lock` | REMOVE | Python dependency management | PENDING |
| `benches/` | REMOVE | Relocated to `benchmarks/` | PENDING |
| All `*.rs` files | REMOVE | Rewritten as `*.py` | PENDING |

---

## 9. Target Repository Invariants

### 9.1 Final Baseline Invariants

| # | Invariant | Enforcement | Severity |
|---|---|---|---|
| FAB-R-001 | Target repository SHALL conform to the approved final tree | Structural check | Critical |
| FAB-R-002 | Python source SHALL reside under `src/cortex/` | Layout check | Critical |
| FAB-R-003 | Documentation SHALL reside under `docs/` | Layout check | High |
| FAB-R-004 | Schemas SHALL reside under `schemas/` | Layout check | High |
| FAB-R-005 | Tests SHALL reside under `tests/` | Layout check | Critical |
| FAB-R-006 | Deployment artifacts SHALL reside under `deployment/` | Layout check | High |
| FAB-R-007 | Build/test/audit/release scripts SHALL reside under `scripts/` | Layout check | High |
| FAB-R-008 | Migration artifacts SHALL reside under `migrations/` | Layout check | High |
| FAB-R-009 | Security implementation SHALL remain under `src/cortex/security/` | Layout check | Critical |
| FAB-R-010 | BLAKE3 integrity implementation SHALL remain within `security/hashing/` and `security/integrity/` | Code boundary check | Critical |
| FAB-R-011 | Final repository SHALL NOT depend on obsolete Rust/Cargo structure unless explicitly classified as migration/legacy material | Dependency check | High |
| FAB-R-012 | Every target directory SHALL have an explicitly defined responsibility | Documentation check | High |
| FAB-R-013 | Every implementation package SHALL be traceable to the architecture specification | Traceability check | High |
| FAB-R-014 | Every requirement SHALL be traceable to implementation and tests | Traceability check | Critical |
| FAB-R-015 | Final baseline SHALL distinguish current state from target state | Documentation check | High |

### 9.2 Invariant Severity Classification

| Severity | Meaning | Action on Violation |
|---|---|---|
| Critical | System integrity at risk | Block merge, require immediate fix |
| High | Specification non-conformance | Block merge, require fix before release |
| Medium | Quality degradation | Warning, allow with technical debt ticket |
| Low | Style/convention deviation | Informational, address in next cycle |

---

## 10. Definition of Done

### 10.1 Baseline Realization Criteria

The Final Architectural Baseline is NOT considered realized merely because this document exists. The baseline is considered **REALIZED** only when ALL of the following criteria are satisfied:

| # | Criterion | Validation |
|---|---|---|
| DOD-01 | Target root structure exists (`README.md`, `LICENSE`, `CHANGELOG.md`, `VERSION`, `pyproject.toml`, `Makefile`, `.gitignore`, `.editorconfig`) | File existence check |
| DOD-02 | Python package exists under `src/cortex/` with `__init__.py` | Package check |
| DOD-03 | Documentation structure exists under `docs/` | Directory check |
| DOD-04 | DOC-01 through DOC-11 are synchronized (same version) | Version check |
| DOD-05 | `schemas/` exists and is populated | Directory check |
| DOD-06 | `tests/` hierarchy exists with all 9 categories | Directory check |
| DOD-07 | `config/` hierarchy exists with all 4 environments | Directory check |
| DOD-08 | `scripts/` hierarchy exists with all 5 categories | Directory check |
| DOD-09 | `deployment/` hierarchy exists with all 4 platforms | Directory check |
| DOD-10 | `benchmarks/` hierarchy exists with all 5 categories | Directory check |
| DOD-11 | `examples/` hierarchy exists with all 4 categories | Directory check |
| DOD-12 | `migrations/` hierarchy exists | Directory check |
| DOD-13 | `artifacts/` hierarchy exists (gitignored) | Directory check |
| DOD-14 | `.github/` CI structure exists with all 4 workflows | File check |
| DOD-15 | Current → Target migration is complete (all paths migrated) | Migration matrix verification |
| DOD-16 | No obsolete Rust/Cargo structure remains (unless explicitly legacy) | Codebase scan |
| DOD-17 | BLAKE3 integrity architecture is consistent across all packages | Code boundary check |
| DOD-18 | Cross-document traceability passes | Traceability audit |
| DOD-19 | Repository consistency audit passes | Audit report |
| DOD-20 | Final architectural audit reports zero unresolved conflicts | Audit report |

### 10.2 Interim State

Until the Final Architectural Baseline is realized, the repository exists in an **interim state** where:

- DOC-11 documents the current (Rust) repository architecture.
- DOC-FINAL documents the target (Python) repository architecture.
- The migration matrix (§8) tracks progress from current to target.
- Both documents are authoritative for their respective states.

---

## 11. Conflict Disclosure

### 11.1 Known Conflicts Between Current and Target

The following conflicts exist between the current repository and the target architecture. These are NOT hidden — they are explicit migration requirements.

| # | Conflict | Current State | Target State | Required Change | Status |
|---|---|---|---|---|---|
| C-01 | Language | Rust | Python | Full language migration | PENDING |
| C-02 | Build system | Cargo (`Cargo.toml`) | pyproject.toml | Replace build system | PENDING |
| C-03 | Module layout | `src/<module>/` (71 modules) | `src/cortex/<package>/` (22 packages) | Restructure packages | PENDING |
| C-04 | Documentation location | Root (`CORTEX-DOC-NN.md`) | `docs/DOC-NN-*.md` | Relocate all docs | PENDING |
| C-05 | Test location | `tests/*.rs` (flat) | `tests/<category>/` (hierarchical) | Restructure tests | PENDING |
| C-06 | Benchmarks | `benches/*.rs` | `benchmarks/<category>/` | Relocate and restructure | PENDING |
| C-07 | Schemas | Not present | `schemas/` | Create new | PENDING |
| C-08 | Configuration profiles | Not present | `config/` | Create new | PENDING |
| C-09 | Scripts | Not present | `scripts/` | Create new | PENDING |
| C-10 | Deployment | Not present | `deployment/` | Create new | PENDING |
| C-11 | Examples | Not present | `examples/` | Create new | PENDING |
| C-12 | Migrations | Not present | `migrations/` | Create new | PENDING |
| C-13 | Artifacts | Not present | `artifacts/` | Create new | PENDING |
| C-14 | CI/CD | Not present | `.github/workflows/` | Create new | PENDING |
| C-15 | Root files | `Cargo.toml`, `Cargo.lock` | `pyproject.toml`, `README.md`, etc. | Replace and add | PENDING |

### 11.2 Conflict Resolution Rules

| Rule | Description |
|---|---|
| CR-01 | All conflicts SHALL be resolved through the migration matrix (§8) |
| CR-02 | No conflict SHALL be silently resolved — every resolution requires an action entry |
| CR-03 | Current-state information SHALL NOT be deleted until target-state equivalent is verified |
| CR-04 | Interim states SHALL be documented with explicit status |
| CR-05 | Conflict resolution SHALL be tracked in `docs/architecture/consistency-audit.md` |

---

## 12. Boundary Summary

### 12.1 Architectural Boundaries

| Boundary | Current (DOC-11) | Target (DOC-FINAL) |
|---|---|---|
| Source code | `src/*.rs`, `src/<module>/` | `src/cortex/<package>/` |
| Documentation | Root `CORTEX-DOC-*.md` | `docs/DOC-NN-*.md` + supplementary |
| Tests | `tests/*.rs` (flat) | `tests/<category>/` (hierarchical) |
| Schemas | Not present | `schemas/` |
| Configuration | `cortex.toml` (single file) | `cortex.toml` + `config/` profiles |
| Build | `Cargo.toml` | `pyproject.toml` |
| Scripts | Not present | `scripts/` |
| Deployment | Not present | `deployment/` |
| Benchmarks | `benches/*.rs` | `benchmarks/<category>/` |
| Examples | Not present | `examples/` |
| Migrations | Not present | `migrations/` |
| Artifacts | `target/` (gitignored) | `artifacts/` (gitignored) |
| CI/CD | Not present | `.github/workflows/` |

### 12.2 Security Boundaries

| Boundary | Rule |
|---|---|
| Cryptographic operations | All hashing/integrity within `src/cortex/security/` |
| BLAKE3 usage | Integrity hashing only — NOT encryption |
| No AES-256 | Architecture does not include symmetric encryption |
| Key management | `src/cortex/security/key_management/` only |
| Policy isolation | `src/cortex/policy/` is architecturally separate from learned state |

### 12.3 Dependency Boundaries

| Boundary | Current (Rust) | Target (Python) |
|---|---|---|
| Package manifest | `Cargo.toml` | `pyproject.toml` |
| Lock file | `Cargo.lock` | `poetry.lock` / `uv.lock` (TBD) |
| Dependency count | ≤14 production crates | TBD |
| Prohibited categories | Databases, web frameworks, ML libs, GPU | Same categories |
| Integrity library | `blake3` crate | `blake3` Python binding |
| Serialization | `bincode`, `serde_json`, `toml` | `msgpack`/`bincode`, `json`, `toml` (TBD) |
| Compression | `zstd` crate | `zstandard` Python binding |
| Async runtime | `tokio` | `asyncio` (stdlib) |
| HTTP server | `hyper` | TBD (e.g., `uvicorn` + `starlette`) |
| CLI | `clap` | TBD (e.g., `click` or `typer`) |

---

*End of Document — CORTEX Final Architectural Baseline v1.0.0*
*For the current repository state, see CORTEX-DOC-11 Current Repository Architecture & Structure*
