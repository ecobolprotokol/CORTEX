# CORTEX — 11 Repository Architecture & Structure

---

## Document Control

| Property | Value |
|---|---|
| **Document ID** | CORTEX-DOC-11 |
| **Title** | Repository Architecture & Structure |
| **Version** | 1.1.0 |
| **Status** | Final Architectural Baseline |
| **Classification** | Repository Contract |
| **Scope** | Current repository tree, directory/file responsibilities, source organization, naming conventions, dependency boundaries, structural invariants as of the current repository state |
| **Parent Document** | CORTEX-DOC-02 Software Design Specification |
| **Effective Date** | 2026-08-13 |
| **Review Cycle** | Per architecture version transition |

> **IMPORTANT:** This document describes the **REPOSITORY ARCHITECTURE** as the Final Architectural Baseline — the authoritative structure of the repository as designed and implemented.

### Revision History

| Version | Date | Author | Description |
|---|---|---|---|
| 1.0.0 | 2026-08-13 | CORTEX Architecture | Initial final baseline |
| 1.1.0 | 2026-08-13 | CORTEX Architecture | Establish as Final Architectural Baseline |

### Approval

| Role | Signature | Date |
|---|---|---|
| System Architect | _____________ | _____________ |
| Repository Maintainer | _____________ | _____________ |

### Document Purpose

This document defines **the repository structure of CORTEX** as the Final Architectural Baseline. It constitutes the authoritative record of the repository layout: every directory, every file, every naming convention, every boundary, and every structural invariant.

### Document Scope

This specification covers:

- Repository tree with every directory and file annotated with assembly status.
- Directory responsibilities and ownership boundaries.
- Source-code organization and module layout.
- Documentation structure and naming conventions.
- Test structure and organization.
- Configuration file structure.
- Dependency boundaries at the repository level.
- Repository invariants and structural rules.
- Traceability to other CORTEX documents.

This specification does NOT cover:

- Internal module design or algorithm specification (governed by DOC-02, DOC-03, DOC-04).
- Build pipeline stages or CI gate definitions (governed by DOC-06).
- Configuration parameter semantics or validation rules (governed by DOC-10).
- Security architecture beyond repository-level boundaries (governed by DOC-09).
- Deployment procedures or operational runbooks (governed by DOC-08).
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
| Commit convention | Conventional commits |

---

## 2. Repository Tree

### 2.1 Repository Layout

The following tree represents the **repository structure** as the Final Architectural Baseline. Every entry is annotated with its purpose, governing document, and assembly status.

