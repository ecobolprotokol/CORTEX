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
| **Scope** | Repository tree, Python package architecture, documentation structure, test architecture, schema architecture, configuration, scripts, deployment, CI/CD, invariants, completeness rules |
| **Parent Document** | CORTEX-DOC-02 Software Design Specification |
| **Effective Date** | 2026-08-13 |
| **Review Cycle** | Per architecture version transition |

### Revision History

| Version | Date | Author | Description |
|---|---|---|---|
| 1.0.0 | 2026-08-13 | CORTEX Architecture | Initial final baseline |
| 1.1.0 | 2026-08-13 | CORTEX Architecture | Establish as Final Architectural Baseline for Python architecture |

### Approval

| Role | Signature | Date |
|---|---|---|
| System Architect | _____________ | _____________ |
| Repository Maintainer | _____________ | _____________ |

### Document Purpose

This document defines **the repository structure of CORTEX** as the Final Architectural Baseline. It constitutes the authoritative record of the repository layout: every directory, every file, every naming convention, every boundary, and every structural invariant.

### Document Scope

This specification covers:

- Repository tree with every directory and file annotated.
- Python package architecture and source organization.
- Documentation structure and naming conventions.
- Test structure and organization.
- Schema architecture for `.cx`, API, and configuration.
- Configuration architecture with environment separation.
- Build, test, audit, and release script structure.
- Deployment architecture (Docker, Kubernetes, systemd, reverse-proxy).
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
| Primary language | Python |
| Minimum Python version | 3.11 |
| Build system | pyproject.toml |
| Package name | `cortex` |
| Package version | 1.0.0 |
| Package structure | `src/cortex/` (src layout) |
| State file | `cortex.cx` (BLAKE3-integrity, binary format) |
| Configuration file | `cortex.toml` (TOML format) |

### 1.2 Repository Classification

| Attribute | Value |
|---|---|
| Language ecosystem | Python / pyproject.toml |
| License | Proprietary (all rights reserved) |
| Version control | Git |
| Branching model | Mainline development |
| Commit convention | Conventional commits (`feat:`, `fix:`, `improve:`, `BREAKING CHANGE:`) |

---

## 2. Repository Tree

### 2.1 Repository Layout

The following tree represents the **repository structure** as the Final Architectural Baseline. Every entry is annotated with its purpose and governing document.

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
│   ├── DOC-01-Requirements.md            # Technical Specification (System Contract)
│   ├── DOC-02-Architecture.md            # Software Design Specification (Architecture Contract)
│   ├── DOC-03-Data-Architecture.md       # Data & State Specification (Data Contract)
│   ├── DOC-04-Algorithms.md              # Algorithm Specification (Computational Behavior Contract)
│   ├── DOC-05-API-CLI.md                 # API & CLI Specification (Interface Contract)
│   ├── DOC-06-Build-Release.md           # Build & Release Specification (Build Contract)
│   ├── DOC-07-Testing-Validation.md      # Testing & Validation Specification (Quality Contract)
│   ├── DOC-08-Deployment-Operations.md   # Deployment & Operations Specification (Operations Contract)
│   ├── DOC-09-Security-Privacy.md        # Security & Privacy Specification (Security Contract)
│   ├── DOC-10-Configuration-Reference.md # Configuration Reference (Configuration Contract)
│   ├── DOC-11-Repository-Architecture.md # Repository Architecture & Structure (Repository Contract)
│   │
│   ├── architecture/                     # Architecture-specific documentation
│   │   ├── consistency-audit.md          # Consistency audit results
│   │   ├── decision-records/             # Architecture Decision Records (ADRs)
│   │   └── diagrams/                     # Architecture diagrams
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
│       ├── serialization/                # Serialization layer — msgpack, JSON, TOML
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

## 3. Directory Responsibilities

### 3.1 Root Directory

The root directory (`CORTEX/`) is the workspace root. It contains:

