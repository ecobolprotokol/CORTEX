# CORTEX — 01 Technical Specification

---

## Document Control

| Property | Value |
|---|---|
| **Document ID** | CORTEX-DOC-01 |
| **Title** | Technical Specification |
| **Version** | 1.1.0 |
| **Status** | Final Architectural Baseline |
| **Classification** | System Contract |
| **Scope** | End-to-end technical requirements and boundaries |
| **Parent Document** | CORTEX Complete Technical Specification v1.0.0 |
| **Effective Date** | 2026-08-13 |
| **Review Cycle** | Per architecture version transition |

### Revision History

| Version | Date | Author | Description |
|---|---|---|---|
| 1.0.0 | 2026-08-13 | CORTEX Architecture | Initial final baseline |
| 1.1.0 | 2026-08-13 | CORTEX Architecture | Replace SHA-256 with BLAKE3-256 for all hashing operations |

### Approval

| Role | Signature | Date |
|---|---|---|
| System Architect | _____________ | _____________ |
| Engineering Lead | _____________ | _____________ |
| Security Review | _____________ | _____________ |

### Document Purpose

This document defines **what** CORTEX must build and the **technical boundaries** within which it must operate. It constitutes the system-level technical contract. All implementation, testing, deployment, and validation activities SHALL trace back to requirements defined herein.

### Document Scope

This specification covers:

- All functional and non-functional requirements of the CORTEX system.
- All technical constraints, boundaries, and limitations.
- All acceptance criteria for system validation.
- All external interface contracts.
- All resource, performance, reliability, and security requirements.

This specification does NOT cover:

- Internal algorithm selection or implementation detail (governed by algorithm-level documents).
- Phased development roadmap or MVP scoping.
- Proposal for future architecture beyond the defined baseline.

---

## 1. System Purpose & Scope

### 1.1 System Purpose

CORTEX is a **persistent, state-based, continually learning AI model** implemented entirely as a native Rust system. Its purpose is to provide a self-contained cognitive system capable of:

- Processing natural language input and generating natural language output.
- Maintaining persistent memory, knowledge, and world understanding across sessions.
- Learning continuously from experience without requiring external retraining.
- Reasoning, planning, and verifying claims within bounded computational resources.
- Operating as a single deployed binary with no external AI model dependency.

### 1.2 System Identity

| Attribute | Definition |
|---|---|
| **Project Name** | CORTEX |
| **System Type** | Native Continual-Learning AI Model |
| **Cognitive Substrate** | Native CORTEX Algorithms (no external AI) |
| **Language Substrate** | Native CORTEX Language Core (CLX) |
| **Neural Substrate** | Native CORTEX Neural Core (CNS) |
| **Deployment Unit** | Single binary (`cortex`) |
| **Persistent State** | Single file (`cortex.cx`) |
| **Configuration** | Single file (`cortex.toml`) |

### 1.3 Scope Boundary

#### 1.3.1 In Scope

| Domain | Included |
|---|---|
| Language processing | Full native pipeline: tokenization → syntax → semantics → generation |
| Neural computation | Cell, Column, Field architecture with sparse temporal representation |
| Memory | Working, Episodic, Semantic, Procedural, Associative |
| World modeling | Entity, relation, state, transition, causal hypothesis modeling |
| Reasoning | Hypothesis-based multi-type reasoning engine |
| Planning | Goal-directed bounded planning with simulation |
| Verification | Evidence-based claim verification with confidence |
| Learning | Continual, online, prediction-error-driven learning |
| Consolidation | Long-term memory formation and knowledge generalization |
| Self-model | Operational capability estimation and health tracking |
| Policy enforcement | Risk gate, operation classification, autonomy bounding |
| Persistence | Atomic `.cx` state save/load with integrity verification |
| API | Embedded HTTP API for inference, observation, query, learning |
| CLI | Full command-line operational interface |
| Internet interface | Bounded external observation with provenance tracking |

#### 1.3.2 Out of Scope

| Domain | Excluded |
|---|---|
| External AI model integration | No LLM, no embedding model, no external inference |
| External database | No SQL, NoSQL, or graph database dependency |
| Vector database | No external vector store |
| Agent framework | No LangChain, AutoGen, CrewAI, or equivalent |
| GPU compute | Not a primary target; CPU-first |
| Multi-process deployment | Single process only |
| Distributed operation | No clustering, no sharding |
| Web UI | Not included in this specification |
| Mobile deployment | Not a target platform |

### 1.4 Fundamental Architectural Assertion

> CORTEX is **not** an orchestration layer around another AI model. The cognitive substrate, language processing, memory, world modeling, reasoning, planning, verification, learning, self-model, persistence, and policy enforcement belong to CORTEX itself.

---

## 2. Functional Requirements

### 2.1 Language Processing

| ID | Requirement | Priority |
|---|---|---|
| FR-LANG-001 | CORTEX SHALL accept natural language text input and produce a structured `LanguageState` representation. | MUST |
| FR-LANG-002 | CORTEX SHALL tokenize input into symbols, subwords, words, and structural markers. | MUST |
| FR-LANG-003 | CORTEX SHALL maintain a dynamic vocabulary with capacity up to `language.vocabulary_capacity` (default: 65536). | MUST |
| FR-LANG-004 | CORTEX SHALL support unknown symbol discovery, frequency tracking, and vocabulary expansion without full model rebuild. | MUST |
| FR-LANG-005 | CORTEX SHALL parse syntactic structure including dependency, ordering, roles, nesting, scope, agreement, and structural context. | MUST |
| FR-LANG-006 | CORTEX SHALL construct semantic representations mapping linguistic structures to concepts, entities, relations, and properties. | MUST |
| FR-LANG-007 | CORTEX SHALL maintain hierarchical context: symbol, sentence, conversation, episode, semantic, world, and long-term. | MUST |
| FR-LANG-008 | CORTEX SHALL represent input intent as ranked hypotheses with confidence scores. | MUST |
| FR-LANG-009 | CORTEX SHALL predict candidate continuations using combined language, context, semantic, memory, world, and verification scores. | MUST |
| FR-LANG-010 | CORTEX SHALL generate natural language output from internal meaning representations through a structured realization pipeline. | MUST |
| FR-LANG-011 | CORTEX SHALL support a context window of up to `language.context_window` tokens (default: 4096). | MUST |
| FR-LANG-012 | CORTEX SHALL limit generation output to `language.generation_limit` tokens (default: 1024). | MUST |
| FR-LANG-013 | CORTEX SHALL learn new symbols, vocabulary, semantic associations, syntactic patterns, terminology, domain concepts, and discourse patterns from experience. | MUST |
| FR-LANG-014 | Vocabulary membership and semantic understanding SHALL be tracked as separate states. | MUST |
| FR-LANG-015 | When `language.enabled = false`, input SHALL be treated as raw observation; output limited to structured responses. | MUST |

### 2.2 Neural Processing

| ID | Requirement | Priority |
|---|---|---|
| FR-NEUR-001 | CORTEX SHALL process language and perceptual representations into sparse, temporally-aware neural representations. | MUST |
| FR-NEUR-002 | CORTEX SHALL implement a Cell as the fundamental computational unit with states: Resting, Active, Inhibited, Learning, Predicting. | MUST |
| FR-NEUR-003 | CORTEX SHALL organize Cells into Columns with local competition, sparse selection, and routing. | MUST |
| FR-NEUR-004 | CORTEX SHALL group Columns into Fields representing different learned structures. | MUST |
| FR-NEUR-005 | CORTEX SHALL enforce sparsity: `active_cells ≤ field_size × model.sparsity_ratio`. | MUST |
| FR-NEUR-006 | CORTEX SHALL produce temporal representations encoding sequence, transition, recurrence, context, event order, and temporal dependency. | MUST |
| FR-NEUR-007 | CORTEX SHALL generate predictions of next state as a first-class neural operation. | MUST |
| FR-NEUR-008 | CORTEX SHALL apply bounded plasticity: `ΔW = η × A × C × E × V`. No single observation may arbitrarily destabilize the complete neural state. | MUST |
| FR-NEUR-009 | CORTEX SHALL compute prediction error by comparing predicted state with actual observation. | MUST |

### 2.3 Memory System