```
CORTEX/
│
├── Cargo.toml                          # Package manifest — dependencies, profiles, metadata
├── Cargo.lock                          # Locked dependency graph — reproducible builds
├── rust-toolchain.toml                 # Pinned Rust toolchain (stable, rustfmt, clippy)
├── .gitignore                          # Git ignore rules — build artifacts, state files, temp files
├── .editorconfig                       # Editor configuration — indentation, charset
├── README.md                           # Project overview, quickstart, links
├── LICENSE                             # License file (Proprietary)
├── CHANGELOG.md                        # Version changelog (conventional commits)
├── VERSION                             # Single-source version string
│
├── docs/                               # Documentation — all DOC files and architecture
│   ├── DOC-01-Requirements.md          # Technical Specification (System Contract)
│   ├── DOC-02-Architecture.md          # Software Design Specification (Architecture Contract)
│   ├── DOC-03-Data-Architecture.md     # Data & State Specification (Data Contract)
│   ├── DOC-04-Algorithms.md            # Algorithm Specification (Computational Behavior Contract)
│   ├── DOC-05-API-CLI.md               # API & CLI Specification (Interface Contract)
│   ├── DOC-06-Build-Release.md         # Build & Release Specification (Build Contract)
│   ├── DOC-07-Testing-Validation.md    # Testing & Validation Specification (Quality Contract)
│   ├── DOC-08-Deployment-Operations.md # Deployment & Operations Specification (Operations Contract)
│   ├── DOC-09-Security-Privacy.md      # Security & Privacy Specification (Security Contract)
│   ├── DOC-10-Configuration-Reference.md # Configuration Reference (Configuration Contract)
│   ├── DOC-11-Repository-Architecture.md # Repository Architecture & Structure (Repository Contract)
│   │
│   ├── architecture/                   # Architecture-specific documentation
│   │   ├── final-architectural-baseline.md # Final Architectural Baseline
│   │   ├── consistency-audit.md        # Consistency audit results
│   │   ├── decision-records/           # Architecture Decision Records (ADRs)
│   │   └── diagrams/                   # Architecture diagrams
│   │
│   ├── contracts/                      # Interface contracts
│   │   ├── api/                        # API contract definitions
│   │   ├── cli/                        # CLI contract definitions
│   │   ├── persistence/                # Persistence format contracts
│   │   └── configuration/              # Configuration contracts
│   │
│   └── traceability/                   # Traceability documentation
│       ├── requirements-to-design.md   # Requirement → Design mapping
│       ├── requirements-to-tests.md    # Requirement → Test mapping
│       └── cross-document-matrix.md    # Cross-document traceability matrix
│
├── src/                                # Source code — all production modules
│   ├── main.rs                         # Entry point — CLI dispatch, boot orchestration
│   ├── cortex.rs                       # Global orchestration — CortexRuntime construction
│   ├── config.rs                       # Configuration parsing — TOML deserialization, validation
│   ├── error.rs                        # Error taxonomy — CortexError enum, recovery codes
│   ├── runtime.rs                      # Runtime lifecycle — state machine, boot, shutdown
│   │
│   ├── types/                          # Core Type System — all shared types, IDs, scalars
│   │   ├── mod.rs                      # Module re-exports — public type surface
│   │   ├── ids.rs                      # ID types — 22 ID types via macro
│   │   ├── scalars.rs                  # Scalar type — f32 wrapper with NaN/Infinity guard
│   │   ├── state.rs                    # CortexState — top-level state container, sub-states
│   │   ├── observation.rs              # Observation, Experience — input types
│   │   ├── evidence.rs                 # Evidence, Provenance — knowledge provenance types
│   │   └── common.rs                   # Shared types — Timestamp, Duration, enums, utilities
│   │
│   ├── language/                       # Language Core (CLX) — tokenization, encoding, prediction
│   │   ├── mod.rs                      # LanguageCore trait — orchestration interface
│   │   ├── tokenizer.rs               # Symbol & token encoding — input tokenization
│   │   ├── vocabulary.rs               # Dynamic vocabulary — symbol management, growth
│   │   ├── syntax.rs                   # Syntax representation — structural patterns
│   │   ├── semantics.rs                # Semantic representation — meaning encoding
│   │   ├── language_model.rs           # Language prediction — next-token prediction
│   │   ├── decoder.rs                  # Language realization — symbol-to-text output
│   │   └── context.rs                  # Context model — contextual representation
│   │
│   ├── neural/                         # Neural Core (CNS) — cell/column computation, plasticity
│   │   ├── mod.rs                      # NeuralCore trait — orchestration interface
│   │   ├── cell.rs                     # Cell computation — activation, inhibition, adaptation
│   │   ├── column.rs                   # Column computation — competition, activation
│   │   ├── field.rs                    # Neural field — spatial activation patterns
│   │   ├── temporal.rs                 # Temporal representation — time-based encoding
│   │   └── plasticity.rs              # Plasticity rules — weight update, stability guard
│   │
│   ├── memory/                         # Memory System — 5 subsystems, retrieval, consolidation
│   │   ├── mod.rs                      # MemorySystem trait — orchestration interface
│   │   ├── working.rs                  # Working memory — active context buffer
│   │   ├── episodic.rs                 # Episodic memory — experience storage
│   │   ├── semantic.rs                 # Semantic memory — knowledge storage
│   │   ├── procedural.rs               # Procedural memory — skill/rule storage
│   │   ├── associative.rs              # Associative memory — cross-reference links
│   │   ├── retrieval.rs                # Memory retrieval — relevance scoring, search
│   │   └── consolidation.rs            # Memory consolidation — long-term integration
│   │
│   ├── world/                          # World Model — entities, transitions, simulation
│   │   ├── mod.rs                      # WorldModelInterface trait — orchestration interface
│   │   ├── entity.rs                   # Entity management — creation, update, lifecycle
│   │   ├── transition.rs               # Transition model — state change tracking
│   │   ├── causal.rs                   # Causal hypotheses — cause-effect modeling
│   │   └── simulation.rs              # World simulation — trajectory prediction
│   │
│   ├── reasoning/                      # Reasoning Engine — hypothesis, evidence, contradiction
│   │   ├── mod.rs                      # ReasoningEngine trait — orchestration interface
│   │   ├── hypothesis.rs               # Hypothesis generation & evaluation
│   │   ├── evidence.rs                 # Evidence evaluation — support/refutation scoring
│   │   └── contradiction.rs            # Contradiction detection — conflict resolution
│   │
│   ├── planning/                       # Planning Engine — goal-directed plan generation
│   │   ├── mod.rs                      # PlanningEngine trait — orchestration interface
│   │   ├── plan.rs                     # Plan representation — ranking, selection
│   │   └── risk.rs                     # Risk evaluation — plan risk scoring
│   │
│   ├── verification/                   # Verification Engine — claim verification, confidence
│   │   ├── mod.rs                      # VerificationEngine trait — orchestration interface
│   │   └── confidence.rs              # Confidence model — claim confidence scoring
│   │
│   ├── learning/                       # Continual Learning — signals, attribution, replay
│   │   ├── mod.rs                      # LearningSystem trait — orchestration interface
│   │   ├── signal.rs                   # Learning signal generation — error detection
│   │   ├── attribution.rs              # Error attribution — source identification
│   │   ├── replay.rs                   # Experience replay — priority-based sampling
│   │   └── stability.rs               # Learning stability guards — plasticity bounds
│   │
│   ├── self_model/                     # Self Model — capability estimation, health
│   │   ├── mod.rs                      # SelfModelInterface trait — orchestration interface
│   │   └── capability.rs              # Capability estimation — self-assessment
│   │
│   ├── policy/                         # Policy / Risk Gate — security boundary
│   │   ├── mod.rs                      # PolicyEngine trait — orchestration interface
│   │   ├── risk.rs                     # Risk estimation — 5-factor risk scoring
│   │   └── gate.rs                     # Gate pipeline — operation approval/rejection
│   │
│   ├── internet/                       # Internet Interface — fetch, parse, provenance
│   │   ├── mod.rs                      # InternetInterface trait — orchestration interface
│   │   ├── fetch.rs                    # Network operations — HTTP fetch with policy gate
│   │   └── parse.rs                    # Content extraction — HTML/text parsing
│   │
│   ├── persistence/                    # Persistence Engine — .cx format, checkpoints
│   │   ├── mod.rs                      # PersistenceEngine trait — orchestration interface
│   │   ├── format.rs                   # .cx format handling — binary layout, serialization
│   │   ├── checkpoint.rs               # Checkpoint lifecycle — creation, validation, recovery
│   │   └── migration.rs               # State migration — version upgrades, schema evolution
│   │
│   ├── api/                            # Embedded API — HTTP server, routes, handlers
│   │   ├── mod.rs                      # API server orchestration — startup, shutdown
│   │   ├── routes.rs                   # Route definitions — endpoint mapping
│   │   ├── auth.rs                     # Authentication — Bearer token validation
│   │   └── handlers.rs                # Request handlers — endpoint implementations
│   │
│   ├── cli/                            # CLI Layer — command parsing, dispatch
│   │   ├── mod.rs                      # CLI dispatch — argument parsing, subcommand routing
│   │   └── commands.rs                # Command implementations — all CLI subcommands
│   │
│   └── observability/                  # Observability — metrics, diagnostics
│       ├── mod.rs                      # Metrics & diagnostics — public interface
│       └── diagnostics.rs             # Diagnostic state — runtime health data
│
├── tests/                              # Integration tests — cross-module validation
│   └── cognitive_loop.rs               # Full cognitive loop integration test (stub)
│
├── benches/                            # Performance benchmarks — latency, throughput
│   └── cognitive_loop.rs               # Cognitive loop latency benchmark (stub)
│
├── schemas/                            # Schema definitions
│   ├── cx/                             # .cx file format schemas
│   │   ├── format.md                   # Binary format specification
│   │   ├── sections/                   # Per-section schema definitions
│   │   └── schema.json                 # Machine-readable schema
│   ├── api/                            # API schemas — request/response
│   └── configuration/                  # Configuration schemas — validation rules
│
├── config/                             # Configuration profiles
│   ├── defaults/                       # Default configuration values
│   ├── development/                    # Development environment config
│   ├── testing/                        # Testing environment config
│   └── production/                     # Production environment config
│
├── scripts/                            # Development and operations scripts
│   ├── build/                          # Build scripts — compilation, packaging
│   ├── test/                           # Test scripts — runner orchestration
│   ├── audit/                          # Audit scripts — security, license, dependency
│   ├── migration/                      # Migration scripts — schema/state evolution
│   └── release/                        # Release scripts — tagging, packaging, publishing
│
├── deployment/                         # Deployment configurations
│   ├── docker/                         # Docker — Dockerfile, docker-compose
│   ├── kubernetes/                     # Kubernetes — manifests, Helm charts
│   ├── systemd/                        # systemd — service files
│   └── reverse-proxy/                  # Reverse proxy — nginx, caddy configs
│
├── examples/                           # Usage examples
│   ├── basic/                          # Basic usage examples
│   ├── api/                            # API usage examples
│   ├── cli/                            # CLI usage examples
│   └── persistence/                    # Persistence examples
│
├── migrations/                         # Schema/state migration artifacts
│   └── v1/                             # Version 1 migrations
│
├── artifacts/                          # Generated artifacts (gitignored)
│   ├── builds/                         # Build outputs
│   ├── test-reports/                   # Test report outputs
│   └── audit-reports/                  # Audit report outputs
│
└── .github/                            # GitHub configuration
    ├── workflows/
    │   ├── ci.yml                      # CI pipeline — lint, type-check, test
    │   ├── test.yml                    # Test pipeline — full test suite
    │   ├── security.yml                # Security scanning — audit, dependency check
    │   └── release.yml                 # Release pipeline — build, package, publish
    ├── ISSUE_TEMPLATE/                 # Issue templates
    └── pull_request_template.md        # PR template
```