| Artifact | Purpose | Mutability |
|---|---|---|
| `pyproject.toml` | Package manifest — defines metadata, dependencies, build config | Administrative |
| `Makefile` | Common development commands | Administrative |
| `.gitignore` | Git ignore rules — excludes build artifacts, state files, temp files | Administrative |
| `.editorconfig` | Editor configuration — indentation, charset | Administrative |
| `README.md` | Project overview, quickstart, links | Administrative |
| `LICENSE` | License file | Administrative |
| `CHANGELOG.md` | Version changelog | Administrative |
| `VERSION` | Single-source version string | Administrative |

**Invariant R-001:** The root directory SHALL contain exactly one `pyproject.toml`.

**Invariant R-002:** No source code SHALL exist in the root directory. All production code lives under `src/cortex/`.

**Invariant R-003:** Documentation files SHALL follow the naming convention `DOC-NN-Title.md` where `NN` is a two-digit number.

### 3.2 `src/cortex/` Package

The `src/cortex/` package contains all production source code. It is organized into:

| Package | Responsibility | Architectural Layer |
|---|---|---|
| `core/` | Core types, IDs, scalars, common definitions | Infrastructure |
| `cognitive/` | Cognitive pipeline orchestration | Cognitive Pipeline |
| `world/` | World model — entities, transitions, simulation | Cognitive Pipeline |
| `memory/` | Memory system — working, episodic, semantic, procedural, associative | Cognitive Pipeline |
| `learning/` | Continual learning — signals, attribution, replay, stability | Governance |
| `inference/` | Inference engine — reasoning, deduction | Cognitive Pipeline |
| `prediction/` | Prediction engine — future state estimation | Cognitive Pipeline |
| `hypothesis/` | Hypothesis generation and evaluation | Cognitive Pipeline |
| `self_model/` | Self model — capability estimation, health | Governance |
| `policy/` | Policy / risk gate — security boundary | Governance |
| `runtime/` | Runtime lifecycle — state machine, boot, shutdown | Runtime |
| `state/` | State management — serialization, mutations | Infrastructure |
| `persistence/` | Persistence engine — `.cx` format, checkpoints | Infrastructure |
| `serialization/` | Serialization layer — msgpack, JSON, TOML | Infrastructure |
| `provenance/` | Provenance tracking — origin, lineage | Infrastructure |
| `security/` | Security implementation | Infrastructure |
| `security/hashing/` | BLAKE3 hashing — integrity operations | Infrastructure |
| `security/integrity/` | Integrity verification — state validation | Infrastructure |
| `security/key_management/` | Key management — token handling | Infrastructure |
| `config/` | Configuration — parsing, validation, distribution | Infrastructure |
| `api/` | Embedded API — HTTP server, routes, handlers | Infrastructure |
| `cli/` | CLI — command parsing, dispatch | Infrastructure |
| `errors/` | Error taxonomy — exception hierarchy | Infrastructure |

**Invariant R-004:** Every package under `src/cortex/` SHALL have an `__init__.py` file.

**Invariant R-005:** No package SHALL exceed 800 lines of code. Packages exceeding this limit SHALL be refactored into sub-modules.

### 3.3 `tests/` Directory

The `tests/` directory contains all test categories.

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

**Invariant R-006:** Every test category directory SHALL exist.

**Invariant R-007:** Test files SHALL use descriptive names matching the scenario they validate.

### 3.4 `schemas/` Directory

The `schemas/` directory contains schema definitions.

| Directory | Purpose | Content |
|---|---|---|
| `schemas/cx/` | `.cx` file format schemas | Binary format spec, section definitions, machine-readable schema |
| `schemas/api/` | API schemas | Request/response schemas |
| `schemas/configuration/` | Configuration schemas | Validation rules |

### 3.5 `config/` Directory

The `config/` directory contains configuration profiles.

| Directory | Purpose | Environment |
|---|---|---|
| `config/defaults/` | Default configuration values | All |
| `config/development/` | Development overrides | Development |
| `config/testing/` | Testing overrides | Testing |
| `config/production/` | Production overrides | Production |

### 3.6 `scripts/` Directory

The `scripts/` directory contains development and operations scripts.

