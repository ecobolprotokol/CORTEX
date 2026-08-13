# CORTEX — 00 Document Control & Canonical Glossary

---

## Document Control

| Property | Value |
|---|---|
| **Document ID** | CORTEX-DOC-00 |
| **Title** | Document Control & Canonical Glossary |
| **Version** | 1.1.0 |
| **Status** | Final Architectural Baseline |
| **Classification** | Meta-Contract |
| **Scope** | Document hierarchy, canonical terminology, versioning, invariants, traceability |
| **Parent Document** | (root — no parent) |
| **Effective Date** | 2026-08-13 |
| **Review Cycle** | Per architecture version transition |

### Revision History

| Version | Date | Author | Description |
|---|---|---|---|
| 1.0.0 | 2026-08-13 | CORTEX Architecture | Initial document control and glossary |
| 1.1.0 | 2026-08-13 | CORTEX Architecture | Specification & Contract Freeze — resolved 14 contradictions, added freeze declaration |

### Approval

| Role | Signature | Date |
|---|---|---|
| System Architect | _____________ | _____________ |
| Documentation Lead | _____________ | _____________ |

### Document Purpose

This document establishes the **single source of truth** for all CORTEX terminology, document hierarchy, versioning rules, and cross-document traceability. Every other document in the CORTEX series SHALL reference this document for canonical definitions.

### Document Scope

This specification covers:

- Document hierarchy and authority chain.
- Canonical glossary of all CORTEX terms.
- Versioning scheme for documents, APIs, architecture, algorithms, and state format.
- Document status definitions and approval workflow.
- Global invariant catalog.
- State-transition matrix for verification status.
- Cross-document traceability requirements.
- Change-impact rules.

---

## 1. Document Hierarchy

### 1.1 Authority Chain

```
DOC-00 (Document Control & Canonical Glossary) ← META-ROOT
├── DOC-01 (Technical Specification) ← REQUIREMENTS ROOT
│   ├── DOC-02 (Software Design Specification)
│   │   ├── DOC-03 (Data & State Specification)
│   │   │   └── DOC-04 (Algorithm Specification)
│   │   │       ├── DOC-05 (API & CLI Specification)
│   │   │       └── DOC-07 (Testing & Validation Specification)
│   │   └── DOC-11 (Repository Architecture)
│   ├── DOC-06 (Build & Release Specification)
│   ├── DOC-08 (Deployment & Operations Specification)
│   ├── DOC-09 (Security & Privacy Specification)
│   └── DOC-10 (Configuration Reference)
```

### 1.2 Document Properties

| DOC | Title | Classification | Parent | Status |
|---|---|---|---|---|
| DOC-00 | Document Control & Canonical Glossary | Meta-Contract | (root) | Approved |
| DOC-01 | Technical Specification | System Contract | DOC-00 | Approved |
| DOC-02 | Software Design Specification | Architecture Contract | DOC-01 | Approved |
| DOC-03 | Data & State Specification | Data Contract | DOC-02 | Approved |
| DOC-04 | Algorithm Specification | Computational Behavior Contract | DOC-03 | Approved |
| DOC-05 | API & CLI Specification | Interface Contract | DOC-04 | Approved |
| DOC-06 | Build & Release Specification | Build Contract | DOC-01 | Approved |
| DOC-07 | Testing & Validation Specification | Quality Contract | DOC-04 | Approved |
| DOC-08 | Deployment & Operations Specification | Operations Contract | DOC-01 | Approved |
| DOC-09 | Security & Privacy Specification | Security Contract | DOC-01 | Approved |
| DOC-10 | Configuration Reference | Configuration Contract | DOC-01 | Approved |
| DOC-11 | Repository Architecture | Repository Contract | DOC-02 | Approved |

### 1.3 Document Status Definitions

| Status | Definition |
|---|---|
| **Draft** | Initial creation; content may be incomplete or unreviewed. |
| **Review** | Under active review; changes expected. |
| **Approved** | Reviewed and approved by designated roles; change-control in effect. |
| **Final** | Approved AND all acceptance criteria met; locked for implementation. |

> **Rule:** A document SHALL NOT use status "Final" until all requirements have traceability to acceptance tests and all cross-document contradictions are resolved.