| ID | Requirement | Priority |
|---|---|---|
| FR-MEM-001 | CORTEX SHALL maintain five memory subsystems: Working, Episodic, Semantic, Procedural, and Associative. | MUST |
| FR-MEM-002 | Working Memory SHALL hold current input, conversation context, active concepts, hypotheses, goals, reasoning state, world assumptions, and generation state, bounded by `memory.working_mb`. | MUST |
| FR-MEM-003 | Episodic Memory SHALL store experience episodes with observation, context, action, outcome, prediction, prediction error, confidence, source, importance, and timestamp, bounded by `memory.episodic_mb`. | MUST |
| FR-MEM-004 | Semantic Memory SHALL store revisable knowledge with concept, properties, relations, evidence, confidence, and provenance, bounded by `memory.semantic_mb`. | MUST |
| FR-MEM-005 | Procedural Memory SHALL store procedures with condition, steps, expected outcome, success/failure counts, confidence, context requirements, risk, and provenance, bounded by `memory.procedural_mb`. | MUST |
| FR-MEM-006 | Associative Memory SHALL store typed associations (Semantic, Temporal, Contextual, Causal, Episodic, Procedural) with strength, confidence, context, and provenance, bounded by `memory.associative_mb`. | MUST |
| FR-MEM-007 | CORTEX SHALL support memory retrieval with context analysis, relevance scoring, confidence filtering, and contradiction detection. | MUST |
| FR-MEM-008 | CORTEX SHALL support memory consolidation: merge, compress, strengthen, generalize, decay, forget. | MUST |
| FR-MEM-009 | CORTEX SHALL implement controlled forgetting based on importance, retrieval frequency, confidence, redundancy, age, memory pressure, and contradiction. | MUST |
| FR-MEM-010 | All retrieved memories SHALL preserve provenance and confidence. | MUST |
| FR-MEM-011 | Semantic Memory SHALL NOT contain unverifiable claims without their verification status. | MUST |

### 2.4 World Model

| ID | Requirement | Priority |
|---|---|---|
| FR-WRLD-001 | CORTEX SHALL maintain a world model with entities, properties, states, relations, events, transitions, temporal patterns, causal hypotheses, and uncertainty. | MUST |
| FR-WRLD-002 | CORTEX SHALL support entity kinds: Person, Object, Place, Organization, ConceptualObject, Event, System, Process. | MUST |
| FR-WRLD-003 | CORTEX SHALL predict state transitions: `S(t) + A(t) → Predicted S(t+1)`. | MUST |
| FR-WRLD-004 | CORTEX SHALL distinguish correlation, association, temporal relationship, causal hypothesis, and verified causal relationship. | MUST |
| FR-WRLD-005 | CORTEX SHALL support counterfactual world trajectories with explicit uncertainty. | MUST |
| FR-WRLD-006 | World state SHALL persist across sessions in `.cx`. | MUST |
| FR-WRLD-007 | When `world.enabled = false`, the world model SHALL return empty state; reasoning operates without world context. | MUST |

### 2.5 Reasoning

| ID | Requirement | Priority |
|---|---|---|
| FR-RSN-001 | CORTEX SHALL implement hypothesis-based reasoning with evidence evaluation, counter-evidence search, contradiction detection, and hypothesis ranking. | MUST |
| FR-RSN-002 | CORTEX SHALL support: deductive, inductive, abductive, analogical, temporal, causal, counterfactual, constraint, and consistency reasoning. | MUST |
| FR-RSN-003 | Reasoning SHALL be bounded by `reasoning.max_steps`. | MUST |
| FR-RSN-004 | No reasoning result SHALL automatically become verified knowledge. | MUST |
| FR-RSN-005 | When knowledge conflicts exist, CORTEX SHALL retain the conflict until resolved, evaluating source quality, recency, confirmation, consistency, context, and verification status. | MUST |
| FR-RSN-006 | When `reasoning.enabled = false`, hypothesis generation is skipped; conclusions based on direct memory retrieval and world state. | MUST |

### 2.6 Planning

| ID | Requirement | Priority |
|---|---|---|
| FR-PLN-001 | CORTEX SHALL perform goal-directed planning with world simulation, risk evaluation, and utility evaluation. | MUST |
| FR-PLN-002 | Planning SHALL be bounded by `planning.max_depth` and `planning.max_branches`. | MUST |
| FR-PLN-003 | Plans SHALL include goal, steps, predicted outcomes, estimated cost, estimated risk, uncertainty, and confidence. | MUST |
| FR-PLN-004 | When `planning.enabled = false`, no goal-directed planning occurs; responses based on immediate reasoning only. | MUST |

### 2.7 Verification

| ID | Requirement | Priority |
|---|---|---|
| FR-VER-001 | CORTEX SHALL classify claims into: Unknown, Observed, Inferred, Supported, Provisional, Verified, Contradicted. Transitions follow the matrix in DOC-00 §5. | MUST |
| FR-VER-002 | Verification SHALL evaluate: evidence retrieval, source evaluation, consistency analysis, independent evidence, contradiction analysis, and confidence update. | MUST |
| FR-VER-003 | Verification SHALL NOT silently upgrade UNKNOWN to VERIFIED without satisfying configured evidence conditions. | MUST |
| FR-VER-004 | Confidence and verification status SHALL be tracked as separate dimensions. | MUST |
| FR-VER-005 | The `verification.minimum_confidence` threshold (default: 0.80) SHALL gate the transition from Supported/Provisional to Verified. | MUST |
| FR-VER-006 | When `verification.enabled = false`, all claims remain provisional; `minimum_confidence` not applied. | MUST |

### 2.8 Continual Learning

| ID | Requirement | Priority |
|---|---|---|
| FR-LRN-001 | CORTEX SHALL learn from experience without requiring complete retraining. | MUST |
| FR-LRN-002 | CORTEX SHALL support three learning speeds: Fast (working state), Medium (episodic/semantic/procedural/world), Slow (neural/language/long-term consolidation). | MUST |
| FR-LRN-003 | CORTEX SHALL learn from: conversation, user information, environment observations, internet information, feedback, prediction errors, verified information, successful procedures, and failed procedures. | MUST |
| FR-LRN-004 | Prediction error SHALL be the principal learning signal. | MUST |
| FR-LRN-005 | CORTEX SHALL implement error attribution across: Input, Memory, World, Reasoning, Procedure, and Environment error sources. | MUST |
| FR-LRN-006 | CORTEX SHALL support experience replay with priority based on prediction error, novelty, importance, uncertainty, recurrence, and learning value. | MUST |
| FR-LRN-007 | Consolidation SHALL avoid allowing a single anomalous event to dominate long-term state. | MUST |
| FR-LRN-008 | All learning SHALL be: bounded, attributable, policy-respecting, resource-limit-respecting, provenance-preserving, and change-recording. | MUST |
| FR-LRN-009 | When `learning.enabled = false`, no state mutation from experience occurs; all learning signals discarded. | MUST |

### 2.9 Self Model

| ID | Requirement | Priority |
|---|---|---|
| FR-SLF-001 | CORTEX SHALL maintain a computational self-model of its own operational state. | MUST |
| FR-SLF-002 | The self-model SHALL track: capabilities, limitations, prediction accuracy, uncertainty, memory health, language capability, reasoning performance, resource state, learning statistics, and historical performance. | MUST |
| FR-SLF-003 | The self-model SHALL NOT be interpreted by the architecture as proof of consciousness or subjective experience. | MUST |
| FR-SLF-004 | The self-model SHALL NOT gain authority to change policy. | MUST |

### 2.10 Policy / Risk Gate

| ID | Requirement | Priority |
|---|---|---|
| FR-POL-001 | All potentially consequential operations SHALL pass through the Policy/Risk Gate. | MUST |
| FR-POL-002 | The gate SHALL classify operations, estimate risk, evaluate policy, and produce ALLOW / LIMIT / DENY decisions. | MUST |
| FR-POL-003 | Learning SHALL NOT modify: root policy, authorization boundary, security credentials, runtime executable, or policy enforcement code. | MUST |
| FR-POL-004 | Self-modification Level 3 (Security/Policy Modification) SHALL be restricted at the highest level. | MUST |
| FR-POL-005 | Policy SHALL be represented separately from learned knowledge. | MUST |
| FR-POL-006 | The planner SHALL NOT bypass the policy layer. | MUST |

### 2.11 Internet Interface

| ID | Requirement | Priority |
|---|---|---|
| FR-INT-001 | CORTEX SHALL treat external internet information as observation, not ground truth. | MUST |
| FR-INT-002 | All internet operations SHALL pass through the Policy Gate. | MUST |
| FR-INT-003 | Internet content SHALL carry full provenance. | MUST |
| FR-INT-004 | Internet operations SHALL respect `internet.timeout_seconds` and `internet.max_response_mb`. | MUST |
| FR-INT-005 | When `internet.enabled = false`, no network access occurs; internet observation pipeline disabled. | MUST |