### 2.2 Repository Layout Status

The following table documents the repository structure. Items marked `ASSEMBLED` exist in the repository. Items marked `PLANNED` are defined by specification but not yet implemented.

| Path | Status | Governing Doc |
|---|---|---|
| `Cargo.toml` | ASSEMBLED | DOC-02 §5.2, DOC-06 §1 |
| `Cargo.lock` | ASSEMBLED | DOC-06 §6.3 |
| `.gitignore` | ASSEMBLED | DOC-11 §8 |
| `rust-toolchain.toml` | PLANNED | DOC-06 §1.2 |
| `cortex.toml` | PLANNED | DOC-10 §1 |
| `README.md` | PLANNED | DOC-11 §10.1 |
| `src/main.rs` | ASSEMBLED | DOC-02 §4.1 |
| `src/cortex.rs` | ASSEMBLED | DOC-02 §4.1 |
| `src/config.rs` | ASSEMBLED | DOC-02 §4.1 |
| `src/error.rs` | ASSEMBLED | DOC-02 §4.1 |
| `src/runtime.rs` | PLANNED | DOC-02 §4.1 |
| `src/types/` (7 files) | ASSEMBLED | DOC-03, DOC-02 §4.1 |
| `src/language/` (8 files) | ASSEMBLED | DOC-02 §4.1 |
| `src/neural/` (6 files) | ASSEMBLED | DOC-02 §4.1 |
| `src/memory/` (8 files) | ASSEMBLED | DOC-02 §4.1 |
| `src/world/` (5 files) | ASSEMBLED | DOC-02 §4.1 |
| `src/reasoning/` (4 files) | ASSEMBLED | DOC-02 §4.1 |
| `src/planning/` (3 files) | ASSEMBLED | DOC-02 §4.1 |
| `src/verification/` (2 files) | ASSEMBLED | DOC-02 §4.1 |
| `src/learning/` (5 files) | ASSEMBLED | DOC-02 §4.1 |
| `src/self_model/` (2 files) | ASSEMBLED | DOC-02 §4.1 |
| `src/policy/` (3 files) | ASSEMBLED | DOC-02 §4.1 |
| `src/internet/` (3 files) | ASSEMBLED | DOC-02 §4.1 |
| `src/persistence/` (4 files) | ASSEMBLED | DOC-02 §4.1 |
| `src/api/` (4 files) | ASSEMBLED | DOC-02 §4.1 |
| `src/cli/` (2 files) | ASSEMBLED | DOC-02 §4.1 |
| `src/observability/` (2 files) | ASSEMBLED | DOC-02 §4.1 |
| `tests/` (6 files) | PLANNED | DOC-07 §2 |
| `benches/` (3 files) | PLANNED | DOC-07 §6 |
| `docs/` | PLANNED | DOC-11 §10 |
| `.github/` (CI) | PLANNED | DOC-06 §3 |
| `Dockerfile` | NOT PLANNED | DOC-08 (optional) |

---

## 3. Directory Responsibilities

### 3.1 Root Directory

The root directory (`CORTEX/`) is the workspace root. It contains:

| Artifact | Purpose | Mutability |
|---|---|---|
| `Cargo.toml` | Package manifest — defines crate metadata, dependencies, profiles | Administrative |
| `Cargo.lock` | Locked dependency graph — ensures reproducible builds | Auto-generated |
| `.gitignore` | Git ignore rules — excludes build artifacts, state files, temp files | Administrative |
| `CORTEX-DOC-*.md` | Architecture documents — versioned specifications | Administrative |

**Invariant R-001:** The root directory SHALL contain exactly one `Cargo.toml` and one `Cargo.lock`.

**Invariant R-002:** No source code SHALL exist in the root directory. All production code lives under `src/`.

**Invariant R-003:** Documentation files SHALL follow the naming convention `CORTEX-DOC-NN.md` where `NN` is a zero-padded two-digit number.

### 3.2 `src/` Directory