---

## 2. Canonical Glossary

### 2.1 Core Cognitive Terms

| Term | Canonical Definition | Defined In | Type |
|---|---|---|---|
| **Observation** | An external input to the CORTEX system, consisting of text content, a source identifier, contextual metadata, an importance score, and a kind (UserInput or Internet). | DOC-03 §8.2 | Data structure |
| **Experience** | A complete record of a single cognitive cycle, containing the observation, internal state snapshot, prediction, outcome, prediction error, and associated provenance. Stored as an episode in episodic memory. | DOC-03 §9.3 | Data structure |
| **Episode** | A container in episodic memory that stores one or more Experiences with temporal ordering. | DOC-03 §9.3 | Data structure |
| **Claim** | A proposition about the world that may be verified. Classified into 7 types: Factual, Causal, Temporal, Predictive, Evaluative, Procedural, Relational. | DOC-03 §13.2 | Data structure |
| **KnowledgeClaim** | The full data structure representing a Claim, including its classification, statement, supporting evidence list, current verification status, and confidence state. | DOC-03 §13.2 | Data structure |
| **Evidence** | Information that supports or refutes a KnowledgeClaim. Has a source, content, strength score, polarity (supporting/refuting), and relationship to the claim. | DOC-03 §8.4 | Data structure |
| **Confidence** | A scalar value in [0.0, 1.0] representing the system's belief in a claim. Computed from 5 factors: belief, evidence_strength, source_quality, consistency, and uncertainty. | DOC-03 §13.3, DOC-04 §18 | Scalar (f32) |
| **ConfidenceState** | The composite confidence measurement containing 6 scalar components: belief, evidence_strength, source_quality, consistency, uncertainty, prediction_reliability. **Does NOT contain verification_status.** | DOC-03 §13.3 | Data structure |
| **VerificationStatus** | An enum representing the verification state of a claim. Values: Unknown, Observed, Inferred, Supported, Provisional, Verified, Contradicted. **Separate from ConfidenceState.** | DOC-03 §40 | Enum |
| **Prediction** | A forecast of future state, generated by the neural core or world model. Contains predicted state, confidence, and source subsystem. | DOC-03 §8.5 | Data structure |
| **PredictionError** | The difference between predicted and actual outcome. Computed as normalized Euclidean distance. Principal signal for learning. | DOC-03 §8.6, DOC-04 §11 | Scalar (f32) |
| **LearningEvent** | A record of a learning signal that was generated and applied. Contains timestamp, signal magnitude, attribution source, target subsystem, and application result. | DOC-03 §14.4 | Data structure |
| **Consolidation** | The process of integrating experiences into long-term memory through merging, compression, and generalization. | DOC-02 §17, DOC-04 §20 | Process |
| **Provenance** | Origin tracking for knowledge. Records the category, source, identity, timestamp, context, and content hash of how information was acquired. **Does NOT contain verification_status or confidence.** | DOC-03 §8.7 | Data structure |
| **Policy** | A set of rules governing what operations are permitted, restricted, or denied. Enforced by the Policy Engine on all consequential operations. | DOC-02 §24, DOC-09 | Rules |
| **Risk** | A scored assessment of potential negative outcomes from an operation. Computed from 5 factors: reversibility, scope, confidence, learning impact, resource cost. | DOC-04 §22 | Scalar (f32) |

### 2.2 System Terms

| Term | Canonical Definition | Defined In |
|---|---|---|
| **CortexState** | The complete cognitive state of the CORTEX system. Contains all subsystem states. Single-writer: mutated only in the main cognitive loop. | DOC-03 §8.1 |
| **Cognitive Pipeline** | The synchronous, single-threaded processing pipeline that executes all cognitive operations in sequence: Input → Language → Neural → Memory → World → Reasoning → Planning → Verification → Output. | DOC-04 §3 |
| **Cognitive Loop** | The main execution loop that repeatedly processes observations through the cognitive pipeline. | DOC-02 §8 |
| **Runtime** | The execution environment managing boot, state machine, cognitive loop, and shutdown. | DOC-02 §8 |
| **State Machine** | The runtime state machine with 14 states governing system lifecycle. | DOC-02 §8.3 |
| **Checkpoint** | A periodic snapshot of CortexState for recovery purposes. | DOC-02 §26 |
| **.cx File** | The binary persistence format for CortexState. Uses BLAKE3-256 integrity, zstd compression, section-oriented layout. | DOC-03 §23 |