### 2.12 Persistence

| ID | Requirement | Priority |
|---|---|---|
| FR-PRS-001 | CORTEX SHALL persist complete cognitive state to a single `.cx` binary file. | MUST |
| FR-PRS-002 | State writes SHALL use atomic strategy: write temp → flush → verify → atomic replace. | MUST |
| FR-PRS-003 | A failed write SHALL NOT silently destroy the last valid state. | MUST |
| FR-PRS-004 | CORTEX SHALL support periodic checkpointing at `persistence.checkpoint_interval`. | MUST |
| FR-PRS-005 | State loading SHALL verify integrity, version, and perform migration if required. | MUST |
| FR-PRS-006 | Invalid critical state SHALL trigger STOP or recovery from valid checkpoint, never silent continuation. | MUST |

### 2.13 API

| ID | Requirement | Priority |
|---|---|---|
| FR-API-001 | CORTEX SHALL provide an embedded HTTP API with endpoints: `/v1/inference`, `/v1/observe`, `/v1/experience`, `/v1/learn`, `/v1/query`, `/v1/status`, `/v1/checkpoint`. | MUST |
| FR-API-002 | API authentication SHALL use bearer token via environment variable `CORTEX_API_KEY`. | MUST |
| FR-API-003 | API requests SHALL NOT directly mutate arbitrary internal memory structures. | MUST |
| FR-API-004 | When `api.enabled = false`, no API server is started; only CLI is operational. | MUST |

### 2.14 CLI

| ID | Requirement | Priority |
|---|---|---|
| FR-CLI-001 | CORTEX SHALL provide CLI commands: `run`, `serve`, `observe`, `experience`, `learn`, `query`, `inspect`, `verify`, `checkpoint`, `status`, `init`, `migrate`. | MUST |

---

## 3. Non-Functional Requirements

### 3.1 Summary Matrix

| Category | Requirement Summary |
|---|---|
| Performance | Bounded cognitive operations; resource-aware degradation |
| Reliability | Atomic persistence; checkpoint recovery; fail-closed |
| Availability | Single-binary deployment; no external service dependency |
| Scalability | Bounded by configuration; not horizontally scalable |
| Security | Policy gate; fail-closed; secret isolation |
| Privacy | No external data transmission without policy; provenance tracking |
| Maintainability | Replaceable algorithms; versioned state; modular architecture |
| Portability | Linux x86_64 primary; single binary |
| Testability | Full test contract; regression; persistence round-trip |
| Observability | Runtime metrics; diagnostic state; cognitive metrics |

---

## 4. System Capabilities

### 4.1 Capability Matrix

| Capability | Description | Bounded By |
|---|---|---|
| Language Understanding | Parse, encode, and semantically interpret natural language | `language.vocabulary_capacity`, `language.context_window` |
| Language Generation | Produce natural language from internal meaning | `language.generation_limit` |
| Neural Representation | Sparse temporal neural encoding | `model.cells`, `model.columns`, `model.dimension`, `model.sparsity_ratio` |
| Prediction | Predict next state from current representation | Neural field size, temporal buffer |
| Memory Storage & Retrieval | Five-tier memory with provenance | `memory.*_mb` budgets |
| World Modeling | Entity, relation, state, transition, causal modeling | `world.prediction_horizon` |
| Reasoning | Multi-type hypothesis-based reasoning | `reasoning.max_steps` |
| Planning | Goal-directed action planning with simulation | `planning.max_depth`, `planning.max_branches` |
| Verification | Evidence-based claim verification | `verification.minimum_confidence` |
| Continual Learning | Online learning from experience | `learning.learning_rate`, `learning.plasticity` |
| Consolidation | Long-term knowledge formation | `learning.consolidation_interval` |
| Self-Assessment | Operational capability estimation | Self-model metrics |
| Policy Enforcement | Risk-bounded autonomy | Policy configuration |
| Internet Observation | Bounded external information acquisition | `internet.timeout_seconds`, `internet.max_response_mb` |
| Persistence | Atomic cognitive state save/load | `.cx` format |
| API Service | Embedded HTTP cognitive API | `api.bind` |
| CLI Operation | Full command-line interface | N/A |

### 4.2 Capability Boundaries

| Capability | Explicitly NOT Included |
|---|---|
| Language | No external LLM, no external tokenizer, no external embedding |
| Neural | No GPU inference, no external tensor library as cognitive substrate |
| Memory | No external database, no external vector store |
| Reasoning | No external theorem prover, no external SAT solver |
| Planning | No external planning service |
| Verification | No external fact-checking service |
| Learning | No external training pipeline, no external fine-tuning |

---

## 5. Input & Output Requirements

### 5.1 Input Requirements

| Input Type | Format | Source | Constraint |
|---|---|---|---|
| Natural language text | UTF-8 string | CLI, API | Bounded by `language.context_window` |
| Structured observation | JSON | API (`/v1/observe`) | Validated schema |
| Explicit experience | JSON | API (`/v1/experience`) | Validated schema |
| Cognitive query | JSON | API (`/v1/query`) | Validated schema |
| Configuration | TOML | Filesystem (`cortex.toml`) | Schema-validated |
| Persistent state | Binary `.cx` | Filesystem | Integrity-verified |
| Internet content | HTTP response | Network | Bounded by `internet.max_response_mb` |
| API key | String | Environment variable | Never persisted in `.cx` |

### 5.2 Output Requirements

| Output Type | Format | Destination | Constraint |
|---|---|---|---|
| Natural language response | UTF-8 string | CLI, API | Bounded by `language.generation_limit` |
| Structured response | JSON | API | Includes confidence, verification status |
| Status report | JSON | CLI, API | Runtime metrics |
| State inspection | Structured | CLI | Read-only |
| Persistent state | Binary `.cx` | Filesystem | Atomic write |
| Checkpoint | Binary `.cx` | Filesystem (`checkpoints/`) | Versioned |
| Verification result | Structured | Internal, API | Status + confidence |
| Error diagnostic | Structured | CLI, API, log | Bounded |

### 5.3 Input Validation

All inputs SHALL pass through:

```
Raw Input
  ↓
Format Validation
  ↓
Size Validation
  ↓
Policy Check
  ↓
Cognitive Pipeline
```

Invalid inputs SHALL produce a defined error response, never undefined behavior.

---

## 6. Supported Platforms

### 6.1 Primary Target

| Property | Value |
|---|---|
| **Operating System** | Linux |
| **Architecture** | x86_64 |
| **Minimum Kernel** | 5.10+ |
| **Filesystem** | POSIX-compliant with atomic rename support |
| **Networking** | TCP/IP stack |

### 6.2 Secondary Targets (Future Consideration)

| Platform | Status |
|---|---|
| Linux aarch64 | Possible; not baseline |
| macOS x86_64 / aarch64 | Possible; not baseline |
| FreeBSD x86_64 | Possible; not baseline |
| Windows x86_64 | Not planned for baseline |

### 6.3 Platform Constraints

- CORTEX SHALL NOT require a GPU.
- CORTEX SHALL NOT require a specific Linux distribution.
- CORTEX SHALL NOT require containerization for operation.
- CORTEX SHALL NOT require a specific init system.

---

## 7. Runtime Requirements

### 7.1 Execution Model

| Property | Requirement |
|---|---|
| Process model | Single process |
| Binary model | Single executable |
| Thread model | Internal concurrency permitted for I/O, network, background tasks |
| State mutation | Explicit synchronization or ownership-based state transition |
| External runtime | None required |
| JIT / interpreter | None |

### 7.2 Runtime State Machine

```
BOOT → LOAD_CONFIGURATION → LOAD_STATE → VALIDATE → INITIALIZE → READY
READY → PROCESSING → LEARNING → CONSOLIDATING → CHECKPOINTING → READY
ANY → FAULT → RECOVERY → READY
ANY → FAULT → SAFE STOP
```

### 7.3 First Boot Requirement

If `cortex.cx` does not exist, CORTEX SHALL:

1. Read and validate `cortex.toml`.
2. Initialize all subsystems (Language, Neural, Memory, World, Reasoning, Planning, Verification, Learning, Self Model, Policy).
3. Create initial cognitive state.
4. Persist `cortex.cx`.
5. Transition to READY.

> The system SHALL be operational without an externally supplied trained model. Initial state may have limited knowledge; capability grows through learning.

