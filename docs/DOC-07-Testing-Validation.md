# CORTEX — 07 Testing & Validation Specification

---

## Document Control

| Property | Value |
|---|---|
| **Document ID** | CORTEX-DOC-07 |
| **Title** | Testing & Validation Specification |
| **Version** | 1.1.0 |
| **Status** | Final Architectural Baseline |
| **Classification** | Quality Contract |
| **Scope** | Test strategy, test categories, test requirements, validation criteria |
| **Parent Document** | CORTEX-DOC-04 Algorithm Specification |
| **Effective Date** | 2026-08-13 |
| **Review Cycle** | Per architecture version transition |

### Revision History

| Version | Date | Author | Description |
|---|---|---|---|
| 1.0.0 | 2026-08-13 | CORTEX Architecture | Initial final baseline |
| 1.1.0 | 2026-08-13 | CORTEX Architecture | Update cross-references for BLAKE3 migration |

### Approval

| Role | Signature | Date |
|---|---|---|
| System Architect | _____________ | _____________ |
| QA Lead | _____________ | _____________ |

### Document Purpose

This document defines **how CORTEX is tested and validated**. It constitutes the quality contract: every test category, every test requirement, every acceptance criterion, and every validation procedure. Canonical invariants and transition matrices are defined in DOC-00.

---

## 1. Test Strategy

### 1.1 Test Pyramid

```
                    ┌─────────┐
                    │ Stress  │  ← Few, slow, expensive
                    │ Tests   │
                    ├─────────┤
                    │Security │
                    │ Tests   │
                    ├─────────┤
                  │ Integration│  ← Moderate count
                  │   Tests    │
                  ├────────────┤
                │   Regression │
                │    Tests     │
                ├──────────────┤
              │    Unit Tests   │  ← Many, fast, cheap
              │                 │
              └─────────────────┘
```

### 1.2 Test Categories

| Category | Count Target | Speed | Isolation | Purpose |
|---|---|---|---|---|
| Unit | > 500 | < 1ms each | Module-level | Algorithm correctness |
| Integration | > 50 | < 100ms each | Cross-module | Subsystem interaction |
| Regression | > 20 | < 1s each | State-level | State compatibility |
| Security | > 30 | < 1s each | Policy-level | Policy enforcement |
| Stress | > 10 | < 10s each | System-level | Resource limits |
| Performance | > 10 | < 30s each | System-level | Latency/throughput |

### 1.3 Test Execution Order

```
1. Unit tests (fastest feedback)
2. Integration tests
3. Regression tests
4. Security tests
5. Stress tests
6. Performance tests (slowest)
```

---

## 2. Unit Tests

### 2.1 Unit Test Requirements

| ID | Requirement | Priority |
|---|---|---|
| UT-001 | Every public function SHALL have at least one unit test | MUST |
| UT-002 | Every algorithm defined in DOC-04 SHALL have unit tests | MUST |
| UT-003 | Every edge case documented in DOC-04 SHALL have a unit test | MUST |
| UT-004 | Unit tests SHALL be deterministic (same input → same output) | MUST |
| UT-005 | Unit tests SHALL NOT depend on external services | MUST |
| UT-006 | Unit tests SHALL complete within 1ms each | SHOULD |
| UT-007 | Unit tests SHALL use mock/stub for I/O boundaries | SHOULD |

### 2.2 Unit Test Categories by Module

