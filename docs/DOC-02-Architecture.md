# CORTEX — 02 Software Design Specification

---

## Document Control

| Property | Value |
|---|---|
| **Document ID** | CORTEX-DOC-02 |
| **Title** | Software Design Specification |
| **Version** | 1.1.0 |
| **Status** | Final Architectural Baseline |
| **Classification** | Architecture Contract |
| **Scope** | Software architecture, module design, type system, runtime design |
| **Parent Document** | CORTEX-DOC-01 Technical Specification |
| **Effective Date** | 2026-08-13 |
| **Review Cycle** | Per architecture version transition |

### Revision History

| Version | Date | Author | Description |
|---|---|---|---|
| 1.0.0 | 2026-08-13 | CORTEX Architecture | Initial final baseline |
| 1.1.0 | 2026-08-13 | CORTEX Architecture | Replace SHA-256/HMAC with BLAKE3 for all hashing operations |

### Approval

| Role | Signature | Date |
|---|---|---|
| System Architect | _____________ | _____________ |
| Software Design Lead | _____________ | _____________ |
| Module Owners | _____________ | _____________ |

### Document Purpose

This document translates the technical requirements defined in CORTEX-DOC-01 into a concrete software architecture and design. It defines **how** CORTEX is structured as software: modules, types, dependencies, runtime behavior, data flow, and internal contracts.

### Document Scope

This specification covers:

- Complete module architecture and responsibility assignment.
- Core type-system design for all subsystems.
- Runtime architecture including boot, state machine, and lifecycle.
- Internal data flow and cross-subsystem contracts.
- Persistence, concurrency, security, and error architecture.
- Build, deployment, and testing architecture.

This specification does NOT cover:

- Algorithm-level implementation detail (governed by algorithm-level documents).
- Phased development roadmap or sprint planning.
- External user documentation.

---

## 1. Design Principles

### 1.1 Architectural Design Principles

| # | Principle | Design Implication |
|---|---|---|
| DP-001 | Single-package composition | All modules compile into one `cortex` package; no plugin loading |
| DP-002 | Single-process execution | All subsystems run within one OS process; no IPC |
| DP-003 | Ownership-based state | Each subsystem owns its state; mutation through defined interfaces |
| DP-004 | Trait-based abstraction | Subsystem boundaries defined by Rust traits; implementations swappable |
| DP-005 | Explicit data flow | Data moves through typed pipelines; no implicit global state mutation |
| DP-006 | Bounded execution | All cognitive operations have explicit resource bounds |
| DP-007 | Fail-before-persist | Invalid state transitions fail before reaching persistence layer |
| DP-008 | Provenance-preserving | Every knowledge mutation preserves origin, confidence, and evidence |
| DP-009 | Policy as boundary | Policy gate is architecturally separate from learned state |
| DP-010 | Versioned everything | State, algorithms, configuration, and format all carry version metadata |
| DP-011 | Deterministic infrastructure | Serialization, indexing, and policy are deterministic; learning may be stochastic |
| DP-012 | No cognitive external dependency | No external AI model, database, or framework in the cognitive path |
| DP-013 | Graceful degradation | Disabled subsystems return defined defaults; pipeline adapts |
| DP-014 | Inspectability | Internal state is queryable through controlled interfaces |
| DP-015 | Separation of concerns | Knowledge/language, reasoning/generation, planning/policy are distinct |

### 1.2 Python-Specific Design Principles

| # | Principle | Application |
|---|---|---|
| RP-001 | Dataclass-based state | Each subsystem uses dataclasses for structured state; frozen where immutability required |
| RP-002 | Protocol-based abstraction | Subsystem interfaces defined as Protocols; concrete implementations behind type hints |
| RP-003 | Enum for state machines | Runtime states, cell states, verification statuses as Python Enums |
| RP-004 | Exception for error propagation | All fallible operations raise `CortexError` subclasses |
| RP-005 | Type safety for IDs | Distinct ID dataclasses per entity (CellId, EpisodeId, etc.) prevent cross-contamination |
| RP-006 | Immutability by default | Frozen dataclasses preferred; mutation through explicit methods |
| RP-007 | Type hints everywhere | All public functions have complete type annotations |
| RP-008 | No global state | All state held in subsystem instances; no module-level mutable state |

---

## 2. Architectural Overview

### 2.1 System Composition

```
┌─────────────────────────────────────────────────────────────────────┐
│                         CORTEX BINARY                                │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                      RUNTIME LAYER                           │   │
│  │  ┌─────────┐ ┌──────────┐ ┌──────────┐ ┌───────────────┐  │   │
│  │  │  Boot   │ │  State   │ │ Cognitive│ │  Shutdown /   │  │   │
│  │  │Sequence │ │  Machine │ │   Loop   │ │  Recovery     │  │   │
│  │  └─────────┘ └──────────┘ └──────────┘ └───────────────┘  │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                   COGNITIVE PIPELINE                         │   │
│  │                                                             │   │
│  │  ┌──────────┐    ┌──────────┐    ┌──────────────────────┐  │   │
│  │  │ Language │───→│  Neural  │───→│    Memory System     │  │   │
│  │  │  Core    │    │   Core   │    │  (5 subsystems)      │  │   │
│  │  │  (CLX)   │    │  (CNS)   │    │                      │  │   │
│  │  └──────────┘    └──────────┘    └──────────────────────┘  │   │
│  │       │                │                    │               │   │
│  │       ↓                ↓                    ↓               │   │
│  │  ┌──────────┐    ┌──────────┐    ┌──────────────────────┐  │   │
│  │  │  World   │───→│Reasoning │───→│     Planning         │  │   │
│  │  │  Model   │    │  Engine  │    │     Engine           │  │   │
│  │  └──────────┘    └──────────┘    └──────────────────────┘  │   │
│  │       │                │                    │               │   │
│  │       ↓                ↓                    ↓               │   │
│  │  ┌──────────┐    ┌──────────┐    ┌──────────────────────┐  │   │
│  │  │Verification│  │ Learning │    │   Consolidation      │  │   │
│  │  │  Engine   │  │  System  │    │     System           │  │   │
│  │  └──────────┘    └──────────┘    └──────────────────────┘  │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                  GOVERNANCE LAYER                            │   │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌───────────────┐  │   │
│  │  │  Policy  │ │  Self    │ │Provenance│ │   Error       │  │   │
│  │  │  / Risk  │ │  Model   │ │  System  │ │   Handling    │  │   │
│  │  └──────────┘ └──────────┘ └──────────┘ └───────────────┘  │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                INFRASTRUCTURE LAYER                          │   │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌───────────────┐  │   │
│  │  │Persistence│ │   API   │ │   CLI    │ │  Internet     │  │   │
│  │  │  Engine  │ │  Server  │ │  Layer   │ │  Interface    │  │   │
│  │  └──────────┘ └──────────┘ └──────────┘ └───────────────┘  │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                  CONFIGURATION LAYER                         │   │
│  │  ┌──────────────────────────────────────────────────────┐   │   │
│  │  │  cortex.toml → Config Parser → Validated Config      │   │   │
│  │  └──────────────────────────────────────────────────────┘   │   │
│  └─────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────┘
```

### 2.2 Layer Responsibilities

| Layer | Responsibility |
|---|---|
| Runtime Layer | Boot, state machine, cognitive loop orchestration, shutdown, recovery |
| Cognitive Pipeline | All cognitive processing: language, neural, memory, world, reasoning, planning, verification, learning, consolidation |
| Governance Layer | Policy enforcement, self-assessment, provenance tracking, error taxonomy |
| Infrastructure Layer | Persistence, API serving, CLI interaction, internet access |
| Configuration Layer | Configuration parsing, validation, and distribution |

### 2.3 Data Flow Architecture

```
Input (CLI/API)
    │
    ↓
┌─────────────────────────────────────────────────────┐
│              COGNITIVE PIPELINE                      │
│                                                     │
│  Input → Observation → LanguageState                │
│              → NeuralRepresentation                 │
│              → MemoryRetrieval                      │
│              → WorldState                           │
│              → ReasoningResult                      │
│              → Plan (optional)                      │
│              → VerifiedResult                       │
│              → GeneratedResponse                    │
│                                                     │
│  Side effects:                                      │
│    → Experience recording                           │
│    → Learning signal generation                     │
│    → State mutation (bounded)                       │
│    → Checkpoint (periodic)                          │
└─────────────────────────────────────────────────────┘
    │
    ↓
Output (CLI/API)
```

---

## 3. Process & Execution Model

### 3.1 Process Architecture

| Property | Design |
|---|---|
| Process count | 1 |
| Binary | `cortex` (single ELF/PE/Mach-O) |
| Address space | Single unified address space |
| Thread model | Main thread + bounded worker threads |
| Async model | Optional async runtime for I/O (tokio) |
| Signal handling | SIGTERM → graceful shutdown; SIGKILL → immediate |

### 3.2 Thread Architecture

```
Main Thread
  ├── Cognitive Loop (synchronous processing)
  ├── State Machine Transitions
  └── CLI Command Dispatch

Worker Threads (bounded pool)
  ├── I/O Operations (persistence writes)
  ├── Network Operations (API server, internet)
  ├── Background Consolidation
  ├── Background Checkpoint
  └── Replay Processing
```

### 3.3 Execution Modes

| Mode | Entry Point | Behavior |
|---|---|---|
| `run` | `cortex run` | Full cognitive loop; accepts input, processes, learns |
| `serve` | `cortex serve` | API server mode; HTTP endpoints active |
| `observe` | `cortex observe <text>` | Single observation; no response generation |
| `experience` | `cortex experience <json>` | Single experience ingestion |
| `learn` | `cortex learn` | Trigger learning cycle explicitly |
| `query` | `cortex query <text>` | Query cognitive state; read-only |
| `inspect` | `cortex inspect` | State inspection; diagnostic output |
| `verify` | `cortex verify <claim>` | Verification of a specific claim |
| `checkpoint` | `cortex checkpoint` | Manual checkpoint creation |
| `status` | `cortex status` | Runtime status display |
| `init` | `cortex init` | Initialize new state |
| `migrate` | `cortex migrate` | State format migration |

---

## 4. Module Architecture

### 4.1 Module Hierarchy

```
src/cortex/
├── __init__.py           # Package initialization, version export
│
├── core/                 # Core types, IDs, scalars, common definitions
├── cognitive/            # Cognitive pipeline orchestration
├── world/                # World Model
├── memory/               # Memory System
├── reasoning/            # Reasoning Engine
├── planning/             # Planning Engine
├── verification/         # Verification Engine
├── learning/             # Continual Learning System
├── self_model/           # Self Model
├── policy/               # Policy / Risk Gate
├── internet/             # Internet Interface
├── persistence/          # Persistence Engine
├── api/                  # Embedded API
├── cli/                  # CLI Layer
├── observability/        # Observability
│
└── types/                # Core Type System
```

### 4.2 Module Count

| Category | Package Count |
|---|---|
| Core types | 1 |
| Cognitive pipeline | 1 |
| World Model | 1 |
| Memory System | 1 |
| Reasoning | 1 |
| Planning | 1 |
| Verification | 1 |
| Learning | 1 |
| Self Model | 1 |
| Policy | 1 |
| Internet | 1 |
| Persistence | 1 |
| API | 1 |
| CLI | 1 |
| Observability | 1 |
| Types | 1 |
| **Total** | **22** |

---

## 5. Repository & Module Structure

### 5.1 Repository Layout

> The repository tree, directory responsibilities, naming conventions, and structural invariants are documented in **CORTEX-DOC-11 Repository Architecture & Structure**. The layout below is a summary reference; for the full specification see DOC-11 §2.1.

```
cortex/
├── Cargo.toml              # Workspace root manifest
├── Cargo.lock              # Locked dependency versions
├── rust-toolchain.toml     # Pinned Rust toolchain (DOC-11 §6.3)
├── cortex.toml             # Default configuration template (DOC-11 §6.4)
├── README.md               # Project documentation
│
├── CORTEX-DOC-01.md        # Technical Specification
├── CORTEX-DOC-02.md        # Software Design Specification
├── ...                     # (DOC-03 through DOC-10)
├── CORTEX-DOC-11.md        # Repository Architecture & Structure
│
├── src/                    # All source code (71 modules) (DOC-11 §4)
│   └── ...                 # (see Module Hierarchy above)
│
├── tests/                  # Integration tests (DOC-11 §7)
│   ├── cognitive_pipeline.rs
│   ├── persistence_roundtrip.rs
│   ├── learning_stability.rs
│   ├── security_policy.rs
│   ├── api_endpoints.rs
│   └── corruption_recovery.rs
│
├── benches/                # Performance benchmarks (DOC-11 §7)
│   ├── cognitive_loop.rs
│   ├── memory_retrieval.rs
│   └── persistence.rs
│
└── docs/                   # Supplementary documentation (DOC-11 §3.5)
```

### 5.2 Cargo.toml Structure

```toml
[package]
name = "cortex"
version = "1.0.0"
edition = "2021"
rust-version = "1.75"

[dependencies]
# Serialization
serde = { version = "1", features = ["derive"] }
bincode = "1"

# Compression
zstd = "0.13"

# Cryptography (integrity)
blake3 = "1"

# UUID
uuid = { version = "1", features = ["v4"] }

# Async runtime (I/O only)
tokio = { version = "1", features = ["rt-multi-thread", "net", "fs", "time"] }

# HTTP server
hyper = { version = "1", features = ["server", "http1"] }

# CLI
clap = { version = "4", features = ["derive"] }

# TOML parsing
toml = "0.8"

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"

[dev-dependencies]
criterion = "0.5"
tempfile = "3"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true

[[bench]]
name = "cognitive_loop"
harness = false
```

---

## 6. Module Responsibilities

### 6.1 Responsibility Matrix