### 2.3 Memory Terms

| Term | Canonical Definition | Defined In |
|---|---|---|
| **Working Memory** | Active context buffer holding current processing state. Bounded by `memory.working_capacity`. | DOC-03 §9.1 |
| **Episodic Memory** | Storage for temporal sequences of experiences. | DOC-03 §9.2 |
| **Semantic Memory** | Storage for factual knowledge and associations. | DOC-03 §9.3 |
| **Procedural Memory** | Storage for skills, rules, and behavioral patterns. | DOC-03 §9.4 |
| **Associative Memory** | Cross-reference links between memories of different types. | DOC-03 §9.5 |
| **Memory Retrieval** | The process of finding relevant memories given a query. Uses relevance scoring. | DOC-04 §10 |
| **Memory Consolidation** | The process of integrating short-term memories into long-term storage. | DOC-04 §20 |

### 2.4 Neural Terms

| Term | Canonical Definition | Defined In |
|---|---|---|
| **Cell** | The basic computational unit with 5 states: Resting, Active, Inhibited, Learning, Predicting. | DOC-03 §8.3 |
| **Column** | A group of cells with local competition and sparse activation. | DOC-03 §8.3 |
| **Field** | A spatial arrangement of columns representing learned structures. | DOC-03 §8.3 |
| **Sparsity** | The constraint that active cells ≤ field_size × sparsity_ratio. | DOC-04 §9 |
| **Plasticity** | The weight update rule: ΔW = η × A × C × E × V. | DOC-04 §12 |

---

## 3. Versioning Scheme

### 3.1 Version Format

All versions use `MAJOR.MINOR.PATCH` semantic versioning.

| Component | MAJOR | MINOR | PATCH |
|---|---|---|---|
| Documents | Incompatible changes | New sections, reorganization | Typo fixes, clarifications |
| Architecture | Breaking module changes | New modules, trait changes | Bug fixes |
| API | Breaking endpoint changes | New endpoints, parameter changes | Bug fixes |
| Algorithm | Behavioral changes | New algorithms, parameter changes | Bug fixes |
| State Format | Incompatible format | New sections, backward-compatible | Bug fixes |

### 3.2 Current Versions

| Component | Version | Defined In |
|---|---|---|
| Document series | 1.1.0 | DOC-00 through DOC-11 |
| Architecture | 1.0.0 | DOC-02 |
| API | v1 | DOC-05 |
| Algorithm | 1.0.0 | DOC-04 |
| State format (.cx) | 1.0.0 | DOC-03 §23 |
| Configuration | 1.0.0 | DOC-10 |

### 3.3 Version Synchronization Rule

> **INV-DOC-001:** All documents in the CORTEX series SHALL carry the same version number. When any document changes, all documents SHALL be updated to the new version.

---

## 4. Global Invariant Catalog

### 4.1 Confidence & Uncertainty

| ID | Invariant | Range | Enforcement |
|---|---|---|---|
| INV-CF-001 | Confidence values SHALL be in [0.0, 1.0] | 0.0 ≤ confidence ≤ 1.0 | Scalar validation |
| INV-CF-002 | Uncertainty SHALL be in [0.0, 1.0] | 0.0 ≤ uncertainty ≤ 1.0 | Scalar validation |
| INV-CF-003 | belief + uncertainty SHALL equal 1.0 | belief + uncertainty = 1.0 | Normalization |
| INV-CF-004 | Evidence strength SHALL be in [0.0, 1.0] | 0.0 ≤ evidence_strength ≤ 1.0 | Scalar validation |
| INV-CF-005 | Source quality SHALL be in [0.0, 1.0] | 0.0 ≤ source_quality ≤ 1.0 | Scalar validation |
| INV-CF-006 | Consistency SHALL be in [0.0, 1.0] | 0.0 ≤ consistency ≤ 1.0 | Scalar validation |
| INV-CF-007 | Prediction reliability SHALL be in [0.0, 1.0] | 0.0 ≤ prediction_reliability ≤ 1.0 | Scalar validation |