### 7.4 Graceful Shutdown

```
STOP ACCEPTING NEW WORK → FINISH SAFE OPERATIONS → CONSOLIDATE IF REQUIRED
→ CHECKPOINT → FLUSH → VERIFY → EXIT
```

Emergency shutdown MAY skip non-critical consolidation but SHALL attempt to preserve the last valid state.

### 7.5 Restart

```
Load Configuration → Load .cx → Verify Integrity → Restore State
→ Restore Algorithm Versions → Restore Learning State → Restore Memory
→ Restore World Model → READY
```

CORTEX SHALL NOT reset to an empty model unless explicitly instructed to initialize a new state.

---

## 8. Resource Requirements

### 8.1 Memory (RAM) Budget

| Component | Default Budget | Minimum |
|---|---|---|
| Working Memory | 128 MB | 16 MB |
| Episodic Memory | 512 MB | 32 MB |
| Semantic Memory | 512 MB | 32 MB |
| Procedural Memory | 256 MB | 16 MB |
| Associative Memory | 256 MB | 16 MB |
| Language Core | Implementation-defined | — |
| Neural Core | Implementation-defined | — |
| World Model | Implementation-defined | — |
| Reasoning | Implementation-defined | — |
| Runtime Cache | Implementation-defined | — |

**Total default memory budget: ~1,664 MB + Language + Neural + World + Reasoning + Runtime overhead.**

Memory pressure response: **compress → consolidate → evict → forget**.

CORTEX SHALL NOT assume unlimited memory.

### 8.2 Compute Budget

| Parameter | Source | Default |
|---|---|---|
| `max_reasoning_steps` | `reasoning.max_steps` | 32 |
| `max_planning_depth` | `planning.max_depth` | 8 |
| `max_planning_branches` | `planning.max_branches` | 16 |
| `max_simulation_steps` | `world.prediction_horizon` | 8 |
| `max_generation_length` | `language.generation_limit` | 1024 |
| `max_memory_retrieval` | Derived | min(counts)/4 |
| `max_replay_count` | Derived | max(1, consolidation_interval/10) |

A cognitive operation that reaches its budget SHALL terminate with an explicit bounded result.

### 8.3 Storage

| Artifact | Size Consideration |
|---|---|
| `cortex` binary | Implementation-defined |
| `cortex.toml` | < 16 KB |
| `cortex.cx` | Grows with learning; bounded by memory budgets |
| Checkpoints | Per checkpoint; bounded by checkpoint count policy |

### 8.4 Network

| Parameter | Default |
|---|---|
| Timeout | 15 seconds |
| Max response size | 4 MB |
| Concurrent connections | Implementation-defined |

### 8.5 Resource-Aware Cognition

High uncertainty + high reasoning cost + low available compute MAY result in:
- Bounded reasoning
- Lower plan depth
- Explicit uncertainty declaration

Rather than unbounded execution.

---

## 9. Performance Requirements

### 9.1 Latency Requirements

| Operation | Target | Constraint |
|---|---|---|
| Configuration load + validation | < 1 second | At startup |
| State load (`.cx`) | Proportional to state size | Bounded by I/O |
| Language encoding (per input) | Bounded by context window | CPU-bound |
| Full cognitive pipeline (single inference) | Bounded by compute budget | Resource-bounded |
| State save (`.cx`) | Atomic; proportional to state size | Must not block cognitive loop indefinitely |
| Checkpoint creation | Background; non-blocking where possible | Must not corrupt state |

### 9.2 Throughput Requirements

CORTEX is a single-process, CPU-first system. Throughput is bounded by:
- Single-process execution.
- CPU compute capacity.
- Memory bandwidth.
- Configuration-defined resource budgets.

CORTEX is NOT designed for high-throughput concurrent inference serving.

### 9.3 Performance Degradation

When resource limits are reached, CORTEX SHALL:
1. Return bounded results with explicit uncertainty.
2. Log resource pressure.
3. NOT silently drop operations.
4. NOT produce corrupt state.

---

## 10. Reliability & Availability Requirements

### 10.1 Reliability

| ID | Requirement |
|---|---|
| REL-001 | Atomic persistence SHALL guarantee no partial state writes. |
| REL-002 | A failed write SHALL preserve the last valid state. |
| REL-003 | Corrupt `.cx` SHALL trigger recovery from valid checkpoint, never silent continuation. |
| REL-004 | Recovery priority: Current Valid State → Latest Valid Checkpoint → Previous Valid Checkpoint → Initial State → Safe Stop. |
| REL-005 | State invariants (memory references, neural topology, vocabulary references, world-model relationships, provenance, algorithm versions, policy state, `.cx` structure) SHALL be preserved at all times. |
| REL-006 | Any invalid state transition SHALL fail before persistence. |

### 10.2 Availability

| ID | Requirement |
|---|---|
| AVAIL-001 | CORTEX SHALL be operational after deployment of a single binary + configuration. |
| AVAIL-002 | No external service dependency for core cognitive operation. |
| AVAIL-003 | Internet unavailability SHALL NOT prevent core cognitive operation. |
| AVAIL-004 | API unavailability SHALL NOT prevent CLI operation. |

### 10.3 Failure Handling

| Error Category | Response |
|---|---|
| Recoverable error | Log, continue |
| Cognitive error | Attribute, learn, continue |
| Input error | Reject with defined error |
| Network error | Record failed observation, continue |
| State corruption | Validate checkpoint, recover |
| Configuration error | Prevent startup |
| Policy violation | Deny operation |
| Resource exhaustion | Bounded result, log |
| Fatal runtime error | Safe stop with state preservation attempt |

### 10.4 Fail-Closed Behavior

Security-sensitive operations SHALL default to DENY when:
- Policy is ambiguous.
- Risk cannot be estimated.
- Authorization cannot be verified.
- State integrity cannot be confirmed.

---

## 11. Determinism Requirements

### 11.1 Deterministic Operations

The following SHALL be deterministic under identical conditions:

| Operation | Determinism |
|---|---|
| State serialization | Deterministic |
| Configuration interpretation | Deterministic |
| Algorithm selection | Deterministic |
| Memory indexing | Deterministic |
| Verification rules | Deterministic |
| Policy decisions | Deterministic |
| Checkpoint structure | Deterministic |

### 11.2 Non-Deterministic Operations

| Operation | Condition |
|---|---|
| Learning updates | MAY be stochastic if explicitly configured |
| Neural activation noise | MAY be present if configured |

### 11.3 Reproducibility

CORTEX SHALL record:
- Random seed
- Architecture version
- Algorithm versions
- Configuration hash
- State version
- Runtime version

This allows experiments to identify why two runs diverged.

---

## 12. Persistence Requirements

### 12.1 `.cx` Format Requirements

| ID | Requirement |
|---|---|
| PRS-001 | `.cx` SHALL be a binary, versioned, section-oriented cognitive state container. |
| PRS-002 | `.cx` SHALL contain sections: HEADER, ARCHITECTURE, LANGUAGE, NEURAL, CELLS, COLUMNS, FIELDS, WORKING_MEMORY, EPISODIC_MEMORY, SEMANTIC_MEMORY, PROCEDURAL_MEMORY, ASSOCIATIVE_MEMORY, WORLD_MODEL, REASONING, PLANNING, VERIFICATION, LEARNING, SELF_MODEL, PROVENANCE, CHECKPOINT_METADATA, INTEGRITY. |
| PRS-003 | Each section SHALL have: TYPE (u16), VERSION (u16), FLAGS (u32), OFFSET (u64), LENGTH (u64), CHECKSUM (u128), DATA (bytes). |
| PRS-004 | File header SHALL contain: magic (`b"CORTEX\0\0"`), format_version, architecture_version, algorithm_version, config_hash (BLAKE3-256), state_id (UUID), created_at, last_checkpoint, integrity metadata. |
| PRS-005 | `.cx` SHALL record algorithm versions for: cell, column, plasticity, memory, language, reasoning, planning, verification, consolidation. |
| PRS-006 | `.cx` SHALL support partial loading, validation, migration, recovery, and checkpointing. |

### 12.2 Save Contract

```
Runtime State → Serialization → Integrity Calculation → Atomic Write → cortex.cx
```

Atomic strategy: Write Temporary → Flush → Verify → Atomic Replace.

### 12.3 Load Contract

```
cortex.cx → Integrity Check → Version Check → Migration (if required)
→ State Validation → Runtime Reconstruction
```

### 12.4 State Versioning

