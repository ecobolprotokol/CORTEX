# CORTEX — Final Architectural Baseline

---

## Document Control

| Property | Value |
|---|---|
| **Document ID** | CORTEX-DOC-11 |
| **Title** | Repository Architecture & Struktur |
| **Version** | 1.1.0 |
| **Status** | Final Architectural Baseline |
| **Classification** | Repository Contract |
| **Scope** | Repository tree, Rust package architecture, documentation structure, test architecture, schema architecture, configuration, scripts, deployment, CI/CD, invariants, definition of done |
| **Parent Document** | CORTEX-DOC-02 Software Design Specification |
| **Effective Date** | 2026-08-13 |
| **Review Cycle** | Per architecture version transition |

### Revision History

| Version | Date | Author | Description |
|---|---|---|---|
| 1.0.0 | 2026-08-13 | CORTEX Architecture | Initial final baseline |
| 1.1.0 | 2026-08-13 | CORTEX Architecture | Establish as Final Architectural Baseline for Rust architecture |

### Approval

| Role | Signature | Date |
|---|---|---|
| System Architect | _____________ | _____________ |
| Repository Maintainer | _____________ | _____________ |

### Document Purpose

This document defines **the repository architecture of CORTEX** as the Final Architectural Baseline. It constitutes the authoritative repository structure: every directory, every file, every naming convention, every boundary, and every structural invariant.

### Document Scope

This specification covers:

- Complete repository tree with all directories and files.
- Rust package architecture and source organization.
- Documentation structure and contracts.
- Test architecture with category definitions.
- Schema architecture for `.cx`, API, and configuration.
- Configuration architecture with environment separation.
- Build, test, audit, and release script structure.
- Deployment architecture (Docker, Kubernetes, systemd, reverse-proxy).
- Artifact architecture for generated outputs.
- CI/CD architecture with workflow definitions.
- Repository invariants (FAB-R-xxx namespace).
- Definition of done for baseline conformance.

This specification does NOT cover:

- Internal module design or algorithm semantics (governed by DOC-02, DOC-03, DOC-04).
- Build pipeline stage semantics (governed by DOC-06).
- Configuration parameter semantics (governed by DOC-10).
- Security architecture beyond repository boundaries (governed by DOC-09).
- Deployment procedures or runbooks (governed by DOC-08).
- Testing strategy or test case specifications (governed by DOC-07).

---

## 1. Repository Identity

### 1.1 Repository Properties

| Property | Value |
|---|---|
| Repository name | `CORTEX` |
| Primary language | **Rust** |
| Edition | 2021 |
| Minimum Rust version | 1.75 |
| Build system | **Cargo** |
| Package name | `cortex` |
| Package version | 1.0.0 |
| Binary output | `cortex` (single binary) |
| State file | `cortex.cx` (BLAKE3-integrity, binary format) |
| Configuration file | `cortex.toml` (TOML format) |

### 1.2 Repository Classification

| Attribute | Value |
|---|---|
| Language ecosystem | Rust / Cargo |
| License | Proprietary (all rights reserved) |
| Version control | Git |
| Branching model | Mainline development |
| Commit convention | Conventional commits (`feat:`, `fix:`, `improve:`, `BREAKING CHANGE:`) |

---

## 2. Repository Tree

### 2.1 Authoritative Repository Layout

The following tree represents the **repository structure** as the Final Architectural Baseline.