| Module | Primary Responsibility | Secondary Responsibility |
|---|---|---|
| `main.rs` | Entry point, CLI arg parsing | Dispatch to runtime modes |
| `cortex.rs` | Global orchestration, CortexRuntime struct | Subsystem initialization order |
| `config.rs` | TOML parsing, schema validation | Range/dependency/policy validation |
| `error.rs` | CortexError enum, error taxonomy | Error context, recovery hints |
| `runtime.rs` | State machine, boot sequence | Graceful shutdown, recovery |
| `language/mod.rs` | LanguageCore trait implementation | CLX orchestration |
| `language/tokenizer.rs` | Text → Symbol/Token conversion | Normalization, segmentation |
| `language/vocabulary.rs` | Vocabulary storage, expansion | Frequency tracking, unknown handling |
| `language/syntax.rs` | Dependency parsing, role assignment | Structural context |
| `language/semantics.rs` | Concept/relation extraction | Semantic graph construction |
| `language/language_model.rs` | Next-token prediction | Candidate scoring |
| `language/decoder.rs` | Meaning → text realization | Lexical selection, syntax generation |
| `language/context.rs` | Hierarchical context management | Context window, context scoring |
| `neural/mod.rs` | NeuralCore trait implementation | CNS orchestration |
| `neural/cell.rs` | Cell state machine, activation | Inhibition, prediction, adaptation |
| `neural/column.rs` | Column competition, sparse selection | Routing, column representation |
| `neural/field.rs` | Field management, global context | Field-level prediction |
| `neural/temporal.rs` | Temporal encoding, sequence | Transition, recurrence detection |
| `neural/plasticity.rs` | Weight update rules | Bounded plasticity enforcement |
| `memory/mod.rs` | MemorySystem trait implementation | Memory orchestration |
| `memory/working.rs` | Active state management | Context, hypotheses, goals |
| `memory/episodic.rs` | Episode storage, retrieval | Importance scoring |
| `memory/semantic.rs` | Knowledge storage, retrieval | Evidence, confidence tracking |
| `memory/procedural.rs` | Procedure storage, retrieval | Success/failure tracking |
| `memory/associative.rs` | Association storage, retrieval | Strength, context tracking |
| `memory/retrieval.rs` | Query → ranked memory set | Relevance scoring, filtering |
| `memory/consolidation.rs` | Episode → long-term memory | Merge, compress, generalize, forget |
| `world/mod.rs` | WorldModelInterface implementation | World orchestration |
| `world/entity.rs` | Entity CRUD, identity management | Property, state tracking |
| `world/transition.rs` | S(t)+A(t)→S(t+1) prediction | Transition model |
| `world/causal.rs` | Causal hypothesis management | Correlation vs causation |
| `world/simulation.rs` | Trajectory simulation | Counterfactual evaluation |
| `reasoning/mod.rs` | ReasoningEngine implementation | Reasoning orchestration |
| `reasoning/hypothesis.rs` | Hypothesis generation, evaluation | Ranking, evidence weighting |
| `reasoning/evidence.rs` | Evidence collection, scoring | Source quality assessment |
| `reasoning/contradiction.rs` | Contradiction detection | Conflict resolution strategies |
| `planning/mod.rs` | PlanningEngine implementation | Planning orchestration |
| `planning/plan.rs` | Plan construction, ranking | Step sequencing |
| `planning/risk.rs` | Risk estimation for plans | Utility evaluation |
| `verification/mod.rs` | VerificationEngine implementation | Claim verification pipeline |
| `verification/confidence.rs` | ConfidenceState computation | Multi-dimensional confidence |
| `learning/mod.rs` | LearningSystem implementation | Learning orchestration |
| `learning/signal.rs` | LearningSignal generation | Signal routing |
| `learning/attribution.rs` | Error attribution | Subsystem error routing |
| `learning/replay.rs` | Experience replay | Priority-based replay |
| `learning/stability.rs` | Stability guards | Catastrophic change prevention |
| `self_model/mod.rs` | SelfModelInterface implementation | Capability estimation |
| `self_model/capability.rs` | Performance tracking | Accuracy estimation |
| `policy/mod.rs` | PolicyEngine implementation | Policy orchestration |
| `policy/risk.rs` | Risk estimation | Risk scoring |
| `policy/gate.rs` | ALLOW/LIMIT/DENY decisions | Operation classification |
| `internet/mod.rs` | InternetInterface implementation | Internet orchestration |
| `internet/fetch.rs` | HTTP operations | Timeout, size enforcement |
| `internet/parse.rs` | Content extraction | Provenance attachment |
| `persistence/mod.rs` | PersistenceEngine implementation | Persistence orchestration |
| `persistence/format.rs` | .cx binary format | Section read/write |
| `persistence/checkpoint.rs` | Checkpoint creation, recovery | Checkpoint metadata |
| `persistence/migration.rs` | State version migration | Format compatibility |
| `api/mod.rs` | API server lifecycle | Route registration |
| `api/routes.rs` | Endpoint definitions | Method/path mapping |
| `api/auth.rs` | Bearer token validation | Key management |
| `api/handlers.rs` | Request → cognitive operation | Response construction |
| `cli/mod.rs` | CLI dispatch | Command routing |
| `cli/commands.rs` | Command implementations | Output formatting |
| `observability/mod.rs` | Metrics collection | Status reporting |
| `observability/diagnostics.rs` | Error diagnostics | Bounded diagnostic state |
| `types/mod.rs` | Type re-exports | Common type definitions |
| `types/ids.rs` | All ID newtypes | ID generation |
| `types/scalars.rs` | Scalar type definition | Precision handling |
| `types/state.rs` | CortexState, sub-states | State construction |
| `types/observation.rs` | Observation, Experience | Input representation |
| `types/evidence.rs` | Evidence, Provenance | Evidence set management |
| `types/common.rs` | Shared enums, structs | Timestamp, ContextState |

---

## 7. Public/Private Module APIs

### 7.1 Visibility Rules

| Visibility | Usage |
|---|---|
| `pub` | Trait definitions, primary types, orchestration functions |
| `pub(crate)` | Internal types shared across modules |
| Private (no modifier) | Implementation details, helper functions |

### 7.2 Public API Surface

Each subsystem module exposes:

```rust
// Public: trait definition
pub trait SubsystemInterface {
    fn primary_operation(&self, input: &InputType) -> Result<OutputType, CortexError>;
    fn secondary_operation(&mut self, signal: &SignalType) -> Result<(), CortexError>;
    fn state_accessor(&self) -> &StateType;
}

// Public: concrete implementation constructor
pub struct SubsystemImpl { /* private fields */ }

impl SubsystemImpl {
    pub fn new(config: &SubsystemConfig) -> Result<Self, CortexError>;
}

impl SubsystemInterface for SubsystemImpl {
    // trait implementation
}
```

### 7.3 Private Implementation Boundary

Internal implementation details are hidden:

```rust
// Private: internal helper
fn compute_internal_score(items: &[Item], weights: &Weights) -> Scalar {
    // implementation detail
}

// Private: internal state
struct InternalBuffer {
    data: Vec<u8>,
    position: usize,
}
```

---

## 8. Dependency Rules

### 8.1 Allowed Dependencies (Direction)

```
main.rs → cortex.rs → runtime.rs → [all subsystems]
                                   ↓
                              config.rs
                              error.rs
                              types/

language/ → types/, error.rs
neural/   → types/, error.rs, language/ (LanguageState input)
memory/   → types/, error.rs
world/    → types/, error.rs, memory/ (MemoryRetrieval input)
reasoning/→ types/, error.rs, memory/, world/
planning/ → types/, error.rs, world/, reasoning/
verification/ → types/, error.rs, reasoning/
learning/ → types/, error.rs, memory/, neural/, world/
self_model/ → types/, error.rs
policy/   → types/, error.rs
internet/ → types/, error.rs, policy/
persistence/ → types/, error.rs, format/
api/      → types/, error.rs, cortex.rs (runtime access)
cli/      → types/, error.rs, cortex.rs (runtime access)
```

### 8.2 Dependency Direction Rules

| Rule | Description |
|---|---|
| DEP-001 | Dependencies flow DOWNWARD: runtime → cognitive → infrastructure |
| DEP-002 | Cognitive subsystems depend on `types/` and `error.rs` |
| DEP-003 | Higher-level subsystems may depend on lower-level outputs (e.g., reasoning depends on memory output types) |
| DEP-004 | No circular dependencies between modules |
| DEP-005 | `types/` has NO dependencies on any subsystem |
| DEP-006 | `error.rs` has NO dependencies on any subsystem |
| DEP-007 | `config.rs` has NO dependencies on any subsystem |
| DEP-008 | Infrastructure (API, CLI) depends on runtime, not directly on subsystems |
| DEP-009 | Policy is a CROSS-CUTTING concern; accessed via trait object injection |
| DEP-010 | Persistence depends on state types, not on cognitive logic |

### 8.3 Forbidden Dependencies

| # | Forbidden Dependency | Reason |
|---|---|---|
| FD-001 | `types/` → any subsystem | Types are foundational; no upward dependency |
| FD-002 | `error.rs` → any subsystem | Error taxonomy is independent |
| FD-003 | `config.rs` → any subsystem | Configuration is parsed before subsystems exist |
| FD-004 | `language/` → `neural/` | Language produces LanguageState; neural consumes it; no reverse |
| FD-005 | `memory/` → `reasoning/` | Memory is passive storage; reasoning queries it |
| FD-006 | `persistence/` → any cognitive subsystem | Persistence serializes state; does not interpret it |
| FD-007 | `api/` → any cognitive subsystem directly | API goes through runtime orchestration |
| FD-008 | `cli/` → any cognitive subsystem directly | CLI goes through runtime orchestration |
| FD-009 | Any subsystem → `policy/` (direct mutation) | Policy is injected; not imported for mutation |
| FD-010 | Any subsystem → `internet/` (direct) | Internet access goes through policy gate |
| FD-011 | Circular dependencies between any two modules | Architectural invariant |
| FD-012 | External AI/ML framework dependencies | Cognitive substrate is native |
| FD-013 | External database driver dependencies | Memory is native |
| FD-014 | External agent framework dependencies | Autonomy is native |

---

## 9. Dependency Graph

### 9.1 Module Dependency Graph

```
                          ┌──────────┐
                          │ main.rs  │
                          └────┬─────┘
                               │
                          ┌────▼─────┐
                          │cortex.rs │
                          └────┬─────┘
                               │
              ┌────────────────┼────────────────┐
              │                │                │
         ┌────▼────┐     ┌────▼────┐     ┌────▼────┐
         │runtime  │     │config   │     │ error   │
         └────┬────┘     └─────────┘     └─────────┘
              │
    ┌─────────┼─────────────────────────────────────────┐
    │         │         │         │         │           │
┌───▼───┐ ┌──▼──┐ ┌────▼───┐ ┌──▼──┐ ┌────▼────┐ ┌───▼────┐
│language│ │neural│ │ memory │ │world│ │reasoning│ │planning│
└───┬───┘ └──┬──┘ └────┬───┘ └──┬──┘ └────┬────┘ └───┬────┘
    │         │         │         │         │           │
    │         │         │         │    ┌────▼────┐      │
    │         │         │         │    │verific. │      │
    │         │         │         │    └─────────┘      │
    │         │         │         │                     │
    └─────────┴─────────┴─────────┴─────────────────────┘
                               │
                          ┌────▼────┐
                          │learning │
                          └────┬────┘
                               │
              ┌────────────────┼────────────────┐
              │                │                │
         ┌────▼────┐     ┌────▼────┐     ┌────▼────┐
         │self_model│     │ policy  │     │internet │
         └─────────┘     └─────────┘     └─────────┘
                               │
              ┌────────────────┼────────────────┐
              │                │                │
         ┌────▼────┐     ┌────▼────┐     ┌────▼────┐
         │persist  │     │  api    │     │  cli    │
         └─────────┘     └─────────┘     └─────────┘

    ┌─────────────────────────────────────────────────┐
    │              types/ (foundational)               │
    │  ids.rs, scalars.rs, state.rs, observation.rs,  │
    │  evidence.rs, common.rs                         │
    └─────────────────────────────────────────────────┘
```

### 9.2 External Dependency Graph

```
cortex binary
├── serde + bincode     (serialization)
├── zstd               (compression)
├── blake3             (integrity)
├── uuid               (state identity)
├── tokio              (async I/O)
├── hyper              (HTTP server)
├── clap               (CLI parsing)
├── toml               (config parsing)
└── tracing            (logging)
```

---

## 10. Core Type-System Design

### 10.1 Foundational Types

```rust
// types/scalars.rs
pub type Scalar = f32;  // Default precision; configurable

// types/ids.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CellId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ColumnId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FieldId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EpisodeId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConceptId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProcedureId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AssociationId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HypothesisId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MemoryId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SourceId(pub u64);
```

### 10.2 Core State Types

```rust
// types/state.rs
#[derive(Serialize, Deserialize)]
pub struct CortexState {
    pub language: LanguageState,
    pub neural: NeuralState,
    pub memory: MemoryState,
    pub world: WorldState,
    pub reasoning: ReasoningState,
    pub planning: PlanningState,
    pub verification: VerificationState,
    pub learning: LearningState,
    pub self_model: SelfModel,
    pub provenance: ProvenanceState,
    pub metadata: StateMetadata,
}

#[derive(Serialize, Deserialize)]
pub struct StateMetadata {
    pub state_id: Uuid,
    pub created_at: Timestamp,
    pub last_updated: Timestamp,
    pub architecture_version: u32,
    pub algorithm_versions: AlgorithmVersions,
    pub config_hash: [u8; 32],
    pub episode_count: u64,
    pub total_learning_events: u64,
    pub checkpoint_count: u32,
}

#[derive(Serialize, Deserialize)]
pub struct AlgorithmVersions {
    pub cell_algorithm: u32,
    pub column_algorithm: u32,
    pub plasticity_algorithm: u32,
    pub memory_algorithm: u32,
    pub language_algorithm: u32,
    pub reasoning_algorithm: u32,
    pub planning_algorithm: u32,
    pub verification_algorithm: u32,
    pub consolidation_algorithm: u32,
}
```

### 10.3 Observation & Experience Types

```rust
// types/observation.rs
#[derive(Serialize, Deserialize)]
pub struct Observation {
    pub text: String,
    pub source: Provenance,
    pub timestamp: Timestamp,
    pub context: ContextState,
}

#[derive(Serialize, Deserialize)]
pub struct Experience {
    pub observation: Observation,
    pub internal_state: StateSnapshot,
    pub prediction: Prediction,
    pub action: Option<Action>,
    pub outcome: Option<Outcome>,
    pub error: PredictionError,
    pub attribution: ErrorAttribution,
    pub evidence: EvidenceSet,
    pub provenance: Provenance,
}

#[derive(Serialize, Deserialize)]
pub struct PredictionError {
    pub magnitude: Scalar,
    pub dimensions: HashMap<String, Scalar>,
    pub timestamp: Timestamp,
}

#[derive(Serialize, Deserialize)]
pub enum ErrorAttribution {
    InputError,
    MemoryError,
    WorldError,
    ReasoningError,
    ProcedureError,
    EnvironmentError,
}
```

### 10.4 Evidence & Provenance Types

```rust
// types/evidence.rs
#[derive(Serialize, Deserialize)]
pub struct Evidence {
    pub id: u64,
    pub source: Provenance,
    pub content: EvidenceContent,
    pub strength: Scalar,
    pub timestamp: Timestamp,
}

#[derive(Serialize, Deserialize)]
pub struct EvidenceSet {
    pub items: Vec<Evidence>,
    pub total_strength: Scalar,
}

#[derive(Serialize, Deserialize)]
pub struct Provenance {
    pub category: ProvenanceCategory,
    pub source: Source,
    pub source_identity: SourceIdentity,
    pub timestamp: Timestamp,
    pub retrieval_context: Option<RetrievalContext>,
    pub content_hash: [u8; 32],
    pub evidence: EvidenceSet,
    pub verification_status: VerificationStatus,
    pub confidence: ConfidenceState,
}

#[derive(Serialize, Deserialize)]
pub enum ProvenanceCategory {
    Observed,
    UserProvided,
    Internet,
    Derived,
    Inferred,
    Replayed,
    Verified,
}

#[derive(Serialize, Deserialize)]
pub enum VerificationStatus {
    Observed,
    Inferred,
    Supported,
    Provisional,
    Verified,
    Unknown,
    Contradicted,
}

#[derive(Serialize, Deserialize)]
pub struct ConfidenceState {
    pub belief: Scalar,
    pub evidence_strength: Scalar,
    pub source_quality: Scalar,
    pub consistency: Scalar,
    pub uncertainty: Scalar,
    pub prediction_reliability: Scalar,
    pub verification_status: VerificationStatus,
}
```