| Module | Test Focus | Example Tests |
|---|---|---|
| `language/tokenizer` | Token encoding, normalization | UTF-8 handling, unknown tokens, vocabulary overflow |
| `language/vocabulary` | Lookup, creation, frequency | Capacity limits, frequency tracking, new symbol discovery |
| `language/syntax` | Parsing, role assignment | Dependency parsing, structural context |
| `language/semantics` | Concept extraction, relations | Entity recognition, relation extraction |
| `neural/cell` | Cell state machine | Activation, inhibition, prediction, adaptation |
| `neural/column` | Competition, sparse selection | Top-k selection, sparsity enforcement |
| `neural/plasticity` | Weight updates | ΔW computation, bounding, single-observation limit |
| `memory/working` | Active state management | Context assembly, hypothesis tracking |
| `memory/episodic` | Storage, retrieval, eviction | Capacity management, value-based eviction |
| `memory/semantic` | Knowledge storage, retrieval | Evidence tracking, confidence updates |
| `memory/procedural` | Procedure storage, usage | Success/failure tracking, context requirements |
| `memory/associative` | Association storage, lookup | Index management, strength updates |
| `memory/retrieval` | Query, relevance scoring | Multi-factor scoring, contradiction detection |
| `memory/consolidation` | Pattern extraction, integration | Candidate evaluation, single-event prevention |
| `world/entity` | CRUD, identity | Entity creation, update, deduplication |
| `world/transition` | State prediction | S(t)+A(t)→S(t+1), prediction confidence |
| `world/causal` | Causal hypotheses | Correlation vs causation distinction |
| `reasoning/hypothesis` | Generation, evaluation | Evidence gathering, confidence update |
| `reasoning/evidence` | Collection, scoring | Source quality, consistency checking |
| `reasoning/contradiction` | Detection, severity | Pairwise comparison, severity scoring |
| `planning/plan` | Construction, ranking | Goal extraction, plan scoring |
| `planning/risk` | Risk estimation | Multi-factor risk assessment |
| `verification/confidence` | Confidence computation | Multi-dimensional aggregation |
| `learning/signal` | Signal generation | Experience → signal conversion |
| `learning/attribution` | Error attribution | 6-source attribution |
| `learning/stability` | Stability guard | Catastrophic change prevention |
| `learning/replay` | Priority, execution | 5-factor priority, budget enforcement |
| `policy/risk` | Risk estimation | 5-factor risk scoring |
| `policy/gate` | Decision pipeline | ALLOW/LIMIT/DENY decisions |
| `persistence/format` | Section read/write | Section serialization, checksum |
| `persistence/checkpoint` | Checkpoint lifecycle | Creation, recovery, cleanup |
| `persistence/migration` | State migration | Version detection, sequential migration |
| `config` | Validation pipeline | Schema, range, dependency, policy validation |
| `error` | Taxonomy, classification | Error kind mapping, recovery flags |

---

## 3. Integration Tests

### 3.1 Integration Test Requirements

| ID | Requirement | Priority |
|---|---|---|
| IT-001 | Full cognitive pipeline: input → response with state update | MUST |
| IT-002 | Persistence round-trip: save → load produces equivalent state | MUST |
| IT-003 | Learning stability: single observation does not destabilize state | MUST |
| IT-004 | Policy enforcement: prohibited operations are denied | MUST |
| IT-005 | Corruption recovery: corrupt `.cx` triggers recovery | MUST |
| IT-006 | API endpoints: all endpoints respond correctly | MUST |
| IT-007 | CLI commands: all commands execute correctly | MUST |
| IT-008 | Configuration validation: invalid config prevents startup | MUST |
| IT-009 | Graceful shutdown: state is preserved on SIGTERM | MUST |
| IT-010 | First boot: state is created from configuration | MUST |

### 3.2 Integration Test Scenarios

#### 3.2.1 Cognitive Pipeline Test

```
INPUT: "What is gravity?"
STEPS:
  1. Parse observation
  2. Encode language
  3. Process neural representation
  4. Retrieve memories
  5. Integrate world state
  6. Evaluate reasoning
  7. Evaluate planning (optional)
  8. Verify claims
  9. Generate response
  10. Record experience
  11. Apply learning
  12. Checkpoint (if interval reached)
EXPECTED: Non-empty response with confidence and verification status
INVARIANT: State is consistent after pipeline completion
```

#### 3.2.2 Persistence Round-Trip Test