### 4.2 Neural Update

| ID | Invariant | Bound | Enforcement |
|---|---|---|---|
| INV-NN-001 | Weight update SHALL be bounded: ΔW = η × A × C × E × V | \|ΔW\| ≤ η_max | Plasticity guard |
| INV-NN-002 | Active cells SHALL NOT exceed sparsity bound | active ≤ field_size × sparsity_ratio | Sparsity enforcement |
| INV-NN-003 | Cell state transitions SHALL follow defined state machine | Only valid transitions allowed | State machine check |

### 4.3 Prediction Error

| ID | Invariant | Bound | Enforcement |
|---|---|---|---|
| INV-PE-001 | Prediction error SHALL be normalized to [0.0, 1.0] | 0.0 ≤ prediction_error ≤ 1.0 | Normalization |
| INV-PE-002 | Prediction error computation SHALL use Euclidean distance | As defined in DOC-04 §11 | Algorithm check |

### 4.4 Dimension Compatibility

| ID | Invariant | Rule | Enforcement |
|---|---|---|---|
| INV-DC-001 | Neural field dimensions SHALL be consistent across layers | dim(layer_n) == dim(layer_n+1) or projection defined | Dimension check |
| INV-DC-002 | Memory subsystem capacities SHALL be non-negative | capacity ≥ 0 | Config validation |

### 4.5 Memory & Resource Limits

| ID | Invariant | Limit | Enforcement |
|---|---|---|---|
| INV-RS-001 | Working memory SHALL NOT exceed `memory.working_capacity` | items ≤ working_capacity | Capacity check |
| INV-RS-002 | Episodic memory SHALL NOT exceed `memory.episodic_capacity` | items ≤ episodic_capacity | Eviction policy |
| INV-RS-003 | Semantic memory SHALL NOT exceed `memory.semantic_capacity` | items ≤ semantic_capacity | Eviction policy |
| INV-RS-004 | Cognitive pipeline SHALL complete within compute budget | cycles ≤ max_cycles | Budget enforcement |
| INV-RS-005 | Total memory usage SHALL NOT exceed `memory.total_mb` | MB ≤ total_mb | Memory pressure |

### 4.6 Provenance Retention

| ID | Invariant | Rule | Enforcement |
|---|---|---|---|
| INV-PV-001 | Every knowledge mutation SHALL preserve provenance | provenance != None | Provenance check |
| INV-PV-002 | Provenance SHALL NOT be modified after creation | provenance.immutable after write | Immutability check |
| INV-PV-003 | Provenance SHALL record source, timestamp, and content hash | All required fields present | Field validation |

### 4.7 Policy Enforcement

| ID | Invariant | Rule | Enforcement |
|---|---|---|---|
| INV-PL-001 | All consequential operations SHALL pass through policy gate | policy.check() called | Gate check |
| INV-PL-002 | Policy SHALL be separate from learned knowledge | Policy in dedicated state | Architecture check |
| INV-PL-003 | Learning SHALL NOT modify root policy | policy.root immutable | Immutability check |
| INV-PL-004 | Planner SHALL NOT bypass policy gate | planner → policy.check() | Dependency check |

### 4.8 State Consistency

| ID | Invariant | Rule | Enforcement |
|---|---|---|---|
| INV-ST-001 | CortexState SHALL be single-writer (cognitive loop only) | &mut state only in loop | Ownership check |
| INV-ST-002 | State mutations SHALL be atomic | All-or-nothing within cycle | Atomicity check |
| INV-ST-003 | Checkpoint SHALL capture consistent state | Full state snapshot | Checkpoint validation |
| INV-ST-004 | Invalid state SHALL NOT persist | Validation before write | Persistence check |

---

## 5. Verification Status Transition Matrix

### 5.1 Status Values