```
CORTEX/
│
├── README.md                             # Project overview, quickstart, links
├── LICENSE                               # License file
├── CHANGELOG.md                          # Version changelog (conventional commits)
├── VERSION                               # Single-source version string
├── Cargo.toml                            # Package manifest — dependencies, profiles, metadata
├── Cargo.lock                            # Locked dependency graph — reproducible builds
├── rust-toolchain.toml                   # Pinned Rust toolchain (DOC-06 §1.2)
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
│   ├── DOC-11-Repository-Architecture.md # Repository Architecture (this document mirror)
│   │
│   ├── architecture/                     # Architecture-specific documentation
│   │   ├── final-architectural-baseline.md   # This document
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
├── src/                                  # Source code root — all production modules
│   ├── main.rs                           # Entry point — CLI dispatch, boot orchestration
│   ├── cortex.rs                         # Global orchestration — CortexRuntime construction
│   ├── config.rs                         # Configuration parsing — TOML deserialization, validation
│   ├── error.rs                          # Error taxonomy — CortexError enum, recovery codes
│   │
│   ├── types/                            # Core Type System — all shared types, IDs, scalars
│   │   ├── mod.rs                        # Module re-exports — public type surface
│   │   ├── ids.rs                        # ID types — 22 ID types via macro
│   │   ├── scalars.rs                    # Scalar type — f32 wrapper with NaN/Infinity guard
│   │   ├── state.rs                      # CortexState — top-level state container, sub-states
│   │   ├── observation.rs                # Observation, Experience — input types
│   │   ├── evidence.rs                   # Evidence, Provenance — knowledge provenance types
│   │   └── common.rs                     # Shared types — Timestamp, Duration, enums, utilities
│   │
│   ├── language/                         # Language Core (CLX) — tokenization, encoding, prediction
│   │   ├── mod.rs                        # LanguageCore trait — orchestration interface
│   │   ├── tokenizer.rs                  # Symbol & token encoding — input tokenization
│   │   ├── vocabulary.rs                 # Dynamic vocabulary — symbol management, growth
│   │   ├── syntax.rs                     # Syntax representation — structural patterns
│   │   ├── semantics.rs                  # Semantic representation — meaning encoding
│   │   ├── language_model.rs             # Language prediction — next-token prediction
│   │   ├── decoder.rs                    # Language realization — symbol-to-text output
│   │   └── context.rs                    # Context model — contextual representation
│   │
│   ├── neural/                           # Neural Core (CNS) — cell/column computation, plasticity
│   │   ├── mod.rs                        # NeuralCore trait — orchestration interface
│   │   ├── cell.rs                       # Cell computation — activation, inhibition, adaptation
│   │   ├── column.rs                     # Column computation — competition, activation
│   │   ├── field.rs                      # Neural field — spatial activation patterns
│   │   ├── temporal.rs                   # Temporal representation — time-based encoding
│   │   └── plasticity.rs                # Plasticity rules — weight update, stability guard
│   │
│   ├── memory/                           # Memory System — 5 subsystems, retrieval, consolidation
│   │   ├── mod.rs                        # MemorySystem trait — orchestration interface
│   │   ├── working.rs                    # Working memory — active context buffer
│   │   ├── episodic.rs                   # Episodic memory — experience storage
│   │   ├── semantic.rs                   # Semantic memory — knowledge storage
│   │   ├── procedural.rs                 # Procedural memory — skill/rule storage
│   │   ├── associative.rs                # Associative memory — cross-reference links
│   │   ├── retrieval.rs                  # Memory retrieval — relevance scoring, search
│   │   └── consolidation.rs              # Memory consolidation — long-term integration
│   │
│   ├── world/                            # World Model — entities, transitions, simulation
│   │   ├── mod.rs                        # WorldModelInterface trait — orchestration interface
│   │   ├── entity.rs                     # Entity management — creation, update, lifecycle
│   │   ├── transition.rs                 # Transition model — state change tracking
│   │   ├── causal.rs                     # Causal hypotheses — cause-effect modeling
│   │   └── simulation.rs                # World simulation — trajectory prediction
│   │
│   ├── reasoning/                        # Reasoning Engine — hypothesis, evidence, contradiction
│   │   ├── mod.rs                        # ReasoningEngine trait — orchestration interface
│   │   ├── hypothesis.rs                 # Hypothesis generation & evaluation
│   │   ├── evidence.rs                   # Evidence evaluation — support/refutation scoring
│   │   └── contradiction.rs              # Contradiction detection — conflict resolution
│   │
│   ├── planning/                         # Planning Engine — goal-directed plan generation
│   │   ├── mod.rs                        # PlanningEngine trait — orchestration interface
│   │   ├── plan.rs                       # Plan representation — ranking, selection
│   │   └── risk.rs                       # Risk evaluation — plan risk scoring
│   │
│   ├── verification/                     # Verification Engine — claim verification, confidence
│   │   ├── mod.rs                        # VerificationEngine trait — orchestration interface
│   │   └── confidence.rs                 # Confidence model — claim confidence scoring
│   │
│   ├── learning/                         # Continual Learning — signals, attribution, replay
│   │   ├── mod.rs                        # LearningSystem trait — orchestration interface
│   │   ├── signal.rs                     # Learning signal generation — error detection
│   │   ├── attribution.rs                # Error attribution — source identification
│   │   ├── replay.rs                     # Experience replay — priority-based sampling
│   │   └── stability.rs                 # Learning stability guards — plasticity bounds
│   │
│   ├── self_model/                       # Self Model — capability estimation, health
│   │   ├── mod.rs                        # SelfModelInterface trait — orchestration interface
│   │   └── capability.rs                # Capability estimation — self-assessment
│   │
│   ├── policy/                           # Policy / Risk Gate — security boundary
│   │   ├── mod.rs                        # PolicyEngine trait — orchestration interface
│   │   ├── risk.rs                       # Risk estimation — 5-factor risk scoring
│   │   └── gate.rs                       # Gate pipeline — operation approval/rejection
│   │
│   ├── internet/                         # Internet Interface — fetch, parse, provenance
│   │   ├── mod.rs                        # InternetInterface trait — orchestration interface
│   │   ├── fetch.rs                      # Network operations — HTTP fetch with policy gate
│   │   └── parse.rs                      # Content extraction — HTML/text parsing
│   │
│   ├── persistence/                      # Persistence Engine — .cx format, checkpoints
│   │   ├── mod.rs                        # PersistenceEngine trait — orchestration interface
│   │   ├── format.rs                     # .cx format handling — binary layout, serialization
│   │   ├── checkpoint.rs                 # Checkpoint lifecycle — creation, validation, recovery
│   │   └── migration.rs                 # State migration — version upgrades, schema evolution
│   │
│   ├── api/                              # Embedded API — HTTP server, routes, handlers
│   │   ├── mod.rs                        # API server orchestration — startup, shutdown
│   │   ├── routes.rs                     # Route definitions — endpoint mapping
│   │   ├── auth.rs                       # Authentication — Bearer token validation
│   │   └── handlers.rs                  # Request handlers — endpoint implementations
│   │
│   ├── cli/                              # CLI Layer — command parsing, dispatch
│   │   ├── mod.rs                        # CLI dispatch — argument parsing, subcommand routing
│   │   └── commands.rs                  # Command implementations — all CLI subcommands
│   │
│   ├── observability/                    # Observability — metrics, diagnostics
│   │   ├── mod.rs                        # Metrics & diagnostics — public interface
│   │   └── diagnostics.rs               # Diagnostic state — runtime health data
│   │
│   └── runtime.rs                        # Runtime lifecycle — state machine, boot, shutdown
│
├── tests/                                # Integration tests — cross-module validation
│   ├── cognitive_pipeline.rs             # Full cognitive loop integration test
│   ├── persistence_roundtrip.rs          # Save/load/state verification roundtrip
│   ├── learning_stability.rs             # Learning stability guard validation
│   ├── security_policy.rs                # Policy gate enforcement tests
│   ├── api_endpoints.rs                  # API endpoint contract tests
│   └── corruption_recovery.rs            # State corruption detection and recovery
│
├── benches/                              # Performance benchmarks — latency, throughput
│   ├── cognitive_loop.rs                 # Cognitive loop latency benchmark
│   ├── memory_retrieval.rs              # Memory retrieval throughput benchmark
│   └── persistence.rs                    # Persistence I/O benchmark
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

## 3. Source Architecture

### 3.1 Rust Package Structure

The repository uses a `src/` layout with Rust modules organized by architectural layer.

| Module | Responsibility | Layer | Governing Doc |
|---|---|---|---|
| `types/` | Core types, IDs, scalars, common definitions | Infrastructure | DOC-03 |
| `language/` | Language Core (CLX) — tokenization, encoding, prediction | Cognitive Pipeline | DOC-02 |
| `neural/` | Neural Core (CNS) — cell, column, field, temporal, plasticity | Cognitive Pipeline | DOC-02 |
| `memory/` | Memory System — working, episodic, semantic, procedural, associative | Cognitive Pipeline | DOC-02 |
| `world/` | World Model — entities, transitions, simulation | Cognitive Pipeline | DOC-02 |
| `reasoning/` | Reasoning Engine — hypothesis, evidence, contradiction | Cognitive Pipeline | DOC-02 |
| `planning/` | Planning Engine — plan, risk | Cognitive Pipeline | DOC-02 |
| `verification/` | Verification Engine — confidence | Cognitive Pipeline | DOC-02 |
| `learning/` | Continual Learning — signals, attribution, replay, stability | Governance | DOC-02 |
| `self_model/` | Self Model — capability estimation, health | Governance | DOC-02 |
| `policy/` | Policy / Risk Gate — security boundary | Governance | DOC-02 |
| `internet/` | Internet Interface — fetch, parse | Infrastructure | DOC-02 |
| `persistence/` | Persistence Engine — `.cx` format, checkpoints | Infrastructure | DOC-02 |
| `api/` | Embedded API — HTTP server, routes, handlers | Infrastructure | DOC-05 |
| `cli/` | CLI — command parsing, dispatch | Infrastructure | DOC-05 |
| `observability/` | Observability — metrics, diagnostics | Infrastructure | DOC-02 |

### 3.2 Security Module Boundary

The `security/` package is the designated boundary for all cryptographic operations:

| Sub-module | Function | Algorithm |
|---|---|---|
| Integrity hashing | State file integrity | BLAKE3-256 |
| Checksum verification | `.cx` section and file validation | BLAKE3-256 |
| Key management | API token handling | Constant-time comparison |

> **Security Boundary Rule:** All hashing and integrity operations reside within the security boundary. No other module performs cryptographic operations directly.

> **BLAKE3 Contract:** BLAKE3 is used exclusively for integrity hashing. It is NOT used for encryption, key derivation, or password hashing. There is no AES-256 or symmetric encryption in the architecture.

### 3.3 Dependency Flow Rules

Dependency flow follows the rules defined in DOC-02 §8:

| Allowed Direction | From | To |
|---|---|---|
| 1 | `types/` | (no internal deps — leaf module) |
| 2 | Any cognitive module | `types/` |
| 3 | `policy/` | `types/` |
| 4 | `persistence/` | `types/` |
| 5 | `api/` | `types/`, `cli/` |
| 6 | `cli/` | `types/` |
| 7 | `main.rs` | All modules (orchestration only) |
| 8 | `cortex.rs` | All modules (orchestration only) |
| 9 | `config.rs` | `types/` |
| 10 | `error.rs` | `types/` |

| Forbidden Direction | Reason |
|---|---|
| `types/` → any module | Leaf module — no upward dependencies |
| `learning/` → `policy/` | Learning must not bypass policy gate |
| `memory/` → `api/` | Memory has no awareness of API layer |
| `neural/` → `memory/` | Neural core is independent of memory subsystem |
| `language/` → `neural/` | Language core is independent of neural core |
| Any cognitive module → `main.rs` or `cortex.rs` | No circular dependency to orchestrator |

---

## 4. Documentation Architecture

### 4.1 Documentation Root

All documentation resides under `docs/`. The DOC-01 through DOC-11 series constitutes the complete architectural specification.

| DOC | Title | Classification | Parent |
|---|---|---|---|
| DOC-01 | Technical Specification | System Contract | (root) |
| DOC-02 | Software Design Specification | Architecture Contract | DOC-01 |
| DOC-03 | Data & State Specification | Data Contract | DOC-02 |
| DOC-04 | Algorithm Specification | Computational Behavior Contract | DOC-03 |
| DOC-05 | API & CLI Specification | Interface Contract | DOC-04 |
| DOC-06 | Build & Release Specification | Build Contract | DOC-01 |
| DOC-07 | Testing & Validation Specification | Quality Contract | DOC-04 |
| DOC-08 | Deployment & Operations Specification | Operations Contract | DOC-01 |
| DOC-09 | Security & Privacy Specification | Security Contract | DOC-01 |
| DOC-10 | Configuration Reference | Configuration Contract | DOC-01 |
| DOC-11 | Repository Architecture | Repository Contract | DOC-02 |

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

## 5. Test Architecture

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
implementation (src/<module>/)
    ↓
test category (tests/<category>/)
    ↓
acceptance criterion
```