```
INPUT: Populated CortexState
STEPS:
  1. Save state to .cx
  2. Load state from .cx
  3. Compare loaded state with original
EXPECTED: Semantically equivalent state
INVARIANT: Save(State) → Load(State) produces equivalent state
```

#### 3.2.3 Learning Stability Test

```
INPUT: CortexState + single high-magnitude observation
STEPS:
  1. Record state snapshot before observation
  2. Process observation through cognitive pipeline
  3. Record state snapshot after observation
  4. Compute state difference
EXPECTED: State difference < 10% of total state
INVARIANT: Single observation cannot change > 10% of state
```

#### 3.2.4 Corruption Recovery Test

```
INPUT: Valid .cx file
STEPS:
  1. Corrupt .cx file (overwrite random bytes)
  2. Attempt to load .cx
  3. Verify recovery behavior
EXPECTED: Recovery from checkpoint or fresh initialization
INVARIANT: Corrupt state triggers recovery, never silent continuation
```

---

## 4. Regression Tests

### 4.1 Regression Test Requirements

| ID | Requirement | Priority |
|---|---|---|
| RT-001 | State format backward compatibility: old `.cx` loads in new version | MUST |
| RT-002 | Algorithm version compatibility: old algorithm versions produce valid state | MUST |
| RT-003 | Configuration backward compatibility: old config loads in new version | MUST |
| RT-004 | API backward compatibility: old API calls work with new version | MUST |
| RT-005 | CLI backward compatibility: old CLI commands work with new version | MUST |

### 4.2 Regression Test Scenarios

#### 4.2.1 State Compatibility Test

```
INPUT: .cx file from previous version
STEPS:
  1. Load .cx with current version
  2. Verify integrity
  3. Verify invariants
  4. Verify semantic equivalence
EXPECTED: State loads successfully and is semantically equivalent
```

#### 4.2.2 Migration Test

```
INPUT: .cx file from N-2 versions ago
STEPS:
  1. Load .cx with current version
  2. Apply sequential migrations
  3. Verify each migration step
  4. Verify final state
EXPECTED: State migrates through all versions successfully
```

---

## 5. Security Tests

### 5.1 Security Test Requirements

| ID | Requirement | Priority |
|---|---|---|
| ST-001 | API authentication: unauthenticated requests rejected | MUST |
| ST-002 | Policy gate: prohibited operations denied | MUST |
| ST-003 | Learning isolation: learning cannot modify Level 3 policy | MUST |
| ST-004 | Secret isolation: API key not in `.cx` or logs | MUST |
| ST-005 | Fail-closed: ambiguous security decisions default to DENY | MUST |
| ST-006 | Input validation: malformed inputs rejected | MUST |
| ST-007 | State integrity: tampered `.cx` detected and rejected | MUST |
| ST-008 | Overflow protection: resource limits enforced | MUST |

### 5.2 Security Test Scenarios

#### 5.2.1 Authentication Test

```
INPUT: API request without Authorization header
STEPS:
  1. Send request to any endpoint
  2. Verify response status
EXPECTED: 401 Unauthorized
```

#### 5.2.2 Policy Denial Test

```
INPUT: Learning operation with policy.learning = false
STEPS:
  1. Configure policy.learning = false
  2. Submit learning experience
  3. Verify policy decision
EXPECTED: PolicyDecision::Denied
```

#### 5.2.3 Secret Isolation Test

```
INPUT: Completed session with API interactions
STEPS:
  1. Save state to .cx
  2. Inspect .cx binary content
  3. Search for API key string
EXPECTED: API key not found in .cx
```

---

## 6. Stress Tests

### 6.1 Stress Test Requirements