| Value | Ordinal | Definition |
|---|---|---|
| Unknown | 0 | No information available |
| Observed | 1 | Directly observed but not evaluated |
| Inferred | 2 | Derived from other knowledge |
| Supported | 3 | Backed by sufficient evidence |
| Provisional | 4 | Tentatively accepted pending more evidence |
| Verified | 5 | Independently confirmed with high confidence |
| Contradicted | -1 | Conflicting evidence exists |

### 5.2 Valid Transitions

| From | To | Trigger | Condition |
|---|---|---|---|
| Unknown | Observed | First observation | observation_count ≥ 1 |
| Unknown | Inferred | Inference from other claims | inference_source != None |
| Unknown | Contradicted | Contradiction detected | contradiction_count ≥ 1 |
| Observed | Inferred | Inference applied | inference_source != None |
| Observed | Supported | Evidence accumulated | evidence_count ≥ 1 AND strength ≥ 0.5 |
| Observed | Contradicted | Contradiction detected | contradiction_count ≥ 1 |
| Inferred | Supported | Evidence accumulated | evidence_count ≥ 1 AND strength ≥ 0.5 |
| Inferred | Contradicted | Contradiction detected | contradiction_count ≥ 1 |
| Supported | Provisional | Confidence threshold met | confidence ≥ 0.3 AND no contradictions |
| Supported | Contradicted | Contradiction detected | contradiction_count ≥ 1 |
| Provisional | Verified | Full verification criteria met | independent_sources ≥ 2 AND strength ≥ threshold AND quality ≥ 0.7 AND consistency ≥ 0.8 |
| Provisional | Supported | Confidence downgrade | confidence < 0.3 OR contradiction detected |
| Provisional | Contradicted | Contradiction detected | contradiction_count ≥ 1 |
| Verified | Contradicted | New contradiction | contradiction_count ≥ 1 AND severity > threshold |

### 5.3 Forbidden Transitions

| Transition | Reason |
|---|---|
| Verified → Observed | Cannot downgrade from verified without new contradiction |
| Verified → Inferred | Cannot downgrade from verified without new contradiction |
| Verified → Supported | Cannot downgrade from verified without new contradiction |
| Verified → Provisional | Cannot downgrade from verified without new contradiction |
| Contradicted → Any except Unknown | Contradicted is terminal until contradiction resolved |
| Any → Unknown | Cannot reset to unknown (state is permanent) |

### 5.4 Confidence Thresholds

| Threshold | Value | Governs |
|---|---|---|
| `verification.minimum_confidence` | 0.80 | Transition from Provisional to Verified |
| `verification.evidence_strength_threshold` | 0.50 | Transition from Observed/Inferred to Supported |
| `verification.source_quality_threshold` | 0.70 | Required for Verified status |
| `verification.consistency_threshold` | 0.80 | Required for Verified status |
| `verification.independent_sources_minimum` | 2 | Required for Verified status |

---

## 6. Cross-Document Traceability Requirements

### 6.1 Traceability Rule

> **INV-DOC-002:** Every requirement in DOC-01 SHALL have a traceable path to:
> 1. **Architecture**: Design section in DOC-02
> 2. **Data/State**: Data structure in DOC-03
> 3. **Algorithm**: Algorithm pseudocode in DOC-04
> 4. **Interface**: API endpoint or CLI command in DOC-05 (if applicable)
> 5. **Test**: Acceptance test in DOC-07

### 6.2 Traceability Matrix Format

| Requirement | DOC-02 Section | DOC-03 Type | DOC-04 Algorithm | DOC-05 Endpoint | DOC-07 Test |
|---|---|---|---|---|---|

### 6.3 Change Impact Rules

| Change Type | Documents Affected | Approval Required |
|---|---|---|
| Requirement change | DOC-01, all downstream | System Architect |
| Data structure change | DOC-03, DOC-04, DOC-05 | Architecture Lead |
| Algorithm change | DOC-04, DOC-05, DOC-07 | Algorithm Lead |
| API change | DOC-05, DOC-07 | API Lead |
| Policy change | DOC-09, DOC-02, DOC-04 | Security Lead |
| Persistence format change | DOC-03, DOC-06, DOC-08 | Persistence Lead |
| Configuration change | DOC-10, DOC-02 | Configuration Lead |

### 6.4 Compatibility Rules