Every test category SHALL be traceable to DOC-01 requirements through DOC-02 design specifications.

---

## 6. Schema Architecture

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

## 7. Configuration Architecture

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
| File location | `config/` directory + `cortex.toml` | DOC-FINAL §7 |
| Loading boundary | Application loads from `config/` + `cortex.toml` | DOC-10 §1.2 |
| Environment separation | Defaults + environment overlay | DOC-FINAL §7 |

---

## 8. Repository Invariants

### 8.1 Final Baseline Invariants

| # | Invariant | Enforcement | Severity |
|---|---|---|---|
| FAB-R-001 | Repository SHALL conform to the approved layout | Structural check | Critical |
| FAB-R-002 | Rust source SHALL reside under `src/` | Layout check | Critical |
| FAB-R-003 | Documentation SHALL reside under `docs/` | Layout check | High |
| FAB-R-004 | Schemas SHALL reside under `schemas/` | Layout check | High |
| FAB-R-005 | Tests SHALL reside under `tests/` | Layout check | Critical |
| FAB-R-006 | Deployment artifacts SHALL reside under `deployment/` | Layout check | High |
| FAB-R-007 | Build/test/audit/release scripts SHALL reside under `scripts/` | Layout check | High |
| FAB-R-008 | Migration artifacts SHALL reside under `migrations/` | Layout check | High |
| FAB-R-009 | Security implementation SHALL remain within the security boundary | Layout check | Critical |
| FAB-R-010 | BLAKE3 integrity implementation SHALL remain within the security boundary | Code boundary check | Critical |
| FAB-R-011 | Repository SHALL NOT depend on external AI models, databases, or agent frameworks | Dependency check | Critical |
| FAB-R-012 | Every target directory SHALL have an explicitly defined responsibility | Documentation check | High |
| FAB-R-013 | Every implementation module SHALL be traceable to the architecture specification | Traceability check | High |
| FAB-R-014 | Every requirement SHALL be traceable to implementation and tests | Traceability check | Critical |
| FAB-R-015 | All documents in the DOC series SHALL have synchronized versions | Version check | High |