Migration path: Old state → Version detection → Compatibility check → Migration → Validation → New state.

Migration SHALL preserve semantic state whenever technically possible.

### 12.5 Integrity Verification

Before loading: Checksum/integrity verification → Structural validation → Semantic validation → Load.

Invalid critical state → STOP or recovery. Never silent continuation.

### 12.6 Checkpointing

Checkpoint metadata SHALL include: state version, algorithm version, configuration hash, timestamp, episode count, learning state, integrity information.

### 12.7 Persistence Invariant

`Save(State)` → `Load(State)` SHALL produce a semantically equivalent cognitive state within defined serialization tolerances.

---

## 13. Configuration Requirements

### 13.1 Configuration File

| Property | Value |
|---|---|
| File | `cortex.toml` |
| Format | TOML |
| Location | Same directory as `cortex` binary (default) |
| Role | Defines HOW CORTEX operates |
| Mutability | Administrative only; learning SHALL NOT silently rewrite |

### 13.2 Configuration Sections

| Section | Purpose |
|---|---|
| `[model]` | Neural architecture: cells, columns, dimension, precision, sparsity |
| `[language]` | Language core: vocabulary, context window, generation limit, learning |
| `[memory]` | Memory budgets per subsystem |
| `[learning]` | Learning parameters: rate, plasticity, replay, consolidation |
| `[world]` | World model: prediction horizon |
| `[reasoning]` | Reasoning: max steps |
| `[planning]` | Planning: max depth, max branches |
| `[verification]` | Verification: minimum confidence |
| `[internet]` | Internet: timeout, max response |
| `[policy]` | Policy: learning, self-modification, runtime modification |
| `[api]` | API: enabled, bind address, key env var |
| `[persistence]` | Persistence: state path, checkpoint interval |

### 13.3 Configuration Validation Pipeline

```
Parse → Schema Validation → Range Validation → Dependency Validation
→ Policy Validation → Runtime Initialization
```

Invalid configuration SHALL prevent startup.

### 13.4 Configuration Immutability Boundary

| Belongs In | Content |
|---|---|
| `cortex.toml` | Architecture limits, resource limits, policy defaults, runtime behavior, persistence, API, learning parameters |
| `.cx` | Runtime state, learned knowledge, memory, world model |
| Environment variables | Secrets (API key) |

Learning SHALL NOT silently rewrite `cortex.toml`.

### 13.5 Disabled Subsystem Behavior

When a subsystem is disabled (`enabled = false`), it SHALL return a defined default (empty set, no-op, or passthrough) rather than causing undefined behavior. The cognitive pipeline adapts to skip disabled subsystems while maintaining valid data flow.

---

## 14. External Interface Requirements

### 14.1 Permitted External Infrastructure

| Infrastructure | Usage |
|---|---|
| Operating system | Process management, filesystem, networking |
| Filesystem | `.cx` persistence, configuration, checkpoints |
| Network / TCP/IP | Internet observation, API serving |
| Time | Timestamps, scheduling |
| Process scheduling | OS-level scheduling |

### 14.2 Permitted External Libraries

| Category | Examples | Constraint |
|---|---|---|
| Serialization | bincode, serde | Infrastructure only |
| Compression | zstd, lz4 | Infrastructure only |
| Cryptography | BLAKE3 | Integrity only |
| Networking | TCP/HTTP stack | API, internet |
| OS interaction | libc, tokio | Runtime infrastructure |

> External libraries SHALL NOT constitute the cognitive substrate.

### 14.3 Prohibited External Dependencies

| Prohibited | Reason |
|---|---|
| External AI model / LLM | Cognitive substrate is native |
| External database | Memory is native |
| Vector database | Representation is native |
| Agent framework | Autonomy is native |
| External reasoning engine | Reasoning is native |
| External memory service | Memory is native |
| External embedding server | Language core is native |

### 14.4 API Interface

| Method | Path | Auth | Description |
|---|---|---|---|
| POST | `/v1/inference` | Bearer | Process input, return response |
| POST | `/v1/observe` | Bearer | Submit observation |
| POST | `/v1/experience` | Bearer | Submit learning experience |
| POST | `/v1/learn` | Bearer | Trigger learning |
| POST | `/v1/query` | Bearer | Query cognitive state |
| GET | `/v1/status` | Bearer | Runtime status |
| POST | `/v1/checkpoint` | Bearer | Manual checkpoint |

API safety: External requests SHALL NOT directly mutate arbitrary internal state. All mutations pass through: Validated Command → Policy → Cognitive Operation → State Transition.

### 14.5 CLI Interface

| Command | Description |
|---|---|
| `cortex run` | Normal cognitive runtime |
| `cortex serve` | Embedded API server |
| `cortex observe <text>` | Submit observation |
| `cortex experience <json>` | Submit experience |
| `cortex learn` | Trigger learning cycle |
| `cortex query <text>` | Query cognitive state |
| `cortex inspect` | Inspect state |
| `cortex verify <claim>` | Verify claim |
| `cortex checkpoint` | Create checkpoint |
| `cortex status` | Show status |
| `cortex init` | Initialize new state |
| `cortex migrate` | Migrate state format |

---

## 15. Security Requirements

### 15.1 Security Boundary

| ID | Requirement |
|---|---|
| SEC-001 | Security-sensitive resources (API keys, policy, runtime executable, filesystem, network, persistent state) SHALL NOT be controlled solely by learned model output. |
| SEC-002 | API key SHALL be provided via environment variable, never persisted in `.cx`. |
| SEC-003 | Secret flow: Environment → Runtime → Authentication. NOT: Secret → Memory → `.cx`. |
| SEC-004 | `.cx` integrity SHALL be verified before loading. |
| SEC-005 | Fail-closed: ambiguous security decisions default to DENY. |
| SEC-006 | Policy is a non-learned boundary. The learned model cannot redefine the gate by generating a different internal belief. |
| SEC-007 | Normal continual learning SHALL NOT modify Level 3 (Security/Policy). |
| SEC-008 | Where configured, `.cx` MAY use authenticated integrity, encryption, and access control without changing cognitive architecture. |

### 15.2 Self-Modification Levels

| Level | Scope | Default |
|---|---|---|
| 1 — Cognitive State Adaptation | Memory, language state, world model, learned parameters, procedures, associations | Allowed |
| 2 — Algorithm Adaptation | Learning, reasoning, language, runtime algorithms | Restricted |
| 3 — Security / Policy Modification | Policy, authorization, risk boundary, security enforcement | Restricted (highest) |

### 15.3 API Security

| ID | Requirement |
|---|---|
| SEC-API-001 | All API endpoints require bearer token authentication. |
| SEC-API-002 | API requests pass through policy evaluation before cognitive operation. |
| SEC-API-003 | API SHALL NOT expose internal state mutation beyond defined endpoints. |
| SEC-API-004 | Sensitive internal structures SHALL NOT become publicly writable. |

### 15.4 Internet Security

| ID | Requirement |
|---|---|
| SEC-INT-001 | All network operations pass through risk assessment and policy. |
| SEC-INT-002 | Network results are observations, not trusted inputs. |
| SEC-INT-003 | Internet content carries provenance and is subject to verification. |
| SEC-INT-004 | Network access bounded by timeout and response size. |

---

## 16. Privacy & Data Handling Requirements

### 16.1 Data Handling

| ID | Requirement |
|---|---|
| PRV-001 | All persistent cognitive state resides in `.cx` on the local filesystem. |
| PRV-002 | CORTEX SHALL NOT transmit cognitive state to external services. |
| PRV-003 | Internet observations are bounded and carry provenance. |
| PRV-004 | API keys and secrets SHALL NOT appear in `.cx`, logs, or cognitive state. |
| PRV-005 | User-provided information carries `UserProvided` provenance. |
| PRV-006 | Internet-derived information carries `Internet` provenance and is never treated as ground truth. |

### 16.2 Data Lifecycle

```
Observation → Representation → Candidate Memory → Evidence → Verification
→ Generalization → Semantic Knowledge → World Model → Prediction
→ New Observation → Belief Update
```

Knowledge is dynamic and revisable.

### 16.3 Forgetting

Forgetting is controlled, not arbitrary. Factors: low importance, low retrieval frequency, low confidence, redundancy, age, memory pressure, contradiction. High-value knowledge receives stronger retention.

---

## 17. Error Handling Requirements

### 17.1 Error Taxonomy

> **Canonical definition:** See DOC-00 §7. Error kinds and severity levels are defined normatively in DOC-00.

