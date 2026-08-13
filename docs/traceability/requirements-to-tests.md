# Requirements to Test Traceability

Mapping from DOC-01 requirements to test implementations.

## Test Coverage Matrix

| Requirement | Description | Test Category | Test Location |
|---|---|---|---|
| **Language Processing** | | | |
| FR-LANG-001 | Natural language input → LanguageState | Unit | `tests/unit/test_language_tokenizer.rs` |
| FR-LANG-002 | Tokenize into symbols | Unit | `tests/unit/test_language_tokenizer.rs` |
| FR-LANG-003 | Dynamic vocabulary | Unit | `tests/unit/test_language_vocabulary.rs` |
| FR-LANG-004 | Unknown symbol discovery | Unit | `tests/unit/test_language_vocabulary.rs` |
| FR-LANG-005 | Syntactic structure parsing | Unit | `tests/unit/test_language_syntax.rs` |
| FR-LANG-006 | Semantic representations | Unit | `tests/unit/test_language_semantics.rs` |
| FR-LANG-007 | Hierarchical context | Unit | `tests/unit/test_language_context.rs` |
| FR-LANG-008 | Intent as ranked hypotheses | Unit | `tests/unit/test_language_model.rs` |
| FR-LANG-009 | Predict candidate continuations | Unit | `tests/unit/test_language_model.rs` |
| FR-LANG-010 | Generate natural language output | Unit | `tests/unit/test_language_decoder.rs` |
| FR-LANG-011 | Context window limit | Stress | `tests/stress/test_resource_limits.rs` |
| FR-LANG-012 | Generation limit | Stress | `tests/stress/test_resource_limits.rs` |
| FR-LANG-013 | Learn new symbols | Unit | `tests/unit/test_language_vocabulary.rs` |
| FR-LANG-014 | Vocabulary vs semantic tracking | Unit | `tests/unit/test_language_vocabulary.rs` |
| FR-LANG-015 | Disabled language mode | Integration | `tests/integration/test_disabled_subsystems.rs` |
| **Neural Processing** | | | |
| FR-NEUR-001 | Sparse temporal representations | Unit | `tests/unit/test_neural_cell.rs` |
| FR-NEUR-002 | Cell state machine (5 states) | Unit | `tests/unit/test_neural_cell.rs` |
| FR-NEUR-003 | Column competition | Unit | `tests/unit/test_neural_column.rs` |
| FR-NEUR-004 | Field management | Unit | `tests/unit/test_neural_field.rs` |
| FR-NEUR-005 | Sparsity enforcement | Unit | `tests/unit/test_neural_field.rs` |
| FR-NEUR-006 | Temporal encoding | Unit | `tests/unit/test_neural_temporal.rs` |
| FR-NEUR-007 | Next-state prediction | Unit | `tests/unit/test_neural_field.rs` |
| FR-NEUR-008 | Bounded plasticity | Unit | `tests/unit/test_neural_plasticity.rs` |
| FR-NEUR-009 | Prediction error computation | Unit | `tests/unit/test_neural_field.rs` |
| **Memory System** | | | |
| FR-MEM-001 | Five memory subsystems | Integration | `tests/integration/test_memory_system.rs` |
| FR-MEM-002 | Working memory bounded | Unit | `tests/unit/test_memory_working.rs` |
| FR-MEM-003 | Episodic memory storage | Unit | `tests/unit/test_memory_episodic.rs` |
| FR-MEM-004 | Semantic memory knowledge | Unit | `tests/unit/test_memory_semantic.rs` |
| FR-MEM-005 | Procedural memory | Unit | `tests/unit/test_memory_procedural.rs` |
| FR-MEM-006 | Associative memory | Unit | `tests/unit/test_memory_associative.rs` |
| FR-MEM-007 | Memory retrieval | Unit | `tests/unit/test_memory_retrieval.rs` |
| FR-MEM-008 | Memory consolidation | Unit | `tests/unit/test_memory_consolidation.rs` |
| FR-MEM-009 | Controlled forgetting | Unit | `tests/unit/test_memory_consolidation.rs` |
| FR-MEM-010 | Provenance preservation | Unit | `tests/unit/test_memory_retrieval.rs` |
| FR-MEM-011 | Semantic verification tracking | Unit | `tests/unit/test_memory_semantic.rs` |
| **World Model** | | | |
| FR-WRLD-001 | World model with entities | Unit | `tests/unit/test_world_entity.rs` |
| FR-WRLD-002 | Entity kinds | Unit | `tests/unit/test_world_entity.rs` |
| FR-WRLD-003 | State transition prediction | Unit | `tests/unit/test_world_transition.rs` |
| FR-WRLD-004 | Correlation vs causation | Unit | `tests/unit/test_world_causal.rs` |
| FR-WRLD-005 | Counterfactual trajectories | Unit | `tests/unit/test_world_simulation.rs` |
| FR-WRLD-006 | World state persistence | Integration | `tests/integration/test_persistence_roundtrip.rs` |
| FR-WRLD-007 | Disabled world mode | Integration | `tests/integration/test_disabled_subsystems.rs` |
| **Reasoning** | | | |
| FR-RSN-001 | Hypothesis-based reasoning | Unit | `tests/unit/test_reasoning_hypothesis.rs` |
| FR-RSN-002 | 9 reasoning types | Unit | `tests/unit/test_reasoning_hypothesis.rs` |
| FR-RSN-003 | Bounded reasoning steps | Stress | `tests/stress/test_resource_limits.rs` |
| FR-RSN-004 | No automatic knowledge promotion | Unit | `tests/unit/test_reasoning_hypothesis.rs` |
| FR-RSN-005 | Conflict retention | Unit | `tests/unit/test_reasoning_contradiction.rs` |
| FR-RSN-006 | Disabled reasoning mode | Integration | `tests/integration/test_disabled_subsystems.rs` |
| **Planning** | | | |
| FR-PLN-001 | Goal-directed planning | Unit | `tests/unit/test_planning_plan.rs` |
| FR-PLN-002 | Bounded planning | Stress | `tests/stress/test_resource_limits.rs` |
| FR-PLN-003 | Plan structure | Unit | `tests/unit/test_planning_plan.rs` |
| FR-PLN-004 | Disabled planning mode | Integration | `tests/integration/test_disabled_subsystems.rs` |
| **Verification** | | | |
| FR-VER-001 | 7 claim classifications | Unit | `tests/unit/test_verification_confidence.rs` |
| FR-VER-002 | Evidence evaluation pipeline | Unit | `tests/unit/test_verification_confidence.rs` |
| FR-VER-003 | No silent status upgrade | Security | `tests/security/test_policy_enforcement.rs` |
| FR-VER-004 | Confidence as separate dimension | Unit | `tests/unit/test_verification_confidence.rs` |
| FR-VER-005 | Minimum confidence threshold | Unit | `tests/unit/test_verification_confidence.rs` |
| FR-VER-006 | Disabled verification mode | Integration | `tests/integration/test_disabled_subsystems.rs` |
| **Learning** | | | |
| FR-LRN-001 | Learn without retraining | Unit | `tests/unit/test_learning_signal.rs` |
| FR-LRN-002 | Three learning speeds | Unit | `tests/unit/test_learning_signal.rs` |
| FR-LRN-003 | 9 learning sources | Unit | `tests/unit/test_learning_signal.rs` |
| FR-LRN-004 | Prediction error as signal | Unit | `tests/unit/test_learning_signal.rs` |
| FR-LRN-005 | Error attribution (6 sources) | Unit | `tests/unit/test_learning_attribution.rs` |
| FR-LRN-006 | Experience replay | Unit | `tests/unit/test_learning_replay.rs` |
| FR-LRN-007 | Single-event prevention | Stability | `tests/integration/test_learning_stability.rs` |
| FR-LRN-008 | Bounded, attributable learning | Unit | `tests/unit/test_learning_stability.rs` |
| FR-LRN-009 | Disabled learning mode | Integration | `tests/integration/test_disabled_subsystems.rs` |
| **Self Model** | | | |
| FR-SLF-001 | Computational self-model | Unit | `tests/unit/test_self_model_capability.rs` |
| FR-SLF-002 | Track capabilities | Unit | `tests/unit/test_self_model_capability.rs` |
| FR-SLF-003 | Not consciousness proof | Security | `tests/security/test_policy_enforcement.rs` |
| FR-SLF-004 | No policy authority | Security | `tests/security/test_policy_enforcement.rs` |
| **Policy** | | | |
| FR-POL-001 | Policy gate on operations | Security | `tests/security/test_policy_enforcement.rs` |
| FR-POL-002 | ALLOW/LIMIT/DENY decisions | Security | `tests/security/test_policy_enforcement.rs` |
| FR-POL-003 | Learning cannot modify root policy | Security | `tests/security/test_policy_enforcement.rs` |
| FR-POL-004 | Level 3 restriction | Security | `tests/security/test_policy_enforcement.rs` |
| FR-POL-005 | Policy separate from knowledge | Security | `tests/security/test_policy_enforcement.rs` |
| FR-POL-006 | Planner cannot bypass policy | Security | `tests/security/test_policy_enforcement.rs` |
| **Persistence** | | | |
| FR-PRS-001 | Single .cx binary file | Integration | `tests/integration/test_persistence_roundtrip.rs` |
| FR-PRS-002 | Atomic write | Integration | `tests/integration/test_persistence_roundtrip.rs` |
| FR-PRS-003 | Failed write preserves state | Corruption | `tests/integration/test_corruption_recovery.rs` |
| FR-PRS-004 | Periodic checkpointing | Integration | `tests/integration/test_persistence_roundtrip.rs` |
| FR-PRS-005 | Integrity on load | Corruption | `tests/integration/test_corruption_recovery.rs` |
| FR-PRS-006 | Invalid state triggers STOP | Corruption | `tests/integration/test_corruption_recovery.rs` |
| **API** | | | |
| FR-API-001 | 7 API endpoints | API | `tests/integration/test_api_endpoints.rs` |
| FR-API-002 | Bearer token auth | Security | `tests/security/test_policy_enforcement.rs` |
| FR-API-003 | No arbitrary mutation | Security | `tests/security/test_policy_enforcement.rs` |
| FR-API-004 | Disabled API mode | Integration | `tests/integration/test_disabled_subsystems.rs` |
| **CLI** | | | |
| FR-CLI-001 | 12 CLI commands | Integration | `tests/integration/test_cli_commands.rs` |
| **Acceptance Criteria** | | | |
| AC-DEP-001 to AC-DEP-006 | Deployment acceptance | Acceptance | `tests/acceptance/test_deployment.rs` |
| AC-COG-001 to AC-COG-009 | Cognitive pipeline acceptance | Acceptance | `tests/acceptance/test_cognitive_pipeline.rs` |
| AC-LRN-001 to AC-LRN-007 | Learning acceptance | Acceptance | `tests/acceptance/test_learning.rs` |
| AC-PRS-001 to AC-PRS-005 | Persistence acceptance | Acceptance | `tests/acceptance/test_persistence.rs` |
| AC-SEC-001 to AC-SEC-005 | Security acceptance | Acceptance | `tests/acceptance/test_security.rs` |
| AC-RES-001 to AC-RES-004 | Resource acceptance | Acceptance | `tests/acceptance/test_resources.rs` |

## Test Category Summary

| Category | Minimum Count | Purpose |
|---|---|---|
| Unit | > 500 | Algorithm correctness |
| Integration | > 50 | Cross-module interaction |
| System | > 10 | End-to-end pipeline |
| Acceptance | > 30 | DOC-01 criteria |
| Regression | > 20 | Backward compatibility |
| Property | > 10 | Invariant verification |
| Performance | > 10 | Latency/throughput |
| Security | > 30 | Policy, auth, injection |