### 8.2 Invariant Severity Classification

| Severity | Meaning | Action on Violation |
|---|---|---|
| Critical | System integrity at risk | Block merge, require immediate fix |
| High | Specification non-conformance | Block merge, require fix before release |
| Medium | Quality degradation | Warning, allow with technical debt ticket |
| Low | Style/convention deviation | Informational, address in next cycle |

---

## 9. Definition of Done

### 9.1 Baseline Conformance Criteria

The Final Architectural Baseline is considered **conformant** when ALL of the following criteria are satisfied:

| # | Criterion | Validation |
|---|---|---|
| DOD-01 | Root structure exists (`README.md`, `LICENSE`, `CHANGELOG.md`, `VERSION`, `Cargo.toml`, `Makefile`, `.gitignore`, `.editorconfig`) | File existence check |
| DOD-02 | Rust source exists under `src/` with `main.rs` entry point | Package check |
| DOD-03 | Documentation structure exists under `docs/` with DOC-01 through DOC-11 | Directory check |
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
| DOD-15 | 71 source modules exist under `src/` | Module count check |
| DOD-16 | BLAKE3 integrity architecture is consistent across all modules | Code boundary check |
| DOD-17 | Cross-document traceability passes | Traceability audit |
| DOD-18 | Repository consistency audit passes | Audit report |