CORTEX SHALL implement the following error kinds (DOC-00 §7.1):

```
InputError, EncodingError, LanguageError, MemoryError, WorldModelError,
ReasoningError, PlanningError, VerificationError, LearningError,
PersistenceError, PolicyError, ResourceError, NetworkError, RuntimeError
```

Additionally, the API layer SHALL implement extended error kinds (DOC-00 §7.2):

```
AuthenticationError, AuthorizationError, NotFoundError, ValidationError,
ConfigError, RateLimitError, TimeoutError, StateError, SerializationError,
SubsystemDisabled
```

Error severity levels (DOC-00 §7.3): Recoverable, StateCorruption, Fatal, Configuration.

### 17.2 Error Handling Contract

| ID | Requirement |
|---|---|
| ERR-001 | Every error SHALL be classified by kind and severity. |
| ERR-002 | Error taxonomy feeds diagnostics and learning attribution. |
| ERR-003 | Recoverable errors SHALL be logged and processing continued. |
| ERR-004 | Fatal errors SHALL trigger safe stop with state preservation attempt. |
| ERR-005 | Corrupt state SHALL never be silently treated as valid. |
| ERR-006 | Invalid configuration SHALL prevent startup. |
| ERR-007 | Diagnostics SHALL be bounded and SHALL NOT become uncontrolled persistent memory. |

### 17.3 Failure Response Matrix

| Error | Response |
|---|---|
| Network failure | Record failed observation, continue |
| Corrupt `.cx` | Validate checkpoint, recover |
| Invalid policy | Restricted mode |
| Invalid configuration | Prevent startup |
| Resource exhaustion | Bounded result, log |
| Fatal runtime | Safe stop |

---

## 18. Compatibility Requirements

### 18.1 State Compatibility

| ID | Requirement |
|---|---|
| CMP-001 | `.cx` format SHALL be versioned. |
| CMP-002 | Algorithm changes SHALL create detectable architectural state transitions. |
| CMP-003 | State migration SHALL preserve semantic state whenever technically possible. |
| CMP-004 | Changing an algorithm SHALL test: state compatibility, memory compatibility, language behavior, reasoning behavior, verification behavior, learning stability, `.cx` migration. |

### 18.2 Configuration Compatibility

| ID | Requirement |
|---|---|
| CMP-005 | Configuration schema changes SHALL be versioned. |
| CMP-006 | Invalid or incompatible configuration SHALL prevent startup with clear error. |

### 18.3 Algorithm Replacement

CORTEX exposes internal algorithm boundaries:

- `CellAlgorithm`
- `ColumnAlgorithm`
- `PlasticityAlgorithm`
- `MemoryRetrievalAlgorithm`
- `ReasoningAlgorithm`
- `PlanningAlgorithm`
- `VerificationAlgorithm`
- `LanguageAlgorithm`
- `ConsolidationAlgorithm`

Implementations can change without requiring architectural replacement of the entire system. Each algorithm defines: input state, output state, parameters, resource bounds, error conditions, version, determinism characteristics, and state compatibility.

---

## 19. Constraints & Limitations

### 19.1 Architectural Constraints

| # | Constraint |
|---|---|
| 1 | Single executable |
| 2 | Single process |
| 3 | Single configuration file |
| 4 | Single persistent cognitive state file |
| 5 | Native Rust implementation |
| 6 | Native cognitive algorithms |
| 7 | Native Language Core |
| 8 | No external AI model |
| 9 | No external database |
| 10 | No vector database |
| 11 | No agent framework |
| 12 | CPU-first execution |
| 13 | No mandatory external runtime |
| 14 | No GPU requirement |
| 15 | No distributed operation |
| 16 | No horizontal scaling |

### 19.2 Cognitive Limitations

| Limitation | Description |
|---|---|
| Bounded reasoning | Reasoning limited by `reasoning.max_steps` |
| Bounded planning | Planning limited by depth and branches |
| Bounded generation | Output limited by `language.generation_limit` |
| Bounded memory | Each memory subsystem bounded by MB budget |
| Bounded vocabulary | Limited by `language.vocabulary_capacity` |
| Bounded context | Limited by `language.context_window` |
| Initial knowledge gap | First boot has limited knowledge; grows through learning |
| No guaranteed correctness | Verification is evidence-based, not proof-based |
| Prediction uncertainty | All predictions carry uncertainty |

### 19.3 Operational Limitations

| Limitation | Description |
|---|---|
| Single-user primary | Not designed for multi-tenant serving |
| No clustering | Single instance operation |
| No hot-reload of configuration | Configuration loaded at startup |
| Internet dependency optional | Core operation works offline |

---

## 20. Technology Constraints

### 20.1 Language & Toolchain

| Property | Requirement |
|---|---|
| Implementation language | Rust |
| Toolchain | Stable Rust (defined in `rust-toolchain.toml`) |
| Build system | Cargo |
| Minimum Rust edition | 2021+ |
| Unsafe code | Minimized; justified where used |

### 20.2 Dependency Constraints

| Category | Permitted | Prohibited |
|---|---|---|
| Serialization | serde, bincode, or equivalent | — |
| Compression | zstd, lz4, or equivalent | — |
| Cryptography | BLAKE3 | — |
| Networking | std::net, tokio, hyper, or equivalent | — |
| OS interaction | libc, std::fs | — |
| AI/ML frameworks | **None** | PyTorch, TensorFlow, ONNX, candle (as cognitive substrate) |
| Databases | **None** | SQLite, PostgreSQL, Redis, etc. |
| Vector stores | **None** | Pinecone, Weaviate, Qdrant, etc. |
| Agent frameworks | **None** | LangChain, AutoGen, CrewAI, etc. |
| LLM inference | **None** | llama.cpp, vLLM, TGI, etc. |

### 20.3 Build & Deployment

| Property | Requirement |
|---|---|
| Build output | Single static or dynamic binary |
| Deployment | Copy binary + config to target |
| No install script required | Binary is self-contained |
| No package manager dependency at runtime | No pip, npm, apt at runtime |

---

## 21. Acceptance Criteria

### 21.1 Deployment Acceptance

| # | Criterion | Validation |
|---|---|---|
| AC-DEP-001 | `cortex` binary starts with valid `cortex.toml` and no `cortex.cx` (first boot). | Integration test |
| AC-DEP-002 | `cortex` binary creates `cortex.cx` on first boot. | Integration test |
| AC-DEP-003 | `cortex` binary loads existing `cortex.cx` on subsequent boots. | Integration test |
| AC-DEP-004 | `cortex` binary rejects invalid `cortex.toml` with clear error. | Unit test |
| AC-DEP-005 | No external service is required for core operation. | Deployment verification |
| AC-DEP-006 | Deployment consists of `cortex` + `cortex.toml` + auto-created `cortex.cx`. | Deployment verification |

### 21.2 Cognitive Pipeline Acceptance

| # | Criterion | Validation |
|---|---|---|
| AC-COG-001 | Text input produces `LanguageState` with tokens, syntax, semantics. | Unit test |
| AC-COG-002 | `LanguageState` produces `NeuralRepresentation` with sparse activation. | Unit test |
| AC-COG-003 | Memory retrieval returns relevant memories with provenance. | Unit test |
| AC-COG-004 | World model integrates observations and updates state. | Unit test |
| AC-COG-005 | Reasoning produces ranked hypotheses with evidence. | Unit test |
| AC-COG-006 | Planning produces bounded plans with risk assessment. | Unit test |
| AC-COG-007 | Verification classifies claims with correct status. | Unit test |
| AC-COG-008 | Language generation produces coherent output from verified meaning. | Unit test |
| AC-COG-009 | Full pipeline: input → response with state update. | Integration test |

### 21.3 Learning Acceptance

| # | Criterion | Validation |
|---|---|---|
| AC-LRN-001 | Prediction error is computed and attributed. | Unit test |
| AC-LRN-002 | Learning signal modifies state within bounds. | Unit test |
| AC-LRN-003 | Single observation does not destabilize complete state. | Stability test |
| AC-LRN-004 | Replay produces learning from prior episodes. | Unit test |
| AC-LRN-005 | Consolidation forms long-term knowledge from patterns. | Unit test |
| AC-LRN-006 | Vocabulary expands with new symbols without rebuild. | Unit test |
| AC-LRN-007 | Learning respects policy constraints. | Policy test |

### 21.4 Persistence Acceptance

