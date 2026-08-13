# Requirements to Design Traceability

Mapping from DOC-01 requirements to DOC-02 design specifications.

## Language Processing Requirements

| Requirement | Description | DOC-02 Design Section |
|---|---|---|
| FR-LANG-001 | Accept natural language input → LanguageState | §16 Language Core, LanguageCore trait |
| FR-LANG-002 | Tokenize into symbols, subwords, words, markers | §16 `language/tokenizer.rs` |
| FR-LANG-003 | Dynamic vocabulary up to `language.vocabulary_capacity` | §16 `language/vocabulary.rs` |
| FR-LANG-004 | Unknown symbol discovery, frequency tracking | §16 `language/vocabulary.rs` |
| FR-LANG-005 | Parse syntactic structure | §16 `language/syntax.rs` |
| FR-LANG-006 | Construct semantic representations | §16 `language/semantics.rs` |
| FR-LANG-007 | Hierarchical context management | §16 `language/context.rs` |
| FR-LANG-008 | Intent as ranked hypotheses | §16 `language/language_model.rs` |
| FR-LANG-009 | Predict candidate continuations | §16 `language/language_model.rs` |
| FR-LANG-010 | Generate natural language output | §16 `language/decoder.rs` |
| FR-LANG-011 | Context window up to `language.context_window` | §16 `language/context.rs` |
| FR-LANG-012 | Generation limit `language.generation_limit` | §16 `language/decoder.rs` |
| FR-LANG-013 | Learn new symbols, vocabulary, patterns | §16 `language/vocabulary.rs` |
| FR-LANG-014 | Vocabulary membership vs semantic understanding | §16 LanguageCore trait |
| FR-LANG-015 | Disabled language: raw observation mode | §16 LanguageCore trait, graceful degradation |

## Neural Processing Requirements

| Requirement | Description | DOC-02 Design Section |
|---|---|---|
| FR-NEUR-001 | Process into sparse temporal neural representations | §15 NeuralCore trait |
| FR-NEUR-002 | Cell with 5 states: Resting, Active, Inhibited, Learning, Predicting | §15 `neural/cell.rs` |
| FR-NEUR-003 | Columns with local competition, sparse selection | §15 `neural/column.rs` |
| FR-NEUR-004 | Fields representing learned structures | §15 `neural/field.rs` |
| FR-NEUR-005 | Sparsity enforcement: active ≤ field_size × sparsity_ratio | §15 `neural/field.rs` |
| FR-NEUR-006 | Temporal representations | §15 `neural/temporal.rs` |
| FR-NEUR-007 | Next-state prediction as first-class operation | §15 `neural/field.rs` |
| FR-NEUR-008 | Bounded plasticity: ΔW = η × A × C × E × V | §15 `neural/plasticity.rs` |
| FR-NEUR-009 | Prediction error computation | §15 `neural/field.rs` |

## Memory Requirements

| Requirement | Description | DOC-02 Design Section |
|---|---|---|
| FR-MEM-001 | Five memory subsystems | §17 MemorySystem trait |
| FR-MEM-002 | Working memory bounded by `memory.working_mb` | §17 `memory/working.rs` |
| FR-MEM-003 | Episodic memory with episode structure | §17 `memory/episodic.rs` |
| FR-MEM-004 | Semantic memory with knowledge structure | §17 `memory/semantic.rs` |
| FR-MEM-005 | Procedural memory with procedure structure | §17 `memory/procedural.rs` |
| FR-MEM-006 | Associative memory with typed associations | §17 `memory/associative.rs` |
| FR-MEM-007 | Memory retrieval with relevance scoring | §17 `memory/retrieval.rs` |
| FR-MEM-008 | Memory consolidation: merge, compress, generalize | §17 `memory/consolidation.rs` |
| FR-MEM-009 | Controlled forgetting | §17 `memory/consolidation.rs` |
| FR-MEM-010 | Provenance preservation | §17 MemorySystem trait |
| FR-MEM-011 | Semantic memory verification tracking | §17 `memory/semantic.rs` |

## World Model Requirements

| Requirement | Description | DOC-02 Design Section |
|---|---|---|
| FR-WRLD-001 | World model with entities, relations, events | §18 WorldModelInterface trait |
| FR-WRLD-002 | Entity kinds: Person, Object, Place, etc. | §18 `world/entity.rs` |
| FR-WRLD-003 | State transition prediction: S(t) + A(t) → S(t+1) | §18 `world/transition.rs` |
| FR-WRLD-004 | Correlation vs causation distinction | §18 `world/causal.rs` |
| FR-WRLD-005 | Counterfactual trajectories | §18 `world/simulation.rs` |
| FR-WRLD-006 | World state persistence in .cx | §18 WorldModelInterface |
| FR-WRLD-007 | Disabled world: empty state | §18 WorldModelInterface, graceful degradation |

## Reasoning Requirements