---

## 10. Boundary Summary

### 10.1 Architectural Boundaries

| Boundary | Value |
|---|---|
| Source code | `src/*.rs`, `src/<module>/` |
| Documentation | `docs/DOC-NN-*.md` + supplementary |
| Tests | `tests/<category>/` (hierarchical) |
| Schemas | `schemas/` |
| Configuration | `cortex.toml` + `config/` profiles |
| Build | `Cargo.toml` |
| Scripts | `scripts/` |
| Deployment | `deployment/` |
| Benchmarks | `benchmarks/<category>/` |
| Examples | `examples/` |
| Migrations | `migrations/` |
| Artifacts | `artifacts/` (gitignored) |
| CI/CD | `.github/workflows/` |

### 10.2 Security Boundaries

| Boundary | Rule |
|---|---|
| Cryptographic operations | All hashing/integrity within security boundary |
| BLAKE3 usage | Integrity hashing only — NOT encryption |
| No AES-256 | Architecture does not include symmetric encryption |
| Key management | Security module only |
| Policy isolation | Policy module is architecturally separate from learned state |

### 10.3 Dependency Boundaries

| Boundary | Value |
|---|---|
| Package manifest | `Cargo.toml` |
| Lock file | `Cargo.lock` |
| Dependency count | ≤15 production crates |
| Prohibited categories | Databases, web frameworks, ML libs, GPU |
| Integrity library | `blake3` crate |
| Serialization | `bincode`, `serde_json`, `toml` |
| Compression | `zstd` crate |
| Async runtime | `tokio` |
| HTTP server | `hyper` |
| CLI | `clap` |

---

*End of Document — CORTEX Final Architectural Baseline v1.1.0*
*For repository architecture details, see CORTEX-DOC-11 Repository Architecture & Structure*