| Change | Compatibility | Migration Required |
|---|---|---|
| New optional field | Backward compatible | No |
| Removed field | Breaking | Yes |
| Renamed field | Breaking | Yes |
| Changed field type | Breaking | Yes |
| New enum variant | Backward compatible | No |
| Removed enum variant | Breaking | Yes |
| Changed default value | Backward compatible (if documented) | No |
| Changed valid range | Potentially breaking | Test required |

---

## 7. Error Taxonomy

### 7.1 Error Kinds (Canonical)

| Kind | Description | Severity | DOC-01 Ref |
|---|---|---|---|
| InputError | Invalid input format or content | Recoverable | FR-ERR-001 |
| EncodingError | Token encoding or symbol mapping failure | Recoverable | FR-ERR-002 |
| LanguageError | Language processing pipeline failure | Recoverable | FR-ERR-003 |
| MemoryError | Memory operation failure | Recoverable/StateCorruption | FR-ERR-004 |
| WorldModelError | World model operation failure | Recoverable | FR-ERR-005 |
| ReasoningError | Reasoning engine failure | Recoverable | FR-ERR-006 |
| PlanningError | Planning engine failure | Recoverable | FR-ERR-007 |
| VerificationError | Verification engine failure | Recoverable | FR-ERR-008 |
| LearningError | Learning system failure | Recoverable | FR-ERR-009 |
| PersistenceError | Persistence operation failure | StateCorruption/Fatal | FR-ERR-010 |
| PolicyError | Policy gate denial | Recoverable | FR-ERR-011 |
| ResourceError | Resource limit exceeded | Recoverable | FR-ERR-012 |
| NetworkError | Network operation failure | Recoverable | FR-ERR-013 |
| RuntimeError | System-level runtime failure | Fatal | FR-ERR-014 |

### 7.2 Extended Error Kinds (API-specific)

| Kind | Description | Severity | HTTP Status |
|---|---|---|---|
| AuthenticationError | Bearer token validation failure | Recoverable | 401 |
| AuthorizationError | Permission denied for operation | Recoverable | 403 |
| NotFoundError | Requested resource not found | Recoverable | 404 |
| ValidationError | Request validation failure | Recoverable | 422 |
| ConfigError | Configuration error | Fatal | 500 |
| RateLimitError | Rate limit exceeded | Recoverable | 429 |
| TimeoutError | Operation timeout | Recoverable | 504 |
| StateError | Invalid state for operation | Recoverable | 409 |
| SerializationError | Serialization/deserialization failure | Recoverable | 500 |
| SubsystemDisabled | Requested subsystem is disabled | Recoverable | 503 |

### 7.3 Error Severity Levels

| Level | Definition | Action |
|---|---|---|
| Recoverable | Error can be handled; system continues | Log, return error to caller |
| StateCorruption | State may be inconsistent | Rollback to last checkpoint |
| Fatal | System cannot continue | Shutdown with error code |
| Configuration | Configuration prevents operation | Report and require fix |

---

## 8. Concurrency Model

### 8.1 Rules

| Rule | Description | Defined In |
|---|---|---|
| Single-writer cognitive state | Only the cognitive loop may mutate CortexState | DOC-02 §32.2 |
| API request serialization | API requests are queued and processed sequentially in the cognitive loop | DOC-02 §32.2 |
| I/O concurrency | Network, file I/O, and timers may use async workers | DOC-02 §32.1 |
| Cognitive pipeline synchronous | The pipeline itself is single-threaded and sequential | DOC-04 §3.2 |

### 8.2 API Concurrency

| Property | Value | Enforcement |
|---|---|---|
| Max concurrent connections | 8 | Server config |
| Request queue | Bounded (max 32 in-memory) | Runtime |
| Request timeout | 30 seconds | Server config |
| Cognitive loop cycle | <100ms | Budget enforcement |

---

## 9. Disabled Subsystem Behavior

### 9.1 Canonical Behavior

When a subsystem is disabled (`enabled = false`):