### 10.5 Common Types

```rust
// types/common.rs
#[derive(Serialize, Deserialize)]
pub struct Timestamp(pub u64);  // Unix timestamp in milliseconds

#[derive(Serialize, Deserialize)]
pub struct ContextState {
    pub conversation_id: Option<u64>,
    pub episode_context: Vec<EpisodeId>,
    pub active_concepts: Vec<ConceptId>,
    pub world_assumptions: Vec<EntityId>,
    pub temporal_context: TemporalContext,
}

#[derive(Serialize, Deserialize)]
pub struct TemporalContext {
    pub current_time: Timestamp,
    pub sequence_position: u64,
    pub prior_states: Vec<Timestamp>,
}

#[derive(Serialize, Deserialize)]
pub struct ComputeBudget {
    pub max_reasoning_steps: u32,
    pub max_planning_depth: u32,
    pub max_planning_branches: u32,
    pub max_simulation_steps: u32,
    pub max_generation_length: u32,
    pub max_memory_retrieval: u32,
    pub max_replay_count: u32,
}
```

---

## 11. Configuration Architecture

### 11.1 Configuration Parsing Pipeline

```
cortex.toml (file)
    │
    ↓
┌─────────────────────────┐
│  TOML Parsing (toml crate)│
└────────────┬────────────┘
             │
             ↓
┌─────────────────────────┐
│  Schema Validation       │
│  (field presence, types) │
└────────────┬────────────┘
             │
             ↓
┌─────────────────────────┐
│  Range Validation        │
│  (min/max bounds)        │
└────────────┬────────────┘
             │
             ↓
┌─────────────────────────┐
│  Dependency Validation   │
│  (cross-field constraints)│
└────────────┬────────────┘
             │
             ↓
┌─────────────────────────┐
│  Policy Validation       │
│  (security constraints)  │
└────────────┬────────────┘
             │
             ↓
┌─────────────────────────┐
│  CortexConfig (validated)│
└─────────────────────────┘
```

### 11.2 Configuration Struct

```rust
// config.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CortexConfig {
    pub model: ModelConfig,
    pub language: LanguageConfig,
    pub memory: MemoryConfig,
    pub learning: LearningConfig,
    pub world: WorldConfig,
    pub reasoning: ReasoningConfig,
    pub planning: PlanningConfig,
    pub verification: VerificationConfig,
    pub internet: InternetConfig,
    pub policy: PolicyConfig,
    pub api: ApiConfig,
    pub persistence: PersistenceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub cells: u32,           // min: 256
    pub columns: u32,         // min: 16
    pub dimension: u32,       // min: 64
    pub precision: Precision, // f32 | f16 | bf16
    pub sparsity_ratio: f32,  // (0, 1]
}

// ... (all other config structs follow same pattern)
```

### 11.3 Configuration Validation Rules

| Field | Constraint | Error on Violation |
|---|---|---|
| `model.cells` | ≥ 256 | `ConfigError::RangeViolation` |
| `model.columns` | ≥ 16 | `ConfigError::RangeViolation` |
| `model.dimension` | ≥ 64 | `ConfigError::RangeViolation` |
| `model.sparsity_ratio` | (0, 1] | `ConfigError::RangeViolation` |
| `language.vocabulary_capacity` | ≥ 256 | `ConfigError::RangeViolation` |
| `language.context_window` | ≥ 64 | `ConfigError::RangeViolation` |
| `language.generation_limit` | ≥ 32 | `ConfigError::RangeViolation` |
| `memory.working_mb` | ≥ 16 | `ConfigError::RangeViolation` |
| `memory.episodic_mb` | ≥ 32 | `ConfigError::RangeViolation` |
| `learning.learning_rate` | (0, 1] | `ConfigError::RangeViolation` |
| `learning.plasticity` | [0, 1] | `ConfigError::RangeViolation` |
| `verification.minimum_confidence` | [0, 1] | `ConfigError::RangeViolation` |
| `policy.self_modification` | false (default) | Warning if true |
| `policy.policy_modification` | false (default) | Warning if true |

### 11.4 Configuration Distribution

After validation, `CortexConfig` is distributed:

```rust
// Each subsystem receives its relevant config slice
let language_core = LanguageCoreImpl::new(&config.language)?;
let neural_core = NeuralCoreImpl::new(&config.model)?;
let memory_system = MemorySystemImpl::new(&config.memory)?;
let learning_system = LearningSystemImpl::new(&config.learning)?;
// ... etc.
```

Configuration is IMMUTABLE after boot. No runtime mutation of config.

---

## 12. Runtime Architecture

### 12.1 CortexRuntime Structure

```rust
// cortex.rs
pub struct CortexRuntime {
    pub state: CortexState,
    pub config: CortexConfig,
    pub policy: Box<dyn PolicyEngine>,
    pub persistence: Box<dyn PersistenceEngine>,
    
    // Cognitive subsystems
    pub language: Box<dyn LanguageCore>,
    pub neural: Box<dyn NeuralCore>,
    pub memory: Box<dyn MemorySystem>,
    pub world: Box<dyn WorldModelInterface>,
    pub reasoning: Box<dyn ReasoningEngine>,
    pub planning: Box<dyn PlanningEngine>,
    pub verification: Box<dyn VerificationEngine>,
    pub learning: Box<dyn LearningSystem>,
    pub self_model: Box<dyn SelfModelInterface>,
    
    // Infrastructure
    pub internet: Option<Box<dyn InternetInterface>>,
    
    // Runtime state
    pub runtime_state: RuntimeState,
    pub budget: ComputeBudget,
}
```

### 12.2 Runtime Trait

```rust
// runtime.rs
pub trait Runtime {
    fn boot(config: CortexConfig) -> Result<Self, CortexError> where Self: Sized;
    fn ready(&self) -> bool;
    fn process(&mut self, input: Input) -> Result<Response, CortexError>;
    fn observe(&mut self, observation: Observation) -> Result<(), CortexError>;
    fn experience(&mut self, experience: Experience) -> Result<(), CortexError>;
    fn query(&self, query: CognitiveQuery) -> Result<CognitiveResponse, CortexError>;
    fn checkpoint(&self) -> Result<CheckpointId, CortexError>;
    fn status(&self) -> Result<RuntimeStatus, CortexError>;
    fn shutdown(&mut self) -> Result<(), CortexError>;
}
```

---

## 13. Boot Sequence

### 13.1 Boot Pipeline

```rust
impl Runtime for CortexRuntime {
    fn boot(config: CortexConfig) -> Result<Self, CortexError> {
        // Phase 1: Configuration
        let validated_config = config.validate()?;
        
        // Phase 2: State Loading or Initialization
        let state = if Path::new(&validated_config.persistence.state).exists() {
            Self::load_state(&validated_config)?
        } else {
            Self::initialize_state(&validated_config)?
        };
        
        // Phase 3: Subsystem Initialization
        let language = LanguageCoreImpl::new(&validated_config.language)?;
        let neural = NeuralCoreImpl::new(&validated_config.model)?;
        let memory = MemorySystemImpl::new(&validated_config.memory)?;
        let world = WorldModelImpl::new(&validated_config.world)?;
        let reasoning = ReasoningEngineImpl::new(&validated_config.reasoning)?;
        let planning = PlanningEngineImpl::new(&validated_config.planning)?;
        let verification = VerificationEngineImpl::new(&validated_config.verification)?;
        let learning = LearningSystemImpl::new(&validated_config.learning)?;
        let self_model = SelfModelImpl::new()?;
        let policy = PolicyEngineImpl::new(&validated_config.policy)?;
        let persistence = PersistenceEngineImpl::new(&validated_config.persistence)?;
        
        // Phase 4: Internet (conditional)
        let internet = if validated_config.internet.enabled {
            Some(InternetInterfaceImpl::new(&validated_config.internet)?)
        } else {
            None
        };
        
        // Phase 5: Budget computation
        let budget = ComputeBudget::from_config(&validated_config);
        
        // Phase 6: State validation
        state.validate_invariants()?;
        
        // Phase 7: Runtime assembly
        Ok(CortexRuntime {
            state,
            config: validated_config,
            policy: Box::new(policy),
            persistence: Box::new(persistence),
            language: Box::new(language),
            neural: Box::new(neural),
            memory: Box::new(memory),
            world: Box::new(world),
            reasoning: Box::new(reasoning),
            planning: Box::new(planning),
            verification: Box::new(verification),
            learning: Box::new(learning),
            self_model: Box::new(self_model),
            internet,
            runtime_state: RuntimeState::Ready,
            budget,
        })
    }
}
```

### 13.2 First Boot Initialization

```rust
fn initialize_state(config: &CortexConfig) -> Result<CortexState, CortexError> {
    let state = CortexState {
        language: LanguageState::initial(config.language.vocabulary_capacity),
        neural: NeuralState::initial(config.model.cells, config.model.columns),
        memory: MemoryState::initial(&config.memory),
        world: WorldState::initial(),
        reasoning: ReasoningState::initial(),
        planning: PlanningState::initial(),
        verification: VerificationState::initial(),
        learning: LearningState::initial(),
        self_model: SelfModel::initial(),
        provenance: ProvenanceState::initial(),
        metadata: StateMetadata {
            state_id: Uuid::new_v4(),
            created_at: Timestamp::now(),
            last_updated: Timestamp::now(),
            architecture_version: ARCHITECTURE_VERSION,
            algorithm_versions: AlgorithmVersions::current(),
            config_hash: compute_config_hash(config)?,
            episode_count: 0,
            total_learning_events: 0,
            checkpoint_count: 0,
        },
    };
    
    // Persist initial state
    let persistence = PersistenceEngineImpl::new(&config.persistence)?;
    persistence.save(&state, Path::new(&config.persistence.state))?;
    
    Ok(state)
}
```

---

## 14. Runtime State Machine

### 14.1 State Definitions

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeState {
    Boot,
    LoadConfiguration,
    LoadState,
    Validate,
    Initialize,
    Ready,
    Processing,
    Learning,
    Consolidating,
    Checkpointing,
    Fault { error: CortexError },
    Recovery,
    SafeStop,
    Shutdown,
}
```

### 14.2 State Transitions

```rust
impl RuntimeState {
    pub fn transition(&mut self, event: RuntimeEvent) -> Result<(), CortexError> {
        let new_state = match (&self, event) {
            (RuntimeState::Boot, RuntimeEvent::ConfigLoaded) => RuntimeState::LoadConfiguration,
            (RuntimeState::LoadConfiguration, RuntimeEvent::StateLoaded) => RuntimeState::LoadState,
            (RuntimeState::LoadState, RuntimeEvent::StateValidated) => RuntimeState::Validate,
            (RuntimeState::Validate, RuntimeEvent::Initialized) => RuntimeState::Initialize,
            (RuntimeState::Initialize, RuntimeEvent::Ready) => RuntimeState::Ready,
            (RuntimeState::Ready, RuntimeEvent::InputReceived) => RuntimeState::Processing,
            (RuntimeState::Processing, RuntimeEvent::ProcessingComplete) => RuntimeState::Learning,
            (RuntimeState::Learning, RuntimeEvent::LearningComplete) => RuntimeState::Consolidating,
            (RuntimeState::Consolidating, RuntimeEvent::ConsolidationComplete) => RuntimeState::Checkpointing,
            (RuntimeState::Checkpointing, RuntimeEvent::CheckpointComplete) => RuntimeState::Ready,
            (RuntimeState::Ready, RuntimeEvent::ShutdownRequested) => RuntimeState::Shutdown,
            (_, RuntimeEvent::FatalError(e)) => RuntimeState::Fault { error: e },
            (RuntimeState::Fault { .. }, RuntimeEvent::RecoveryPossible) => RuntimeState::Recovery,
            (RuntimeState::Recovery, RuntimeEvent::RecoveryComplete) => RuntimeState::Ready,
            (RuntimeState::Recovery, RuntimeEvent::RecoveryFailed) => RuntimeState::SafeStop,
            (RuntimeState::Fault { .. }, RuntimeEvent::RecoveryImpossible) => RuntimeState::SafeStop,
            _ => return Err(CortexError::RuntimeError("Invalid state transition".into())),
        };
        *self = new_state;
        Ok(())
    }
}
```

### 14.3 State Machine Diagram

```
BOOT → LOAD_CONFIG → LOAD_STATE → VALIDATE → INITIALIZE → READY
                                                              │
                                                              ↓
                                                         PROCESSING
                                                              │
                                                              ↓
                                                          LEARNING
                                                              │
                                                              ↓
                                                       CONSOLIDATING
                                                              │
                                                              ↓
                                                       CHECKPOINTING
                                                              │
                                                              ↓
                                                           READY ←──┐
                                                              │      │
                                                              └──────┘

ANY STATE ──FatalError──→ FAULT ──RecoveryPossible──→ RECOVERY ──→ READY
                              │
                              └──RecoveryImpossible──→ SAFE_STOP

READY ──ShutdownRequested──→ SHUTDOWN
```

---

## 15. Language Core Design

### 15.1 Language Core Architecture

```rust
// language/mod.rs
pub struct LanguageCoreImpl {
    tokenizer: Tokenizer,
    vocabulary: Vocabulary,
    syntax_engine: SyntaxEngine,
    semantic_engine: SemanticEngine,
    language_model: LanguageModel,
    decoder: Decoder,
    context_model: ContextModel,
    config: LanguageConfig,
}

impl LanguageCore for LanguageCoreImpl {
    fn encode(&self, input: &str, context: &ContextState) -> Result<LanguageState, CortexError> {
        // 1. Normalization
        let normalized = self.tokenizer.normalize(input);
        // 2. Segmentation
        let segments = self.tokenizer.segment(&normalized);
        // 3. Symbol encoding
        let symbols = self.tokenizer.encode_symbols(&segments);
        // 4. Token sequence
        let tokens = self.tokenizer.to_tokens(&symbols, &self.vocabulary);
        // 5. Lexical state
        let lexical = self.vocabulary.resolve(&tokens);
        // 6. Syntax analysis
        let syntax = self.syntax_engine.parse(&lexical)?;
        // 7. Semantic analysis
        let semantics = self.semantic_engine.extract(&syntax, &lexical)?;
        // 8. Context integration
        let context_state = self.context_model.integrate(context, &semantics);
        // 9. Intent detection
        let intent = self.detect_intent(&semantics, &context_state);
        
        Ok(LanguageState {
            symbols,
            tokens,
            concepts: semantics.concepts,
            entities: semantics.entities,
            relations: semantics.relations,
            syntax,
            semantics: semantics.graph,
            context: context_state,
            intent,
            confidence: ConfidenceState::from_encoding(&semantics),
        })
    }
    
    fn decode(&self, state: &LanguageState, meaning: &MeaningRepresentation) -> Result<String, CortexError> {
        // 1. Response planning
        let plan = self.decoder.plan_response(meaning);
        // 2. Lexical selection
        let lexemes = self.decoder.select_lexemes(&plan, &self.vocabulary);
        // 3. Syntax generation
        let syntax = self.decoder.generate_syntax(&lexemes);
        // 4. Semantic validation
        self.decoder.validate_semantics(&syntax, meaning)?;
        // 5. Token realization
        let tokens = self.decoder.realize_tokens(&syntax);
        // 6. Text output
        Ok(self.tokenizer.decode_tokens(&tokens))
    }
    