| ID | Requirement | Priority |
|---|---|---|
| SRT-001 | Memory pressure: system degrades gracefully at budget limits | MUST |
| SRT-002 | Compute budget: operations terminate at configured limits | MUST |
| SRT-003 | Vocabulary capacity: system handles vocabulary overflow | MUST |
| SRT-004 | Memory capacity: eviction triggers at budget boundaries | MUST |
| SRT-005 | Concurrent API requests: system handles max connections | MUST |
| SRT-006 | Large input: system handles input at context window limit | MUST |
| SRT-007 | Long session: system maintains state integrity over extended operation | MUST |

### 6.2 Stress Test Scenarios

#### 6.2.1 Memory Pressure Test

```
INPUT: Continuous observations until memory budget exceeded
STEPS:
  1. Configure small memory budgets
  2. Submit observations until budget exceeded
  3. Verify pressure response
  4. Verify no data corruption
EXPECTED: Pressure response triggers; state remains valid
```

#### 6.2.2 Compute Budget Exhaustion Test

```
INPUT: Complex reasoning query with small budget
STEPS:
  1. Set max_reasoning_steps = 4
  2. Submit complex reasoning query
  3. Verify bounded result
EXPECTED: Reasoning terminates at budget; result carries uncertainty flag
```

---

## 7. Acceptance Criteria

### 7.1 Deployment Acceptance

| # | Criterion | Test |
|---|---|---|
| AC-DEP-001 | Binary starts with valid config and no state (first boot) | Integration test |
| AC-DEP-002 | Binary creates `.cx` on first boot | Integration test |
| AC-DEP-003 | Binary loads existing `.cx` on subsequent boots | Integration test |
| AC-DEP-004 | Binary rejects invalid config with clear error | Unit test |
| AC-DEP-005 | No external service required for core operation | Deployment verification |
| AC-DEP-006 | Deployment = binary + config + auto-created `.cx` | Deployment verification |

### 7.2 Cognitive Pipeline Acceptance

| # | Criterion | Test |
|---|---|---|
| AC-COG-001 | Text input produces LanguageState | Unit test |
| AC-COG-002 | LanguageState produces NeuralRepresentation | Unit test |
| AC-COG-003 | Memory retrieval returns relevant memories | Unit test |
| AC-COG-004 | World model integrates observations | Unit test |
| AC-COG-005 | Reasoning produces ranked hypotheses | Unit test |
| AC-COG-006 | Planning produces bounded plans | Unit test |
| AC-COG-007 | Verification classifies claims correctly | Unit test |
| AC-COG-008 | Language generation produces coherent output | Unit test |
| AC-COG-009 | Full pipeline: input → response with state update | Integration test |

### 7.3 Learning Acceptance

| # | Criterion | Test |
|---|---|---|
| AC-LRN-001 | Prediction error is computed and attributed | Unit test |
| AC-LRN-002 | Learning signal modifies state within bounds | Unit test |
| AC-LRN-003 | Single observation does not destabilize state | Stability test |
| AC-LRN-004 | Replay produces learning from prior episodes | Unit test |
| AC-LRN-005 | Consolidation forms long-term knowledge | Unit test |
| AC-LRN-006 | Vocabulary expands without rebuild | Unit test |
| AC-LRN-007 | Learning respects policy constraints | Policy test |

### 7.4 Persistence Acceptance

| # | Criterion | Test |
|---|---|---|
| AC-PRS-001 | Save → Load produces semantically equivalent state | Round-trip test |
| AC-PRS-002 | Atomic write preserves last valid state on failure | Fault injection test |
| AC-PRS-003 | Corrupt `.cx` triggers recovery | Corruption test |
| AC-PRS-004 | Checkpoint creation and recovery works | Integration test |
| AC-PRS-005 | State migration preserves semantic content | Migration test |

### 7.5 Security Acceptance

| # | Criterion | Test |
|---|---|---|
| AC-SEC-001 | API requires valid bearer token | Auth test |
| AC-SEC-002 | Policy gate denies prohibited operations | Policy test |
| AC-SEC-003 | Learning cannot modify Level 3 policy | Security test |
| AC-SEC-004 | API key not present in `.cx` | State inspection |
| AC-SEC-005 | Fail-closed on ambiguous security decisions | Security test |