| Requirement | Description | DOC-02 Design Section |
|---|---|---|
| FR-RSN-001 | Hypothesis-based reasoning with evidence | §19 ReasoningEngine trait |
| FR-RSN-002 | 9 reasoning types | §19 `reasoning/hypothesis.rs` |
| FR-RSN-003 | Bounded by `reasoning.max_steps` | §19 ReasoningEngine trait |
| FR-RSN-004 | No automatic knowledge promotion | §19 ReasoningEngine trait |
| FR-RSN-005 | Conflict retention with evaluation | §19 `reasoning/contradiction.rs` |
| FR-RSN-006 | Disabled reasoning: direct memory retrieval | §19 ReasoningEngine trait |

## Planning Requirements

| Requirement | Description | DOC-02 Design Section |
|---|---|---|
| FR-PLN-001 | Goal-directed planning with simulation | §20 PlanningEngine trait |
| FR-PLN-002 | Bounded by depth and branches | §20 PlanningEngine trait |
| FR-PLN-003 | Plan structure: goal, steps, outcomes, cost, risk | §20 `planning/plan.rs` |
| FR-PLN-004 | Disabled planning: no goal-directed planning | §20 PlanningEngine trait |

## Verification Requirements

| Requirement | Description | DOC-02 Design Section |
|---|---|---|
| FR-VER-001 | 7 claim classifications | §21 VerificationEngine trait |
| FR-VER-002 | Evidence evaluation pipeline | §21 VerificationEngine trait |
| FR-VER-003 | No silent UNKNOWN → VERIFIED upgrade | §21 VerificationEngine trait |
| FR-VER-004 | Confidence and verification as separate dimensions | §21 `verification/confidence.rs` |
| FR-VER-005 | `verification.minimum_confidence` threshold | §21 VerificationEngine trait |
| FR-VER-006 | Disabled verification: provisional claims | §21 VerificationEngine trait |

## Learning Requirements

| Requirement | Description | DOC-02 Design Section |
|---|---|---|
| FR-LRN-001 | Learn without retraining | §22 LearningSystem trait |
| FR-LRN-002 | Three learning speeds: Fast, Medium, Slow | §22 LearningSystem trait |
| FR-LRN-003 | 9 learning sources | §22 `learning/signal.rs` |
| FR-LRN-004 | Prediction error as principal signal | §22 `learning/signal.rs` |
| FR-LRN-005 | Error attribution across 6 sources | §22 `learning/attribution.rs` |
| FR-LRN-006 | Experience replay with priority | §22 `learning/replay.rs` |
| FR-LRN-007 | Single-event prevention in consolidation | §22 `learning/stability.rs` |
| FR-LRN-008 | Bounded, attributable, policy-respecting learning | §22 LearningSystem trait |
| FR-LRN-009 | Disabled learning: no state mutation | §22 LearningSystem trait |

## Self Model Requirements

| Requirement | Description | DOC-02 Design Section |
|---|---|---|
| FR-SLF-001 | Computational self-model | §23 SelfModelInterface trait |
| FR-SLF-002 | Track capabilities, limitations, accuracy | §23 `self_model/capability.rs` |
| FR-SLF-003 | Not interpreted as consciousness | §23 SelfModelInterface trait |
| FR-SLF-004 | No authority to change policy | §23 SelfModelInterface trait |

## Policy Requirements

| Requirement | Description | DOC-02 Design Section |
|---|---|---|
| FR-POL-001 | Policy gate on all consequential operations | §24 PolicyEngine trait |
| FR-POL-002 | ALLOW / LIMIT / DENY decisions | §24 `policy/gate.rs` |
| FR-POL-003 | Learning cannot modify root policy | §24 PolicyEngine trait |
| FR-POL-004 | Level 3 restriction | §24 PolicyEngine trait |
| FR-POL-005 | Policy separate from learned knowledge | §24 PolicyEngine trait |
| FR-POL-006 | Planner cannot bypass policy | §24 PolicyEngine trait |

## Persistence Requirements

| Requirement | Description | DOC-02 Design Section |
|---|---|---|
| FR-PRS-001 | Single .cx binary file | §26 PersistenceEngine trait |
| FR-PRS-002 | Atomic write: temp → flush → verify → replace | §26 `persistence/format.rs` |
| FR-PRS-003 | Failed write preserves last valid state | §26 PersistenceEngine trait |
| FR-PRS-004 | Periodic checkpointing | §26 `persistence/checkpoint.rs` |
| FR-PRS-005 | Integrity, version, migration on load | §26 `persistence/format.rs` |
| FR-PRS-006 | Invalid state triggers STOP or recovery | §26 PersistenceEngine trait |

## API Requirements

| Requirement | Description | DOC-02 Design Section |
|---|---|---|
| FR-API-001 | 7 API endpoints | §29 API Server |
| FR-API-002 | Bearer token via `CORTEX_API_KEY` | §29 `api/auth.rs` |
| FR-API-003 | No direct arbitrary state mutation | §29 `api/handlers.rs` |
| FR-API-004 | Disabled API: CLI only | §29 API Server |

## CLI Requirements

| Requirement | Description | DOC-02 Design Section |
|---|---|---|
| FR-CLI-001 | 12 CLI commands | §30 CLI Layer |