    fn generate(&self, meaning: &VerifiedResult) -> Result<GeneratedResponse, CortexError> {
        let meaning_repr = MeaningRepresentation::from_verified(meaning);
        let text = self.decode(&LanguageState::default(), &meaning_repr)?;
        Ok(GeneratedResponse {
            text,
            confidence: meaning.confidence.clone(),
            verification_status: meaning.verification_status,
        })
    }
    
    fn predict(&self, state: &LanguageState) -> Result<Vec<CandidateContinuation>, CortexError> {
        self.language_model.predict(state)
    }
    
    fn update(&mut self, learning_signal: &LearningSignal) -> Result<(), CortexError> {
        // Vocabulary expansion, pattern learning
        self.vocabulary.learn(&learning_signal)?;
        self.syntax_engine.learn(&learning_signal)?;
        self.semantic_engine.learn(&learning_signal)?;
        Ok(())
    }
    
    fn vocabulary_size(&self) -> usize { self.vocabulary.len() }
    fn context_window_size(&self) -> usize { self.config.context_window as usize }
}
```

### 15.2 Vocabulary Design

```rust
// language/vocabulary.rs
pub struct Vocabulary {
    symbols: HashMap<u32, Symbol>,
    token_to_id: HashMap<String, u32>,
    id_to_token: HashMap<u32, String>,
    next_id: u32,
    capacity: u32,
    frequency_tracker: FrequencyTracker,
}

impl Vocabulary {
    pub fn lookup_or_create(&mut self, token: &str) -> u32 {
        if let Some(&id) = self.token_to_id.get(token) {
            self.frequency_tracker.increment(id);
            id
        } else if self.next_id < self.capacity {
            let id = self.next_id;
            self.next_id += 1;
            self.token_to_id.insert(token.to_string(), id);
            self.id_to_token.insert(id, token.to_string());
            self.symbols.insert(id, Symbol::new(id, SymbolKind::Word));
            id
        } else {
            // Capacity reached: return unknown token ID
            0 // reserved unknown token
        }
    }
}
```

---

## 16. Neural Core Design

### 16.1 Neural Core Architecture

```rust
// neural/mod.rs
pub struct NeuralCoreImpl {
    fields: Vec<Field>,
    temporal_buffer: TemporalBuffer,
    prediction_state: PredictionState,
    config: ModelConfig,
}

impl NeuralCore for NeuralCoreImpl {
    fn process(&self, input: &LanguageState, context: &ContextState) -> Result<NeuralRepresentation, CortexError> {
        // 1. Input → initial cell activation
        let initial_activation = self.encode_input(input);
        
        // 2. Column processing (competition, sparse selection)
        let column_activations = self.process_columns(&initial_activation);
        
        // 3. Field-level integration
        let field_state = self.integrate_fields(&column_activations, context);
        
        // 4. Temporal encoding
        let temporal = self.temporal_buffer.encode(&field_state);
        
        // 5. Prediction generation
        let prediction = self.predict_from_state(&field_state, &temporal);
        
        Ok(NeuralRepresentation {
            active_cells: field_state.active_cells,
            active_columns: field_state.active_columns,
            field_activations: field_state.field_activations,
            temporal_encoding: temporal,
            prediction,
            confidence: ConfidenceState::from_neural(&field_state),
        })
    }
    
    fn predict(&self, state: &NeuralState) -> Result<Prediction, CortexError> {
        self.prediction_state.predict_next(state)
    }
    
    fn compute_error(&self, predicted: &Prediction, actual: &Observation) -> Result<PredictionError, CortexError> {
        let actual_repr = self.encode_observation(actual);
        let error = predicted.compare(&actual_repr);
        Ok(PredictionError {
            magnitude: error.magnitude(),
            dimensions: error.dimensions(),
            timestamp: Timestamp::now(),
        })
    }
    
    fn adapt(&mut self, error: &PredictionError, signal: &LearningSignal) -> Result<(), CortexError> {
        // Bounded plasticity: ΔW = η × A × C × E × V
        for field in &mut self.fields {
            field.apply_plasticity(error, signal, self.config.sparsity_ratio)?;
        }
        Ok(())
    }
    
    fn field_count(&self) -> usize { self.fields.len() }
    fn active_cells(&self) -> usize { /* count active */ 0 }
    fn active_columns(&self) -> usize { /* count active */ 0 }
}
```

### 16.2 Cell Design

```rust
// neural/cell.rs
#[derive(Serialize, Deserialize)]
pub struct Cell {
    pub id: CellId,
    pub state: CellState,
    pub activation: Scalar,
    pub context: Vec<Scalar>,       // ContextVector
    pub prediction: Vec<Scalar>,    // PredictionVector
    pub confidence: Scalar,
    pub plasticity: Scalar,
    pub connections: Connections,
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum CellState {
    Resting,
    Active,
    Inhibited,
    Learning,
    Predicting,
}

impl Cell {
    pub fn receive(&mut self, input: Scalar) {
        self.activation += input;
    }
    
    pub fn activate(&mut self, threshold: Scalar) {
        if self.activation >= threshold {
            self.state = CellState::Active;
        }
    }
    
    pub fn inhibit(&mut self) {
        self.state = CellState::Inhibited;
        self.activation = 0.0;
    }
    
    pub fn predict(&mut self, target: &[Scalar]) {
        self.state = CellState::Predicting;
        self.prediction = target.to_vec();
    }
    
    pub fn adapt(&mut self, error: Scalar, learning_rate: Scalar) {
        let delta = learning_rate * self.plasticity * error;
        // Bounded update
        let bounded_delta = delta.clamp(-0.1, 0.1);
        self.activation += bounded_delta;
        self.state = CellState::Learning;
    }
    
    pub fn decay(&mut self, rate: Scalar) {
        self.activation *= (1.0 - rate);
        if self.activation < 0.01 {
            self.state = CellState::Resting;
        }
    }
    
    pub fn reset(&mut self) {
        self.state = CellState::Resting;
        self.activation = 0.0;
        self.prediction.clear();
    }
}
```

### 16.3 Sparsity Enforcement

```rust
impl NeuralCoreImpl {
    fn enforce_sparsity(&self, activations: &mut Vec<(CellId, Scalar)>) {
        let max_active = (self.config.cells as f32 * self.config.sparsity_ratio) as usize;
        
        // Sort by activation descending
        activations.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        // Keep only top max_active
        activations.truncate(max_active);
    }
}
```

---

## 17. Memory Design

### 17.1 Memory System Architecture

```rust
// memory/mod.rs
pub struct MemorySystemImpl {
    working: WorkingMemory,
    episodic: EpisodicMemory,
    semantic: SemanticMemory,
    procedural: ProceduralMemory,
    associative: AssociativeMemory,
    config: MemoryConfig,
}

impl MemorySystem for MemorySystemImpl {
    fn store(&mut self, episode: Episode) -> Result<(), CortexError> {
        self.episodic.store(episode)?;
        Ok(())
    }
    
    fn retrieve(&self, query: &MemoryQuery, context: &ContextState) -> Result<MemoryRetrieval, CortexError> {
        let mut retrieval = MemoryRetrieval::default();
        
        match query.query_type {
            MemoryQueryType::Episodic | MemoryQueryType::All => {
                retrieval.episodic = self.episodic.retrieve(query, context)?;
            }
            MemoryQueryType::Semantic | MemoryQueryType::All => {
                retrieval.semantic = self.semantic.retrieve(query, context)?;
            }
            MemoryQueryType::Procedural | MemoryQueryType::All => {
                retrieval.procedural = self.procedural.retrieve(query, context)?;
            }
            MemoryQueryType::Associative | MemoryQueryType::All => {
                retrieval.associative = self.associative.retrieve(query, context)?;
            }
        }
        
        // Compute relevance scores
        retrieval.relevance_scores = self.compute_relevance(&retrieval, context);
        
        Ok(retrieval)
    }
    
    fn consolidate(&mut self) -> Result<ConsolidationResult, CortexError> {
        // Extract patterns from episodic → semantic/procedural candidates
        let candidates = self.episodic.consolidation_candidates()?;
        let mut result = ConsolidationResult::default();
        
        for candidate in candidates {
            match candidate.target {
                ConsolidationTarget::Semantic => {
                    self.semantic.integrate(candidate.knowledge)?;
                    result.semantic_integrations += 1;
                }
                ConsolidationTarget::Procedural => {
                    self.procedural.integrate(candidate.procedure)?;
                    result.procedural_integrations += 1;
                }
            }
        }
        
        Ok(result)
    }
    
    fn forget(&mut self, policy: &ForgettingPolicy) -> Result<ForgettingResult, CortexError> {
        let mut result = ForgettingResult::default();
        result.episodic_forgotten = self.episodic.forget(policy)?;
        result.semantic_forgotten = self.semantic.forget(policy)?;
        result.associative_forgotten = self.associative.forget(policy)?;
        Ok(result)
    }
    
    fn working_memory(&self) -> &WorkingMemory { &self.working }
    fn working_memory_mut(&mut self) -> &mut WorkingMemory { &mut self.working }
    fn episode_count(&self) -> usize { self.episodic.count() }
    fn knowledge_count(&self) -> usize { self.semantic.count() }
    fn memory_usage(&self) -> MemoryUsage { /* compute */ MemoryUsage::default() }
}
```

### 17.2 Memory Pressure Response

```rust
impl MemorySystemImpl {
    pub fn handle_pressure(&mut self, pressure: MemoryPressure) -> Result<(), CortexError> {
        match pressure {
            MemoryPressure::Low => { /* no action */ }
            MemoryPressure::Moderate => {
                self.consolidate()?;  // compress
            }
            MemoryPressure::High => {
                self.consolidate()?;  // compress
                self.forget(&ForgettingPolicy::aggressive())?;  // evict
            }
            MemoryPressure::Critical => {
                self.consolidate()?;
                self.forget(&ForgettingPolicy::emergency())?;  // aggressive forget
            }
        }
        Ok(())
    }
}
```

---

## 18. World Model Design

```rust
// world/mod.rs
pub struct WorldModelImpl {
    entities: HashMap<EntityId, Entity>,
    relations: Vec<Relation>,
    events: Vec<Event>,
    transitions: TransitionModel,
    causal_hypotheses: Vec<CausalHypothesis>,
    temporal_patterns: Vec<TemporalPattern>,
    uncertainty: UncertaintyState,
    config: WorldConfig,
}

impl WorldModelInterface for WorldModelImpl {
    fn integrate(&mut self, representation: &NeuralRepresentation, memories: &MemoryRetrieval) -> Result<WorldState, CortexError> {
        // Extract entities, relations from representation
        // Update existing entities, create new ones
        // Integrate memory context
        // Return current WorldState snapshot
        Ok(self.current_state())
    }
    
    fn predict_transition(&self, state: &WorldState, action: &Action) -> Result<PredictedState, CortexError> {
        self.transitions.predict(state, action, self.config.prediction_horizon)
    }
    
    fn observe(&mut self, observation: &Observation, provenance: &Provenance) -> Result<(), CortexError> {
        // Update world state based on observation
        // Track provenance
        Ok(())
    }
    
    fn simulate(&self, state: &WorldState, actions: &[Action]) -> Result<SimulatedTrajectory, CortexError> {
        let mut trajectory = SimulatedTrajectory::new();
        let mut current = state.clone();
        
        for action in actions.iter().take(self.config.prediction_horizon as usize) {
            let predicted = self.predict_transition(&current, action)?;
            trajectory.add_step(predicted.clone());
            current = predicted.to_world_state();
        }
        
        Ok(trajectory)
    }
    
    fn entity_count(&self) -> usize { self.entities.len() }
    fn relation_count(&self) -> usize { self.relations.len() }
}
```

---

## 19. Reasoning Design

```rust
// reasoning/mod.rs
pub struct ReasoningEngineImpl {
    config: ReasoningConfig,
}

impl ReasoningEngine for ReasoningEngineImpl {
    fn evaluate(&self, representation: &NeuralRepresentation, memories: &MemoryRetrieval, world: &WorldState) -> Result<ReasoningResult, CortexError> {
        // 1. Problem representation
        let problem = self.represent_problem(representation)?;
        
        // 2. Hypothesis generation
        let hypotheses = self.generate_hypotheses(&problem)?;
        
        // 3. Evidence evaluation (bounded by max_steps)
        let mut budget = self.config.max_steps;
        let mut evaluated = Vec::new();
        
        for hypothesis in hypotheses.iter().take(budget as usize) {
            let evaluation = self.evaluate_hypothesis(hypothesis, &memories.to_evidence())?;
            evaluated.push(evaluation);
            budget -= 1;
        }
        
        // 4. Contradiction detection
        let contradictions = self.detect_contradictions(&evaluated)?;
        
        // 5. Ranking
        let ranked = self.rank_hypotheses(&evaluated, &contradictions)?;
        
        Ok(ReasoningResult {
            hypotheses: ranked,
            contradictions,
            budget_remaining: budget,
        })
    }
    
    fn bounded_conclusion(&self, hypotheses: &[Hypothesis], budget: &ComputeBudget) -> Result<BoundedConclusion, CortexError> {
        // If budget exhausted, return best hypothesis with uncertainty
        if budget.max_reasoning_steps == 0 {
            return Ok(BoundedConclusion::uncertain());
        }
        // Otherwise, conclude from top hypothesis
        Ok(BoundedConclusion::from_hypotheses(hypotheses))
    }
}
```

---

## 20. Planning Design

```rust
// planning/mod.rs
pub struct PlanningEngineImpl {
    config: PlanningConfig,
}

impl PlanningEngine for PlanningEngineImpl {
    fn evaluate(&self, reasoning: &ReasoningResult, world: &WorldState) -> Result<Option<Plan>, CortexError> {
        if !self.config.enabled {
            return Ok(None);
        }
        
        // 1. Extract goal from reasoning
        let goal = reasoning.primary_goal()?;
        
        // 2. Generate candidate actions
        let candidates = self.generate_candidates(&goal, world)?;
        
        // 3. Simulate (bounded by max_depth × max_branches)
        let mut plans = Vec::new();
        for candidate in candidates.iter().take(self.config.max_branches as usize) {
            let plan = self.construct_plan(&goal, candidate, world)?;
            if plan.steps.len() <= self.config.max_depth as usize {
                plans.push(plan);
            }
        }
        
        // 4. Risk evaluation
        for plan in &mut plans {
            plan.estimated_risk = self.evaluate_risk(plan, world)?.score;
        }
        
        // 5. Rank and select
        plans.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        
        Ok(plans.into_iter().next())
    }
}
```

---

## 21. Verification Design

```rust
// verification/mod.rs
pub struct VerificationEngineImpl {
    config: VerificationConfig,
}

impl VerificationEngine for VerificationEngineImpl {
    fn evaluate(&self, reasoning: &ReasoningResult) -> Result<VerifiedResult, CortexError> {
        if !self.config.enabled {
            return Ok(VerifiedResult::provisional(reasoning));
        }
        
        let claim = reasoning.primary_claim()?;
        let evidence = reasoning.evidence_set();
        
        let verification = self.verify_claim(&claim, &evidence)?;
        
        Ok(VerifiedResult {
            claim,
            verification_status: verification.status,
            confidence: verification.confidence,
            evidence: evidence.clone(),
        })
    }
    