The `src/` directory contains all production source code. It is organized into:

| Subdirectory | Responsibility | Module Count | Architectural Layer |
|---|---|---|---|
| Root files | Entry point, orchestration, config, error, runtime | 5 | Runtime |
| `types/` | Core type system — IDs, scalars, state, observation, evidence | 7 | Infrastructure |
| `language/` | Language Core — tokenization, vocabulary, syntax, semantics | 8 | Cognitive Pipeline |
| `neural/` | Neural Core — cell, column, field, temporal, plasticity | 6 | Cognitive Pipeline |
| `memory/` | Memory System — working, episodic, semantic, procedural, associative | 8 | Cognitive Pipeline |
| `world/` | World Model — entities, transitions, causal, simulation | 5 | Cognitive Pipeline |
| `reasoning/` | Reasoning Engine — hypothesis, evidence, contradiction | 4 | Cognitive Pipeline |
| `planning/` | Planning Engine — plan, risk | 3 | Cognitive Pipeline |
| `verification/` | Verification Engine — confidence | 2 | Cognitive Pipeline |
| `learning/` | Continual Learning — signal, attribution, replay, stability | 5 | Governance |
| `self_model/` | Self Model — capability estimation | 2 | Governance |
| `policy/` | Policy / Risk Gate — risk, gate | 3 | Governance |
| `internet/` | Internet Interface — fetch, parse | 3 | Infrastructure |
| `persistence/` | Persistence Engine — format, checkpoint, migration | 4 | Infrastructure |
| `api/` | Embedded API — routes, auth, handlers | 4 | Infrastructure |
| `cli/` | CLI Layer — dispatch, commands | 2 | Infrastructure |
| `observability/` | Observability — diagnostics | 2 | Infrastructure |
| **Total** | | **71** | |

**Invariant R-004:** Every module under `src/` SHALL have a corresponding `mod.rs` file that declares its public trait interface.

**Invariant R-005:** No module under `src/` SHALL exceed 800 lines of code. Modules exceeding this limit SHALL be refactored into sub-modules.

**Invariant R-006:** The total module count across all `src/` subdirectories SHALL remain exactly 71.

### 3.3 `tests/` Directory

The `tests/` directory contains integration tests. Each file tests a cross-module scenario.

| File | Scope | Governing Doc |
|---|---|---|
| `cognitive_pipeline.rs` | Full cognitive loop — language through reasoning | DOC-07 §2.2 |
| `persistence_roundtrip.rs` | State save/load/verify cycle | DOC-07 §2.3 |
| `learning_stability.rs` | Learning guard effectiveness | DOC-07 §2.4 |
| `security_policy.rs` | Policy gate enforcement | DOC-07 §2.5 |
| `api_endpoints.rs` | API contract compliance | DOC-07 §2.6 |
| `corruption_recovery.rs` | State corruption detection and recovery | DOC-07 §2.7 |

**Invariant R-007:** Integration test files SHALL use descriptive filenames matching the scenario they validate.

**Invariant R-008:** No integration test file SHALL test a single module in isolation. Cross-module interaction is the minimum scope.

### 3.4 `benches/` Directory

The `benches/` directory contains performance benchmarks using the `criterion` crate.

| File | Metric | Target |
|---|---|---|
| `cognitive_loop.rs` | End-to-end cognitive loop latency | <100ms per cycle |
| `memory_retrieval.rs` | Memory query throughput | >1000 queries/sec |
| `persistence.rs` | State save/load throughput | >10MB/s |

**Invariant R-009:** Every benchmark SHALL produce a deterministic result given identical input state.

### 3.5 `docs/` Directory

The `docs/` directory contains supplementary documentation not covered by the CORTEX-DOC series.

| Expected Content | Purpose |
|---|---|
| Architecture diagrams | Visual representations of system architecture |
| Migration guides | Step-by-step upgrade procedures |
| API examples | Request/response examples for API consumers |
| State file format diagrams | Binary layout visualizations |

**Invariant R-010:** Supplementary documentation SHALL NOT contradict or redefine anything in the CORTEX-DOC series. In case of conflict, the DOC series is authoritative.

---

## 4. Source-Code Organization

### 4.1 Module Structure Pattern

Every module under `src/` follows a consistent structure:

```
src/<module>/
├── mod.rs          # Public trait + orchestration + re-exports
├── <component>.rs  # Individual component implementation
└── ...
```

**Pattern rules:**

| Rule | Description |
|---|---|
| Trait declaration | `mod.rs` declares the primary trait interface |
| Re-exports | `mod.rs` re-exports public types from sub-modules |
| Implementation | Individual files implement trait methods |
| Naming | File names are lowercase, singular nouns (e.g., `cell.rs`, not `cells.rs`) |
| Test units | Unit tests are inline in each file (no separate `tests/` subdirectory per module) |

### 4.2 Root Source Files

| File | Responsibility | Key Types/Traits |
|---|---|---|
| `main.rs` | Entry point, CLI dispatch, boot orchestration | `main()` function |
| `cortex.rs` | CortexRuntime construction, global orchestration | `CortexRuntime` struct |
| `config.rs` | TOML parsing, validation, distribution | `Config` struct |
| `error.rs` | Error taxonomy, recovery codes | `CortexError` enum |
| `runtime.rs` | Runtime lifecycle, state machine, boot/shutdown | `Runtime` trait |

### 4.3 Dependency Flow Rules

Source-code dependency flow follows the rules defined in DOC-02 §6. Repository-level enforcement:

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

**Invariant R-011:** The dependency graph SHALL remain acyclic at the module level.

**Invariant R-012:** `types/` SHALL have zero internal dependencies — it is the foundational leaf module.

---

## 5. Documentation Structure

### 5.1 Document Series

The CORTEX-DOC series constitutes the complete architectural specification. Documents are organized hierarchically:

```
DOC-01 (Technical Specification) ← ROOT
├── DOC-02 (Software Design Specification)
│   ├── DOC-03 (Data & State Specification)
│   │   └── DOC-04 (Algorithm Specification)
│   │       ├── DOC-05 (API & CLI Specification)
│   │       └── DOC-07 (Testing & Validation Specification)
│   └── DOC-11 (Repository Architecture) ← THIS DOCUMENT
├── DOC-06 (Build & Release Specification)
├── DOC-08 (Deployment & Operations Specification)
├── DOC-09 (Security & Privacy Specification)
├── DOC-10 (Configuration Reference)
└── FINAL-BASELINE (Final Architectural Baseline)
```

### 5.2 Document Properties

| DOC | Title | Classification | Parent | Lines (approx) |
|---|---|---|---|---|
| DOC-01 | Technical Specification | System Contract | (root) | 1,414 |
| DOC-02 | Software Design Specification | Architecture Contract | DOC-01 | 3,753 |
| DOC-03 | Data & State Specification | Data Contract | DOC-02 | 3,550 |
| DOC-04 | Algorithm Specification | Computational Behavior Contract | DOC-03 | 3,500 |
| DOC-05 | API & CLI Specification | Interface Contract | DOC-04 | 2,959 |
| DOC-06 | Build & Release Specification | Build Contract | DOC-01 | 471 |
| DOC-07 | Testing & Validation Specification | Quality Contract | DOC-04 | 500 |
| DOC-08 | Deployment & Operations Specification | Operations Contract | DOC-01 | 441 |
| DOC-09 | Security & Privacy Specification | Security Contract | DOC-01 | 399 |
| DOC-10 | Configuration Reference | Configuration Contract | DOC-01 | 648 |
| DOC-11 | Repository Architecture | Repository Contract | DOC-02 | ~960 |
| FINAL-BASELINE | Final Architectural Baseline | Repository Contract | DOC-02 | ~750 |

### 5.3 Cross-Reference Conventions

All documents use the following conventions:

| Convention | Example | Meaning |
|---|---|---|
| Requirement ID | `FR-LANG-001` | Functional requirement — Language domain |
| Non-functional ID | `REL-001` | Non-functional requirement — Reliability |
| Acceptance criteria | `AC-DEP-001` | Acceptance criterion — Deployment |
| Section reference | `§13.2` | Section 13.2 of the referencing document |
| Document reference | `DOC-02` | CORTEX-DOC-02 Software Design Specification |
| Invariant ID | `INV-001` | State invariant (DOC-03) or `ALG-001` (DOC-04) |
| Repository invariant (current) | `R-001` | Current repository-level invariant (DOC-11, this document) |
| Repository invariant (target) | `FAB-R-001` | Final Architectural Baseline invariant (DOC-FINAL §9) |

### 5.4 Document Naming Convention

| Rule | Pattern | Example |
|---|---|---|
| File name | `CORTEX-DOC-NN.md` | `CORTEX-DOC-11.md` |
| Document ID | `CORTEX-DOC-NN` | `CORTEX-DOC-11` |
| Section numbering | Hierarchical decimal | `§3.2.1` |
| Version format | `MAJOR.MINOR.PATCH` | `1.1.0` |
| Date format | ISO 8601 | `2026-08-13` |

**Invariant R-013:** All document versions across the DOC series SHALL be synchronized. No document SHALL have a version mismatch with any other.

**Invariant R-014:** Every document SHALL contain a traceability matrix mapping its coverage back to DOC-01 requirements.

---

## 6. Configuration File Structure

### 6.1 Cargo.toml

The `Cargo.toml` defines the package manifest. Its structure is governed by DOC-02 §5.2 and DOC-06 §1.

| Section | Purpose | Mutability |
|---|---|---|
| `[package]` | Crate metadata — name, version, edition, description | Administrative |
| `[dependencies]` | Production dependencies — 14 crates | Administrative |
| `[dev-dependencies]` | Development dependencies — 1 crate | Administrative |
| `[profile.release]` | Release build profile — opt-level, LTO, strip | Administrative |

**Current dependency inventory:**

| Category | Crates |
|---|---|
| Serialization | `serde`, `serde_json`, `bincode` |
| Compression | `zstd` |
| Cryptography | `blake3` |
| UUID | `uuid` |
| Async runtime | `tokio` |
| HTTP server | `hyper`, `hyper-util`, `http-body-util` |
| CLI | `clap` |
| TOML parsing | `toml` |
| Logging | `tracing`, `tracing-subscriber` |
| Dev | `tempfile` |

### 6.2 .gitignore

The `.gitignore` excludes:

| Pattern | Reason | Governing Doc |
|---|---|---|
| `/target` | Build artifacts | DOC-06 §8 |
| `cortex.cx` | Runtime state file | DOC-03 §23, DOC-08 §3 |
| `cortex.cx.tmp` | Temporary state during atomic writes | DOC-03 §23 |
| `checkpoints/` | Checkpoint directory | DOC-08 §5 |
| `*.bak` | Backup files | DOC-08 §5 |

**Invariant R-015:** The `.gitignore` SHALL exclude all build artifacts, runtime state files, temporary files, and backup files. No derived artifact SHALL be committed to version control.

### 6.3 rust-toolchain.toml (PLANNED)

When implemented, this file pins the Rust toolchain version and components. Defined by DOC-06 §1.2.

```toml
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
targets = ["x86_64-unknown-linux-gnu"]
```

### 6.4 cortex.toml (PLANNED)

When implemented, this is the runtime configuration file. Fully defined by DOC-10. Located at the working directory or specified via `--config` / `CORTEX_CONFIG`.

---

## 7. Test Structure

### 7.1 Test Organization

| Test Category | Location | Minimum Count | Governing Doc |
|---|---|---|---|
| Unit tests | Inline in `src/**/*.rs` | >500 | DOC-07 §2.1 |
| Integration tests | `tests/*.rs` | >50 | DOC-07 §2.2 |
| Regression tests | `tests/regression/` | >20 | DOC-07 §3 |
| Security tests | `tests/security/` | >30 | DOC-07 §4 |
| Stress tests | `tests/stress/` | >10 | DOC-07 §5 |
| Benchmarks | `benches/*.rs` | >10 | DOC-07 §6 |

### 7.2 Test Naming Conventions