### 7.6 Resource Acceptance

| # | Criterion | Test |
|---|---|---|
| AC-RES-001 | Memory usage stays within configured budgets | Resource test |
| AC-RES-002 | Reasoning terminates at max_steps | Bound test |
| AC-RES-003 | Planning terminates at max_depth × max_branches | Bound test |
| AC-RES-004 | Generation terminates at generation_limit | Bound test |
| AC-RES-005 | Budget-exhausted operations return bounded results | Bound test |

---

## 8. Test Data Management

### 8.1 Test Data Rules

| Rule | Description |
|---|---|
| TDM-001 | Test data SHALL be self-contained (no external dependencies) |
| TDM-002 | Test configurations SHALL use minimal valid values |
| TDM-003 | Test states SHALL be created programmatically, not from files |
| TDM-004 | Test cleanup SHALL restore clean state after each test |
| TDM-005 | Test data SHALL NOT contain real user data |

### 8.2 Test Helpers

```rust
// Test helper functions
pub fn create_test_config() -> CortexConfig { /* minimal valid config */ }
pub fn create_test_state() -> CortexState { /* minimal valid state */ }
pub fn create_test_runtime() -> CortexRuntime { /* full runtime with test config */ }
pub fn corrupt_cx_file(path: &Path) { /* corrupt a .cx file */ }
pub fn create_populated_state() -> CortexState { /* state with test data */ }
```

---

## 9. Test Invariants

### 9.1 Test Invariant List

| # | Invariant | Enforcement |
|---|---|---|
| TST-001 | All unit tests are deterministic | Test execution |
| TST-002 | All integration tests clean up after themselves | Test teardown |
| TST-003 | No test depends on another test's state | Test isolation |
| TST-004 | All acceptance criteria have corresponding tests | Traceability matrix |
| TST-005 | Test coverage is tracked per module | Coverage tool |
| TST-006 | Failing tests block CI pipeline | CI gate |
| TST-007 | Test results are recorded and retained | CI artifacts |

---

## 10. Traceability

### 10.1 Traceability to Requirements

| DOC-01 Requirement | DOC-07 Test Coverage |
|---|---|
| FR-LANG-* | Unit tests for language modules |
| FR-NEUR-* | Unit tests for neural modules |
| FR-MEM-* | Unit + integration tests for memory modules |
| FR-WRLD-* | Unit tests for world model modules |
| FR-RSN-* | Unit tests for reasoning modules |
| FR-PLN-* | Unit tests for planning modules |
| FR-VER-* | Unit tests for verification modules |
| FR-LRN-* | Unit + stability tests for learning modules |
| FR-SLF-* | Unit tests for self model |
| FR-POL-* | Security tests for policy enforcement |
| FR-INT-* | Integration tests for internet interface |
| FR-PRS-* | Persistence round-trip + corruption tests |
| AC-* | Acceptance tests per criterion |

### 10.2 Final Testing Contract Statement

> **This document constitutes the testing and validation contract for CORTEX.** It defines every test category, every test requirement, and every acceptance criterion.
>
> The testing contract ensures:
> - **Algorithm correctness**: Every algorithm has unit tests.
> - **Pipeline integrity**: Integration tests verify full cognitive pipeline.
> - **State compatibility**: Regression tests ensure backward compatibility.
> - **Security enforcement**: Security tests verify policy and authentication.
> - **Resource bounds**: Stress tests verify resource limit enforcement.
> - **Acceptance criteria**: Every DOC-01 acceptance criterion has a corresponding test.
>
> **CORTEX testing contract: 6 test categories, 120+ test requirements, 30+ acceptance criteria, full traceability to DOC-01.**

---

*End of Document — CORTEX-DOC-07 Testing & Validation Specification v1.1.0*