    fn verify_claim(&self, claim: &KnowledgeClaim, evidence: &EvidenceSet) -> Result<VerificationResult, CortexError> {
        // 1. Source evaluation
        let source_quality = self.evaluate_sources(evidence);
        
        // 2. Consistency analysis
        let consistency = self.check_consistency(claim, evidence);
        
        // 3. Independent evidence check
        let independent = self.count_independent_evidence(evidence);
        
        // 4. Contradiction analysis
        let contradictions = self.find_contradictions(claim, evidence);
        
        // 5. Confidence computation
        let confidence = ConfidenceState {
            belief: claim.confidence,
            evidence_strength: evidence.total_strength,
            source_quality,
            consistency,
            uncertainty: 1.0 - consistency,
            prediction_reliability: 0.0,
            verification_status: VerificationStatus::Unknown,
        };
        
        // 6. Status determination
        let status = if contradictions.is_empty() 
            && confidence.evidence_strength >= self.config.minimum_confidence 
            && independent >= 2 
        {
            VerificationStatus::Verified
        } else if evidence.total_strength > 0.5 {
            VerificationStatus::Supported
        } else {
            VerificationStatus::Provisional
        };
        
        Ok(VerificationResult { status, confidence })
    }
}
```

---

## 22. Continual Learning Design

### 22.1 Learning System

```rust
// learning/mod.rs
pub struct LearningSystemImpl {
    config: LearningConfig,
    total_events: u64,
    total_replay_events: u64,
    total_consolidation_events: u64,
    average_prediction_error: Scalar,
    replay_buffer: Vec<Episode>,
}

impl LearningSystem for LearningSystemImpl {
    fn record(&mut self, experience: &Experience) -> Result<LearningSignal, CortexError> {
        if !self.config.enabled {
            return Ok(LearningSignal::none());
        }
        
        // Generate learning signal from experience
        let signal = LearningSignal {
            prediction_error: experience.error.clone(),
            attribution: experience.attribution.clone(),
            evidence: experience.evidence.clone(),
            source: experience.provenance.clone(),
            magnitude: experience.error.magnitude,
        };
        
        self.total_events += 1;
        self.update_average_error(experience.error.magnitude);
        
        Ok(signal)
    }
    
    fn attribute_error(&self, error: &PredictionError) -> Result<ErrorAttribution, CortexError> {
        // Determine which subsystem caused the error
        // Based on error dimensions, context, and history
        Ok(ErrorAttribution::analyze(error))
    }
    
    fn apply_signal(&mut self, signal: &LearningSignal, policy: &PolicyState) -> Result<LearningResult, CortexError> {
        // Check policy
        if !policy.allows_learning() {
            return Ok(LearningResult::denied());
        }
        
        // Apply bounded update
        let result = LearningResult {
            applied: true,
            magnitude: signal.magnitude.min(self.config.learning_rate),
            attribution: signal.attribution.clone(),
        };
        
        Ok(result)
    }
    
    fn replay(&mut self, episodes: &[Episode], budget: &ComputeBudget) -> Result<ReplayResult, CortexError> {
        let max_replay = budget.max_replay_count as usize;
        let mut result = ReplayResult::default();
        
        // Priority-based replay
        let prioritized = self.prioritize_replay(episodes);
        
        for episode in prioritized.iter().take(max_replay) {
            // Reconstruct context, predict, compare, learn
            result.replayed += 1;
        }
        
        self.total_replay_events += result.replayed as u64;
        Ok(result)
    }
    
    fn consolidation_candidates(&self) -> Result<Vec<ConsolidationCandidate>, CortexError> {
        // Identify patterns ready for long-term consolidation
        Ok(self.replay_buffer
            .iter()
            .filter(|e| e.importance > 0.7)
            .map(|e| ConsolidationCandidate::from_episode(e))
            .collect())
    }
    
    fn learning_events(&self) -> u64 { self.total_events }
}
```

### 22.2 Plasticity Design

```rust
// learning/plasticity.rs (or neural/plasticity.rs)
pub struct PlasticityEngine {
    learning_rate: Scalar,
    plasticity: Scalar,
}

impl PlasticityEngine {
    /// ΔW = η × A × C × E × V
    pub fn compute_update(
        &self,
        activation_relationship: Scalar,  // A
        context_factor: Scalar,           // C
        prediction_error: Scalar,         // E
        evidence_confidence: Scalar,      // V
    ) -> Scalar {
        let delta = self.learning_rate 
            * activation_relationship 
            * context_factor 
            * prediction_error 
            * evidence_confidence;
        
        // Bounded update: prevent single observation from destabilizing
        delta.clamp(-self.plasticity, self.plasticity)
    }
}
```

### 22.3 Replay Design

```rust
// learning/replay.rs
pub struct ReplayEngine {
    buffer: Vec<Episode>,
    capacity: usize,
}

impl ReplayEngine {
    pub fn prioritize(&self, episodes: &[Episode]) -> Vec<(usize, Scalar)> {
        episodes.iter().enumerate()
            .map(|(i, e)| {
                let priority = e.prediction_error.magnitude * 0.4
                    + e.importance * 0.3
                    + (1.0 - e.confidence.belief) * 0.2
                    + e.novelty() * 0.1;
                (i, priority)
            })
            .collect()
    }
}
```

### 22.4 Consolidation Design

```rust
// memory/consolidation.rs
pub struct ConsolidationEngine {
    threshold: Scalar,
}

impl ConsolidationEngine {
    pub fn consolidate(
        &mut self,
        candidates: &[ConsolidationCandidate],
        policy: &PolicyState,
    ) -> Result<ConsolidationResult, CortexError> {
        let mut result = ConsolidationResult::default();
        
        for candidate in candidates {
            // Evaluate: is this ready for long-term memory?
            let evaluation = self.evaluate_candidate(candidate)?;
            
            if evaluation.should_consolidate && evaluation.confidence >= self.threshold {
                // Check: not a single anomalous event
                if candidate.supporting_episodes >= 3 {
                    result.consolidated += 1;
                    // Merge into semantic/procedural memory
                }
            }
        }
        
        Ok(result)
    }
    
    fn evaluate_candidate(&self, candidate: &ConsolidationCandidate) -> Result<EvaluationResult, CortexError> {
        Ok(EvaluationResult {
            should_consolidate: candidate.pattern_strength > 0.6,
            confidence: candidate.evidence_strength,
            risk: candidate.contradiction_risk,
        })
    }
}
```

---

## 23. Self Model Design

```rust
// self_model/mod.rs
pub struct SelfModelImpl {
    capabilities: CapabilitySet,
    limitations: LimitationSet,
    prediction_accuracy: Scalar,
    uncertainty: UncertaintyState,
    memory_health: MemoryHealth,
    language_capability: LanguageCapability,
    reasoning_performance: ReasoningPerformance,
    resource_state: ResourceState,
    learning_statistics: LearningStatistics,
    historical_performance: HistoricalPerformance,
}

impl SelfModelInterface for SelfModelImpl {
    fn estimate_capability(&self, capability: Capability) -> Result<CapabilityEstimate, CortexError> {
        match capability {
            Capability::Language => Ok(CapabilityEstimate {
                accuracy: self.language_capability.accuracy,
                confidence: self.language_capability.confidence,
            }),
            Capability::Prediction => Ok(CapabilityEstimate {
                accuracy: self.prediction_accuracy,
                confidence: 1.0 - self.uncertainty.level,
            }),
            Capability::Reasoning => Ok(CapabilityEstimate {
                accuracy: self.reasoning_performance.consistency,
                confidence: self.reasoning_performance.confidence,
            }),
            // ... other capabilities
        }
    }
    
    fn health_status(&self) -> Result<HealthStatus, CortexError> {
        Ok(HealthStatus {
            memory_pressure: self.memory_health.pressure,
            resource_availability: self.resource_state.available,
            prediction_reliability: self.prediction_accuracy,
            overall: self.compute_overall_health(),
        })
    }
    
    fn update(&mut self, metrics: &PerformanceMetrics) -> Result<(), CortexError> {
        self.prediction_accuracy = metrics.prediction_accuracy;
        self.memory_health = metrics.memory_health.clone();
        self.learning_statistics = metrics.learning_stats.clone();
        self.historical_performance.record(metrics);
        Ok(())
    }
}
```

---

## 24. Policy & Risk Gate Design

```rust
// policy/mod.rs
pub struct PolicyEngineImpl {
    config: PolicyConfig,
    risk_model: RiskModel,
}

impl PolicyEngine for PolicyEngineImpl {
    fn evaluate(&self, operation: &ProposedOperation) -> Result<PolicyDecision, CortexError> {
        // 1. Classify operation
        let classification = self.classify(operation);
        
        // 2. Estimate risk
        let risk = self.risk_estimate(operation)?;
        
        // 3. Policy evaluation
        let decision = match classification {
            OperationClass::CognitiveStateAdaptation => {
                if self.config.learning {
                    PolicyDecision::Allowed
                } else {
                    PolicyDecision::Denied { reason: DenialReason::LearningDisabled }
                }
            }
            OperationClass::AlgorithmAdaptation => {
                if self.config.self_modification {
                    PolicyDecision::Limited { constraints: OperationConstraints::bounded() }
                } else {
                    PolicyDecision::Denied { reason: DenialReason::SelfModificationDisabled }
                }
            }
            OperationClass::SecurityPolicyModification => {
                if self.config.policy_modification {
                    PolicyDecision::Limited { constraints: OperationConstraints::strict() }
                } else {
                    PolicyDecision::Denied { reason: DenialReason::PolicyModificationDisabled }
                }
            }
            OperationClass::RuntimeModification => {
                if self.config.runtime_modification {
                    PolicyDecision::Limited { constraints: OperationConstraints::strict() }
                } else {
                    PolicyDecision::Denied { reason: DenialReason::RuntimeModificationDisabled }
                }
            }
        };
        
        // 4. Risk override
        if risk.level >= RiskLevel::Critical {
            return Ok(PolicyDecision::Denied { reason: DenialReason::CriticalRisk });
        }
        
        Ok(decision)
    }
    
    fn risk_estimate(&self, operation: &ProposedOperation) -> Result<RiskEstimate, CortexError> {
        self.risk_model.estimate(operation)
    }
    
    fn is_allowed(&self, operation: &ProposedOperation) -> bool {
        matches!(self.evaluate(operation), Ok(PolicyDecision::Allowed))
    }
}
```

---

## 25. Internet Interface Design

```rust
// internet/mod.rs
pub struct InternetInterfaceImpl {
    config: InternetConfig,
    client: HttpClient,
}

impl InternetInterface for InternetInterfaceImpl {
    fn fetch(&self, request: &NetworkRequest, policy: &PolicyState) -> Result<NetworkObservation, CortexError> {
        if !self.config.enabled {
            return Err(CortexError::NetworkError("Internet disabled".into()));
        }
        
        // Policy check
        if !policy.allows_internet() {
            return Err(CortexError::PolicyError("Internet access denied by policy".into()));
        }
        
        // Fetch with timeout and size limit
        let response = self.client
            .timeout(Duration::from_secs(self.config.timeout_seconds))
            .max_size(self.config.max_response_mb * 1024 * 1024)
            .fetch(request)?;
        
        Ok(NetworkObservation {
            content: response.body,
            status: response.status,
            timestamp: Timestamp::now(),
        })
    }
    
    fn parse(&self, response: &NetworkResponse) -> Result<ExtractedContent, CortexError> {
        // Content extraction, cleaning
        Ok(ExtractedContent {
            text: response.extract_text(),
            metadata: response.metadata(),
        })
    }
    
    fn to_observation(&self, content: &ExtractedContent, provenance: &Provenance) -> Result<Observation, CortexError> {
        Ok(Observation {
            text: content.text.clone(),
            source: provenance.clone(),
            timestamp: Timestamp::now(),
            context: ContextState::default(),
        })
    }
}
```

---

## 26. Persistence Architecture

### 26.1 Persistence Engine

```rust
// persistence/mod.rs
pub struct PersistenceEngineImpl {
    config: PersistenceConfig,
    state_path: PathBuf,
    checkpoint_dir: PathBuf,
}

impl PersistenceEngine for PersistenceEngineImpl {
    fn save(&self, state: &CortexState, path: &Path) -> Result<SaveResult, CortexError> {
        // 1. Serialize
        let serialized = self.serialize(state)?;
        
        // 2. Compute integrity
        let checksum = self.compute_checksum(&serialized);
        
        // 3. Atomic write: temp → flush → verify → replace
        let temp_path = path.with_extension("tmp");
        self.write_atomic(&temp_path, &serialized, checksum)?;
        
        // 4. Verify written file
        self.verify_file(&temp_path)?;
        
        // 5. Atomic replace
        std::fs::rename(&temp_path, path)?;
        
        Ok(SaveResult { bytes_written: serialized.len(), checksum })
    }
    
    fn load(&self, path: &Path) -> Result<CortexState, CortexError> {
        // 1. Read file
        let data = std::fs::read(path)?;
        
        // 2. Verify integrity
        self.verify_integrity(&data)?;
        
        // 3. Check version
        let header = self.parse_header(&data)?;
        
        // 4. Migration if needed
        let data = if header.format_version < CURRENT_FORMAT_VERSION {
            self.migrate(&data, header.format_version)?
        } else {
            data
        };
        
        // 5. Deserialize
        let state = self.deserialize(&data)?;
        
        // 6. Validate
        state.validate_invariants()?;
        
        Ok(state)
    }
    
    fn maybe_checkpoint(&self, state: &CortexState, interval: u64) -> Result<Option<CheckpointId>, CortexError> {
        if state.metadata.episode_count % interval == 0 {
            let checkpoint_id = self.create_checkpoint(state)?;
            Ok(Some(checkpoint_id))
        } else {
            Ok(None)
        }
    }
    
    fn validate(&self, path: &Path) -> Result<ValidationResult, CortexError> {
        // Full validation pipeline
        Ok(ValidationResult::valid())
    }
    