| # | Criterion | Validation |
|---|---|---|
| AC-PRS-001 | `Save(State)` → `Load(State)` produces semantically equivalent state. | Round-trip test |
| AC-PRS-002 | Atomic write preserves last valid state on failure. | Fault injection test |
| AC-PRS-003 | Corrupt `.cx` triggers recovery, not silent continuation. | Corruption test |
| AC-PRS-004 | Checkpoint creation and recovery works. | Integration test |
| AC-PRS-005 | State migration preserves semantic content. | Migration test |

### 21.5 Security Acceptance

| # | Criterion | Validation |
|---|---|---|
| AC-SEC-001 | API requires valid bearer token. | Auth test |
| AC-SEC-002 | Policy gate denies prohibited operations. | Policy test |
| AC-SEC-003 | Learning cannot modify Level 3 policy. | Security test |
| AC-SEC-004 | API key not present in `.cx`. | State inspection |
| AC-SEC-005 | Fail-closed on ambiguous security decisions. | Security test |

### 21.6 API Acceptance

| # | Criterion | Validation |
|---|---|---|
| AC-API-001 | All defined endpoints respond correctly. | API test |
| AC-API-002 | Unauthenticated requests are rejected. | Auth test |
| AC-API-003 | API does not allow arbitrary state mutation. | Security test |
| AC-API-004 | Status endpoint returns correct runtime metrics. | API test |

### 21.7 Resource Acceptance

| # | Criterion | Validation |
|---|---|---|
| AC-RES-001 | Memory usage stays within configured budgets. | Resource test |
| AC-RES-002 | Reasoning terminates at `max_steps`. | Bound test |
| AC-RES-003 | Planning terminates at `max_depth` × `max_branches`. | Bound test |
| AC-RES-004 | Generation terminates at `generation_limit`. | Bound test |
| AC-RES-005 | Budget-exhausted operations return bounded results. | Bound test |

---

## 22. Gap Resolution: Additional Requirements

The following subsections close gaps identified during cross-document audit. They are normative and supplementary to the requirements in §2-§21.

### 22.1 State Versioning & Migration Requirements

| ID | Requirement | Priority |
|---|---|---|
| FR-MIG-001 | State migration SHALL be sequential: v1 → v2 → ... → vN. No version skipping. | MUST |
| FR-MIG-002 | Each migration step SHALL be a pure function: old bytes → new bytes. | MUST |
| FR-MIG-003 | Migration SHALL preserve semantic content whenever technically possible. | MUST |
| FR-MIG-004 | Migration SHALL be idempotent: applying the same migration twice produces the same result. | MUST |
| FR-MIG-005 | Failed migration SHALL trigger recovery from valid checkpoint, never partial state. | MUST |
| FR-MIG-006 | Original data SHALL be preserved in a backup until migration succeeds. | MUST |
| FR-MIG-007 | Downgrade (newer version to older version) SHALL NOT be supported. | MUST |
| FR-MIG-008 | Migration SHALL log before/after version information. | MUST |

### 22.2 World-State Inference Requirements

| ID | Requirement | Priority |
|---|---|---|
| FR-INF-001 | When direct observation is unavailable, CORTEX SHALL infer world state from memory and reasoning. | MUST |
| FR-INF-002 | Inferred state SHALL carry lower confidence than directly observed state. | MUST |
| FR-INF-003 | Inference SHALL be bounded by `reasoning.max_steps`. | MUST |
| FR-INF-004 | Inferred state SHALL NOT be treated as ground truth without verification. | MUST |

### 22.3 Memory Pressure Management Requirements

| ID | Requirement | Priority |
|---|---|---|
| FR-MEM-P-001 | CORTEX SHALL compute memory pressure as: ratio of total used bytes to total budget across all subsystems. | MUST |
| FR-MEM-P-002 | Pressure levels: Low (< 0.7), Moderate (0.7-0.85), High (0.85-0.95), Critical (≥ 0.95). | MUST |
| FR-MEM-P-003 | Low pressure: no action. | MUST |
| FR-MEM-P-004 | Moderate pressure: trigger consolidation. | MUST |
| FR-MEM-P-005 | High pressure: consolidation + aggressive forgetting. | MUST |
| FR-MEM-P-006 | Critical pressure: consolidation + emergency forgetting + working memory compression. | MUST |
| FR-MEM-P-007 | Pressure response SHALL NOT cause data corruption. | MUST |

### 22.4 Hypothesis Generation Requirements

| ID | Requirement | Priority |
|---|---|---|
| FR-HYP-001 | Hypothesis generation SHALL produce at most `max_hypotheses` (default: 10) hypotheses. | MUST |
| FR-HYP-002 | If no hypotheses can be generated, reasoning SHALL return uncertain result. | MUST |
| FR-HYP-003 | Each hypothesis SHALL carry provenance from its source (memory, world, episode). | MUST |
| FR-HYP-004 | Analogical hypotheses SHALL carry a discount factor of 0.7 relative to direct hypotheses. | MUST |

### 22.5 Internet Provenance & Staleness Requirements

| ID | Requirement | Priority |
|---|---|---|
| FR-INT-P-001 | Internet-sourced knowledge SHALL carry `Internet` provenance category. | MUST |
| FR-INT-P-002 | Internet-sourced knowledge SHALL have initial `verification_status = Unknown`. | MUST |
| FR-INT-P-003 | Internet-sourced knowledge SHALL be subject to verification before promotion to Verified status. | MUST |
| FR-INT-P-004 | Staleness SHALL be computed as: `age_hours / staleness_half_life` where staleness_half_life defaults to 168 hours (7 days). | MUST |
| FR-INT-P-005 | Internet-sourced knowledge older than `staleness_max_age` (default: 720 hours / 30 days) SHALL be flagged for re-verification or forgetting. | MUST |

### 22.6 Error Recovery Cascade Requirements

| ID | Priority |
|---|---|
| FR-REC-001 | Recovery cascade: Recoverable error → log and continue; Cognitive error → attribute and learn; Input error → reject with defined error; Network error → record failed observation and continue; State corruption → validate checkpoint and recover; Configuration error → prevent startup; Policy violation → deny operation; Resource exhaustion → bounded result and log; Fatal runtime error → safe stop with state preservation attempt. | MUST |
| FR-REC-002 | Recovery actions SHALL be logged with error kind, severity, and action taken. | MUST |
| FR-REC-003 | Recovery from state corruption SHALL follow priority: current valid state → latest valid checkpoint → previous valid checkpoint → initial state → safe stop. | MUST |

### 22.7 Self-Model Calibration Requirements

| ID | Requirement | Priority |
|---|---|---|
| FR-SLF-C-001 | Self-model prediction accuracy SHALL be updated using exponential moving average with α = 0.1. | MUST |
| FR-SLF-C-002 | Self-model SHALL be updated after every cognitive pipeline completion with performance metrics. | MUST |
| FR-SLF-C-003 | Self-model SHALL NOT be interpreted as proof of consciousness or subjective experience. | MUST |
| FR-SLF-C-004 | Self-model SHALL NOT gain authority to change policy. | MUST |
| FR-SLF-C-005 | Self-model historical performance SHALL be bounded to 100 snapshots. | MUST |

### 22.8 Selective Learning Gate Requirements

| ID | Requirement | Priority |
|---|---|---|
| FR-SLG-001 | CORTEX SHALL implement a selective learning gate that evaluates each learning signal before application. | MUST |
| FR-SLG-002 | The gate SHALL evaluate: signal magnitude, prediction error magnitude, source reliability, current memory pressure, and policy state. | MUST |
| FR-SLG-003 | Signals with magnitude < `learning_rate × 0.01` SHALL be discarded (noise filtering). | MUST |
| FR-SLG-004 | Signals from single observations with magnitude > 0.5 SHALL be discounted by factor 0.3 (single-observation guard). | MUST |
| FR-SLG-005 | When memory pressure is Critical, learning SHALL be throttled to highest-priority signals only. | MUST |
| FR-SLG-006 | Learning signals that would cause > 10% state change SHALL be rejected by the stability guard. | MUST |

---

## 23. Technical Assumptions

| # | Assumption |
|---|---|
| TA-001 | The target system runs Linux x86_64 with a POSIX-compliant filesystem. |
| TA-002 | The filesystem supports atomic file rename operations. |
| TA-003 | Sufficient RAM is available for configured memory budgets. |
| TA-004 | CPU provides adequate compute for bounded cognitive operations. |
| TA-005 | Network access, when enabled, uses standard TCP/IP. |
| TA-006 | System clock provides monotonically increasing timestamps for ordering. |
| TA-007 | The Rust stable toolchain provides sufficient language features. |
| TA-008 | No GPU is available or required. |
| TA-009 | Single-user or low-concurrency operation is the primary use case. |
| TA-010 | The operator has filesystem write access to the deployment directory. |
| TA-011 | Environment variables are available for secret injection. |
| TA-012 | Initial boot produces a functional but knowledge-limited system. |
| TA-013 | Capability grows through defined learning mechanisms over time. |