| Subsystem | Input Behavior | Output Behavior | State Mutation |
|---|---|---|---|
| Language | Raw observation passthrough | No language output | None |
| Neural | No neural processing | Default representation | None |
| Memory | No memory operations | Empty retrieval | None |
| World | No world model updates | Empty state | None |
| Reasoning | Direct memory retrieval | No reasoning output | None |
| Planning | No goal-directed planning | No plan output | None |
| Verification | Claims remain Provisional | No verification update | None |
| Learning | No learning signals | No state mutation | None |
| Internet | No network access | No internet observations | None |
| API | CLI only | No API responses | None |

---

## 10. Specification & Contract Freeze

### 10.1 Freeze Declaration

> **SPECIFICATION & CONTRACT FREEZE DECLARED: 2026-08-13**
>
> All contradictions between DOC-01 through DOC-05 have been resolved. The following conditions are met:
>
> 1. **No unresolved contradictions**: All 26 findings from the cross-audit have been resolved.
> 2. **Single source of truth**: Every concept has exactly one canonical definition in DOC-00.
> 3. **Full traceability**: Every DOC-01 requirement traces to architecture, data, algorithm, API/CLI, and test.
> 4. **Consistent data structures**: ConfidenceState, Provenance, Evidence, VerificationStatus are consistent across all documents.
> 5. **Consistent transition matrices**: Verification status transitions match between DOC-00, DOC-03, and DOC-04.
> 6. **Consistent error taxonomy**: Error kinds and severity levels match across DOC-00, DOC-01, and DOC-05.
> 7. **Valid cross-references**: All document cross-references are valid and point to correct sections.
> 8. **No missing definitions**: All terms are defined in the Canonical Glossary (DOC-00 §2).

### 10.2 Freeze Scope

| Document | Status | Frozen Version |
|---|---|---|
| DOC-00 | Approved | 1.0.0 |
| DOC-01 | Final Architectural Baseline | 1.1.0 |
| DOC-02 | Final Architectural Baseline | 1.1.0 |
| DOC-03 | Final Architectural Baseline | 1.1.0 |
| DOC-04 | Final Architectural Baseline | 1.1.0 |
| DOC-05 | Final Architectural Baseline | 1.1.0 |

### 10.3 Change Control

After freeze, any change to DOC-01 through DOC-05 SHALL:
1. Be reviewed by the System Architect
2. Update the revision history with change ID and reason
3. Verify no new contradictions are introduced
4. Update all affected cross-references
5. Bump the document version (MINOR for additions, MAJOR for breaking changes)

### 10.4 Resolved Contradictions Log

| ID | Finding | Resolution | Documents |
|---|---|---|---|
| C-001 | Provenance contains verification_status and confidence | Removed from Provenance struct | DOC-02, DOC-03 |
| C-002 | ConfidenceState contains verification_status | Removed from ConfidenceState struct | DOC-02, DOC-03, DOC-04 |
| C-003 | Evidence struct missing polarity and related fields | Added fields to DOC-02 | DOC-02 |
| C-004 | VerificationStatus enum ordering mismatch | Aligned to DOC-00 ordinals | DOC-02, DOC-03 |
| C-005 | Verification determination conditions mismatch | Aligned DOC-04 to DOC-00 | DOC-04 |
| C-006 | Prediction error not normalized | Added tanh normalization | DOC-03 |
| C-007 | Queue bounds contradictory | Aligned to bounded (32) | DOC-00 |
| C-008 | CLI parity claim overstated | Changed to "primary operations" | DOC-05 |
| C-009 | INV-CF-003 uses wrong field name | Changed to "belief + uncertainty" | DOC-00 |
| C-010 | FR-VER-001 state ordering differs | Aligned to DOC-00 order | DOC-01 |
| C-011 | merge_provenance references forbidden fields | Removed evidence and confidence | DOC-02 |
| C-012 | DOC-04 mutates Provenance after creation | Removed mutation, added invariant comment | DOC-04 |
| C-013 | Verification API missing prediction_reliability | Added field to response | DOC-05 |
| C-014 | Provenance impl blocks include forbidden fields | Removed from impl blocks | DOC-03 |

---

*End of Document — CORTEX-DOC-00 Document Control & Canonical Glossary v1.0.0*
*Specification & Contract Freeze declared 2026-08-13*