| Directory | Purpose |
|---|---|
| `scripts/build/` | Build scripts — compilation, packaging |
| `scripts/test/` | Test scripts — runner orchestration |
| `scripts/audit/` | Audit scripts — security, license, dependency |
| `scripts/migration/` | Migration scripts — schema/state evolution |
| `scripts/release/` | Release scripts — tagging, packaging, publishing |

### 3.7 `deployment/` Directory

The `deployment/` directory contains deployment configurations.

| Directory | Purpose |
|---|---|
| `deployment/docker/` | Docker — Dockerfile, docker-compose |
| `deployment/kubernetes/` | Kubernetes — manifests, Helm charts |
| `deployment/systemd/` | systemd — service files |
| `deployment/reverse-proxy/` | Reverse proxy — nginx, caddy configs |

### 3.8 `benchmarks/` Directory

The `benchmarks/` directory contains performance benchmarks.

| Directory | Purpose | Target |
|---|---|---|
| `benchmarks/cognitive/` | Cognitive pipeline benchmarks | <100ms per cycle |
| `benchmarks/memory/` | Memory retrieval benchmarks | >1000 queries/sec |
| `benchmarks/learning/` | Learning system benchmarks | Bounded learning rate |
| `benchmarks/inference/` | Inference benchmarks | <50ms per query |
| `benchmarks/persistence/` | Persistence I/O benchmarks | >10MB/s |

### 3.9 `examples/` Directory

The `examples/` directory contains usage examples.

| Directory | Purpose |
|---|---|
| `examples/basic/` | Basic usage examples |
| `examples/api/` | API usage examples |
| `examples/cli/` | CLI usage examples |
| `examples/persistence/` | Persistence examples |

### 3.10 `migrations/` Directory

The `migrations/` directory contains schema/state migration artifacts.

| Directory | Purpose |
|---|---|
| `migrations/v1/` | Version 1 migrations |

### 3.11 `artifacts/` Directory

The `artifacts/` directory contains generated outputs (gitignored).

| Directory | Purpose |
|---|---|
| `artifacts/builds/` | Build outputs |
| `artifacts/test-reports/` | Test report outputs |
| `artifacts/audit-reports/` | Audit report outputs |

### 3.12 `.github/` Directory

The `.github/` directory contains GitHub configuration.

| Path | Purpose |
|---|---|
| `.github/workflows/ci.yml` | CI pipeline — lint, type-check, test |
| `.github/workflows/test.yml` | Test pipeline — full test suite |
| `.github/workflows/security.yml` | Security scanning — audit, dependency check |
| `.github/workflows/release.yml` | Release pipeline — build, package, publish |
| `.github/ISSUE_TEMPLATE/` | Issue templates |
| `.github/pull_request_template.md` | PR template |

---

## 4. Source-Code Organization

### 4.1 Package Structure Pattern

Every package under `src/cortex/` follows a consistent structure:

```
src/cortex/<package>/
├── __init__.py       # Package initialization, re-exports
├── <module>.py       # Individual module implementation
└── ...
```

**Pattern rules:**

| Rule | Description |
|---|---|
| Initialization | `__init__.py` declares public interface and re-exports |
| Implementation | Individual files implement module logic |
| Naming | File names are `snake_case.py` |
| Type hints | All public functions have type hints |
| Docstrings | All public modules have docstrings |

### 4.2 Dependency Flow Rules

Source-code dependency flow follows the rules defined in DOC-02 §6. Repository-level enforcement:

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

**Invariant R-008:** The dependency graph SHALL remain acyclic at the package level.