| Convention | Pattern | Example |
|---|---|---|
| Unit test module | `#[cfg(test)] mod tests` | Inline in source file |
| Unit test function | `#[test] fn <descriptive_name>` | `fn test_tokenizer_encodes_empty_input` |
| Integration test file | `snake_case.rs` | `cognitive_pipeline.rs` |
| Integration test function | `#[test] fn <scenario>_<action>` | `fn test_cognitive_loop_processes_observation` |
| Benchmark function | `fn <metric>_<scenario>` | `fn bench_cognitive_loop_latency` |

**Invariant R-016:** Every module under `src/` SHALL contain at least one unit test. Modules without unit tests are non-conformant.

---

## 8. Build Artifact Structure

### 8.1 Build Output Locations

| Artifact | Location | Excluded from Git |
|---|---|---|
| Debug binary | `target/debug/cortex` | Yes |
| Release binary | `target/release/cortex` | Yes |
| Dependency artifacts | `target/{debug,release}/deps/` | Yes |
| Build scripts | `target/{debug,release}/build/` | Yes |
| Incremental cache | `target/{debug,release}/incremental/` | Yes |
| Fingerprints | `target/{debug,release}/.fingerprint/` | Yes |

### 8.2 Release Profile

Defined in `Cargo.toml`:

| Setting | Value | Purpose |
|---|---|---|
| `opt-level` | 3 | Maximum optimization |
| `lto` | true | Link-time optimization — smaller binary |
| `codegen-units` | 1 | Single codegen unit — maximum optimization |
| `strip` | true | Strip debug symbols — smaller binary |

**Invariant R-017:** Release binaries SHALL be stripped of debug symbols, use LTO, and compile with `codegen-units = 1`.

### 8.3 Runtime Artifact Locations

| Artifact | Location | Created By | Governed By |
|---|---|---|---|
| State file | `./cortex.cx` | Runtime on first boot | DOC-03 §23 |
| Temp state | `./cortex.cx.tmp` | Persistence engine during atomic write | DOC-03 §23 |
| Checkpoints | `./checkpoints/` | Persistence engine | DOC-08 §5 |

---

## 9. CI/CD Structure

### 9.1 CI Directory Layout (PLANNED)

When implemented, CI configuration follows:

```
.github/
├── workflows/
│   ├── ci.yml              # Main CI pipeline — format, lint, test, audit, build
│   ├── release.yml         # Release pipeline — tag-triggered build and publish
│   └── security.yml        # Security scanning — cargo-audit, cargo-deny
└── CODEOWNERS              # Code ownership for review assignments
```

### 9.2 CI Pipeline Stages

Defined by DOC-06 §2:

| Stage | Command | Gate |
|---|---|---|
| Format | `cargo fmt --check` | Must pass |
| Lint | `cargo clippy -- -D warnings` | Must pass |
| Unit tests | `cargo test --lib` | Must pass |
| Integration tests | `cargo test --test '*'` | Must pass |
| Security audit | `cargo audit` | No critical/high |
| Release build | `cargo build --release` | Must succeed |
| Binary validation | `./target/release/cortex status` | Must produce output |
| Artifact packaging | Archive binary + checksums | Must complete |

### 9.3 CI Environment Variables

| Variable | Purpose | Required |
|---|---|---|
| `CORTEX_API_KEY` | API authentication key | Runtime (not CI) |
| `CORTEX_CONFIG` | Configuration file path | Runtime (not CI) |

**Invariant R-018:** CI pipelines SHALL NOT expose secrets in logs. All secrets SHALL use CI-provided secret management.

---

## 10. Naming Conventions

### 10.1 File Naming

| Context | Convention | Example |
|---|---|---|
| Source files | `snake_case.rs` | `language_model.rs` |
| Module directories | `snake_case/` | `self_model/` |
| Test files | `snake_case.rs` | `cognitive_pipeline.rs` |
| Benchmark files | `snake_case.rs` | `memory_retrieval.rs` |
| Documentation | `CORTEX-DOC-NN.md` | `CORTEX-DOC-11.md` |
| Config files | `lowercase.toml` | `cortex.toml` |

### 10.2 Rust Naming

| Context | Convention | Example |
|---|---|---|
| Types | `PascalCase` | `CortexRuntime` |
| Traits | `PascalCase` | `MemorySystem` |
| Functions | `snake_case` | `process_observation` |
| Constants | `SCREAMING_SNAKE_CASE` | `MAX_EPISODE_COUNT` |
| Modules | `snake_case` | `working.rs` |
| Enums | `PascalCase` | `CortexError` |
| Enum variants | `PascalCase` | `StateCorrupted` |

### 10.3 Identifier Naming

| Context | Prefix | Example |
|---|---|---|
| DOC-01 requirements | `FR-<domain>-` | `FR-LANG-001` |
| Non-functional requirements | `<category>-` | `REL-001`, `SEC-001` |
| Acceptance criteria | `AC-<domain>-` | `AC-DEP-001` |
| Algorithm invariants | `ALG-` | `ALG-001` |
| State invariants | `INV-` | `INV-001` |
| Config invariants | `CFG-` | `CFG-001` |
| Build invariants | `BLD-` | `BLD-001` |
| Operational invariants | `OPS-` | `OPS-001` |
| Security invariants | `SEC-` | `SEC-001` |
| Repository invariants | `R-` | `R-001` |

**Invariant R-019:** All identifiers SHALL follow the naming conventions defined in this section. No identifier SHALL use an undeclared prefix.

---

## 11. Dependency Boundaries

### 11.1 External Dependency Rules

| Rule | Description | Governing Doc |
|---|---|---|
| Minimal surface | Only crates essential for functionality are permitted | DOC-01 §15 |
| No async framework leakage | `tokio` is I/O-only; cognitive pipeline is synchronous | DOC-02 §2 RP-004 |
| No network in cognition | `hyper` is only for API server; cognitive modules use no network | DOC-01 §15 |
| Integrity only | `blake3` is for integrity checking only — not password hashing | DOC-03 §35 |
| Serialization bounded | `bincode` for binary, `serde_json` for API, `toml` for config | DOC-03 §32 |
| Compression bounded | `zstd` only for `.cx` file compression | DOC-03 §23 |

### 11.2 Permitted Dependencies (Current)