    fn recover(&self, checkpoints: &[Path]) -> Result<CortexState, CortexError> {
        // Try each checkpoint in order (newest first)
        for checkpoint in checkpoints {
            match self.load(checkpoint) {
                Ok(state) => return Ok(state),
                Err(_) => continue,
            }
        }
        Err(CortexError::PersistenceError("No valid checkpoint found".into()))
    }
}
```

### 26.2 Atomic Write Implementation

```rust
impl PersistenceEngineImpl {
    fn write_atomic(&self, temp_path: &Path, data: &[u8], checksum: u128) -> Result<(), CortexError> {
        // Write to temp file
        let mut file = File::create(temp_path)?;
        file.write_all(data)?;
        file.flush()?;
        file.sync_all()?;  // Ensure data is on disk
        
        // Write checksum metadata
        self.write_checksum(temp_path, checksum)?;
        
        Ok(())
    }
}
```

---

## 27. `.cx` Format Architecture

### 27.1 Format Layout

```
┌─────────────────────────────────────────────────────────────────┐
│                        .cx FILE                                  │
├─────────────────────────────────────────────────────────────────┤
│  HEADER (fixed size)                                            │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ magic: [u8; 8]           = b"CORTEX\0\0"                  │  │
│  │ format_version: u32                                       │  │
│  │ architecture_version: u32                                 │  │
│  │ algorithm_version: u32                                    │  │
│  │ config_hash: [u8; 32]    (BLAKE3-256 of cortex.toml)    │  │
│  │ state_id: [u8; 16]       (UUID)                          │  │
│  │ created_at: u64          (timestamp)                      │  │
│  │ last_checkpoint: u64     (timestamp)                      │  │
│  │ section_count: u32                                        │  │
│  │ integrity: IntegrityMetadata                              │  │
│  └───────────────────────────────────────────────────────────┘  │
├─────────────────────────────────────────────────────────────────┤
│  SECTION TABLE (index of all sections)                          │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ For each section:                                         │  │
│  │   TYPE: u16                                               │  │
│  │   VERSION: u16                                            │  │
│  │   FLAGS: u32                                              │  │
│  │   OFFSET: u64                                             │  │
│  │   LENGTH: u64                                             │  │
│  │   CHECKSUM: u128                                          │  │
│  └───────────────────────────────────────────────────────────┘  │
├─────────────────────────────────────────────────────────────────┤
│  SECTION DATA (variable)                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ ARCHITECTURE section data                                  │  │
│  │ LANGUAGE section data                                      │  │
│  │ NEURAL section data                                        │  │
│  │ CELLS section data                                         │  │
│  │ COLUMNS section data                                       │  │
│  │ FIELDS section data                                        │  │
│  │ WORKING_MEMORY section data                                │  │
│  │ EPISODIC_MEMORY section data                               │  │
│  │ SEMANTIC_MEMORY section data                               │  │
│  │ PROCEDURAL_MEMORY section data                             │  │
│  │ ASSOCIATIVE_MEMORY section data                            │  │
│  │ WORLD_MODEL section data                                   │  │
│  │ REASONING section data                                     │  │
│  │ PLANNING section data                                      │  │
│  │ VERIFICATION section data                                  │  │
│  │ LEARNING section data                                      │  │
│  │ SELF_MODEL section data                                    │  │
│  │ PROVENANCE section data                                    │  │
│  │ CHECKPOINT_METADATA section data                           │  │
│  │ INTEGRITY section data                                     │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

### 27.2 Section Type IDs

| Section | Type ID |
|---|---|
| ARCHITECTURE | 0x0001 |
| LANGUAGE | 0x0002 |
| NEURAL | 0x0003 |
| CELLS | 0x0004 |
| COLUMNS | 0x0005 |
| FIELDS | 0x0006 |
| WORKING_MEMORY | 0x0007 |
| EPISODIC_MEMORY | 0x0008 |
| SEMANTIC_MEMORY | 0x0009 |
| PROCEDURAL_MEMORY | 0x000A |
| ASSOCIATIVE_MEMORY | 0x000B |
| WORLD_MODEL | 0x000C |
| REASONING | 0x000D |
| PLANNING | 0x000E |
| VERIFICATION | 0x000F |
| LEARNING | 0x0010 |
| SELF_MODEL | 0x0011 |
| PROVENANCE | 0x0012 |
| CHECKPOINT_METADATA | 0x0013 |
| INTEGRITY | 0x0014 |

### 27.3 Section Serialization

```rust
// persistence/format.rs
pub struct CxSection {
    pub section_type: u16,
    pub version: u16,
    pub flags: u32,
    pub offset: u64,
    pub length: u64,
    pub checksum: u128,
    pub data: Vec<u8>,
}

impl CxSection {
    pub fn serialize<T: Serialize>(&self, content: &T) -> Result<Vec<u8>, CortexError> {
        let data = bincode::serialize(content)?;
        let compressed = zstd::encode_all(&data[..], 3)?;
        Ok(compressed)
    }
    
    pub fn deserialize<T: DeserializeOwned>(&self) -> Result<T, CortexError> {
        let decompressed = zstd::decode_all(&self.data[..])?;
        let content = bincode::deserialize(&decompressed)?;
        Ok(content)
    }
}
```

---

## 28. State Versioning

### 28.1 Version Detection

```rust
// persistence/migration.rs
pub struct StateMigrator {
    migrations: HashMap<(u32, u32), Box<dyn Migration>>,
}

impl StateMigrator {
    pub fn migrate(&self, data: &[u8], from_version: u32, to_version: u32) -> Result<Vec<u8>, CortexError> {
        let mut current_data = data.to_vec();
        let mut current_version = from_version;
        
        while current_version < to_version {
            let next_version = current_version + 1;
            let migration = self.migrations.get(&(current_version, next_version))
                .ok_or(CortexError::PersistenceError(
                    format!("No migration path from v{} to v{}", current_version, next_version)
                ))?;
            
            current_data = migration.apply(&current_data)?;
            current_version = next_version;
        }
        
        Ok(current_data)
    }
}

pub trait Migration {
    fn apply(&self, data: &[u8]) -> Result<Vec<u8>, CortexError>;
    fn description(&self) -> &str;
}
```

### 28.2 Algorithm Version Tracking

```rust
impl CortexState {
    pub fn detect_algorithm_change(&self, new_versions: &AlgorithmVersions) -> Vec<AlgorithmChange> {
        let mut changes = Vec::new();
        
        if self.metadata.algorithm_versions.cell_algorithm != new_versions.cell_algorithm {
            changes.push(AlgorithmChange {
                component: "cell".into(),
                from: self.metadata.algorithm_versions.cell_algorithm,
                to: new_versions.cell_algorithm,
            });
        }
        // ... check all algorithm versions
        
        changes
    }
}
```

---

## 29. Provenance Architecture

```rust
// Provenance is embedded in every knowledge item
// Architecture ensures provenance is NEVER lost during mutations

pub struct ProvenanceTracker {
    records: Vec<Provenance>,
    source_registry: HashMap<SourceId, SourceInfo>,
}

impl ProvenanceTracker {
    pub fn track(&mut self, item: &mut dyn Provenanceable, source: Provenance) {
        item.set_provenance(source.clone());
        self.records.push(source);
    }
    
    pub fn merge_provenance(&self, existing: &Provenance, new: &Provenance) -> Provenance {
        // When knowledge is updated, merge provenance
        Provenance {
            category: new.category,
            source: new.source.clone(),
            timestamp: new.timestamp,
            evidence: existing.evidence.merge(&new.evidence),
            confidence: new.confidence,
            ..existing.clone()
        }
    }
}

pub trait Provenanceable {
    fn provenance(&self) -> &Provenance;
    fn set_provenance(&mut self, provenance: Provenance);
}
```

---

## 30. Error Architecture

### 30.1 Error Type

```rust
// error.rs
#[derive(Debug, thiserror::Error)]
pub enum CortexError {
    #[error("Input error: {0}")]
    InputError(String),
    
    #[error("Encoding error: {0}")]
    EncodingError(String),
    
    #[error("Language error: {0}")]
    LanguageError(String),
    
    #[error("Memory error: {0}")]
    MemoryError(String),
    
    #[error("World model error: {0}")]
    WorldModelError(String),
    
    #[error("Reasoning error: {0}")]
    ReasoningError(String),
    
    #[error("Planning error: {0}")]
    PlanningError(String),
    
    #[error("Verification error: {0}")]
    VerificationError(String),
    
    #[error("Learning error: {0}")]
    LearningError(String),
    
    #[error("Persistence error: {0}")]
    PersistenceError(String),
    
    #[error("Policy error: {0}")]
    PolicyError(String),
    
    #[error("Resource error: {0}")]
    ResourceError(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Runtime error: {0}")]
    RuntimeError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

impl CortexError {
    pub fn kind(&self) -> ErrorKind {
        match self {
            CortexError::InputError(_) => ErrorKind::InputError,
            CortexError::EncodingError(_) => ErrorKind::EncodingError,
            CortexError::LanguageError(_) => ErrorKind::LanguageError,
            CortexError::MemoryError(_) => ErrorKind::MemoryError,
            CortexError::WorldModelError(_) => ErrorKind::WorldModelError,
            CortexError::ReasoningError(_) => ErrorKind::ReasoningError,
            CortexError::PlanningError(_) => ErrorKind::PlanningError,
            CortexError::VerificationError(_) => ErrorKind::VerificationError,
            CortexError::LearningError(_) => ErrorKind::LearningError,
            CortexError::PersistenceError(_) => ErrorKind::PersistenceError,
            CortexError::PolicyError(_) => ErrorKind::PolicyError,
            CortexError::ResourceError(_) => ErrorKind::ResourceError,
            CortexError::NetworkError(_) => ErrorKind::NetworkError,
            CortexError::RuntimeError(_) => ErrorKind::RuntimeError,
            CortexError::ConfigError(_) => ErrorKind::RuntimeError,
            CortexError::SerializationError(_) => ErrorKind::PersistenceError,
            CortexError::IoError(_) => ErrorKind::RuntimeError,
        }
    }
    
    pub fn is_recoverable(&self) -> bool {
        matches!(self, 
            CortexError::NetworkError(_) | 
            CortexError::ResourceError(_) |
            CortexError::InputError(_)
        )
    }
    
    pub fn is_fatal(&self) -> bool {
        matches!(self,
            CortexError::PersistenceError(_) if self.to_string().contains("corrupt"),
            CortexError::ConfigError(_)
        )
    }
}
```

### 30.2 Error Propagation

All fallible operations return `Result<T, CortexError>`. Errors propagate upward through `?` operator. The runtime catches errors at the cognitive loop boundary and decides:

```rust
impl CortexRuntime {
    fn process_with_recovery(&mut self, input: Input) -> Result<Response, CortexError> {
        match self.process(input) {
            Ok(response) => Ok(response),
            Err(e) if e.is_recoverable() => {
                self.log_error(&e);
                self.record_failed_observation(&e);
                Err(e)  // Return error to caller, but runtime continues
            }
            Err(e) if e.is_fatal() => {
                self.transition_to_fault(e)?;
                self.attempt_recovery()?;
                Err(e)
            }
            Err(e) => {
                self.transition_to_fault(e)?;
                Err(e)
            }
        }
    }
}
```

---

## 31. Resource Management

### 31.1 Resource Monitor

```rust
pub struct ResourceMonitor {
    memory_budget: MemoryBudget,
    compute_budget: ComputeBudget,
    current_usage: ResourceUsage,
}

impl ResourceMonitor {
    pub fn check_memory(&self) -> MemoryPressure {
        let total_used = self.current_usage.total_memory();
        let total_budget = self.memory_budget.total();
        
        let ratio = total_used as f64 / total_budget as f64;
        
        match ratio {
            r if r < 0.7 => MemoryPressure::Low,
            r if r < 0.85 => MemoryPressure::Moderate,
            r if r < 0.95 => MemoryPressure::High,
            _ => MemoryPressure::Critical,
        }
    }
    
    pub fn check_compute(&self, operation: &CognitiveOperation) -> bool {
        match operation {
            CognitiveOperation::Reasoning => self.compute_budget.max_reasoning_steps > 0,
            CognitiveOperation::Planning => self.compute_budget.max_planning_depth > 0,
            CognitiveOperation::Generation => self.compute_budget.max_generation_length > 0,
            _ => true,
        }
    }
}
```

### 31.2 Budget Enforcement

```rust
impl CortexRuntime {
    fn enforce_budget(&mut self, operation: CognitiveOperation) -> Result<(), CortexError> {
        if !self.resource_monitor.check_compute(&operation) {
            return Err(CortexError::ResourceError(
                format!("Budget exhausted for {:?}", operation)
            ));
        }
        Ok(())
    }
}
```

---

## 32. Concurrency Model

### 32.1 Concurrency Architecture

```rust
// Runtime uses tokio for async I/O, but cognitive loop is synchronous
pub struct CortexRuntime {
    // Synchronous cognitive state (single-threaded access)
    state: CortexState,
    
    // Async runtime for I/O operations
    io_runtime: tokio::runtime::Runtime,
    
    // Channel for background operations
    background_tx: mpsc::Sender<BackgroundTask>,
}

pub enum BackgroundTask {
    Checkpoint { state_snapshot: Vec<u8> },
    Consolidation { candidates: Vec<ConsolidationCandidate> },
    Replay { episodes: Vec<Episode> },
    NetworkFetch { request: NetworkRequest },
}
```

### 32.2 State Mutation Rules

| Rule | Implementation |
|---|---|
| Cognitive state mutation is single-threaded | `&mut self.state` only in main cognitive loop |
| Background tasks receive snapshots | Clone state before sending to background |
| Background results merge through channel | Results sent back, applied in main loop |
| `.cx` writes are atomic | Only one write at a time; file lock |
| API requests queue | Requests queued, processed sequentially in cognitive loop |

### 32.3 Thread Safety

```rust
// CortexRuntime is NOT Send/Sync (single-threaded cognitive state)
// Background tasks use cloned data
// Shared state uses Arc<Mutex<T>> only for non-cognitive data (metrics, logs)
```

---

## 33. API Architecture

### 33.1 API Server Design

```rust
// api/mod.rs
pub struct ApiServer {
    config: ApiConfig,
    runtime_handle: Arc<Mutex<CortexRuntime>>,
    auth: ApiAuthenticator,
}

impl ApiServer {
    pub async fn serve(self) -> Result<(), CortexError> {
        let listener = TcpListener::bind(&self.config.bind).await?;
        
        loop {
            let (stream, addr) = listener.accept().await?;
            let runtime = self.runtime_handle.clone();
            let auth = self.auth.clone();
            
            tokio::spawn(async move {
                handle_connection(stream, runtime, auth).await;
            });
        }
    }
}
```

### 33.2 Request Processing

```rust
async fn handle_inference(
    body: InferenceRequest,
    runtime: &mut CortexRuntime,
    auth: &ApiAuthenticator,
) -> Result<InferenceResponse, CortexError> {
    // 1. Authenticate
    auth.verify(&body.api_key)?;
    
    // 2. Validate input
    let input = Input::from_api_request(&body)?;
    
    // 3. Process through cognitive pipeline
    let response = runtime.process(input)?;
    
    // 4. Format response
    Ok(InferenceResponse {
        output: response.text,
        confidence: response.confidence.belief,
        verification_status: response.verification_status.to_string(),
        state_updated: true,
    })
}
```

### 33.3 API Safety Boundary

```rust
// All API mutations go through:
// API Request → Validated Command → Policy Check → Cognitive Operation → State Transition

impl ApiServer {
    fn process_mutation(&self, command: ApiCommand, runtime: &mut CortexRuntime) -> Result<(), CortexError> {
        // Policy check
        let operation = command.to_proposed_operation();
        let decision = runtime.policy.evaluate(&operation)?;
        
        match decision {
            PolicyDecision::Allowed => {
                runtime.apply_command(command)?;
                Ok(())
            }
            PolicyDecision::Limited { constraints } => {
                runtime.apply_command_bounded(command, constraints)?;
                Ok(())
            }
            PolicyDecision::Denied { reason } => {
                Err(CortexError::PolicyError(format!("Denied: {:?}", reason)))
            }
        }
    }
}
```

---

## 34. CLI Architecture

```rust
// cli/mod.rs
pub struct Cli {
    runtime: CortexRuntime,
}

impl Cli {
    pub fn dispatch(&mut self, command: CliCommand) -> Result<(), CortexError> {
        match command {
            CliCommand::Run => self.run(),
            CliCommand::Serve => self.serve(),
            CliCommand::Observe { text } => self.observe(&text),
            CliCommand::Experience { json } => self.experience(&json),
            CliCommand::Learn => self.learn(),
            CliCommand::Query { text } => self.query(&text),
            CliCommand::Inspect => self.inspect(),
            CliCommand::Verify { claim } => self.verify(&claim),
            CliCommand::Checkpoint => self.checkpoint(),
            CliCommand::Status => self.status(),
            CliCommand::Init => self.init(),
            CliCommand::Migrate => self.migrate(),
        }
    }
    
    fn run(&mut self) -> Result<(), CortexError> {
        // Interactive cognitive loop
        loop {
            let input = read_line()?;
            let response = self.runtime.process(Input::text(&input))?;
            println!("{}", response.text);
        }
    }
    
    fn observe(&mut self, text: &str) -> Result<(), CortexError> {
        let observation = Observation::user_provided(text);
        self.runtime.observe(observation)
    }
    
    fn status(&mut self) -> Result<(), CortexError> {
        let status = self.runtime.status()?;
        println!("{}", serde_json::to_string_pretty(&status)?);
        Ok(())
    }
}
```

---

## 35. Observability Architecture

```rust
// observability/mod.rs
pub struct ObservabilitySystem {
    metrics: MetricsCollector,
    diagnostics: DiagnosticState,
}

pub struct MetricsCollector {
    prediction_error: MovingAverage,
    memory_retrieval_success: Counter,
    knowledge_stability: Gauge,
    verification_confidence: MovingAverage,
    reasoning_consistency: MovingAverage,
    planning_success: Counter,
    language_prediction_quality: MovingAverage,
    learning_rate: Gauge,
    forgetting_rate: Counter,
    consolidation_rate: Counter,
}

pub struct DiagnosticState {
    last_errors: BoundedVec<CortexError>,  // Last N errors
    error_frequency: HashMap<ErrorKind, u64>,
    subsystem_errors: HashMap<String, u64>,
    severity_counts: HashMap<Severity, u64>,
    recovery_actions: Vec<RecoveryAction>,
}

impl DiagnosticState {
    pub fn record_error(&mut self, error: &CortexError) {
        self.last_errors.push(error.clone());
        *self.error_frequency.entry(error.kind()).or_insert(0) += 1;
        // Bounded: never grows unboundedly
        if self.last_errors.len() > 100 {
            self.last_errors.remove(0);
        }
    }
}
```

---

## 36. Security Architecture

### 36.1 Security Layers

```
┌─────────────────────────────────────────────────────────────────┐
│  Layer 1: Input Validation                                       │
│  All inputs validated before entering cognitive pipeline         │
├─────────────────────────────────────────────────────────────────┤
│  Layer 2: Authentication (API)                                   │
│  Bearer token required for all API endpoints                     │
├─────────────────────────────────────────────────────────────────┤
│  Layer 3: Policy Gate                                            │
│  All consequential operations pass through PolicyEngine          │
├─────────────────────────────────────────────────────────────────┤
│  Layer 4: State Invariants                                       │
│  Invalid state transitions fail before persistence               │
├─────────────────────────────────────────────────────────────────┤
│  Layer 5: Persistence Integrity                                  │
│  .cx checksum verification before load                           │
├─────────────────────────────────────────────────────────────────┤
│  Layer 6: Secret Isolation                                       │
│  API keys in environment only; never in .cx or cognitive state   │
└─────────────────────────────────────────────────────────────────┘
```

### 36.2 Policy Enforcement Point

```rust
// Every consequential operation passes through policy
impl CortexRuntime {
    fn gated_operation<T>(
        &mut self,
        operation: ProposedOperation,
        f: impl FnOnce(&mut Self) -> Result<T, CortexError>,
    ) -> Result<T, CortexError> {
        let decision = self.policy.evaluate(&operation)?;
        
        match decision {
            PolicyDecision::Allowed => f(self),
            PolicyDecision::Limited { constraints } => {
                self.apply_constraints(constraints);
                f(self)
            }
            PolicyDecision::Denied { reason } => {
                Err(CortexError::PolicyError(format!("Operation denied: {:?}", reason)))
            }
        }
    }
}
```

### 36.3 Secret Handling

```rust
// api/auth.rs
pub struct ApiAuthenticator {
    // Key read from environment at startup; NOT stored in state
    expected_key: String,
}

impl ApiAuthenticator {
    pub fn from_env(env_var: &str) -> Result<Self, CortexError> {
        let key = std::env::var(env_var)
            .map_err(|_| CortexError::ConfigError(format!("Missing {}", env_var)))?;
        Ok(Self { expected_key: key })
    }
    
    pub fn verify(&self, provided: &str) -> Result<(), CortexError> {
        if provided == self.expected_key {
            Ok(())
        } else {
            Err(CortexError::PolicyError("Invalid API key".into()))
        }
    }
}
// NOTE: expected_key is NEVER serialized to .cx
```

---

## 37. Testing Architecture

### 37.1 Test Hierarchy

```
tests/
├── unit/                    # Per-module unit tests
│   ├── language/
│   ├── neural/
│   ├── memory/
│   ├── world/
│   ├── reasoning/
│   ├── planning/
│   ├── verification/
│   ├── learning/
│   ├── policy/
│   └── persistence/
│
├── integration/             # Cross-subsystem integration tests
│   ├── cognitive_pipeline.rs
│   ├── persistence_roundtrip.rs
│   ├── learning_stability.rs
│   ├── security_policy.rs
│   ├── api_endpoints.rs
│   └── corruption_recovery.rs
│
├── regression/              # Algorithm change regression
│   ├── state_compatibility.rs
│   ├── memory_compatibility.rs
│   └── cx_migration.rs
│
└── stress/                  # Resource limit tests
    ├── memory_pressure.rs
    ├── compute_budget.rs
    └── concurrent_access.rs
```

### 37.2 Test Contracts

| Test Category | Contract |
|---|---|
| Persistence round-trip | `Save(State)` → `Load(State)` produces semantically equivalent state |
| Learning stability | Single observation does not destabilize complete state |
| Policy enforcement | Prohibited operations are denied |
| Corruption recovery | Corrupt `.cx` triggers recovery, not silent continuation |
| Resource bounds | Operations terminate at budget limits |
| API authentication | Unauthenticated requests rejected |
| Configuration validation | Invalid config prevents startup |

### 37.3 Test Helpers

```rust
// tests/helpers.rs
pub fn create_test_config() -> CortexConfig {
    // Minimal valid configuration for testing
}

pub fn create_test_state() -> CortexState {
    // Minimal valid state for testing
}

pub fn create_test_runtime() -> CortexRuntime {
    // Full runtime with test configuration
}

pub fn corrupt_cx_file(path: &Path) {
    // Corrupt a .cx file for recovery testing
}
```

---

## 38. Build Architecture

### 38.1 Build Pipeline

```
Source Code (src/)
    │
    ↓
┌─────────────────────────┐
│  cargo check             │  (type checking)
└────────────┬────────────┘
             │
             ↓
┌─────────────────────────┐
│  cargo clippy            │  (lint)
└────────────┬────────────┘
             │
             ↓
┌─────────────────────────┐
│  cargo test              │  (unit + integration tests)
└────────────┬────────────┘
             │
             ↓
┌─────────────────────────┐
│  cargo build --release   │  (optimized binary)
└────────────┬────────────┘
             │
             ↓
┌─────────────────────────┐
│  cortex binary           │  (single executable)
└─────────────────────────┘
```

### 38.2 Build Configuration

```toml
# rust-toolchain.toml
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]

# Cargo.toml [profile.release]
[profile.release]
opt-level = 3
lto = true           # Link-time optimization
codegen-units = 1    # Maximum optimization
strip = true         # Remove debug symbols
panic = "abort"      # Smaller binary, no unwinding
```

### 38.3 CI Pipeline

```
Push / PR
    │
    ├── cargo fmt --check
    ├── cargo clippy -- -D warnings
    ├── cargo test
    ├── cargo build --release
    ├── Integration tests
    ├── Persistence round-trip test
    ├── Security policy test
    └── Binary size check
```

---

## 39. Deployment Architecture

### 39.1 Deployment Artifact

```
Deployment Package:
├── cortex              # Single binary (~10-50 MB)
├── cortex.toml         # Configuration file
└── (optional) README.md
```

### 39.2 Deployment Procedure

```bash
# 1. Copy binary
cp cortex /opt/cortex/cortex
chmod +x /opt/cortex/cortex

# 2. Provide configuration
cp cortex.toml /opt/cortex/cortex.toml

# 3. Set API key (if API enabled)
export CORTEX_API_KEY="your-secret-key"

# 4. First run (auto-creates cortex.cx)
cd /opt/cortex
./cortex run

# Subsequent runs load existing cortex.cx
```

### 39.3 Deployment Validation

```bash
# Verify deployment
./cortex status
# Expected: {"status": "ready", ...}

# Verify first boot created state
ls -la cortex.cx
# Expected: cortex.cx exists

# Verify cognitive pipeline
echo "Hello" | ./cortex observe "Test observation"
./cortex query "What do you know?"
```

---

## 40. State Invariants

### 40.1 Invariant Enforcement

```rust
impl CortexState {
    pub fn validate_invariants(&self) -> Result<(), CortexError> {
        // 1. Valid memory references
        self.validate_memory_references()?;
        
        // 2. Valid neural topology
        self.validate_neural_topology()?;
        
        // 3. Valid vocabulary references
        self.validate_vocabulary_references()?;
        
        // 4. Valid world-model relationships
        self.validate_world_relationships()?;
        
        // 5. Valid provenance
        self.validate_provenance()?;
        
        // 6. Valid algorithm versions
        self.validate_algorithm_versions()?;
        
        // 7. Valid policy state
        self.validate_policy_state()?;
        
        // 8. Valid .cx structure
        self.validate_structure()?;
        
        Ok(())
    }
    
    fn validate_memory_references(&self) -> Result<(), CortexError> {
        // All EpisodeId references must exist in episodic memory
        // All ConceptId references must exist in semantic memory
        // All ProcedureId references must exist in procedural memory
        // All AssociationId references must have valid source/target
        Ok(())
    }
    
    fn validate_neural_topology(&self) -> Result<(), CortexError> {
        // All CellId must be within valid range
        // All ColumnId must reference existing columns
        // All FieldId must reference existing fields
        // Active cell count must not exceed sparsity bound
        Ok(())
    }
}
```

### 40.2 Invariant Enforcement Point

Invariants are checked:
1. Before every persistence write.
2. After every state load.
3. After every learning mutation.
4. After every consolidation.

If invariants fail → error is raised → state is NOT persisted → recovery is attempted.

---

## 41. Cross-Subsystem Contracts

### 41.1 Data Flow Contracts

| From → To | Data Type | Contract |
|---|---|---|
| Language → Neural | `LanguageState` | Valid tokens, syntax, semantics |
| Neural → Memory | `NeuralRepresentation` | Valid cell/column IDs, sparse activation |
| Memory → World | `MemoryRetrieval` | Ranked memories with provenance |
| World → Reasoning | `WorldState` | Valid entities, relations, uncertainty |
| Reasoning → Planning | `ReasoningResult` | Ranked hypotheses with evidence |
| Planning → Verification | `Plan` (optional) | Valid steps, risk assessment |
| Reasoning → Verification | `ReasoningResult` | Claims with evidence |
| Verification → Language | `VerifiedResult` | Status, confidence, claim |
| Language → Output | `GeneratedResponse` | Text, confidence, status |
| Experience → Learning | `Experience` | Observation, prediction, error, attribution |
| Learning → All | `LearningSignal` | Bounded, attributed, policy-respecting |
| All → Persistence | `CortexState` | Valid invariants |

### 41.2 Interface Contracts Summary

```rust
// Each subsystem exposes a trait. The runtime holds trait objects.
// This allows implementation replacement without changing orchestration.

pub trait LanguageCore { /* encode, decode, predict, generate, update */ }
pub trait NeuralCore { /* process, predict, compute_error, adapt */ }
pub trait MemorySystem { /* store, retrieve, consolidate, forget */ }
pub trait WorldModelInterface { /* integrate, predict_transition, observe, simulate */ }
pub trait ReasoningEngine { /* evaluate, generate_hypotheses, detect_contradictions */ }
pub trait PlanningEngine { /* evaluate, simulate_plan, evaluate_risk */ }
pub trait VerificationEngine { /* evaluate, verify_claim, confidence_dimensions */ }
pub trait LearningSystem { /* record, attribute_error, apply_signal, replay */ }
pub trait SelfModelInterface { /* estimate_capability, health_status, update */ }
pub trait PolicyEngine { /* evaluate, risk_estimate, is_allowed */ }
pub trait InternetInterface { /* fetch, parse, to_observation */ }
pub trait PersistenceEngine { /* save, load, maybe_checkpoint, validate, recover */ }
pub trait Runtime { /* boot, ready, process, observe, experience, query, checkpoint, status, shutdown */ }
```

---

## 42. Design Decisions

### 42.1 Decision Log

| # | Decision | Rationale | Alternatives Considered |
|---|---|---|---|
| DD-001 | Single binary deployment | Simplicity, no dependency management | Multi-binary, plugin system |
| DD-002 | Trait-based subsystem boundaries | Swappable implementations, testability | Concrete types only |
| DD-003 | Ownership-based state (no shared mutable) | Rust safety guarantees, no data races | Arc<Mutex> everywhere |
| DD-004 | Synchronous cognitive loop | Deterministic processing order | Async cognitive pipeline |
| DD-005 | tokio for I/O only | Non-blocking persistence/network without complicating cognition | Fully synchronous |
| DD-006 | bincode + zstd for .cx serialization | Fast, compact, schema-flexible | JSON, protobuf, custom binary |
| DD-007 | Distinct ID newtypes per entity | Compile-time prevention of ID confusion | Single u64 ID type |
| DD-008 | Policy as injected trait object | Testability, separation from learned state | Hardcoded policy checks |
| DD-009 | Configuration immutable after boot | Predictable behavior, no runtime surprises | Hot-reload configuration |
| DD-010 | `.cx` section-oriented format | Partial loading, targeted migration, recovery | Monolithic serialization |
| DD-011 | Provenance embedded in every knowledge item | Never lose origin during mutations | Separate provenance store |
| DD-012 | BoundedVec for diagnostics | Prevent unbounded memory growth | Unbounded log |
| DD-013 | f32 default Scalar | Balance precision and memory/CPU | f64, f16 |
| DD-014 | Enum-based state machines | Exhaustive matching, compiler-verified | String-based states |
| DD-015 | Error taxonomy as enum | Structured error handling, attribution | String errors |

### 42.2 Architectural Trade-offs

| Trade-off | Chosen Side | Reason |
|---|---|---|
| Performance vs. Safety | Safety (Rust ownership) | Cognitive state integrity is paramount |
| Flexibility vs. Simplicity | Simplicity (single binary) | Deployment contract requires single binary |
| Async vs. Sync cognition | Sync | Deterministic processing order |
| Dynamic vs. Static dispatch | Static where possible | Performance; dynamic for swappable algorithms |
| Granular vs. Coarse persistence | Coarse (single .cx) | Simplicity, atomicity |
| Strict vs. Lenient validation | Strict | Fail-before-persist invariant |

---

## 43. Open Technical Parameters

| Parameter | Current Design | Open Question | Resolution Path |
|---|---|---|---|
| Scalar precision | f32 | Should f16/bf16 be supported at runtime? | Benchmark on target hardware |
| Async runtime | tokio | Is tokio necessary, or is std::thread sufficient? | Benchmark I/O patterns |
| Serialization | bincode | Should we consider a more schema-evolvable format? | Evaluate migration complexity |
| Compression | zstd level 3 | Optimal compression level for .cx? | Benchmark size vs. speed |
| Thread pool size | CPU count | Optimal worker thread count? | Profile under load |
| Checkpoint frequency | Every 1000 episodes | Optimal interval for recovery vs. I/O? | Operational testing |
| Vocabulary capacity | 65536 | Sufficient for target domains? | Domain-specific evaluation |
| Context window | 4096 tokens | Adequate for complex conversations? | User testing |
| Sparsity ratio | 0.05 | Optimal for representation separation? | Neural experiment |
| Learning rate | 0.001 | Stability vs. adaptability? | Learning stability tests |
| Replay buffer size | Unbounded (within memory) | Should replay buffer have explicit size limit? | Memory pressure testing |
| API concurrency | Sequential processing | Should API support concurrent requests? | Load testing |

These parameters are exposed in configuration or as implementation constants. They do not represent architectural uncertainty but deployment-specific calibration.

---

## 44. Gap Resolution: Additional Design Specifications

The following subsections close gaps identified during cross-document audit.

### 44.1 API ↔ Cognitive-Loop Concurrency Design

```rust
// API requests are queued and processed sequentially in the cognitive loop.
// The cognitive loop is single-threaded; no concurrent state mutations.

pub struct CortexRuntime {
    // Synchronous cognitive state (single-threaded access)
    state: CortexState,
    
    // Channel for incoming API requests
    request_rx: mpsc::Receiver<ApiRequest>,
    
    // Channel for API responses
    response_tx: mpsc::Sender<ApiResponse>,
}

// Concurrency model:
// 1. API server accepts connections on async runtime (tokio)
// 2. Each request is serialized into an ApiRequest
// 3. ApiRequest is sent via channel to cognitive loop
// 4. Cognitive loop processes request synchronously
// 5. Response is sent back via channel
// 6. API server returns response to client
//
// This ensures:
// - No concurrent state mutations
// - Deterministic processing order
// - Single-threaded cognitive safety
// - API requests are serialized, not parallel
```

**API Request Processing Rules:**

| Rule | Description |
|---|---|
| API-CON-001 | API requests are queued, not processed concurrently |
| API-CON-002 | Cognitive loop processes one request at a time |
| API-CON-003 | Background tasks (consolidation, checkpoint) receive cloned state snapshots |
| API-CON-004 | Background task results are merged in the main cognitive loop |
| API-CON-005 | API timeout (30s) is enforced at the HTTP layer, not the cognitive layer |
| API-CON-006 | If cognitive loop is busy, API request waits in queue (bounded by max_connections) |

### 44.2 Configuration Validation Algorithm Design

```rust
// Configuration validation pipeline (deterministic, complete)

impl CortexConfig {
    pub fn validate(&self) -> Result<ValidatedConfig, ConfigError> {
        // 1. Schema validation: all required fields present, correct types
        self.validate_schema()?;
        
        // 2. Range validation: all numeric parameters within bounds
        self.validate_ranges()?;
        
        // 3. Dependency validation: cross-field constraints
        self.validate_dependencies()?;
        
        // 4. Policy validation: security constraints
        self.validate_policy()?;
        
        // 5. Compute derived values
        let derived = self.compute_derived();
        
        Ok(ValidatedConfig {
            config: self.clone(),
            derived,
        })
    }
    
    fn validate_ranges(&self) -> Result<(), ConfigError> {
        // model
        if self.model.cells < 256 { return Err(ConfigError::RangeViolation("model.cells".into())); }
        if self.model.columns < 16 { return Err(ConfigError::RangeViolation("model.columns".into())); }
        if self.model.dimension < 64 { return Err(ConfigError::RangeViolation("model.dimension".into())); }
        if self.model.sparsity_ratio <= 0.0 || self.model.sparsity_ratio > 1.0 {
            return Err(ConfigError::RangeViolation("model.sparsity_ratio".into()));
        }
        // language
        if self.language.vocabulary_capacity < 256 { return Err(ConfigError::RangeViolation("language.vocabulary_capacity".into())); }
        if self.language.context_window < 64 { return Err(ConfigError::RangeViolation("language.context_window".into())); }
        if self.language.generation_limit < 32 { return Err(ConfigError::RangeViolation("language.generation_limit".into())); }
        // memory
        if self.memory.working_mb < 16 { return Err(ConfigError::RangeViolation("memory.working_mb".into())); }
        if self.memory.episodic_mb < 32 { return Err(ConfigError::RangeViolation("memory.episodic_mb".into())); }
        if self.memory.semantic_mb < 32 { return Err(ConfigError::RangeViolation("memory.semantic_mb".into())); }
        if self.memory.procedural_mb < 16 { return Err(ConfigError::RangeViolation("memory.procedural_mb".into())); }
        if self.memory.associative_mb < 16 { return Err(ConfigError::RangeViolation("memory.associative_mb".into())); }
        // learning
        if self.learning.learning_rate <= 0.0 || self.learning.learning_rate > 1.0 {
            return Err(ConfigError::RangeViolation("learning.learning_rate".into()));
        }
        if self.learning.plasticity < 0.0 || self.learning.plasticity > 1.0 {
            return Err(ConfigError::RangeViolation("learning.plasticity".into()));
        }
        // verification
        if self.verification.minimum_confidence < 0.0 || self.verification.minimum_confidence > 1.0 {
            return Err(ConfigError::RangeViolation("verification.minimum_confidence".into()));
        }
        // reasoning
        if self.reasoning.max_steps < 1 { return Err(ConfigError::RangeViolation("reasoning.max_steps".into())); }
        // planning
        if self.planning.max_depth < 1 { return Err(ConfigError::RangeViolation("planning.max_depth".into())); }
        if self.planning.max_branches < 1 { return Err(ConfigError::RangeViolation("planning.max_branches".into())); }
        // world
        if self.world.prediction_horizon < 1 { return Err(ConfigError::RangeViolation("world.prediction_horizon".into())); }
        // internet
        if self.internet.timeout_seconds < 1 { return Err(ConfigError::RangeViolation("internet.timeout_seconds".into())); }
        if self.internet.max_response_mb < 1 { return Err(ConfigError::RangeViolation("internet.max_response_mb".into())); }
        // persistence
        if self.persistence.checkpoint_interval < 1 { return Err(ConfigError::RangeViolation("persistence.checkpoint_interval".into())); }
        Ok(())
    }
    
    fn validate_dependencies(&self) -> Result<(), ConfigError> {
        // cells must be divisible by columns
        if self.model.cells % self.model.columns != 0 {
            return Err(ConfigError::DependencyViolation("model.cells must be divisible by model.columns".into()));
        }
        // context_window must fit generation_limit
        if self.language.context_window < self.language.generation_limit {
            return Err(ConfigError::DependencyViolation("language.context_window must be >= language.generation_limit".into()));
        }
        Ok(())
    }
    
    fn validate_policy(&self) -> Result<(), ConfigError> {
        // Warn on dangerous policy settings
        if self.policy.self_modification {
            tracing::warn!("Self-modification Level 2 enabled");
        }
        if self.policy.policy_modification {
            tracing::warn!("Self-modification Level 3 (policy modification) enabled");
        }
        Ok(())
    }
}
```

### 44.3 Neural vs World Prediction Conflict Resolution Design

```rust
// When neural prediction and world model prediction disagree:
// 1. Both predictions are retained as competing hypotheses
// 2. Confidence-weighted combination is computed
// 3. Disagreement magnitude is recorded as uncertainty
// 4. Neither prediction is silently discarded

pub fn combine_predictions(
    neural_pred: &Prediction,
    world_pred: &Prediction,
) -> CombinedPrediction {
    let neural_weight = neural_pred.confidence;
    let world_weight = world_pred.confidence;
    let total_weight = neural_weight + world_weight;
    
    if total_weight < SCALAR_EPSILON {
        // Both predictions have near-zero confidence
        return CombinedPrediction::uncertain();
    }
    
    // Weighted combination
    let combined_state: Vec<Scalar> = neural_pred.predicted_state.iter()
        .zip(world_pred.predicted_state.iter())
        .map(|(n, w)| (n * neural_weight + w * world_weight) / total_weight)
        .collect();
    
    // Disagreement magnitude
    let disagreement: Scalar = neural_pred.predicted_state.iter()
        .zip(world_pred.predicted_state.iter())
        .map(|(n, w)| (n - w).powi(2))
        .sum::<Scalar>()
        .sqrt();
    
    // Combined confidence (reduced by disagreement)
    let agreement_factor = 1.0 - (disagreement / (combined_state.len() as Scalar).sqrt()).min(1.0);
    let combined_confidence = (neural_weight + world_weight) / 2.0 * agreement_factor;
    
    CombinedPrediction {
        predicted_state: combined_state,
        confidence: combined_confidence,
        neural_confidence: neural_weight,
        world_confidence: world_weight,
        disagreement,
    }
}
```

**Conflict Resolution Rules:**

| Rule | Description |
|---|---|
| PRED-001 | Neural and world predictions are always combined, never discarded |
| PRED-002 | Confidence weighting determines contribution of each source |
| PRED-003 | Disagreement magnitude reduces combined confidence |
| PRED-004 | If both predictions have near-zero confidence, result is uncertain |
| PRED-005 | Combined prediction is used for learning signal computation |

---

## 45. Design Completeness

### 44.1 Completeness Checklist

| Design Aspect | Status | Coverage |
|---|---|---|
| Module architecture | ✅ Complete | 71 modules defined with responsibilities |
| Type system | ✅ Complete | All core types, IDs, states defined |
| Dependency rules | ✅ Complete | Direction rules, forbidden dependencies |
| Configuration architecture | ✅ Complete | Parsing, validation, distribution |
| Runtime architecture | ✅ Complete | Boot, state machine, cognitive loop |
| Language Core design | ✅ Complete | Encode, decode, predict, generate, learn |
| Neural Core design | ✅ Complete | Cell, column, field, temporal, plasticity |
| Memory design | ✅ Complete | 5 subsystems, retrieval, consolidation, forgetting |
| World Model design | ✅ Complete | Entity, transition, causal, simulation |
| Reasoning design | ✅ Complete | Hypothesis, evidence, contradiction, ranking |
| Planning design | ✅ Complete | Goal, simulation, risk, ranking |
| Verification design | ✅ Complete | Claim, evidence, confidence, status |
| Learning design | ✅ Complete | Signal, attribution, replay, stability |
| Plasticity design | ✅ Complete | ΔW = η × A × C × E × V, bounded |
| Consolidation design | ✅ Complete | Pattern extraction, evidence evaluation |
| Self Model design | ✅ Complete | Capability estimation, health tracking |
| Policy design | ✅ Complete | Gate pipeline, risk model, levels |
| Internet design | ✅ Complete | Fetch, parse, provenance, policy |
| Persistence design | ✅ Complete | Atomic write, .cx format, checkpoint |
| .cx format design | ✅ Complete | Header, sections, integrity |
| State versioning | ✅ Complete | Migration path, algorithm versions |
| Provenance design | ✅ Complete | Embedded, mergeable, never lost |
| Error architecture | ✅ Complete | Taxonomy, propagation, recovery |
| Resource management | ✅ Complete | Budgets, pressure response |
| Concurrency model | ✅ Complete | Single-threaded cognition, async I/O |
| API architecture | ✅ Complete | Endpoints, auth, safety boundary |
| CLI architecture | ✅ Complete | Commands, dispatch |
| Observability | ✅ Complete | Metrics, diagnostics, bounded |
| Security architecture | ✅ Complete | 6 layers, policy enforcement, secret isolation |
| Testing architecture | ✅ Complete | Unit, integration, regression, stress |
| Build architecture | ✅ Complete | Pipeline, CI, release profile |
| Deployment architecture | ✅ Complete | Artifact, procedure, validation |
| State invariants | ✅ Complete | 8 invariant categories, enforcement points |
| Cross-subsystem contracts | ✅ Complete | Data flow, interface traits |
| Design decisions | ✅ Complete | 15 decisions with rationale |

### 44.2 Traceability to Requirements

| DOC-01 Requirement | DOC-02 Design Coverage |
|---|---|
| FR-LANG-* | §15 Language Core Design |
| FR-NEUR-* | §16 Neural Core Design |
| FR-MEM-* | §17 Memory Design |
| FR-WRLD-* | §18 World Model Design |
| FR-RSN-* | §19 Reasoning Design |
| FR-PLN-* | §20 Planning Design |
| FR-VER-* | §21 Verification Design |
| FR-LRN-* | §22 Continual Learning Design |
| FR-SLF-* | §23 Self Model Design |
| FR-POL-* | §24 Policy & Risk Gate Design |
| FR-INT-* | §25 Internet Interface Design |
| FR-PRS-* | §26-28 Persistence Architecture |
| FR-API-* | §33 API Architecture |
| FR-CLI-* | §34 CLI Architecture |
| REL-* | §40 State Invariants, §26 Persistence |
| SEC-* | §36 Security Architecture |
| ERR-* | §30 Error Architecture |
| AC-* | §37 Testing Architecture |

### 44.3 Final Design Statement

> **This document constitutes the software architecture contract for CORTEX.** It defines how the system is structured as software: 71 modules organized into 5 architectural layers, connected by trait-based interfaces, governed by explicit dependency rules, and protected by state invariants.
>
> The architecture ensures:
> - **Single-binary composition**: All modules compile into one `cortex` executable.
> - **Single-process execution**: All cognitive processing within one OS process.
> - **Ownership-based safety**: Rust's ownership model prevents data races and use-after-free.
> - **Trait-based swappability**: Algorithm implementations can change without architectural disruption.
> - **Bounded execution**: All cognitive operations have explicit resource limits.
> - **Fail-before-persist**: Invalid state never reaches disk.
> - **Provenance preservation**: Every knowledge mutation preserves origin.
> - **Policy separation**: Security boundary is architecturally distinct from learned state.
>
> **CORTEX software architecture: 71 modules, 5 layers, 12 trait interfaces, 1 binary, 1 process, 1 state file.**
>
> The repository structure is documented in **CORTEX-DOC-11 Repository Architecture & Structure** (§2-§14).

---

*End of Document — CORTEX-DOC-02 Software Design Specification v1.1.0*