**Invariant R-009:** `core/` SHALL have zero internal dependencies — it is the foundational leaf package.

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
└── DOC-10 (Configuration Reference)
```

### 5.2 Document Properties

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
| Repository invariant | `R-001` | Repository-level invariant (DOC-11, this document) |

### 5.4 Document Naming Convention

| Rule | Pattern | Example |
|---|---|---|
| File name | `DOC-NN-Title.md` | `DOC-11-Repository-Architecture.md` |
| Document ID | `CORTEX-DOC-NN` | `CORTEX-DOC-11` |
| Section numbering | Hierarchical decimal | `§3.2.1` |
| Version format | `MAJOR.MINOR.PATCH` | `1.1.0` |
| Date format | ISO 8601 | `2026-08-13` |

**Invariant R-010:** All document versions across the DOC series SHALL be synchronized.

**Invariant R-011:** Every document SHALL contain a traceability matrix mapping its coverage back to DOC-01 requirements.

---

## 6. Configuration File Structure

### 6.1 pyproject.toml

The `pyproject.toml` defines the package manifest. Its structure is governed by DOC-02 §5.2 and DOC-06 §1.

| Section | Purpose | Mutability |
|---|---|---|
| `[build-system]` | Build backend configuration | Administrative |
| `[project]` | Package metadata — name, version, dependencies | Administrative |
| `[project.optional-dependencies]` | Optional dependency groups — dev, test, benchmark | Administrative |
| `[project.scripts]` | CLI entry points | Administrative |
| `[tool.ruff]` | Linter configuration | Administrative |
| `[tool.mypy]` | Type checker configuration | Administrative |
| `[tool.pytest.ini_options]` | Test runner configuration | Administrative |

**Dependency inventory:**

| Category | Packages |
|---|---|
| Integrity | `blake3` |
| Compression | `pyzstd` |
| Serialization | `msgpack` |
| Configuration | `tomli` |
| CLI | `click` |
| HTTP server | `uvicorn`, `starlette`, `httpx` |
| Logging | `structlog` |

### 6.2 .gitignore

The `.gitignore` excludes:

| Pattern | Reason | Governing Doc |
|---|---|---|
| `__pycache__/` | Python bytecode | Standard Python |
| `*.py[cod]` | Compiled Python | Standard Python |
| `.venv/` | Virtual environments | Standard Python |
| `.pytest_cache/` | Test cache | Standard Python |
| `.mypy_cache/` | Type checker cache | Standard Python |
| `cortex.cx` | Runtime state file | DOC-03 §23, DOC-08 §3 |
| `cortex.cx.tmp` | Temporary state during atomic writes | DOC-03 §23 |
| `checkpoints/` | Checkpoint directory | DOC-08 §5 |
| `_archive/` | Legacy implementation | Repository cleanup |
| `artifacts/` | Generated outputs | DOC-06 §8 |

**Invariant R-012:** The `.gitignore` SHALL exclude all build artifacts, runtime state files, temporary files, and backup files.

### 6.3 cortex.toml

The `cortex.toml` is the runtime configuration file. Fully defined by DOC-10. Located at the working directory or specified via `--config` / `CORTEX_CONFIG`.

---

## 7. Test Structure

### 7.1 Test Organization

| Test Category | Location | Minimum Count | Governing Doc |
|---|---|---|---|
| Unit tests | `tests/unit/` | >500 | DOC-07 §2.1 |
| Integration tests | `tests/integration/` | >50 | DOC-07 §2.2 |
| System tests | `tests/system/` | >10 | DOC-07 §2.3 |
| Acceptance tests | `tests/acceptance/` | >20 | DOC-07 §2.4 |
| Regression tests | `tests/regression/` | >20 | DOC-07 §3 |
| Property tests | `tests/property/` | >10 | DOC-07 §4 |
| Security tests | `tests/security/` | >30 | DOC-07 §5 |
| Performance tests | `tests/performance/` | >10 | DOC-07 §6 |

### 7.2 Test Naming Conventions

| Convention | Pattern | Example |
|---|---|---|
| Unit test file | `test_<module>.py` | `test_tokenizer.py` |
| Unit test function | `test_<scenario>_<action>` | `test_tokenizer_encodes_empty_input` |
| Integration test file | `test_<scenario>.py` | `test_cognitive_pipeline.py` |
| Integration test function | `test_<scenario>_<action>` | `test_cognitive_loop_processes_observation` |

**Invariant R-013:** Every package under `src/cortex/` SHALL have corresponding tests in `tests/unit/`.

---

## 8. Build Artifact Structure

### 8.1 Build Output Locations

| Artifact | Location | Excluded from Git |
|---|---|---|
| Python packages | `dist/` | Yes |
| Build artifacts | `build/` | Yes |
| Egg info | `*.egg-info/` | Yes |

### 8.2 Release Profile

Defined in `pyproject.toml`:

| Setting | Value | Purpose |
|---|---|---|
| Build backend | `setuptools` | Package building |
| Python version | `>=3.11` | Minimum supported version |

### 8.3 Runtime Artifact Locations

| Artifact | Location | Created By | Governed By |
|---|---|---|---|
| State file | `./cortex.cx` | Runtime on first boot | DOC-03 §23 |
| Temp state | `./cortex.cx.tmp` | Persistence engine during atomic write | DOC-03 §23 |
| Checkpoints | `./checkpoints/` | Persistence engine | DOC-08 §5 |

---

## 9. CI/CD Structure

### 9.1 CI Directory Layout

```
.github/
├── workflows/
│   ├── ci.yml          # Main CI pipeline — lint, type-check, test
│   ├── test.yml        # Test pipeline — full test suite
│   ├── security.yml    # Security scanning — audit, dependency check
│   └── release.yml     # Release pipeline — build, package, publish
├── ISSUE_TEMPLATE/     # Issue templates
└── pull_request_template.md  # PR template
```

### 9.2 CI Pipeline Stages

Defined by DOC-06 §2:

| Stage | Command | Gate |
|---|---|---|
| Lint | `ruff check src/ tests/` | Must pass |
| Type check | `mypy src/` | Must pass |
| Unit tests | `pytest tests/unit/` | Must pass |
| Integration tests | `pytest tests/integration/` | Must pass |
| Security audit | `pip-audit` | No critical/high |
| Build | `python -m build` | Must succeed |

### 9.3 CI Environment Variables

| Variable | Purpose | Required |
|---|---|---|
| `CORTEX_API_KEY` | API authentication key | Runtime (not CI) |
| `CORTEX_CONFIG` | Configuration file path | Runtime (not CI) |

**Invariant R-014:** CI pipelines SHALL NOT expose secrets in logs.

---

## 10. Naming Conventions

### 10.1 File Naming

| Context | Convention | Example |
|---|---|---|
| Source files | `snake_case.py` | `language_model.py` |
| Package directories | `snake_case/` | `self_model/` |
| Test files | `test_<name>.py` | `test_cognitive_pipeline.py` |
| Benchmark files | `test_<name>.py` | `test_cognitive_loop.py` |
| Documentation | `DOC-NN-Title.md` | `DOC-11-Repository-Architecture.md` |
| Config files | `lowercase.toml` | `cortex.toml` |

### 10.2 Python Naming

| Context | Convention | Example |
|---|---|---|
| Classes | `PascalCase` | `CortexRuntime` |
| Functions | `snake_case` | `process_observation` |
| Constants | `SCREAMING_SNAKE_CASE` | `MAX_EPISODE_COUNT` |
| Modules | `snake_case` | `working.py` |
| Exceptions | `PascalCase` | `CortexError` |

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

**Invariant R-015:** All identifiers SHALL follow the naming conventions defined in this section.

---

## 11. Dependency Boundaries

### 11.1 External Dependency Rules

| Rule | Description | Governing Doc |
|---|---|---|
| Minimal surface | Only packages essential for functionality are permitted | DOC-01 §15 |
| No async framework leakage | `asyncio` is for I/O only; cognitive pipeline is synchronous | DOC-02 §2 |
| No network in cognition | `httpx` is only for API server; cognitive modules use no network | DOC-01 §15 |
| Integrity only | `blake3` is for integrity checking only — not password hashing | DOC-03 §35 |
| Serialization bounded | `msgpack` for binary, `json` for API, `tomli` for config | DOC-03 §32 |
| Compression bounded | `pyzstd` only for `.cx` file compression | DOC-03 §23 |

### 11.2 Permitted Dependencies

| Package | Version | Category | Rationale |
|---|---|---|---|
| `blake3` | >=1.0 | Integrity | BLAKE3-256 integrity hashing |
| `pyzstd` | >=0.15 | Compression | State file compression |
| `msgpack` | >=1.0 | Serialization | Binary format for `.cx` state file |
| `tomli` | >=2.0 | Configuration | TOML config file parsing (stdlib 3.11+) |
| `click` | >=8.0 | CLI | Command-line argument parsing |
| `uvicorn` | >=0.23 | HTTP | ASGI server for embedded API |
| `starlette` | >=0.27 | HTTP | Web framework for API routes |
| `httpx` | >=0.24 | HTTP | HTTP client for internet interface |
| `structlog` | >=23.0 | Logging | Structured logging |

### 11.3 Prohibited Dependencies

| Category | Prohibition | Reason |
|---|---|---|
| Databases | No SQLite, PostgreSQL, Redis, etc. | State is file-based (`.cx`) |
| Web frameworks | No Django, Flask, FastAPI | Embedded API uses raw starlette |
| ORM | No SQLAlchemy, Tortoise, Peewee | No database |
| GPU compute | No CUDA, PyTorch, TensorFlow | CPU-only computation |
| Machine learning | No scikit-learn, transformers | Custom neural implementation |
| Serialization | No JSON-based state persistence | Binary `.cx` format with `msgpack` |
| Crypto | No cryptography, pycryptodome | Only `blake3` for integrity |

**Invariant R-016:** No dependency SHALL be added to `pyproject.toml` without explicit architectural approval.

---

## 12. Traceability

### 12.1 Traceability to Other Documents

| DOC-11 Section | Governing/Related Document | Relationship |
|---|---|---|
| §2 Repository Tree | DOC-02 §5.1 | DOC-11 defines repository structure |
| §3 Directory Responsibilities | DOC-02 §4.1 Module Hierarchy | DOC-11 extends with file-level detail |
| §4 Source Organization | DOC-02 §4 Module Architecture | DOC-11 defines package layout |
| §5 Documentation Structure | DOC-01 through DOC-11 | DOC-11 defines series structure |
| §6 Configuration Files | DOC-10 Configuration Reference | DOC-11 defines file locations; DOC-10 defines parameters |
| §7 Test Structure | DOC-07 Testing & Validation | DOC-11 defines test locations; DOC-07 defines strategy |
| §8 Build Artifacts | DOC-06 Build & Release | DOC-11 defines artifact locations; DOC-06 defines pipeline |
| §9 CI/CD Structure | DOC-06 Build & Release | DOC-11 defines CI layout; DOC-06 defines stages |
| §10 Naming Conventions | DOC-02, DOC-03, DOC-04 | DOC-11 consolidates naming rules |
| §11 Dependency Boundaries | DOC-01 §15, DOC-02 §7 | DOC-11 defines repository-level boundaries |

### 12.2 Traceability to Requirements

| DOC-01 Requirement | DOC-11 Coverage |
|---|---|
| §20.1 Language & Toolchain | §6.1 pyproject.toml, §11.2 Permitted Dependencies |
| §20.2 Dependency Constraints | §11 Dependency Boundaries |
| §20.3 Build & Deployment | §8 Build Artifacts, §9 CI/CD Structure |
| §21 Persistence Requirements | §8.3 Runtime Artifacts |
| §22 Configuration Requirements | §6 Configuration File Structure |
| §23 Repository Layout | §2 Complete Repository Tree |

---

## 13. Repository Invariants

### 13.1 Structural Invariants

| # | Invariant | Enforcement | Violation Severity |
|---|---|---|---|
| R-001 | Exactly one `pyproject.toml` in root | CI check | Critical |
| R-002 | No source code in root directory | CI check | Critical |
| R-003 | DOC files follow `DOC-NN-Title.md` naming | CI check | Warning |
| R-004 | Every package has `__init__.py` | Code review | High |
| R-005 | No package exceeds 800 lines | CI check (line count) | Medium |
| R-006 | All 9 test category directories exist | Directory check | High |
| R-007 | Test files use descriptive filenames | Code review | Warning |
| R-008 | Package dependency graph is acyclic | Import check | Critical |
| R-009 | `core/` has zero internal dependencies | Import check | Critical |
| R-010 | All DOC versions are synchronized | Version check | High |
| R-011 | Every DOC has a traceability matrix to DOC-01 | Doc review | High |
| R-012 | `.gitignore` excludes all derived artifacts | CI check | Medium |
| R-013 | Every package has corresponding tests | Test review | High |
| R-014 | CI pipelines do not expose secrets | CI review | Critical |
| R-015 | All identifiers follow declared naming conventions | Lint/check | Medium |
| R-016 | No dependency added without architectural approval | Code review | Critical |

### 13.2 Invariant Severity Classification

| Severity | Meaning | Action on Violation |
|---|---|---|
| Critical | System integrity at risk | Block merge, require immediate fix |
| High | Specification non-conformance | Block merge, require fix before release |
| Medium | Quality degradation | Warning, allow with technical debt ticket |
| Low | Style/convention deviation | Informational, address in next cycle |

---

## 14. Completeness & Validation

### 14.1 Repository Completeness Checklist

| Category | Required Artifact | Validation |
|---|---|---|
| Package manifest | `pyproject.toml` | `pip install -e .` |
| Git ignore | `.gitignore` | Manual review |
| Config template | `cortex.toml` | DOC-10 validation |
| Entry point | `src/cortex/__init__.py` | Import check |
| Runtime | `src/cortex/runtime/` | DOC-02 §8 |
| 22 source packages | `src/cortex/*/` | Package count check |
| Unit tests | `tests/unit/` | `pytest tests/unit/` |
| Integration tests | `tests/integration/` | `pytest tests/integration/` |
| CI pipeline | `.github/workflows/` | DOC-06 §3 |
| Docs | `docs/DOC-*.md` | DOC-11 §5 |
| README | `README.md` | Manual review |

### 14.2 Package Completeness Rules

Each package SHALL satisfy the following before being considered complete:

| Rule | Description | Validation |
|---|---|---|
| M-01 | `__init__.py` with public interface | Code review |
| M-02 | At least one unit test present | `pytest` |
| M-03 | All public functions have docstrings | `pydoc` |
| M-04 | Error types are defined or re-imported | Code review |
| M-05 | Dependencies are limited to allowed set | `pip list` analysis |
| M-06 | No bare `except:` in production code | Lint check |
| M-07 | All functions have type hints | `mypy` |

### 14.3 Release Completeness Rules

A release SHALL not be created until:

| Rule | Description | Gate |
|---|---|---|
| REL-01 | All packages are assembled | Package count check |
| REL-02 | All integration tests pass | CI test gate |
| REL-03 | Security audit passes (no critical/high) | CI security gate |
| REL-04 | Build produces valid package | Build validation |
| REL-05 | `.cx` created on first boot | Integration test |
| REL-06 | All DOC versions are synchronized | Version check |
| REL-07 | Changelog is complete | Release process |

### 14.4 Validation Commands

The following commands validate repository conformance:

| Command | Validates | Governing Invariant |
|---|---|---|
| `pip install -e .` | Package installation | R-001 |
| `ruff check src/ tests/` | Code formatting | DOC-06 §2 |
| `mypy src/` | Type checking | DOC-06 §2 |
| `pytest tests/unit/` | Unit test coverage | R-013 |
| `pytest tests/integration/` | Integration test coverage | R-007 |
| `pip-audit` | Dependency vulnerabilities | DOC-06 §6 |

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
| Package count | 22 | `find src/cortex -name "__init__.py" \| wc -l` |
| Test coverage | >80% line coverage | `pytest --cov=cortex` |
| Documentation coverage | 100% public items | `pydoc` |
| Dependency count | ≤15 production packages | `pip list` |

---

*End of Document — CORTEX-DOC-11 Repository Architecture & Structure v1.1.0*