| Crate | Version | Category | Rationale |
|---|---|---|---|
| `serde` | 1 | Serialization | Derive macros for all serializable types |
| `serde_json` | 1 | Serialization | JSON for API request/response |
| `bincode` | 1 | Serialization | Binary format for `.cx` state file |
| `zstd` | 0.13 | Compression | State file compression |
| `blake3` | 1 | Cryptography | BLAKE3-256 integrity hashing |
| `uuid` | 1 | Identity | UUID v4 generation for IDs |
| `tokio` | 1 | Async runtime | I/O operations, API server, timers |
| `hyper` | 1 | HTTP | HTTP/1.1 server for embedded API |
| `hyper-util` | 0.1 | HTTP | Server utilities |
| `http-body-util` | 0.1 | HTTP | Body handling |
| `clap` | 4 | CLI | Command-line argument parsing |
| `toml` | 0.8 | Configuration | TOML config file parsing |
| `tracing` | 0.1 | Logging | Structured logging facade |
| `tracing-subscriber` | 0.3 | Logging | Log output formatting |

### 11.3 Prohibited Dependencies

| Category | Prohibition | Reason |
|---|---|---|
| Databases | No SQLite, PostgreSQL, Redis, etc. | State is file-based (`.cx`) |
| Web frameworks | No Actix, Axum, Warp, Rocket | Embedded API uses raw `hyper` |
| ORM | No Diesel, SQLx, SeaORM | No database |
| GPU compute | No CUDA, wgpu, vulkano | CPU-only computation |
| Machine learning | No tch, candle, burn | Custom neural implementation |
| Serialization | No JSON-based state persistence | Binary `.cx` format with `bincode` |
| Crypto | No ring, rustls, openssl | Only `blake3` for integrity |
| Regex | No regex crate | Tokenization is deterministic, not pattern-based |

**Invariant R-020:** No dependency SHALL be added to `Cargo.toml` without explicit architectural approval and update to this document.

---

## 12. Traceability

### 12.1 Traceability to Other Documents

| DOC-11 Section | Governing/Related Document | Relationship |
|---|---|---|
| §2 Current Repository Tree | DOC-02 §5.1, DOC-FINAL §3 | DOC-11 documents current state; DOC-FINAL defines target |
| §3 Directory Responsibilities | DOC-02 §4.1 Module Hierarchy | DOC-11 extends with file-level detail |
| §4 Source Organization | DOC-02 §4 Module Architecture | DOC-11 documents current layout |
| §5 Documentation Structure | DOC-01 through DOC-11 | DOC-11 defines current series structure |
| §6 Configuration Files | DOC-10 Configuration Reference | DOC-11 defines current file locations; DOC-10 defines parameters |
| §7 Test Structure | DOC-07 Testing & Validation | DOC-11 defines current test locations; DOC-07 defines strategy |
| §8 Build Artifacts | DOC-06 Build & Release | DOC-11 defines current artifact locations; DOC-06 defines pipeline |
| §9 CI/CD Structure | DOC-06 Build & Release | DOC-11 defines current CI layout; DOC-06 defines stages |
| §10 Naming Conventions | DOC-02, DOC-03, DOC-04 | DOC-11 consolidates current naming rules |
| §11 Dependency Boundaries | DOC-01 §15, DOC-02 §7 | DOC-11 documents current dependencies |

### 12.2 Repository Structure Summary

| Aspect | Value |
|---|---|
| Language | Rust |
| Build system | Cargo |
| Source root | `src/*.rs`, `src/<module>/` |
| Module count | 71 Rust modules |
| Test location | `tests/*.rs`, inline |
| CI/CD | `.github/workflows/` |
| Documentation | `docs/DOC-NN-*.md` |
| Schemas | `schemas/` |
| Deployment | `deployment/` |
| Scripts | Not present | `scripts/` |

### 12.2 Traceability to Requirements

| DOC-01 Requirement | DOC-11 Coverage |
|---|---|
| §20.1 Language & Toolchain | §6.1 Cargo.toml, §11.2 Permitted Dependencies |
| §20.2 Dependency Constraints | §11 Dependency Boundaries |
| §20.3 Build & Deployment | §8 Build Artifacts, §9 CI/CD Structure |
| §21 Persistence Requirements | §8.3 Runtime Artifacts |
| §22 Configuration Requirements | §6 Configuration File Structure |
| §23 Repository Layout | §2 Complete Repository Tree |
| FR-PRS-001 through FR-PRS-006 | §8.2 Release Profile |

### 12.3 Cross-Reference Update Matrix

This section documents how DOC-11 references repository structure in other documents.

| Document | Section | DOC-11 Relationship |
|---|---|---|
| DOC-02 §5.1 | Repository Layout | DOC-11 §2.1 documents current state; DOC-FINAL §3 defines target |
| DOC-02 §5.2 | Cargo.toml Structure | DOC-11 §6.1 documents current Cargo.toml |
| DOC-06 §1.2 | Toolchain Pinning | DOC-11 §6.3 documents current toolchain file |
| DOC-06 §8 | Binary Specifications | DOC-11 §8.2 documents current release profile |
| DOC-07 §2 | Test Organization | DOC-11 §7 documents current test locations |
| DOC-08 §2 | Deployment Directory | DOC-11 §8.3 documents current runtime artifacts |

---

## 13. Current Repository Invariants

### 13.1 Current Structural Invariants

The following invariants describe the **current** repository structure. The Final Architectural Baseline has its own invariant set (DOC-FINAL §9, namespace `FAB-R-xxx`).