---

## 24. Open Technical Parameters

The following parameters are defined in configuration but may require tuning based on deployment context:

| Parameter | Default | Open Question |
|---|---|---|
| `model.cells` | 4096 | Optimal cell count for target workload |
| `model.columns` | 64 | Optimal column organization |
| `model.dimension` | 256 | Representation dimensionality trade-offs |
| `model.precision` | f32 | f16/bf16 viability on CPU |
| `model.sparsity_ratio` | 0.05 | Optimal sparsity for representation separation |
| `language.vocabulary_capacity` | 65536 | Sufficient for target domain |
| `language.context_window` | 4096 | Adequate for target conversations |
| `language.generation_limit` | 1024 | Sufficient for target responses |
| `memory.*_mb` | Various | Sizing for target deployment |
| `learning.learning_rate` | 0.001 | Stability vs. adaptability trade-off |
| `learning.plasticity` | 0.01 | Neural adaptation rate |
| `learning.consolidation_interval` | 1000 | Consolidation frequency |
| `reasoning.max_steps` | 32 | Sufficient for target reasoning complexity |
| `planning.max_depth` | 8 | Sufficient for target planning complexity |
| `planning.max_branches` | 16 | Sufficient for plan diversity |
| `verification.minimum_confidence` | 0.80 | Appropriate threshold |
| `world.prediction_horizon` | 8 | Sufficient for target prediction needs |
| `persistence.checkpoint_interval` | 1000 | Appropriate checkpoint frequency |
| `internet.timeout_seconds` | 15 | Appropriate for target network conditions |
| `internet.max_response_mb` | 4 | Sufficient for target content |

These parameters are exposed in `cortex.toml` for operator tuning. They do not represent architectural uncertainty but deployment-specific calibration.

---

## 25. Requirements Traceability

### 24.1 Traceability Matrix — Functional Requirements to Subsystems

| Requirement ID | Subsystem | Interface Contract | Test Domain |
|---|---|---|---|
| FR-LANG-* | Language Core (CLX) | `trait LanguageCore` | Language tests |
| FR-NEUR-* | Neural Core (CNS) | `trait NeuralCore` | Neural tests |
| FR-MEM-* | Memory System | `trait MemorySystem` | Memory tests |
| FR-WRLD-* | World Model | `trait WorldModelInterface` | World model tests |
| FR-RSN-* | Reasoning Engine | `trait ReasoningEngine` | Reasoning tests |
| FR-PLN-* | Planning Engine | `trait PlanningEngine` | Planning tests |
| FR-VER-* | Verification Engine | `trait VerificationEngine` | Verification tests |
| FR-LRN-* | Learning System | `trait LearningSystem` | Learning tests |
| FR-SLF-* | Self Model | `trait SelfModelInterface` | Self-model tests |
| FR-POL-* | Policy / Risk Gate | `trait PolicyEngine` | Policy tests |
| FR-INT-* | Internet Interface | `trait InternetInterface` | Internet tests |
| FR-PRS-* | Persistence Engine | `trait PersistenceEngine` | Persistence tests |
| FR-API-* | Embedded API | HTTP endpoints | API tests |
| FR-CLI-* | CLI | Command interface | CLI tests |

### 24.2 Traceability Matrix — Non-Functional Requirements

| Requirement ID | Domain | Validation Method |
|---|---|---|
| REL-* | Reliability | Fault injection, recovery tests |
| AVAIL-* | Availability | Deployment verification |
| SEC-* | Security | Security tests, policy tests |
| PRV-* | Privacy | State inspection, data flow audit |
| ERR-* | Error handling | Error injection tests |
| CMP-* | Compatibility | Migration tests, regression tests |
| AC-DEP-* | Deployment | Integration tests |
| AC-COG-* | Cognitive pipeline | Unit + integration tests |
| AC-LRN-* | Learning | Stability + learning tests |
| AC-PRS-* | Persistence | Round-trip + corruption tests |
| AC-SEC-* | Security | Auth + policy tests |
| AC-API-* | API | API tests |
| AC-RES-* | Resource | Bound + resource tests |

### 24.3 Traceability — Principles to Requirements

| Principle # | Principle | Enforcing Requirements |
|---|---|---|
| 1 | Single executable | AC-DEP-001 through AC-DEP-006 |
| 2 | Single process | §7.1, §19.1 |
| 3 | Single configuration | §13, AC-DEP-004 |
| 4 | Single persistent state | §12, AC-PRS-* |
| 5 | Native Rust | §20.1 |
| 6 | Native cognitive algorithms | §20.2, §1.3.2 |
| 7 | Native Language Core | FR-LANG-*, §7 |
| 8 | No external AI model | §1.3.2, §14.3, §20.2 |
| 9 | No external database | §14.3, §20.2 |
| 10 | No vector database | §14.3, §20.2 |
| 11 | No agent framework | §14.3, §20.2 |
| 12 | Continual learning | FR-LRN-*, AC-LRN-* |
| 13 | Persistent memory | FR-MEM-*, §12 |
| 14 | Persistent world model | FR-WRLD-006 |
| 15 | Persistent language state | FR-LANG-013, §12 |
| 16 | Inspectable state | §33, FR-CLI (inspect) |
| 17 | Replaceable algorithms | §18.3, CMP-004 |
| 18 | Versioned state | §12.4, CMP-001 |
| 19 | Provenance-aware | §20 (Provenance), FR-MEM-010 |
| 20 | Resource-bounded | §8.2, AC-RES-* |
| 21 | Policy-bounded autonomy | FR-POL-*, AC-SEC-* |
| 22 | Fail-closed security | §10.4, SEC-005 |
| 23 | Deterministic where practical | §11 |
| 24 | CPU-first | §6.1, §19.1 |
| 25 | No mandatory external runtime | §7.1, §14.3 |
| 26 | End-to-end after deployment | AC-DEP-005, §40.6 |

---

## 26. Deployment Contract

### 25.1 Minimum Valid Deployment

```
/opt/cortex/
├── cortex          # executable (single binary)
├── cortex.toml     # configuration
└── cortex.cx       # persistent cognitive state (auto-created on first boot)
```

### 25.2 Optional Deployment Structure

```
/opt/cortex/
├── cortex
├── cortex.toml
├── cortex.cx
└── checkpoints/    # periodic checkpoint snapshots
```

### 25.3 Deployment Validation

A deployment is valid when:

1. ✅ `cortex` binary is executable.
2. ✅ `cortex.toml` passes full validation pipeline.
3. ✅ `cortex.cx` loads with integrity verification (or is auto-created on first boot).
4. ✅ CORTEX transitions to READY state.
5. ✅ Full cognitive pipeline processes input and produces output.
6. ✅ Learning persists to `.cx`.
7. ✅ State survives restart.

### 25.4 Ready-for-Use Definition

After deployment, the `cortex` binary is ready for use when it can:

1. Start and load configuration from `cortex.toml`.
2. Load or initialize cognitive state from `cortex.cx`.
3. Accept input through its CLI or embedded API.
4. Process information through the full cognitive pipeline (Language Core → Neural Core → Memory → World Model → Reasoning → Planning → Verification).
5. Generate responses.
6. Learn from experience and feedback.
7. Persist learned state to `.cx`.
8. Continue operation across restarts without requiring another AI model or external cognitive service.

---

## 27. Final Contract Statement

> **This document constitutes the system-level technical contract for CORTEX.** It defines what CORTEX MUST do, what it MUST NOT do, and the boundaries within which it operates. All implementation decisions, test strategies, deployment procedures, and validation activities SHALL conform to the requirements specified herein.
>
> CORTEX is a **native Rust, single-binary, persistent, continually learning AI model** whose cognitive state consists of a native Language Core, Neural Core, Memory System, World Model, Reasoning Engine, Planning Engine, Verification Engine, Learning System, Consolidation System, Self Model, Policy/Risk Gate, and persistent `.cx` state.
>
> **ONE BINARY + ONE CONFIGURATION + ONE COGNITIVE STATE = CORTEX.**

---

*End of Document — CORTEX-DOC-01 Technical Specification v1.1.0*