| # | Invariant | Enforcement | Violation Severity |
|---|---|---|---|
| R-001 | Exactly one `Cargo.toml` and one `Cargo.lock` in root | CI check | Critical |
| R-002 | No source code in root directory | CI check | Critical |
| R-003 | DOC files follow `CORTEX-DOC-NN.md` naming | CI check | Warning |
| R-004 | Every `src/` module has a `mod.rs` with trait interface | Code review | High |
| R-005 | No module exceeds 800 lines | CI check (line count) | Medium |
| R-006 | Total module count remains exactly 71 | CI check (count) | Critical |
| R-007 | Integration test files use descriptive filenames | Code review | Warning |
| R-008 | Integration tests cover cross-module scenarios | Test review | High |
| R-009 | Benchmarks produce deterministic results | Test verification | Medium |
| R-010 | Supplementary docs do not contradict DOC series | Doc review | High |
| R-011 | Module dependency graph is acyclic | `cargo check` | Critical |
| R-012 | `types/` has zero internal dependencies | `cargo check` | Critical |
| R-013 | All DOC versions are synchronized | Version check | High |
| R-014 | Every DOC has a traceability matrix to DOC-01 | Doc review | High |
| R-015 | `.gitignore` excludes all derived artifacts | CI check | Medium |
| R-016 | Every `src/` module has at least one unit test | `cargo test` | High |
| R-017 | Release binaries use LTO, strip, codegen-units=1 | Build profile check | Medium |
| R-018 | CI pipelines do not expose secrets | CI review | Critical |
| R-019 | All identifiers follow declared naming conventions | Lint/check | Medium |
| R-020 | No dependency added without architectural approval | Code review | Critical |

### 13.2 Invariant Severity Classification

| Severity | Meaning | Action on Violation |
|---|---|---|
| Critical | System integrity at risk | Block merge, require immediate fix |
| High | Specification non-conformance | Block merge, require fix before release |
| Medium | Quality degradation | Warning, allow with technical debt ticket |
| Low | Style/convention deviation | Informational, address in next cycle |

---

## 14. Current Completeness & Validation

### 14.1 Current Repository Completeness Checklist

This checklist reflects the **current** repository state. The Final Architectural Baseline has its own Definition of Done (DOC-FINAL §10).

| Category | Required Artifact | Status | Validation |
|---|---|---|---|
| Package manifest | `Cargo.toml` | ASSEMBLED | `cargo check` |
| Lock file | `Cargo.lock` | ASSEMBLED | `cargo check` |
| Git ignore | `.gitignore` | ASSEMBLED | Manual review |
| Toolchain pin | `rust-toolchain.toml` | PLANNED | CI check |
| Config template | `cortex.toml` | PLANNED | DOC-10 validation |
| Entry point | `src/main.rs` | ASSEMBLED | `cargo build` |
| Runtime | `src/runtime.rs` | PLANNED | DOC-02 §8 |
| 71 source modules | `src/**/*.rs` | ASSEMBLED | Module count check |
| Unit tests | `src/**/tests` | PARTIAL | `cargo test --lib` |
| Integration tests | `tests/*.rs` | PLANNED | `cargo test --test '*'` |
| Benchmarks | `benches/*.rs` | PLANNED | `cargo bench` |
| CI pipeline | `.github/workflows/` | PLANNED | DOC-06 §3 |
| Docs | `CORTEX-DOC-*.md` | ASSEMBLED | DOC-11 §5 |
| README | `README.md` | PLANNED | Manual review |

### 14.2 Module Completeness Rules

Each module SHALL satisfy the following before being considered complete:

| Rule | Description | Validation |
|---|---|---|
| M-01 | Public trait interface declared in `mod.rs` | Code review |
| M-02 | At least one unit test present | `cargo test` |
| M-03 | All public functions have doc comments | `cargo doc` |
| M-04 | Error types are defined or re-exported | Code review |
| M-05 | Dependencies are limited to allowed set | `cargo tree` analysis |
| M-06 | No `unwrap()` in production code paths | Clippy lint |
| M-07 | No `panic!()` except in invariant violations | Clippy lint |
| M-08 | Module count contribution matches DOC-02 §4.2 | Count verification |

### 14.3 Release Completeness Rules

A release SHALL not be created until:

| Rule | Description | Gate |
|---|---|---|
| REL-01 | All modules are assembled (71/71) | CI module count |
| REL-02 | All integration tests pass | CI test gate |
| REL-03 | Security audit passes (no critical/high) | CI security gate |
| REL-04 | Release build produces valid binary | Binary validation |
| REL-05 | `.cx` created on first boot by release binary | Integration test |
| REL-06 | All DOC versions are synchronized | Version check |
| REL-07 | Changelog is complete | Release process |
| REL-08 | Binary checksum is computed | Release artifact |

### 14.4 Validation Commands

The following commands validate repository conformance:

| Command | Validates | Governing Invariant |
|---|---|---|
| `cargo check` | Dependency resolution, type checking | R-011, R-012 |
| `cargo fmt --check` | Code formatting | DOC-06 §2 |
| `cargo clippy -- -D warnings` | Lint rules, code quality | DOC-06 §2 |
| `cargo test --lib` | Unit test coverage | R-016 |
| `cargo test --test '*'` | Integration test coverage | R-008 |
| `cargo bench` | Benchmark execution | R-009 |
| `cargo audit` | Dependency vulnerabilities | DOC-06 §6 |
| `cargo deny check` | License compliance | DOC-06 §6 |
| `find src -name "*.rs" \| wc -l` | Module count (excluding mod.rs) | R-006 |
| `wc -l src/**/*.rs` | Module line counts | R-005 |

---

## 15. Repository Lifecycle

### 15.1 File Lifecycle States

| State | Meaning | Allowed Transitions |
|---|---|---|
| PLANNED | Defined in specification, not yet implemented | → ASSEMBLED |
| ASSEMBLED | Present in repository and functional | → DEPRECATED, → MODIFIED |
| DEPRECATED | Present but scheduled for removal | → REMOVED |
| MODIFIED | Changed from original specification | → ASSEMBLED (after update) |
| REMOVED | Deleted from repository | (terminal state) |

### 15.2 Repository Health Metrics

| Metric | Target | Measurement |
|---|---|---|
| Module count | 71 | `find src -name "mod.rs" \| wc -l` |
| Test coverage | >80% line coverage | `cargo tarpaulin` |
| Documentation coverage | 100% public items | `cargo doc` |
| Dependency count | ≤15 production crates | `cargo tree --depth 1` |
| Build time (debug) | <120 seconds | `cargo build` timing |
| Build time (release) | <300 seconds | `cargo build --release` timing |
| Binary size (release) | <20MB | `ls -lh target/release/cortex` |

---

*End of Document — CORTEX-DOC-11 Repository Architecture & Structure v1.1.0*
