# CORTEX — 04 Algorithm Specification

---

## Document Control

| Property | Value |
|---|---|
| **Document ID** | CORTEX-DOC-04 |
| **Title** | Algorithm Specification |
| **Version** | 1.1.0 |
| **Status** | Final Architectural Baseline |
| **Classification** | Computational Behavior Contract |
| **Scope** | All algorithms, computational processes, decision procedures |
| **Parent Document** | CORTEX-DOC-03 Data & State Specification |
| **Effective Date** | 2026-08-13 |
| **Review Cycle** | Per algorithm version transition |

### Revision History

| Version | Date | Author | Description |
|---|---|---|---|
| 1.0.0 | 2026-08-13 | CORTEX Architecture | Initial final baseline |
| 1.1.0 | 2026-08-13 | CORTEX Architecture | Replace SHA-256 with BLAKE3-256 for all hashing operations |

### Approval

| Role | Signature | Date |
|---|---|---|
| System Architect | _____________ | _____________ |
| Algorithm Lead | _____________ | _____________ |
| Verification Lead | _____________ | _____________ |

### Document Purpose

This document defines **how CORTEX computes, thinks, learns, plans, verifies, and acts**. It constitutes the computational behavior contract: every algorithm, every decision procedure, every mathematical formula, and every execution semantic that governs CORTEX's cognitive operations.

### Document Scope

This specification covers:

- Every algorithm in the CORTEX cognitive pipeline with pseudocode.
- Every mathematical formula used in computation.
- Every decision procedure with explicit conditions.
- Every resource-bounded execution semantic.
- Every failure mode and recovery procedure.
- Every numerical stability requirement.
- Every complexity bound.

This specification does NOT cover:

- Data structure definitions (governed by DOC-03).
- Module organization (governed by DOC-02).
- System requirements (governed by DOC-01).

---

## 1. Algorithm Design Principles

| # | Principle | Implication |
|---|---|---|
| ADP-001 | Bounded execution | Every algorithm has explicit termination conditions |
| ADP-002 | Deterministic where practical | Same input → same output unless stochasticity is explicit |
| ADP-003 | Fail-safe | Invalid input produces defined error, not undefined behavior |
| ADP-004 | Provenance-preserving | Algorithms never strip provenance from data |
| ADP-005 | Confidence-aware | Every result carries confidence; algorithms propagate uncertainty |
| ADP-006 | Resource-bounded | Every algorithm respects compute and memory budgets |
| ADP-007 | Incremental | Algorithms prefer incremental updates over full recomputation |
| ADP-008 | Reversible where possible | State mutations should be undoable where practical |
| ADP-009 | Policy-respecting | No algorithm bypasses the policy gate |
| ADP-010 | Attribution-aware | Learning algorithms attribute errors to specific subsystems |
| ADP-011 | Stability-guarded | No single observation may catastrophically destabilize state |
| ADP-012 | Evidence-weighted | Decisions weight evidence by strength, quality, and recency |
| ADP-013 | Contradiction-tolerant | Conflicting information is preserved, not silently dropped |
| ADP-014 | Sparse by default | Neural computation uses sparse activation |
| ADP-015 | Graceful degradation | Disabled subsystems produce defined defaults |

---

## 2. Computational Model

### 2.1 Execution Model

```
CORTEX Computational Model:
  - Single-threaded cognitive pipeline (synchronous)
  - Bounded iteration (all loops have explicit bounds)
  - State-machine driven (explicit state transitions)
  - Prediction-error-driven learning
  - Evidence-weighted decision making
  - Policy-gated actions
```

### 2.2 Computation Paradigms

| Paradigm | Usage |
|---|---|
| Pipeline processing | Main cognitive loop: sequential stages |
| State machine | Runtime lifecycle, cell states |
| Hypothesis evaluation | Reasoning, verification |
| Constraint satisfaction | Planning, resource management |
| Gradient-free adaptation | Plasticity (bounded local updates) |
| Priority-based selection | Memory retrieval, replay |
| Evidence accumulation | Verification, confidence |
| Tree search (bounded) | Planning |
| Sparse activation | Neural processing |

### 2.3 Mathematical Foundations

| Operation | Formula/Method |
|---|---|
| Prediction error | Euclidean distance: `E = √(Σ(pᵢ - aᵢ)²)` |
| Confidence aggregation | Weighted average with evidence weighting |
| Sparsity enforcement | Top-k selection by activation |
| Plasticity update | `ΔW = η × A × C × E × V` |
| Risk estimation | Weighted sum of risk factors |
| Relevance scoring | Multi-factor weighted score |
| Verification | Evidence threshold + consistency check |
| Forgetting | Multi-factor decay scoring |

---

## 3. Execution Semantics

### 3.1 Synchronous Cognitive Loop

The main cognitive pipeline executes **synchronously** in a single thread:

```
ALGORITHM: CognitiveLoop
INPUT: Input
OUTPUT: Response
BOUNDS: max_reasoning_steps, max_planning_depth, max_generation_length

1. observation ← Observe(input)
2. context ← GetContext(working_memory)
3. language_state ← LanguageEncode(observation, context)
4. representation ← NeuralProcess(language_state, context)
5. query ← BuildMemoryQuery(representation)
6. memories ← MemoryRetrieve(query, context)
7. world_state ← WorldIntegrate(representation, memories)
8. reasoning_result ← ReasonEvaluate(representation, memories, world_state)
9. plan ← PlanningEvaluate(reasoning_result, world_state)
10. verified ← VerificationEvaluate(reasoning_result)
11. response ← LanguageGenerate(verified)
12. experience ← ConstructExperience(observation, response, world_state, reasoning_result)
13. learning_signal ← LearningRecord(experience)
14. ApplyLearning(learning_signal)
15. MaybeCheckpoint()
16. RETURN response
```

### 3.2 Execution Guarantees

| Guarantee | Description |
|---|---|
| Sequential execution | Steps 1-16 execute in order; no parallelism within pipeline |
| Bounded execution | Each step has explicit resource bounds |
| Error propagation | Any step failure propagates to runtime for handling |
| State consistency | State is consistent between steps; no partial mutation visible |
| Atomic persistence | State is persisted atomically after pipeline completion |

### 3.3 Early Termination Conditions

| Condition | Action |
|---|---|
| Fatal error in any step | Abort pipeline; enter FAULT state |
| Budget exhaustion | Return bounded result with uncertainty |
| Policy denial | Skip denied operation; continue pipeline |
| Disabled subsystem | Skip step; use defined default |
| Resource exhaustion | Degrade gracefully; return bounded result |

---

## 4. Main Cognitive Pipeline

### 4.1 Pipeline Algorithm

```
ALGORITHM: MainCognitivePipeline
COMPLEXITY: O(n) where n = input token count (dominant factor)
BOUNDS: context_window, max_reasoning_steps, max_planning_depth,
        max_generation_length, memory budgets

PROCEDURE Process(input: Input) -> Response:

    // === PHASE 1: PERCEPTION ===
    observation ← ParseObservation(input)
    IF observation.invalid THEN
        RETURN Error(InputError, "Invalid observation")
    END IF

    // === PHASE 2: ENCODING ===
    context ← working_memory.get_context()
    IF language.enabled THEN
        language_state ← language.encode(observation.text, context)
    ELSE
        language_state ← LanguageState::raw(observation)
    END IF

    // === PHASE 3: NEURAL REPRESENTATION ===
    IF neural.enabled THEN
        representation ← neural.process(language_state, context)
    ELSE
        representation ← NeuralRepresentation::from_language(language_state)
    END IF

    // === PHASE 4: MEMORY ===
    query ← MemoryQuery::from_representation(representation)
    memories ← memory.retrieve(query, context)

    // === PHASE 5: WORLD MODEL ===
    IF world.enabled THEN
        world_state ← world.integrate(representation, memories)
    ELSE
        world_state ← WorldState::empty()
    END IF

    // === PHASE 6: REASONING ===
    IF reasoning.enabled THEN
        reasoning_result ← reasoning.evaluate(
            representation, memories, world_state
        )
    ELSE
        reasoning_result ← ReasoningResult::from_memory(memories)
    END IF

    // === PHASE 7: PLANNING ===
    IF planning.enabled THEN
        plan ← planning.evaluate(reasoning_result, world_state)
    ELSE
        plan ← None
    END IF

    // === PHASE 8: VERIFICATION ===
    IF verification.enabled THEN
        verified ← verification.evaluate(reasoning_result)
    ELSE
        verified ← VerifiedResult::provisional(reasoning_result)
    END IF

    // === PHASE 9: GENERATION ===
    response ← language.generate(verified)

    // === PHASE 10: LEARNING ===
    IF learning.enabled THEN
        experience ← Experience::new(
            observation, response, world_state, reasoning_result
        )
        signal ← learning.record(experience)
        learning.apply_signal(signal, policy.state())
    END IF

    // === PHASE 11: PERSISTENCE ===
    persistence.maybe_checkpoint(state, config.checkpoint_interval)

    RETURN response
END PROCEDURE
```

### 4.2 Pipeline Timing

| Phase | Operation | Complexity |
|---|---|---|
| Perception | Parse input | O(n) |
| Encoding | Language encode | O(n × V) where V = vocab lookup |
| Neural | Process representation | O(C × D) where C = cells, D = dimension |
| Memory | Retrieve | O(M × R) where M = memory size, R = relevance computation |
| World | Integrate | O(E + R) where E = entities, R = relations |
| Reasoning | Evaluate | O(H × S) where H = hypotheses, S = max_steps |
| Planning | Evaluate | O(B × D) where B = branches, D = depth |
| Verification | Evaluate | O(E) where E = evidence items |
| Generation | Generate | O(G × V) where G = generation_limit |
| Learning | Record + apply | O(1) per signal |
| Persistence | Checkpoint | O(S) where S = state size (async) |

---

## 5. Input Processing

### 5.1 Observation Parsing Algorithm

```
ALGORITHM: ParseObservation
INPUT: Raw input (text or structured)
OUTPUT: Observation
BOUNDS: context_window tokens

PROCEDURE ParseObservation(input: Input) -> Observation:
    // 1. Type detection
    IF input.is_text() THEN
        text ← input.as_text()
        kind ← UserInput
    ELSE IF input.is_json() THEN
        parsed ← parse_json(input)
        text ← parsed.get("observation", "")
        kind ← parsed.get("kind", UserInput)
    ELSE
        RETURN Error(InputError, "Unrecognized input format")
    END IF

    // 2. Length validation
    IF length(text) > MAX_INPUT_LENGTH THEN
        RETURN Error(InputError, "Input exceeds maximum length")
    END IF

    // 3. Token count validation
    token_count ← estimate_tokens(text)
    IF token_count > config.language.context_window THEN
        text ← truncate_to_tokens(text, config.language.context_window)
    END IF

    // 4. Construct observation
    observation ← Observation {
        text: text,
        source: Provenance::user_provided(),
        timestamp: Timestamp::now(),
        context: ContextState::initial(),
        kind: kind,
        importance: 0.5,
    }

    RETURN observation
END PROCEDURE
```

### 5.2 Input Validation Rules

| Rule | Check | Action on Failure |
|---|---|---|
| Non-empty | `text.len() > 0` | Reject with InputError |
| Max length | `text.len() ≤ MAX_INPUT_LENGTH` | Truncate |
| Token bound | `tokens ≤ context_window` | Truncate |
| Valid UTF-8 | `text.is_valid_utf8()` | Reject with EncodingError |
| No control chars | No null bytes | Strip |

---

## 6. Context Construction

### 6.1 Context Assembly Algorithm

```
ALGORITHM: ConstructContext
INPUT: WorkingMemory, current observation
OUTPUT: ContextState
BOUNDS: context_window tokens

PROCEDURE ConstructContext(working: &WorkingMemory, obs: &Observation) -> ContextState:
    context ← ContextState::initial()

    // 1. Session context
    context.conversation_id ← working.conversation_context.session_id
    context.window_position ← working.conversation_context.turn_count

    // 2. Episode context (recent relevant episodes)
    recent_episodes ← working.episodic_memory.last_n(10)
    context.episode_context ← recent_episodes.map(|e| e.id)

    // 3. Active concepts
    context.active_concepts ← working.active_concepts.clone()

    // 4. World assumptions
    context.world_assumptions ← working.world_assumptions.clone()

    // 5. Temporal context
    context.temporal_context ← TemporalContext {
        current_time: Timestamp::now(),
        sequence_position: working.conversation_context.turn_count,
        prior_states: working.recent_timestamps(),
        temporal_horizon: Duration::from_secs(3600),
    }

    // 6. Active intents
    context.active_intents ← working.active_hypotheses
        .iter()
        .filter(|h| h.is_intent())
        .map(|h| IntentHypothesis::from(h))
        .collect()

    // 7. Token tracking
    context.tokens_used ← estimate_tokens(obs.text)

    RETURN context
END PROCEDURE
```

---

## 7. Observation Processing

### 7.1 Observation Integration Algorithm

```
ALGORITHM: ProcessObservation
INPUT: Observation, ContextState
OUTPUT: Updated internal state
BOUNDS: Memory budgets

PROCEDURE ProcessObservation(obs: Observation, context: ContextState):
    // 1. Policy check
    decision ← policy.evaluate(ProposedOperation::observation())
    IF decision == Denied THEN
        RETURN Error(PolicyError, decision.reason)
    END IF

    // 2. Create episode
    episode ← Episode {
        id: next_episode_id(),
        observation: obs,
        context: context,
        action: None,
        outcome: None,
        timestamp: obs.timestamp,
        prediction: None,
        prediction_error: PredictionError::zero(),
        confidence: ConfidenceState::from_observation(&obs),
        source: obs.source,
        importance: obs.importance,
    }

    // 3. Store in episodic memory
    memory.episodic.store(episode)

    // 4. Update working memory
    working_memory.input ← Some(obs.as_current_input())
    working_memory.advance_time()

    // 5. Update provenance
    provenance.track(obs.source)

END PROCEDURE
```

---

## 8. Language Processing

### 8.1 Language Encoding Algorithm

```
ALGORITHM: LanguageEncode
INPUT: text, ContextState
OUTPUT: LanguageState
BOUNDS: vocabulary_capacity, context_window

PROCEDURE LanguageEncode(text: &str, context: &ContextState) -> LanguageState:
    // 1. Normalization
    normalized ← normalize(text)
    // - Lowercase (configurable)
    // - Unicode normalization (NFC)
    // - Whitespace normalization
    // - Punctuation preservation

    // 2. Segmentation
    segments ← segment(normalized)
    // - Split on whitespace
    // - Handle punctuation
    // - Identify subword boundaries

    // 3. Symbol encoding
    symbols ← Vec::new()
    FOR each segment IN segments:
        symbol_id ← vocabulary.lookup_or_create(segment)
        symbols.push(Symbol {
            id: symbol_id,
            kind: classify_symbol(segment),
            frequency: vocabulary.frequency(symbol_id),
            activation: 1.0,
            confidence: vocabulary.confidence(symbol_id),
            associations: vocabulary.associations(symbol_id),
        })
    END FOR

    // 4. Token sequence
    tokens ← symbols_to_tokens(symbols)

    // 5. Lexical state
    lexical ← resolve_lexical(tokens, vocabulary)

    // 6. Syntax analysis
    syntax ← parse_syntax(lexical)
    // - Dependency parsing
    // - Role assignment (AGENT, OBJECT, RECIPIENT, etc.)
    // - Structural context
    // - Nesting detection

    // 7. Semantic analysis
    semantics ← extract_semantics(syntax, lexical)
    // - Concept extraction
    // - Relation extraction
    // - Entity recognition
    // - Property extraction

    // 8. Context integration
    integrated_context ← integrate_context(context, semantics)

    // 9. Intent detection
    intent ← detect_intent(semantics, integrated_context)

    // 10. Confidence computation
    confidence ← ConfidenceState {
        belief: compute_encoding_confidence(semantics),
        evidence_strength: 0.5,
        source_quality: 0.7,
        consistency: compute_consistency(semantics),
        uncertainty: 1.0 - compute_consistency(semantics),
        prediction_reliability: 0.0,
        verification_status: VerificationStatus::Observed,
    }

    RETURN LanguageState {
        symbols, tokens,
        concepts: semantics.concepts,
        entities: semantics.entities,
        relations: semantics.relations,
        syntax, semantics: semantics.graph,
        context: integrated_context,
        intent, confidence,
    }
END PROCEDURE
```

### 8.2 Vocabulary Lookup Algorithm

```
ALGORITHM: VocabularyLookupOrCreate
INPUT: token string
OUTPUT: SymbolId
BOUNDS: vocabulary_capacity

PROCEDURE VocabLookupOrCreate(token: &str) -> SymbolId:
    // 1. Exact match
    IF token_to_id.contains(token) THEN
        id ← token_to_id[token]
        frequency_tracker.increment(id)
        RETURN id
    END IF

    // 2. Capacity check
    IF next_id >= vocabulary_capacity THEN
        // Capacity reached: return unknown token
        RETURN SymbolId(0)  // reserved unknown
    END IF

    // 3. Create new entry
    id ← next_id
    next_id ← next_id + 1
    token_to_id[token] ← id
    id_to_token[id] ← token
    symbols[id] ← Symbol::new(id, SymbolKind::Word)
    frequency_tracker.init(id, 1)

    // 4. Learning signal: new symbol discovered
    IF learning.enabled THEN
        emit_learning_signal(LearningSignal::new_symbol(id, token))
    END IF

    RETURN id
END PROCEDURE
```

### 8.3 Intent Detection Algorithm

```
ALGORITHM: DetectIntent
INPUT: SemanticGraph, ContextState
OUTPUT: IntentHypothesis
BOUNDS: max 5 hypotheses

PROCEDURE DetectIntent(semantics: &SemanticGraph, context: &ContextState) -> IntentHypothesis:
    hypotheses ← Vec::new()

    // 1. Syntactic cues
    IF semantics.has_interrogative() THEN
        hypotheses.push(IntentHypothesis {
            intent: Intent::Question,
            confidence: 0.8,
        })
    END IF

    IF semantics.has_imperative() THEN
        hypotheses.push(IntentHypothesis {
            intent: Intent::Instruction,
            confidence: 0.7,
        })
    END IF

    // 2. Semantic cues
    IF semantics.has_assertion() THEN
        hypotheses.push(IntentHypothesis {
            intent: Intent::Statement,
            confidence: 0.6,
        })
    END IF

    IF semantics.has_correction_marker() THEN
        hypotheses.push(IntentHypothesis {
            intent: Intent::Correction,
            confidence: 0.7,
        })
    END IF

    // 3. Context cues
    IF context.expects_response() THEN
        hypotheses.push(IntentHypothesis {
            intent: Intent::Conversation,
            confidence: 0.5,
        })
    END IF

    // 4. Rank by confidence
    hypotheses.sort_by_confidence_desc()

    // 5. Return top hypothesis with alternatives
    primary ← hypotheses[0]
    primary.alternatives ← hypotheses[1..5]

    RETURN primary
END PROCEDURE
```

### 8.4 Language Generation Algorithm

```
ALGORITHM: LanguageGenerate
INPUT: VerifiedResult
OUTPUT: GeneratedResponse
BOUNDS: generation_limit tokens

PROCEDURE LanguageGenerate(verified: &VerifiedResult) -> GeneratedResponse:
    // 1. Response planning
    intent ← determine_response_intent(verified)
    // - If question: provide answer
    // - If instruction: confirm/action
    // - If statement: acknowledge/elaborate
    // - If correction: update understanding

    // 2. Meaning representation
    meaning ← construct_meaning(verified, intent)

    // 3. Response structure
    structure ← plan_structure(meaning)
    // - Determine information ordering
    // - Determine emphasis
    // - Determine level of detail

    // 4. Candidate expressions
    candidates ← generate_candidates(structure, vocabulary)

    // 5. Semantic validation
    FOR each candidate IN candidates:
        IF NOT validate_semantics(candidate, meaning) THEN
            candidates.remove(candidate)
        END IF
    END FOR

    // 6. Syntax realization
    syntax ← realize_syntax(best_candidate)

    // 7. Token selection (bounded by generation_limit)
    tokens ← select_tokens(syntax, generation_limit)

    // 8. Text output
    text ← tokens_to_text(tokens)

    RETURN GeneratedResponse {
        text,
        confidence: verified.confidence,
        verification_status: verified.verification_status,
    }
END PROCEDURE
```

### 8.5 Language Prediction Algorithm

```
ALGORITHM: LanguagePredict
INPUT: LanguageState
OUTPUT: Vec<CandidateContinuation>
BOUNDS: max_candidates = 10

PROCEDURE LanguagePredict(state: &LanguageState) -> Vec<CandidateContinuation>:
    candidates ← Vec::new()

    // For each candidate continuation
    FOR each possible_next IN vocabulary.top_candidates(state.context):
        // Compute multi-factor score
        score ← 0.0
        score ← score + language_score(possible_next, state)
        score ← score + context_score(possible_next, state.context)
        score ← score + semantic_score(possible_next, state.semantics)
        score ← score + memory_score(possible_next, state)
        score ← score + world_score(possible_next, state)
        score ← score + verification_score(possible_next, state)
        score ← score - contradiction_penalty(possible_next, state)
        score ← score - risk_penalty(possible_next, state)

        candidates.push(CandidateContinuation {
            token: possible_next,
            score: score,
        })
    END FOR

    // Sort by score descending
    candidates.sort_by_score_desc()

    // Return top candidates
    RETURN candidates[..10]
END PROCEDURE
```

---

## 9. Neural Processing

### 9.1 Neural Processing Algorithm

```
ALGORITHM: NeuralProcess
INPUT: LanguageState, ContextState
OUTPUT: NeuralRepresentation
BOUNDS: cells × columns, sparsity_ratio

PROCEDURE NeuralProcess(input: &LanguageState, context: &ContextState) -> NeuralRepresentation:
    // 1. Input encoding → initial cell activation
    initial_activation ← encode_input(input)
    // Map language state to cell activation pattern

    // 2. Column processing
    column_activations ← Vec::new()
    FOR each column IN fields.columns:
        // Activate cells in column
        FOR each cell IN column.cells:
            cell.receive(initial_activation[cell.id])
            cell.activate(threshold)
        END FOR

        // Competition: select top-k cells
        active ← column.compete(sparsity_ratio)
        column_activations.push(active)
    END FOR

    // 3. Field-level integration
    field_state ← integrate_fields(column_activations, context)

    // 4. Enforce global sparsity
    max_active ← field_size * sparsity_ratio
    field_state.enforce_sparsity(max_active)

    // 5. Temporal encoding
    temporal ← temporal_buffer.encode(field_state)
    // Encode sequence, transition, recurrence

    // 6. Prediction generation
    prediction ← predict_from_state(field_state, temporal)

    // 7. Confidence computation
    confidence ← ConfidenceState {
        belief: field_state.average_activation,
        evidence_strength: 0.5,
        source_quality: 0.5,
        consistency: field_state.coherence,
        uncertainty: 1.0 - field_state.coherence,
        prediction_reliability: 0.0,
        verification_status: VerificationStatus::Inferred,
    }

    RETURN NeuralRepresentation {
        active_cells: field_state.active_cells,
        active_columns: field_state.active_columns,
        field_activations: field_state.field_activations,
        temporal_encoding: temporal,
        prediction,
        confidence,
    }
END PROCEDURE
```

### 9.2 Cell Competition Algorithm

```
ALGORITHM: ColumnCompete
INPUT: Column cells, sparsity_ratio
OUTPUT: Set of active CellIds
BOUNDS: sparsity_ratio × column_size

PROCEDURE ColumnCompete(column: &Column, sparsity: Scalar) -> HashSet<CellId>:
    // 1. Compute activation for all cells
    activations ← column.cells
        .iter()
        .map(|cell| (cell.id, cell.activation))
        .collect()

    // 2. Sort by activation descending
    activations.sort_by(|a, b| b.1.partial_cmp(&a.1))

    // 3. Select top-k
    max_active ← (column.cells.len() as Scalar * sparsity).ceil() as usize
    max_active ← max_active.max(1)  // At least 1 active cell

    active ← activations[..max_active]
        .iter()
        .map(|(id, _)| *id)
        .collect()

    // 4. Inhibit non-selected cells
    FOR each cell IN column.cells:
        IF NOT active.contains(cell.id) THEN
            cell.inhibit()
        END IF
    END FOR

    RETURN active
END PROCEDURE
```

### 9.3 Temporal Encoding Algorithm

```
ALGORITHM: TemporalEncode
INPUT: Current field state, temporal buffer history
OUTPUT: TemporalEncoding
BOUNDS: temporal_buffer_size

PROCEDURE TemporalEncode(current: &FieldState, buffer: &TemporalBuffer) -> TemporalEncoding:
    // 1. Get temporal context
    history ← buffer.last_n(3)  // X(t-2), X(t-1), X(t)

    // 2. Compute sequence encoding
    sequence ← encode_sequence(history)

    // 3. Compute transition encoding
    transition ← encode_transition(history[-2], history[-1])

    // 4. Compute recurrence detection
    recurrence ← detect_recurrence(history)

    // 5. Compute temporal dependency
    dependency ← compute_temporal_dependency(history)

    // 6. Encode event order
    event_order ← encode_event_order(history)

    RETURN TemporalEncoding {
        sequence,
        transition,
        recurrence,
        dependency,
        event_order,
        timestamp: Timestamp::now(),
    }
END PROCEDURE
```

### 9.4 Neural Prediction Algorithm

```
ALGORITHM: NeuralPredict
INPUT: NeuralState
OUTPUT: Prediction
BOUNDS: prediction dimension

PROCEDURE NeuralPredict(state: &NeuralState) -> Prediction:
    // 1. Extract current active representation
    active_repr ← state.active_representation()

    // 2. Get temporal context
    temporal ← state.temporal_buffer.current()

    // 3. Predict next state
    // Based on: current activation + temporal pattern + learned transitions
    predicted_state ← Vec::new()
    FOR each cell_id IN state.active_cells:
        cell ← state.get_cell(cell_id)
        // Predict based on cell's prediction vector and context
        predicted ← cell.predict(temporal)
        predicted_state.push(predicted)
    END FOR

    // 4. Compute prediction confidence
    confidence ← compute_prediction_confidence(state, predicted_state)

    RETURN Prediction {
        target: PredictionTarget::NextState,
        predicted_state,
        confidence,
        timestamp: Timestamp::now(),
        context: state.context(),
        resolved: false,
        actual: None,
        error: None,
    }
END PROCEDURE
```

### 9.5 Prediction Error Computation

```
ALGORITHM: ComputePredictionError
INPUT: Prediction, actual Observation
OUTPUT: PredictionError
BOUNDS: None (pure computation)

PROCEDURE ComputePredictionError(predicted: &Prediction, actual: &Observation) -> PredictionError:
    // 1. Encode actual observation to comparable representation
    actual_repr ← encode_observation(actual)

    // 2. Compute per-dimension error
    dimensions ← HashMap::new()
    sum_sq ← 0.0
    FOR i IN 0..predicted.predicted_state.len():
        diff ← predicted.predicted_state[i] - actual_repr[i]
        sq ← diff * diff
        sum_sq ← sum_sq + sq
        dimensions[format!("dim_{}", i)] ← sq
    END FOR

    // 3. Compute overall magnitude (Euclidean distance)
    magnitude ← sqrt(sum_sq)

    RETURN PredictionError {
        magnitude,
        dimensions,
        timestamp: Timestamp::now(),
        prediction_id: Some(predicted.id),
    }
END PROCEDURE
```

---

## 10. Memory Retrieval

### 10.1 Memory Retrieval Algorithm

```
ALGORITHM: MemoryRetrieve
INPUT: MemoryQuery, ContextState
OUTPUT: MemoryRetrieval
BOUNDS: max_results, memory budgets

PROCEDURE MemoryRetrieve(query: &MemoryQuery, context: &ContextState) -> MemoryRetrieval:
    retrieval ← MemoryRetrieval::default()

    // 1. Episodic retrieval
    IF query.query_type IN [Episodic, All] THEN
        candidates ← episodic_memory.candidates(query)
        scored ← score_relevance(candidates, context)
        filtered ← filter_by_confidence(scored, query.min_confidence)
        ranked ← rank_by_relevance(filtered)
        retrieval.episodic ← ranked[..query.max_results]
    END IF

    // 2. Semantic retrieval
    IF query.query_type IN [Semantic, All] THEN
        candidates ← semantic_memory.candidates(query)
        scored ← score_relevance(candidates, context)
        filtered ← filter_by_confidence(scored, query.min_confidence)
        ranked ← rank_by_relevance(filtered)
        retrieval.semantic ← ranked[..query.max_results]
    END IF

    // 3. Procedural retrieval
    IF query.query_type IN [Procedural, All] THEN
        candidates ← procedural_memory.candidates(query)
        scored ← score_relevance(candidates, context)
        retrieval.procedural ← scored[..query.max_results]
    END IF

    // 4. Associative retrieval
    IF query.query_type IN [Associative, All] THEN
        candidates ← associative_memory.candidates(query)
        scored ← score_relevance(candidates, context)
        retrieval.associative ← scored[..query.max_results]
    END IF

    // 5. Compute relevance scores for all retrieved items
    retrieval.relevance_scores ← compute_all_relevance(retrieval)

    // 6. Contradiction detection
    contradictions ← detect_contradictions(retrieval)
    IF NOT contradictions.is_empty() THEN
        // Mark contradictions but don't remove items
        FOR each contradiction IN contradictions:
            retrieval.mark_contradiction(contradiction)
        END FOR
    END IF

    RETURN retrieval
END PROCEDURE
```

### 10.2 Relevance Scoring Algorithm

```
ALGORITHM: ScoreRelevance
INPUT: Candidate item, ContextState
OUTPUT: Relevance score [0.0, 1.0]

PROCEDURE ScoreRelevance(item: &MemoryItem, context: &ContextState) -> Scalar:
    score ← 0.0

    // Factor 1: Semantic relevance (concept overlap)
    semantic_rel ← compute_semantic_overlap(item.concepts, context.active_concepts)
    score ← score + semantic_rel * 0.30

    // Factor 2: Context relevance (context match)
    context_rel ← compute_context_match(item.context, context)
    score ← score + context_rel * 0.20

    // Factor 3: Temporal relevance (recency)
    temporal_rel ← compute_temporal_relevance(item.timestamp, context.current_time)
    score ← score + temporal_rel * 0.15

    // Factor 4: Association strength
    assoc_strength ← compute_association_strength(item, context)
    score ← score + assoc_strength * 0.15

    // Factor 5: Importance
    score ← score + item.importance * 0.10

    // Factor 6: Confidence
    score ← score + item.confidence.overall() * 0.10

    RETURN score.clamp(0.0, 1.0)
END PROCEDURE
```

### 10.3 Temporal Relevance Computation

```
ALGORITHM: ComputeTemporalRelevance
INPUT: item timestamp, current time
OUTPUT: Relevance [0.0, 1.0]

PROCEDURE ComputeTemporalRelevance(item_time: Timestamp, now: Timestamp) -> Scalar:
    age_ms ← now.0 - item_time.0
    age_hours ← age_ms / 3_600_000

    // Exponential decay with half-life of 24 hours
    half_life ← 24.0  // hours
    decay ← exp(-0.693 * age_hours / half_life)

    // Minimum relevance floor
    RETURN decay.max(0.05)
END PROCEDURE
```

---

## 11. Memory Update

### 11.1 Episode Storage Algorithm

```
ALGORITHM: StoreEpisode
INPUT: Episode
OUTPUT: Updated EpisodicMemory
BOUNDS: episodic_mb

PROCEDURE StoreEpisode(episode: Episode):
    // 1. Capacity check
    IF episodic_memory.current_usage >= episodic_memory.capacity_bytes THEN
        // Evict before storing
        evict_lowest_value()
    END IF

    // 2. Store episode
    episodic_memory.episodes.push(episode)
    episodic_memory.current_usage += estimate_size(episode)

    // 3. Update index
    episodic_memory.index.add(episode.id, episode.timestamp)

END PROCEDURE
```

### 11.2 Memory Eviction Algorithm

```
ALGORITHM: EvictLowestValue
INPUT: EpisodicMemory
OUTPUT: Evicted episode
BOUNDS: None

PROCEDURE EvictLowestValue():
    // Compute value score for each episode
    scores ← episodic_memory.episodes
        .iter()
        .map(|e| (e.id, compute_episode_value(e)))
        .collect()

    // Sort by value ascending (lowest first)
    scores.sort_by_value_asc()

    // Evict lowest value episode
    evict_id ← scores[0].id
    episodic_memory.remove(evict_id)

END PROCEDURE

PROCEDURE ComputeEpisodeValue(episode: &Episode) -> Scalar:
    // Multi-factor value scoring
    value ← 0.0
    value += episode.importance * 0.30
    value += episode.confidence.overall() * 0.20
    value += (1.0 / (1.0 + age_hours(episode))) * 0.20
    value += min(episode.retrieval_count, 10) / 10.0 * 0.15
    value += if episode.consolidated { 0.15 } else { 0.0 }
    RETURN value
END PROCEDURE
```

---

## 12. World Model Update

### 12.1 World Integration Algorithm

```
ALGORITHM: WorldIntegrate
INPUT: NeuralRepresentation, MemoryRetrieval
OUTPUT: WorldState
BOUNDS: Entity count, relation count

PROCEDURE WorldIntegrate(repr: &NeuralRepresentation, memories: &MemoryRetrieval) -> WorldState:
    // 1. Extract entities from representation
    new_entities ← extract_entities(repr)

    // 2. Extract relations from representation
    new_relations ← extract_relations(repr)

    // 3. Integrate with existing world state
    FOR each entity IN new_entities:
        IF world.has_entity(entity.identity) THEN
            // Update existing entity
            existing ← world.get_entity(entity.identity)
            existing.update(entity)
        ELSE
            // Create new entity
            world.add_entity(entity)
        END IF
    END FOR

    // 4. Integrate relations
    FOR each relation IN new_relations:
        IF NOT world.has_relation(relation) THEN
            world.add_relation(relation)
        ELSE
            world.update_relation(relation)
        END IF
    END FOR

    // 5. Integrate memory context
    FOR each memory IN memories.semantic:
        world.integrate_knowledge(memory)
    END FOR

    // 6. Update uncertainty
    world.uncertainty ← update_uncertainty(world, repr.confidence)

    // 7. Return current state snapshot
    RETURN world.current_state()
END PROCEDURE
```

### 12.2 State Transition Prediction Algorithm

```
ALGORITHM: PredictTransition
INPUT: WorldState, Action
OUTPUT: PredictedState
BOUNDS: prediction_horizon

PROCEDURE PredictTransition(state: &WorldState, action: &Action) -> PredictedState:
    // 1. Identify affected entities
    affected ← identify_affected_entities(state, action)

    // 2. Predict state changes for each affected entity
    predicted_entities ← Vec::new()
    FOR each entity IN affected:
        predicted ← predict_entity_change(entity, action)
        predicted_entities.push(predicted)
    END FOR

    // 3. Predict relation changes
    predicted_relations ← predict_relation_changes(state, action)

    // 4. Compute prediction confidence
    confidence ← compute_transition_confidence(state, action, affected)

    // 5. Compute uncertainty
    uncertainty ← 1.0 - confidence

    RETURN PredictedState {
        predicted_entities,
        predicted_relations,
        confidence,
        uncertainty,
        prediction_horizon: 1,
    }
END PROCEDURE
```

---

## 13. State Estimation

### 13.1 State Estimation Algorithm

```
ALGORITHM: EstimateState
INPUT: Multiple observations, prior state
OUTPUT: Estimated state with uncertainty
BOUNDS: None (pure computation)

PROCEDURE EstimateState(observations: &[Observation], prior: &WorldState) -> WorldState:
    // 1. Start with prior
    estimated ← prior.clone()

    // 2. Integrate each observation
    FOR each obs IN observations:
        // Extract state information
        state_info ← extract_state(obs)

        // Update estimate with evidence weighting
        FOR each entity_state IN state_info:
            IF estimated.has_entity(entity_state.id) THEN
                existing ← estimated.get_entity(entity_state.id)
                // Weighted update based on confidence
                weight ← obs.confidence.overall()
                existing.state ← weighted_merge(existing.state, entity_state, weight)
            END IF
        END FOR
    END FOR

    // 3. Update uncertainty based on observation count and agreement
    agreement ← compute_observation_agreement(observations)
    estimated.uncertainty.level ← 1.0 - agreement

    RETURN estimated
END PROCEDURE
```

---

## 14. Inference

### 14.1 Inference Algorithm

```
ALGORITHM: Infer
INPUT: NeuralRepresentation, MemoryRetrieval, WorldState
OUTPUT: Inference result
BOUNDS: max_inference_steps

PROCEDURE Infer(repr: &NeuralRepresentation, memories: &MemoryRetrieval, world: &WorldState) -> InferenceResult:
    // 1. Identify what needs to be inferred
    gaps ← identify_knowledge_gaps(repr, memories)

    // 2. For each gap, attempt inference
    inferences ← Vec::new()
    FOR each gap IN gaps:
        // Try to infer from existing knowledge
        inference ← attempt_inference(gap, memories, world)
        IF inference.is_some() THEN
            inferences.push(inference.unwrap())
        END IF
    END FOR

    // 3. Validate inferences
    validated ← Vec::new()
    FOR each inf IN inferences:
        IF validate_inference(inf, memories, world) THEN
            validated.push(inf)
        END IF
    END FOR

    RETURN InferenceResult {
        inferences: validated,
        confidence: compute_inference_confidence(validated),
    }
END PROCEDURE
```

---

## 15. Reasoning

### 15.1 Reasoning Evaluation Algorithm

```
ALGORITHM: ReasoningEvaluate
INPUT: NeuralRepresentation, MemoryRetrieval, WorldState
OUTPUT: ReasoningResult
BOUNDS: max_steps

PROCEDURE ReasoningEvaluate(
    repr: &NeuralRepresentation,
    memories: &MemoryRetrieval,
    world: &WorldState
) -> ReasoningResult:
    // 1. Problem representation
    problem ← represent_problem(repr)

    // 2. Hypothesis generation (bounded)
    hypotheses ← generate_hypotheses(problem, memories, world)

    // 3. Evidence evaluation (bounded by max_steps)
    budget ← config.reasoning.max_steps
    evaluated ← Vec::new()

    FOR each hypothesis IN hypotheses:
        IF budget <= 0 THEN BREAK
        evaluation ← evaluate_hypothesis(hypothesis, memories)
        evaluated.push(evaluation)
        budget ← budget - 1
    END FOR

    // 4. Contradiction detection
    contradictions ← detect_contradictions(evaluated)

    // 5. Hypothesis ranking
    ranked ← rank_hypotheses(evaluated, contradictions)

    // 6. Conclusion (if budget allows)
    conclusion ← None
    IF budget > 0 AND NOT ranked.is_empty() THEN
        conclusion ← Some(Conclusion {
            hypothesis_id: ranked[0].id,
            proposition: ranked[0].proposition,
            confidence: ranked[0].confidence,
            evidence_strength: ranked[0].evidence.total_strength(),
            reasoning_steps: config.reasoning.max_steps - budget,
            bounded: budget <= 0,
        })
    END IF

    RETURN ReasoningResult {
        hypotheses: ranked,
        contradictions,
        budget_remaining: budget,
        conclusion,
    }
END PROCEDURE
```

### 15.2 Hypothesis Generation Algorithm

```
ALGORITHM: GenerateHypotheses
INPUT: ProblemRepresentation, MemoryRetrieval, WorldState
OUTPUT: Vec<Hypothesis>
BOUNDS: max_hypotheses = 10

PROCEDURE GenerateHypotheses(
    problem: &ProblemRepresentation,
    memories: &MemoryRetrieval,
    world: &WorldState
) -> Vec<Hypothesis>:
    hypotheses ← Vec::new()
    next_id ← HypothesisId(1)

    // 1. Direct memory-based hypotheses
    FOR each knowledge IN memories.semantic:
        IF knowledge.relevant_to(problem) THEN
            hypotheses.push(Hypothesis {
                id: next_id,
                proposition: Proposition::from_knowledge(knowledge),
                evidence: knowledge.evidence,
                counter_evidence: EvidenceSet::new(),
                confidence: knowledge.confidence.overall(),
                dependencies: Vec::new(),
                contradictions: Vec::new(),
                provenance: knowledge.provenance,
                reasoning_type: ReasoningType::Inductive,
            })
            next_id ← next_id.next()
        END IF
    END FOR

    // 2. World-model-based hypotheses
    FOR each entity IN world.entities:
        IF entity.relevant_to(problem) THEN
            hypotheses.push(Hypothesis {
                id: next_id,
                proposition: Proposition::from_entity(entity),
                evidence: entity.provenance_to_evidence(),
                counter_evidence: EvidenceSet::new(),
                confidence: entity.confidence,
                reasoning_type: ReasoningType::Abductive,
            })
            next_id ← next_id.next()
        END IF
    END FOR

    // 3. Analogical hypotheses (from episodic memory)
    FOR each episode IN memories.episodic:
        IF episode.similar_to(problem) THEN
            hypotheses.push(Hypothesis {
                id: next_id,
                proposition: Proposition::from_episode(episode),
                evidence: EvidenceSet::from_episode(episode),
                confidence: episode.confidence.overall() * 0.7,  // Analogical discount
                reasoning_type: ReasoningType::Analogical,
            })
            next_id ← next_id.next()
        END IF
    END FOR

    // 4. Limit to max_hypotheses
    hypotheses.truncate(10)

    RETURN hypotheses
END PROCEDURE
```

### 15.3 Hypothesis Evaluation Algorithm

```
ALGORITHM: EvaluateHypothesis
INPUT: Hypothesis, EvidenceSet (from memories)
OUTPUT: HypothesisEvaluation
BOUNDS: evidence set size

PROCEDURE EvaluateHypothesis(hypothesis: &Hypothesis, memories: &MemoryRetrieval) -> HypothesisEvaluation:
    // 1. Gather supporting evidence
    supporting ← gather_supporting_evidence(hypothesis, memories)

    // 2. Search for counter-evidence
    counter ← search_counter_evidence(hypothesis, memories)

    // 3. Evaluate evidence quality
    evidence_quality ← evaluate_evidence_quality(supporting)

    // 4. Check consistency with existing knowledge
    consistency ← check_consistency(hypothesis, memories)

    // 5. Compute updated confidence
    base_confidence ← hypothesis.confidence
    evidence_boost ← supporting.total_strength() * 0.3
    counter_penalty ← counter.total_strength() * 0.4
    consistency_factor ← consistency * 0.2

    updated_confidence ← (base_confidence + evidence_boost + consistency_factor - counter_penalty)
        .clamp(0.0, 1.0)

    RETURN HypothesisEvaluation {
        hypothesis_id: hypothesis.id,
        supporting_evidence: supporting,
        counter_evidence: counter,
        evidence_quality,
        consistency,
        updated_confidence,
        reasoning_type: hypothesis.reasoning_type,
    }
END PROCEDURE
```

### 15.4 Contradiction Detection Algorithm

```
ALGORITHM: DetectContradictions
INPUT: Vec<HypothesisEvaluation>
OUTPUT: Vec<Contradiction>
BOUNDS: O(n²) where n = number of hypotheses

PROCEDURE DetectContradictions(evaluations: &[HypothesisEvaluation]) -> Vec<Contradiction>:
    contradictions ← Vec::new()

    FOR i IN 0..evaluations.len():
        FOR j IN (i+1)..evaluations.len():
            a ← evaluations[i]
            b ← evaluations[j]

            // Check if propositions contradict
            IF propositions_contradict(a.proposition, b.proposition) THEN
                contradictions.push(Contradiction {
                    claim_a: a.hypothesis_id,
                    claim_b: b.hypothesis_id,
                    description: format_contradiction(a, b),
                    severity: compute_contradiction_severity(a, b),
                    detected_at: Timestamp::now(),
                    resolved: false,
                })
            END IF
        END FOR
    END FOR

    RETURN contradictions
END PROCEDURE
```

### 15.5 Hypothesis Ranking Algorithm

```
ALGORITHM: RankHypotheses
INPUT: Vec<HypothesisEvaluation>, Vec<Contradiction>
OUTPUT: Vec<Hypothesis> (ranked)
BOUNDS: None (sorting)

PROCEDURE RankHypotheses(evaluations: &[HypothesisEvaluation], contradictions: &[Contradiction]) -> Vec<Hypothesis>:
    // 1. Compute final score for each hypothesis
    scored ← evaluations.iter().map(|eval| {
        score ← eval.updated_confidence

        // Penalty for contradictions
        contradiction_count ← contradictions.iter()
            .filter(|c| c.claim_a == eval.hypothesis_id OR c.claim_b == eval.hypothesis_id)
            .count()
        score ← score - (contradiction_count as Scalar * 0.1)

        // Bonus for evidence quality
        score ← score + eval.evidence_quality * 0.1

        (eval.hypothesis_id, score.clamp(0.0, 1.0))
    }).collect()

    // 2. Sort by score descending
    scored.sort_by_score_desc()

    // 3. Convert back to hypotheses with updated scores
    RETURN scored.iter().map(|(id, score)| {
        hypothesis ← get_hypothesis(id)
        hypothesis.confidence ← score
        hypothesis
    }).collect()
END PROCEDURE
```

---

## 16. Prediction

### 16.1 Prediction Algorithm

```
ALGORITHM: Predict
INPUT: Current state (neural, world, context)
OUTPUT: Prediction
BOUNDS: prediction dimension

PROCEDURE Predict(state: &CortexState) -> Prediction:
    // 1. Neural prediction
    neural_pred ← state.neural.predict()

    // 2. World model prediction
    world_pred ← state.world.predict_next()

    // 3. Combine predictions
    combined ← combine_predictions(neural_pred, world_pred)

    // 4. Compute confidence
    confidence ← min(neural_pred.confidence, world_pred.confidence)

    RETURN Prediction {
        target: PredictionTarget::NextState,
        predicted_state: combined,
        confidence,
        timestamp: Timestamp::now(),
        context: state.context(),
        resolved: false,
        actual: None,
        error: None,
    }
END PROCEDURE
```

### 16.2 Prediction Comparison Algorithm

```
ALGORITHM: ComparePrediction
INPUT: Prediction, actual Observation
OUTPUT: PredictionError, resolved Prediction

PROCEDURE ComparePrediction(prediction: &mut Prediction, actual: &Observation) -> PredictionError:
    // 1. Encode actual observation
    actual_repr ← encode_observation(actual)

    // 2. Compute error
    error ← PredictionError::compute(&prediction.predicted_state, &actual_repr)

    // 3. Update prediction
    prediction.resolved ← true
    prediction.actual ← Some(actual_repr)
    prediction.error ← Some(error.clone())

    RETURN error
END PROCEDURE
```

---

## 17. Planning

### 17.1 Planning Evaluation Algorithm

```
ALGORITHM: PlanningEvaluate
INPUT: ReasoningResult, WorldState
OUTPUT: Option<Plan>
BOUNDS: max_depth × max_branches

PROCEDURE PlanningEvaluate(reasoning: &ReasoningResult, world: &WorldState) -> Option<Plan>:
    IF NOT config.planning.enabled THEN
        RETURN None
    END IF

    // 1. Extract goal from reasoning
    goal ← reasoning.primary_goal()
    IF goal.is_none() THEN
        RETURN None
    END IF

    // 2. Generate candidate actions
    candidates ← generate_candidate_actions(goal, world)

    // 3. Simulate and evaluate plans (bounded)
    plans ← Vec::new()
    budget_depth ← config.planning.max_depth
    budget_branches ← config.planning.max_branches

    FOR each candidate IN candidates[..budget_branches]:
        plan ← construct_plan(goal, candidate, world, budget_depth)
        IF plan.is_valid() THEN
            // Simulate plan
            simulation ← simulate_plan(plan, world)

            // Evaluate risk
            risk ← evaluate_risk(plan, world)

            // Evaluate utility
            utility ← evaluate_utility(plan, simulation, goal)

            plan.estimated_cost ← simulation.cost
            plan.estimated_risk ← risk.score
            plan.uncertainty ← simulation.uncertainty
            plan.confidence ← compute_plan_confidence(simulation, risk)

            plans.push(plan)
        END IF
    END FOR

    // 4. Rank plans
    plans.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence))

    // 5. Select best plan (if confidence threshold met)
    IF NOT plans.is_empty() AND plans[0].confidence > 0.3 THEN
        RETURN Some(plans[0])
    ELSE
        RETURN None
    END IF
END PROCEDURE
```

### 17.2 Plan Simulation Algorithm

```
ALGORITHM: SimulatePlan
INPUT: Plan, WorldState
OUTPUT: SimulatedOutcome
BOUNDS: prediction_horizon

PROCEDURE SimulatePlan(plan: &Plan, world: &WorldState) -> SimulatedOutcome:
    current_state ← world.clone()
    outcomes ← Vec::new()
    total_cost ← 0.0
    total_uncertainty ← 0.0

    FOR each step IN plan.steps:
        // Predict transition
        predicted ← world.predict_transition(current_state, step)

        // Accumulate outcomes
        outcomes.push(predicted.to_outcome())

        // Accumulate cost
        total_cost ← total_cost + step.estimated_cost()

        // Accumulate uncertainty
        total_uncertainty ← total_uncertainty + predicted.uncertainty

        // Advance state
        current_state ← predicted.to_world_state()
    END FOR

    // Average uncertainty
    avg_uncertainty ← total_uncertainty / plan.steps.len() as Scalar

    RETURN SimulatedOutcome {
        outcomes,
        cost: total_cost,
        uncertainty: avg_uncertainty,
        success_probability: 1.0 - avg_uncertainty,
    }
END PROCEDURE
```

### 17.3 Risk Evaluation Algorithm

```
ALGORITHM: EvaluateRisk
INPUT: Plan, WorldState
OUTPUT: RiskAssessment
BOUNDS: None (pure computation)

PROCEDURE EvaluateRisk(plan: &Plan, world: &WorldState) -> RiskAssessment:
    factors ← Vec::new()
    total_risk ← 0.0

    // Factor 1: Reversibility
    reversibility ← assess_reversibility(plan)
    factors.push(RiskFactor {
        description: "Reversibility",
        severity: 1.0 - reversibility,
        likelihood: 0.5,
    })
    total_risk ← total_risk + (1.0 - reversibility) * 0.3

    // Factor 2: Scope of impact
    scope ← assess_scope(plan, world)
    factors.push(RiskFactor {
        description: "Scope",
        severity: scope,
        likelihood: 0.7,
    })
    total_risk ← total_risk + scope * 0.25

    // Factor 3: Uncertainty
    uncertainty ← plan.uncertainty
    factors.push(RiskFactor {
        description: "Uncertainty",
        severity: uncertainty,
        likelihood: 1.0,
    })
    total_risk ← total_risk + uncertainty * 0.25

    // Factor 4: Resource consumption
    resource ← assess_resource_consumption(plan)
    factors.push(RiskFactor {
        description: "Resource",
        severity: resource,
        likelihood: 0.8,
    })
    total_risk ← total_risk + resource * 0.2

    // Determine risk level
    level ← match total_risk:
        r if r < 0.2 => RiskLevel::Low
        r if r < 0.4 => RiskLevel::Moderate
        r if r < 0.7 => RiskLevel::High
        _ => RiskLevel::Critical

    RETURN RiskAssessment {
        score: total_risk.clamp(0.0, 1.0),
        level,
        factors,
        reversibility,
    }
END PROCEDURE
```

---

## 18. Action Selection

### 18.1 Action Selection Algorithm

```
ALGORITHM: SelectAction
INPUT: Plan (optional), ReasoningResult, WorldState, PolicyState
OUTPUT: Action
BOUNDS: Policy constraints

PROCEDURE SelectAction(
    plan: Option<Plan>,
    reasoning: &ReasoningResult,
    world: &WorldState,
    policy: &PolicyState
) -> Action:
    // 1. If plan exists, select first step
    IF plan.is_some() THEN
        action ← plan.unwrap().steps[0]
    ELSE
        // 2. No plan: select direct response action
        action ← Action {
            id: next_action_id(),
            kind: ActionKind::Respond,
            parameters: HashMap::new(),
            expected_outcome: None,
            risk: RiskAssessment::minimal(),
            timestamp: Timestamp::now(),
            provenance: Provenance::derived(&[]),
        }
    END IF

    // 3. Policy check
    decision ← policy.evaluate(ProposedOperation::from_action(action))
    IF decision == Denied THEN
        // Fall back to NoOp
        action ← Action::no_op()
    ELSE IF decision == Limited THEN
        // Apply constraints
        action ← apply_constraints(action, decision.constraints)
    END IF

    RETURN action
END PROCEDURE
```

---

## 19. Verification

### 19.1 Verification Evaluation Algorithm

```
ALGORITHM: VerificationEvaluate
INPUT: ReasoningResult
OUTPUT: VerifiedResult
BOUNDS: evidence set size

PROCEDURE VerificationEvaluate(reasoning: &ReasoningResult) -> VerifiedResult:
    IF NOT config.verification.enabled THEN
        RETURN VerifiedResult::provisional(reasoning)
    END IF

    // 1. Extract primary claim
    claim ← reasoning.primary_claim()
    IF claim.is_none() THEN
        RETURN VerifiedResult::unknown()
    END IF

    // 2. Verify claim
    verification ← verify_claim(claim, reasoning.evidence_set())

    RETURN VerifiedResult {
        claim,
        verification_status: verification.status,
        confidence: verification.confidence,
        evidence: reasoning.evidence_set(),
    }
END PROCEDURE
```

### 19.2 Claim Verification Algorithm

```
ALGORITHM: VerifyClaim
INPUT: KnowledgeClaim, EvidenceSet
OUTPUT: VerificationResult
BOUNDS: None (pure computation)

PROCEDURE VerifyClaim(claim: &KnowledgeClaim, evidence: &EvidenceSet) -> VerificationResult:
    // 1. Source evaluation
    source_quality ← evaluate_sources(evidence)

    // 2. Consistency analysis
    consistency ← check_consistency(claim, evidence)

    // 3. Independent evidence count
    independent_count ← count_independent_sources(evidence)

    // 4. Contradiction analysis
    contradictions ← find_contradictions(claim, evidence)

    // 5. Compute confidence
    confidence ← ConfidenceState {
        belief: claim.confidence.belief,
        evidence_strength: evidence.total_strength(),
        source_quality,
        consistency,
        uncertainty: 1.0 - consistency,
        prediction_reliability: 0.0,
        verification_status: VerificationStatus::Unknown,
    }

    // 6. Determine verification status
    status ← determine_status(
        claim, evidence, source_quality, consistency,
        independent_count, contradictions, confidence
    )

    confidence.verification_status ← status

    RETURN VerificationResult { status, confidence }
END PROCEDURE
```

### 19.3 Verification Status Determination

```
ALGORITHM: DetermineStatus
INPUT: Claim, evidence metrics
OUTPUT: VerificationStatus

PROCEDURE DetermineStatus(
    claim, evidence, source_quality, consistency,
    independent_count, contradictions, confidence
) -> VerificationStatus:
    // Rule 1: Contradicted
    IF NOT contradictions.is_empty() THEN
        RETURN VerificationStatus::Contradicted
    END IF

    // Rule 2: Verified (highest bar)
    IF independent_count >= 2
       AND evidence.total_strength() >= config.verification.minimum_confidence
       AND source_quality >= 0.7
       AND consistency >= 0.8
    THEN
        RETURN VerificationStatus::Verified
    END IF

    // Rule 3: Supported
    IF evidence.total_strength() >= 0.5
       AND source_quality >= 0.5
       AND consistency >= 0.6
    THEN
        RETURN VerificationStatus::Supported
    END IF

    // Rule 4: Provisional
    IF evidence.total_strength() >= 0.3 THEN
        RETURN VerificationStatus::Provisional
    END IF

    // Rule 5: Inferred (no direct evidence)
    IF claim.was_inferred THEN
        RETURN VerificationStatus::Inferred
    END IF

    // Rule 6: Observed (directly observed)
    IF claim.was_observed THEN
        RETURN VerificationStatus::Observed
    END IF

    // Default: Unknown
    RETURN VerificationStatus::Unknown
END PROCEDURE
```

### 19.4 Verification Invariant Enforcement

```
ALGORITHM: EnforceVerificationInvariant
RULE: Verification SHALL never silently upgrade UNKNOWN to VERIFIED
      without satisfying configured evidence conditions.

PROCEDURE EnforceVerificationInvariant(
    current_status: VerificationStatus,
    proposed_status: VerificationStatus,
    evidence: &EvidenceSet
) -> VerificationStatus:
    // Cannot jump from Unknown to Verified
    IF current_status == Unknown AND proposed_status == Verified THEN
        // Must pass through intermediate states
        IF evidence.total_strength() < config.verification.minimum_confidence THEN
            RETURN VerificationStatus::Provisional
        END IF
    END IF

    // Cannot upgrade without evidence
    IF proposed_status > current_status AND evidence.is_empty() THEN
        RETURN current_status  // No upgrade without evidence
    END IF

    RETURN proposed_status
END PROCEDURE
```

---

## 20. Policy Evaluation

### 20.1 Policy Evaluation Algorithm

```
ALGORITHM: PolicyEvaluate
INPUT: ProposedOperation
OUTPUT: PolicyDecision
BOUNDS: None (pure computation)

PROCEDURE PolicyEvaluate(operation: &ProposedOperation) -> PolicyDecision:
    // 1. Classify operation
    classification ← classify_operation(operation)

    // 2. Estimate risk
    risk ← risk_estimate(operation)

    // 3. Check risk threshold
    IF risk.level >= RiskLevel::Critical THEN
        RETURN PolicyDecision::Denied { reason: DenialReason::CriticalRisk }
    END IF

    // 4. Policy evaluation by classification
    MATCH classification:
        OperationClass::CognitiveStateAdaptation =>
            IF config.policy.learning THEN
                RETURN PolicyDecision::Allowed
            ELSE
                RETURN PolicyDecision::Denied { reason: DenialReason::LearningDisabled }
            END IF

        OperationClass::AlgorithmAdaptation =>
            IF config.policy.self_modification THEN
                RETURN PolicyDecision::Limited {
                    constraints: OperationConstraints::bounded()
                }
            ELSE
                RETURN PolicyDecision::Denied { reason: DenialReason::SelfModificationDisabled }
            END IF

        OperationClass::SecurityPolicyModification =>
            IF config.policy.policy_modification THEN
                RETURN PolicyDecision::Limited {
                    constraints: OperationConstraints::strict()
                }
            ELSE
                RETURN PolicyDecision::Denied { reason: DenialReason::PolicyModificationDisabled }
            END IF

        OperationClass::RuntimeModification =>
            IF config.policy.runtime_modification THEN
                RETURN PolicyDecision::Limited {
                    constraints: OperationConstraints::strict()
                }
            ELSE
                RETURN PolicyDecision::Denied { reason: DenialReason::RuntimeModificationDisabled }
            END IF
    END MATCH
END PROCEDURE
```

### 20.2 Risk Estimation Algorithm

```
ALGORITHM: RiskEstimate
INPUT: ProposedOperation
OUTPUT: RiskEstimate
BOUNDS: None (pure computation)

PROCEDURE RiskEstimate(operation: &ProposedOperation) -> RiskEstimate:
    risk_score ← 0.0

    // Factor 1: Impact
    impact ← operation.estimated_impact
    risk_score ← risk_score + impact * 0.30

    // Factor 2: Uncertainty
    uncertainty ← 1.0 - operation.reversibility
    risk_score ← risk_score + uncertainty * 0.25

    // Factor 3: Scope
    scope ← estimate_scope(operation)
    risk_score ← risk_score + scope * 0.20

    // Factor 4: Resource consumption
    resource ← operation.resource_estimate
    risk_score ← risk_score + resource * 0.15

    // Factor 5: External side effects
    external ← estimate_external_effects(operation)
    risk_score ← risk_score + external * 0.10

    // Determine level
    level ← match risk_score:
        r if r < 0.2 => RiskLevel::Low
        r if r < 0.4 => RiskLevel::Moderate
        r if r < 0.7 => RiskLevel::High
        _ => RiskLevel::Critical

    RETURN RiskEstimate {
        score: risk_score.clamp(0.0, 1.0),
        level,
    }
END PROCEDURE
```

---

## 21. Confidence Calculation

### 21.1 Confidence Aggregation Algorithm

```
ALGORITHM: AggregateConfidence
INPUT: Multiple confidence sources
OUTPUT: ConfidenceState
BOUNDS: None (pure computation)

PROCEDURE AggregateConfidence(sources: &[ConfidenceSource]) -> ConfidenceState:
    IF sources.is_empty() THEN
        RETURN ConfidenceState::low()
    END IF

    // Weighted aggregation
    total_weight ← 0.0
    weighted_belief ← 0.0
    weighted_evidence ← 0.0
    weighted_quality ← 0.0
    weighted_consistency ← 0.0

    FOR each source IN sources:
        weight ← source.weight
        total_weight ← total_weight + weight
        weighted_belief ← weighted_belief + source.confidence.belief * weight
        weighted_evidence ← weighted_evidence + source.confidence.evidence_strength * weight
        weighted_quality ← weighted_quality + source.confidence.source_quality * weight
        weighted_consistency ← weighted_consistency + source.confidence.consistency * weight
    END FOR

    IF total_weight == 0.0 THEN
        RETURN ConfidenceState::low()
    END IF

    belief ← weighted_belief / total_weight
    evidence ← weighted_evidence / total_weight
    quality ← weighted_quality / total_weight
    consistency ← weighted_consistency / total_weight
    uncertainty ← 1.0 - consistency

    RETURN ConfidenceState {
        belief,
        evidence_strength: evidence,
        source_quality: quality,
        consistency,
        uncertainty,
        prediction_reliability: 0.0,
        verification_status: VerificationStatus::Unknown,
    }
END PROCEDURE
```

### 21.2 Overall Confidence Score

```
ALGORITHM: OverallConfidence
INPUT: ConfidenceState
OUTPUT: Scalar [0.0, 1.0]

PROCEDURE OverallConfidence(conf: &ConfidenceState) -> Scalar:
    score ← 0.0
    score ← score + conf.belief * 0.30
    score ← score + conf.evidence_strength * 0.25
    score ← score + conf.source_quality * 0.15
    score ← score + conf.consistency * 0.20
    score ← score + (1.0 - conf.uncertainty) * 0.10
    RETURN score.clamp(0.0, 1.0)
END PROCEDURE
```

---

## 22. Uncertainty Handling

### 22.1 Uncertainty Propagation Algorithm

```
ALGORITHM: PropagateUncertainty
INPUT: Input uncertainty, operation type
OUTPUT: Output uncertainty
BOUNDS: None

PROCEDURE PropagateUncertainty(input_uncertainty: Scalar, operation: OperationType) -> Scalar:
    MATCH operation:
        OperationType::Encoding =>
            // Encoding adds small uncertainty
            RETURN min(1.0, input_uncertainty + 0.05)

        OperationType::Reasoning =>
            // Reasoning may increase uncertainty
            RETURN min(1.0, input_uncertainty + 0.10)

        OperationType::Planning =>
            // Planning compounds uncertainty per step
            RETURN min(1.0, input_uncertainty * 1.1)

        OperationType::Prediction =>
            // Prediction uncertainty grows with horizon
            RETURN min(1.0, input_uncertainty + 0.15)

        OperationType::Verification =>
            // Verification may reduce uncertainty
            RETURN max(0.0, input_uncertainty - 0.20)

        OperationType::Learning =>
            // Learning slightly reduces uncertainty
            RETURN max(0.0, input_uncertainty - 0.05)
    END MATCH
END PROCEDURE
```

### 22.2 Uncertainty Reduction Strategy

```
ALGORITHM: ReduceUncertainty
INPUT: Current uncertainty, available evidence
OUTPUT: Updated uncertainty, recommended actions

PROCEDURE ReduceUncertainty(uncertainty: Scalar, evidence: &EvidenceSet) -> (Scalar, Vec<Action>):
    actions ← Vec::new()

    IF uncertainty > 0.7 THEN
        // High uncertainty: seek more evidence
        actions.push(Action::seek_evidence())
        actions.push(Action::verify_claim())
    END IF

    IF uncertainty > 0.5 THEN
        // Moderate uncertainty: cross-reference
        actions.push(Action::cross_reference())
    END IF

    // Evidence reduces uncertainty
    reduction ← evidence.total_strength() * 0.3
    new_uncertainty ← max(0.0, uncertainty - reduction)

    RETURN (new_uncertainty, actions)
END PROCEDURE
```

---

## 23. Learning Algorithms

### 23.1 Learning Signal Generation

```
ALGORITHM: GenerateLearningSignal
INPUT: Experience
OUTPUT: LearningSignal
BOUNDS: None

PROCEDURE GenerateLearningSignal(experience: &Experience) -> LearningSignal:
    IF NOT config.learning.enabled THEN
        RETURN LearningSignal::none()
    END IF

    // 1. Compute learning signal from prediction error
    signal ← LearningSignal {
        prediction_error: experience.error.clone(),
        attribution: experience.attribution.clone(),
        evidence: experience.evidence.clone(),
        source: experience.provenance.clone(),
        magnitude: experience.error.magnitude,
    }

    RETURN signal
END PROCEDURE
```

### 23.2 Error Attribution Algorithm

```
ALGORITHM: AttributeError
INPUT: PredictionError
OUTPUT: ErrorAttribution
BOUNDS: None

PROCEDURE AttributeError(error: &PredictionError) -> ErrorAttribution:
    // Analyze error dimensions to determine source
    dimensions ← error.dimensions

    // Heuristic attribution based on error pattern
    IF dimensions.contains("input_encoding") AND dimensions["input_encoding"] > 0.5 THEN
        RETURN ErrorAttribution::InputError
    END IF

    IF dimensions.contains("memory_retrieval") AND dimensions["memory_retrieval"] > 0.5 THEN
        RETURN ErrorAttribution::MemoryError
    END IF

    IF dimensions.contains("world_prediction") AND dimensions["world_prediction"] > 0.5 THEN
        RETURN ErrorAttribution::WorldError
    END IF

    IF dimensions.contains("reasoning_step") AND dimensions["reasoning_step"] > 0.5 THEN
        RETURN ErrorAttribution::ReasoningError
    END IF

    IF dimensions.contains("procedure_execution") AND dimensions["procedure_execution"] > 0.5 THEN
        RETURN ErrorAttribution::ProcedureError
    END IF

    // Default: environment error (unpredictable external change)
    RETURN ErrorAttribution::EnvironmentError
END PROCEDURE
```

### 23.3 Learning Application Algorithm

```
ALGORITHM: ApplyLearningSignal
INPUT: LearningSignal, PolicyState
OUTPUT: LearningResult
BOUNDS: learning_rate, plasticity

PROCEDURE ApplyLearningSignal(signal: &LearningSignal, policy: &PolicyState) -> LearningResult:
    // 1. Policy check
    IF NOT policy.allows_learning() THEN
        RETURN LearningResult::denied()
    END IF

    // 2. Compute bounded update magnitude
    magnitude ← min(signal.magnitude, config.learning.learning_rate)

    // 3. Route to appropriate subsystem based on attribution
    MATCH signal.attribution:
        ErrorAttribution::InputError =>
            language.update(signal)

        ErrorAttribution::MemoryError =>
            memory.update(signal)

        ErrorAttribution::WorldError =>
            world.update(signal)

        ErrorAttribution::ReasoningError =>
            reasoning.update(signal)

        ErrorAttribution::ProcedureError =>
            procedural.update(signal)

        ErrorAttribution::EnvironmentError =>
            // No specific update; environment is unpredictable
            ()
    END MATCH

    // 4. Update learning statistics
    learning_state.total_learning_events += 1
    learning_state.update_average_error(signal.magnitude)

    RETURN LearningResult {
        applied: true,
        magnitude,
        attribution: signal.attribution,
    }
END PROCEDURE
```

### 23.4 Learning Stability Guard

```
ALGORITHM: LearningStabilityGuard
INPUT: Proposed update, current state
OUTPUT: Approved/Modified/Rejected update
BOUNDS: plasticity_bound

PROCEDURE LearningStabilityGuard(update: &ProposedUpdate, state: &CortexState) -> UpdateDecision:
    // 1. Check update magnitude
    IF update.magnitude > config.learning.plasticity THEN
        // Clamp to plasticity bound
        update.magnitude ← config.learning.plasticity
    END IF

    // 2. Check for catastrophic change
    IF update.would_affect_percentage(state) > 0.10 THEN
        // More than 10% of state would change: reject
        RETURN UpdateDecision::Rejected("Catastrophic change prevented")
    END IF

    // 3. Check for single-observation dominance
    IF update.source_episodes.len() == 1 AND update.magnitude > 0.5 THEN
        // Single observation with high magnitude: reduce
        update.magnitude ← update.magnitude * 0.3
    END IF

    // 4. Check contradiction with existing knowledge
    IF update.contradicts_existing(state) THEN
        // Don't overwrite; add as competing hypothesis
        RETURN UpdateDecision::Modified("Added as competing hypothesis")
    END IF

    RETURN UpdateDecision::Approved(update)
END PROCEDURE
```

---

## 24. Plasticity Algorithms

### 24.1 Plasticity Update Algorithm

```
ALGORITHM: PlasticityUpdate
INPUT: Activation relationship, context factor, prediction error, evidence confidence
OUTPUT: Weight change ΔW
BOUNDS: plasticity_bound

FORMULA: ΔW = η × A × C × E × V

PROCEDURE PlasticityUpdate(
    activation_relationship: Scalar,  // A: correlation between pre/post activation
    context_factor: Scalar,           // C: relevance of context
    prediction_error: Scalar,         // E: magnitude of prediction error
    evidence_confidence: Scalar       // V: confidence in evidence
) -> Scalar:
    // 1. Compute raw update
    delta ← config.learning.learning_rate
        * activation_relationship
        * context_factor
        * prediction_error
        * evidence_confidence

    // 2. Bound the update
    bounded_delta ← delta.clamp(
        -config.learning.plasticity,
        config.learning.plasticity
    )

    RETURN bounded_delta
END PROCEDURE
```

### 24.2 Cell Adaptation Algorithm

```
ALGORITHM: CellAdapt
INPUT: Cell, error signal, learning rate
OUTPUT: Updated cell
BOUNDS: plasticity_bound

PROCEDURE CellAdapt(cell: &mut Cell, error: Scalar, learning_rate: Scalar):
    // 1. Compute adaptation
    delta ← learning_rate * cell.plasticity * error

    // 2. Bound adaptation
    bounded_delta ← delta.clamp(-0.1, 0.1)

    // 3. Apply adaptation
    cell.activation ← cell.activation + bounded_delta

    // 4. Update state
    cell.state ← CellState::Learning

    // 5. Decay activation toward baseline
    cell.activation ← cell.activation * 0.99

    // 6. Clamp activation to valid range
    cell.activation ← cell.activation.clamp(0.0, 1.0)
END PROCEDURE
```

### 24.3 Plasticity Constraints

| Constraint | Rule |
|---|---|
| Single-update bound | `|ΔW| ≤ plasticity` |
| Single-observation bound | One observation cannot change > 10% of total state |
| Cumulative bound | Total change per learning cycle ≤ 5% of state |
| Confidence gate | Low-confidence evidence reduces update magnitude |
| Policy gate | Policy can disable plasticity entirely |
| Stability guard | Catastrophic change detection prevents runaway updates |

---

## 25. Replay Algorithms

### 25.1 Replay Priority Algorithm

```
ALGORITHM: ComputeReplayPriority
INPUT: Episode
OUTPUT: Priority score [0.0, ∞)
BOUNDS: None

PROCEDURE ComputeReplayPriority(episode: &Episode) -> Scalar:
    priority ← 0.0

    // Factor 1: Prediction error (higher error = higher priority)
    priority ← priority + episode.prediction_error.magnitude * 0.40

    // Factor 2: Novelty (less consolidated = higher priority)
    novelty ← if episode.consolidated { 0.2 } else { 0.8 }
    priority ← priority + novelty * 0.20

    // Factor 3: Importance
    priority ← priority + episode.importance * 0.20

    // Factor 4: Uncertainty (higher uncertainty = higher priority)
    priority ← priority + (1.0 - episode.confidence.overall()) * 0.10

    // Factor 5: Recurrence (repeated patterns = higher priority)
    recurrence ← min(episode.retrieval_count, 10) / 10.0
    priority ← priority + recurrence * 0.10

    RETURN priority
END PROCEDURE
```

### 25.2 Replay Execution Algorithm

```
ALGORITHM: ExecuteReplay
INPUT: Episodes, compute budget
OUTPUT: ReplayResult
BOUNDS: max_replay_count

PROCEDURE ExecuteReplay(episodes: &[Episode], budget: &ComputeBudget) -> ReplayResult:
    // 1. Prioritize episodes
    prioritized ← episodes.iter()
        .map(|e| (e, ComputeReplayPriority(e)))
        .collect()
    prioritized.sort_by_priority_desc()

    // 2. Select top episodes within budget
    max_replay ← budget.max_replay_count
    selected ← prioritized[..max_replay]

    // 3. Replay each selected episode
    result ← ReplayResult::default()
    FOR each (episode, priority) IN selected:
        // Reconstruct context
        context ← reconstruct_context(episode)

        // Generate prediction
        prediction ← predict_from_context(context)

        // Compare with actual outcome
        IF episode.outcome.is_some() THEN
            error ← compute_error(prediction, episode.outcome.unwrap())

            // Generate learning signal
            signal ← LearningSignal::from_error(error, episode)

            // Apply learning
            learning.apply_signal(signal, policy.state())

            result.replayed += 1
            result.total_error += error.magnitude
        END IF
    END FOR

    result.average_error ← result.total_error / max(1, result.replayed)

    RETURN result
END PROCEDURE
```

---

## 26. Consolidation

### 26.1 Consolidation Algorithm

```
ALGORITHM: Consolidate
INPUT: Consolidation candidates, policy
OUTPUT: ConsolidationResult
BOUNDS: consolidation threshold

PROCEDURE Consolidate(candidates: &[ConsolidationCandidate], policy: &PolicyState) -> ConsolidationResult:
    result ← ConsolidationResult::default()

    FOR each candidate IN candidates:
        // 1. Evaluate candidate
        evaluation ← evaluate_candidate(candidate)

        // 2. Check consolidation criteria
        IF NOT evaluation.should_consolidate THEN
            result.rejected += 1
            CONTINUE
        END IF

        // 3. Check minimum supporting episodes (prevent single-event dominance)
        IF candidate.supporting_episodes.len() < 3 THEN
            result.rejected += 1
            CONTINUE
        END IF

        // 4. Check confidence threshold
        IF evaluation.confidence < config.learning.consolidation_threshold THEN
            result.rejected += 1
            CONTINUE
        END IF

        // 5. Check contradiction risk
        IF candidate.contradiction_risk > 0.5 THEN
            result.rejected += 1
            CONTINUE
        END IF

        // 6. Policy check
        decision ← policy.evaluate(ProposedOperation::consolidation())
        IF decision == Denied THEN
            result.rejected += 1
            CONTINUE
        END IF

        // 7. Consolidate based on target
        MATCH candidate.target:
            ConsolidationTarget::Semantic =>
                semantic_memory.integrate(candidate.knowledge.unwrap())
                result.semantic_integrations += 1

            ConsolidationTarget::Procedural =>
                procedural_memory.integrate(candidate.procedure.unwrap())
                result.procedural_integrations += 1

            ConsolidationTarget::Associative =>
                associative_memory.integrate(candidate.association.unwrap())
                result.associative_integrations += 1
        END MATCH

        result.consolidated += 1
    END FOR

    RETURN result
END PROCEDURE
```

### 26.2 Consolidation Candidate Evaluation

```
ALGORITHM: EvaluateCandidate
INPUT: ConsolidationCandidate
OUTPUT: EvaluationResult
BOUNDS: None

PROCEDURE EvaluateCandidate(candidate: &ConsolidationCandidate) -> EvaluationResult:
    // 1. Pattern strength
    pattern_strength ← candidate.pattern_strength

    // 2. Evidence strength
    evidence_strength ← candidate.evidence_strength

    // 3. Contradiction risk
    contradiction_risk ← candidate.contradiction_risk

    // 4. Determine if should consolidate
    should_consolidate ← pattern_strength > 0.6
        AND evidence_strength > 0.5
        AND contradiction_risk < 0.5

    // 5. Compute confidence
    confidence ← (pattern_strength + evidence_strength) / 2.0

    RETURN EvaluationResult {
        should_consolidate,
        confidence,
        risk: contradiction_risk,
    }
END PROCEDURE
```

---

## 27. Self-Model Update

### 27.1 Self-Model Update Algorithm

```
ALGORITHM: UpdateSelfModel
INPUT: Performance metrics
OUTPUT: Updated SelfModel
BOUNDS: History capacity (100 snapshots)

PROCEDURE UpdateSelfModel(metrics: &PerformanceMetrics):
    // 1. Update prediction accuracy
    self_model.prediction_accuracy ← moving_average(
        self_model.prediction_accuracy,
        metrics.prediction_accuracy,
        0.1  // smoothing factor
    )

    // 2. Update memory health
    self_model.memory_health ← metrics.memory_health.clone()

    // 3. Update language capability
    self_model.language_capability ← LanguageCapability {
        vocabulary_size: language.vocabulary_size(),
        accuracy: metrics.language_accuracy,
        confidence: metrics.language_confidence,
        unknown_word_rate: metrics.unknown_word_rate,
    }

    // 4. Update reasoning performance
    self_model.reasoning_performance ← ReasoningPerformance {
        consistency: metrics.reasoning_consistency,
        confidence: metrics.reasoning_confidence,
        average_steps: metrics.average_reasoning_steps,
        contradiction_rate: metrics.contradiction_rate,
    }

    // 5. Update learning statistics
    self_model.learning_statistics ← LearningStatistics {
        total_events: learning.total_events(),
        average_error: learning.average_error(),
        learning_rate_effective: learning.learning_rate(),
        consolidation_rate: learning.consolidation_rate(),
        forgetting_rate: learning.forgetting_rate(),
    }

    // 6. Update capabilities
    self_model.capabilities ← compute_capabilities(metrics)

    // 7. Record performance snapshot
    snapshot ← PerformanceSnapshot {
        timestamp: Timestamp::now(),
        prediction_accuracy: metrics.prediction_accuracy,
        memory_pressure: metrics.memory_pressure,
        learning_events: learning.total_events(),
        reasoning_steps: metrics.reasoning_steps,
    }
    self_model.historical_performance.push(snapshot)

    // 8. Update uncertainty
    self_model.uncertainty.level ← compute_uncertainty(metrics)

    self_model.last_updated ← Timestamp::now()
END PROCEDURE
```

### 27.2 Moving Average Computation

```
ALGORITHM: MovingAverage
INPUT: Current value, new value, smoothing factor
OUTPUT: Updated value

PROCEDURE MovingAverage(current: Scalar, new_value: Scalar, alpha: Scalar) -> Scalar:
    RETURN current * (1.0 - alpha) + new_value * alpha
END PROCEDURE
```

---

## 28. Internet Interaction Algorithm

### 28.1 Internet Fetch Algorithm

```
ALGORITHM: InternetFetch
INPUT: NetworkRequest, PolicyState
OUTPUT: NetworkObservation
BOUNDS: timeout_seconds, max_response_mb

PROCEDURE InternetFetch(request: &NetworkRequest, policy: &PolicyState) -> NetworkObservation:
    // 1. Policy check
    IF NOT config.internet.enabled THEN
        RETURN Error(NetworkError, "Internet disabled")
    END IF

    IF NOT policy.allows_internet() THEN
        RETURN Error(PolicyError, "Internet access denied by policy")
    END IF

    // 2. Risk assessment
    risk ← risk_estimate(ProposedOperation::network(request))
    IF risk.level >= RiskLevel::High THEN
        RETURN Error(PolicyError, "Network operation risk too high")
    END IF

    // 3. Execute fetch with timeout
    response ← http_client
        .timeout(Duration::from_secs(config.internet.timeout_seconds))
        .max_size(config.internet.max_response_mb * 1024 * 1024)
        .fetch(request)

    IF response.is_error() THEN
        RETURN Error(NetworkError, response.error_message())
    END IF

    // 4. Construct observation
    observation ← NetworkObservation {
        content: response.body,
        status: response.status,
        timestamp: Timestamp::now(),
        source_url: request.url,
        content_hash: blake3::hash(response.body.as_bytes()).into(),
        size_bytes: response.body.len(),
    }

    RETURN observation
END PROCEDURE
```

### 28.2 Internet Content Processing Algorithm

```
ALGORITHM: ProcessInternetContent
INPUT: NetworkObservation
OUTPUT: Observation (for cognitive pipeline)
BOUNDS: max_response_mb

PROCEDURE ProcessInternetContent(net_obs: &NetworkObservation) -> Observation:
    // 1. Parse content
    extracted ← parse_content(net_obs.content)

    // 2. Create provenance
    provenance ← Provenance::internet(net_obs.source_url)
    provenance.content_hash ← net_obs.content_hash

    // 3. Create observation
    observation ← Observation {
        text: extracted.text,
        source: provenance,
        timestamp: net_obs.timestamp,
        context: ContextState::initial(),
        kind: ObservationKind::Internet,
        importance: 0.3,  // Lower default importance for internet content
    }

    // 4. Mark as requiring verification
    observation.source.verification_status ← VerificationStatus::Unknown

    RETURN observation
END PROCEDURE
```

---

## 29. Error Recovery Algorithms

### 29.1 Error Recovery Algorithm

```
ALGORITHM: RecoverFromError
INPUT: CortexError, current state
OUTPUT: Recovery action
BOUNDS: Checkpoint count

PROCEDURE RecoverFromError(error: &CortexError, state: &CortexState) -> RecoveryAction:
    // 1. Classify error severity
    severity ← classify_severity(error)

    // 2. Determine recovery strategy
    MATCH severity:
        ErrorSeverity::Recoverable =>
            // Log and continue
            log_error(error)
            RETURN RecoveryAction::Continue

        ErrorSeverity::StateCorruption =>
            // Attempt checkpoint recovery
            RETURN attempt_checkpoint_recovery()

        ErrorSeverity::Fatal =>
            // Safe stop
            RETURN RecoveryAction::SafeStop

        ErrorSeverity::Configuration =>
            // Cannot recover from config error at runtime
            RETURN RecoveryAction::SafeStop
    END MATCH
END PROCEDURE
```

### 29.2 Checkpoint Recovery Algorithm

```
ALGORITHM: AttemptCheckpointRecovery
INPUT: None
OUTPUT: Recovery result
BOUNDS: Checkpoint count

PROCEDURE AttemptCheckpointRecovery() -> RecoveryAction:
    // 1. Get checkpoints sorted by timestamp (newest first)
    checkpoints ← persistence.list_checkpoints()
    checkpoints.sort_by_timestamp_desc()

    // 2. Try each checkpoint
    FOR each checkpoint IN checkpoints:
        MATCH persistence.load(checkpoint.path):
            Ok(state) =>
                // Validate loaded state
                IF state.validate_invariants().is_ok() THEN
                    RETURN RecoveryAction::RestoreFromCheckpoint(state)
                END IF

            Err(e) =>
                // Checkpoint is also corrupt; try next
                log_warning("Checkpoint corrupt: {}", e)
                CONTINUE
        END MATCH
    END FOR

    // 3. No valid checkpoint found: initialize new state
    log_error("No valid checkpoint found; initializing new state")
    new_state ← initialize_new_state()
    RETURN RecoveryAction::InitializeNew(new_state)
END PROCEDURE
```

---

## 30. Resource Management Algorithms

### 30.1 Memory Pressure Response Algorithm

```
ALGORITHM: HandleMemoryPressure
INPUT: MemoryPressure level
OUTPUT: Actions taken
BOUNDS: Memory budgets

PROCEDURE HandleMemoryPressure(pressure: MemoryPressure):
    MATCH pressure:
        MemoryPressure::Low =>
            // No action needed
            ()

        MemoryPressure::Moderate =>
            // Consolidate to reduce fragmentation
            memory.consolidate()

        MemoryPressure::High =>
            // Consolidate + evict low-value items
            memory.consolidate()
            memory.forget(ForgettingPolicy::moderate())

        MemoryPressure::Critical =>
            // Aggressive: consolidate + aggressive forgetting
            memory.consolidate()
            memory.forget(ForgettingPolicy::emergency())
            // Compress working memory
            working_memory.compress()
    END MATCH
END PROCEDURE
```

### 30.2 Forgetting Policy Algorithm

```
ALGORITHM: ApplyForgettingPolicy
INPUT: ForgettingPolicy
OUTPUT: ForgettingResult
BOUNDS: Memory budgets

PROCEDURE ApplyForgettingPolicy(policy: &ForgettingPolicy) -> ForgettingResult:
    result ← ForgettingResult::default()

    // Score each episode for forgetting
    FOR each episode IN episodic_memory.episodes:
        forget_score ← compute_forget_score(episode, policy)
        IF forget_score > 0.7 THEN
            episodic_memory.remove(episode.id)
            result.episodic_forgotten += 1
        END IF
    END FOR

    // Similar for semantic, associative, procedural
    // ...

    RETURN result
END PROCEDURE

PROCEDURE ComputeForgetScore(episode: &Episode, policy: &ForgettingPolicy) -> Scalar:
    score ← 0.0

    // Low importance
    IF episode.importance < policy.min_importance THEN
        score ← score + 0.2
    END IF

    // Low confidence
    IF episode.confidence.overall() < policy.min_confidence THEN
        score ← score + 0.2
    END IF

    // Age
    IF policy.max_age.is_some() THEN
        age ← Timestamp::now() - episode.timestamp
        IF age > policy.max_age.unwrap() THEN
            score ← score + 0.2
        END IF
    END IF

    // Low retrieval frequency
    IF episode.retrieval_count < policy.min_retrieval_count THEN
        score ← score + 0.2
    END IF

    // Redundancy (consolidated)
    IF episode.consolidated THEN
        score ← score + 0.1
    END IF

    // Contradiction
    IF episode.contradicted THEN
        score ← score + 0.1
    END IF

    RETURN score.clamp(0.0, 1.0)
END PROCEDURE
```

### 30.3 Compute Budget Enforcement Algorithm

```
ALGORITHM: EnforceComputeBudget
INPUT: Operation type, current budget
OUTPUT: Continue/Bounded/Abort
BOUNDS: Configured limits

PROCEDURE EnforceComputeBudget(operation: OperationType, budget: &mut ComputeBudget) -> BudgetDecision:
    MATCH operation:
        OperationType::Reasoning =>
            IF budget.max_reasoning_steps <= 0 THEN
                RETURN BudgetDecision::Bounded("Reasoning budget exhausted")
            END IF
            budget.max_reasoning_steps -= 1
            RETURN BudgetDecision::Continue

        OperationType::Planning =>
            IF budget.max_planning_depth <= 0 THEN
                RETURN BudgetDecision::Bounded("Planning depth exhausted")
            END IF
            budget.max_planning_depth -= 1
            RETURN BudgetDecision::Continue

        OperationType::Generation =>
            IF budget.max_generation_length <= 0 THEN
                RETURN BudgetDecision::Bounded("Generation limit reached")
            END IF
            budget.max_generation_length -= 1
            RETURN BudgetDecision::Continue

        OperationType::Simulation =>
            IF budget.max_simulation_steps <= 0 THEN
                RETURN BudgetDecision::Bounded("Simulation budget exhausted")
            END IF
            budget.max_simulation_steps -= 1
            RETURN BudgetDecision::Continue
    END MATCH
END PROCEDURE
```

---

## 31. Persistence Algorithms

### 31.1 Atomic Save Algorithm

```
ALGORITHM: AtomicSave
INPUT: CortexState, target path
OUTPUT: SaveResult
BOUNDS: Disk I/O

PROCEDURE AtomicSave(state: &CortexState, path: &Path) -> SaveResult:
    // 1. Validate state before serialization
    state.validate_invariants()?

    // 2. Serialize
    serialized ← serialize_state(state)

    // 3. Compute integrity
    checksum ← compute_file_checksum(serialized)

    // 4. Write to temporary file
    temp_path ← path.with_extension("tmp")
    write_file(temp_path, serialized)
    flush(temp_path)
    sync(temp_path)

    // 5. Verify written file
    verify_file(temp_path)?

    // 6. Atomic replace
    rename(temp_path, path)

    // 7. Sync directory
    sync_directory(path.parent())

    RETURN SaveResult {
        bytes_written: serialized.len(),
        checksum,
        duration_ms: elapsed(),
        timestamp: Timestamp::now(),
    }
END PROCEDURE
```

### 31.2 State Load Algorithm

```
ALGORITHM: LoadState
INPUT: Path
OUTPUT: CortexState
BOUNDS: File size

PROCEDURE LoadState(path: &Path) -> CortexState:
    // 1. Read file
    data ← read_file(path)?

    // 2. Verify magic
    IF data[0..8] != b"CORTEX\0\0" THEN
        RETURN Error(PersistenceError, "Invalid magic bytes")
    END IF

    // 3. Parse header
    header ← parse_header(data)?

    // 4. Verify file checksum
    file_checksum ← compute_file_checksum(data)
    IF file_checksum != header.integrity.file_checksum THEN
        RETURN Error(PersistenceError, "File checksum mismatch")
    END IF

    // 5. Check version
    IF header.format_version > CURRENT_FORMAT_VERSION THEN
        RETURN Error(PersistenceError, "State version is newer than supported")
    END IF

    // 6. Migration if needed
    IF header.format_version < CURRENT_FORMAT_VERSION THEN
        data ← migrate(data, header.format_version, CURRENT_FORMAT_VERSION)?
    END IF

    // 7. Deserialize sections
    state ← deserialize_state(data)?

    // 8. Validate invariants
    state.validate_invariants()?

    // 9. Verify config hash
    current_config_hash ← compute_config_hash(config)
    IF state.metadata.config_hash != current_config_hash THEN
        log_warning("Configuration has changed since state was saved")
    END IF

    RETURN state
END PROCEDURE
```

### 31.3 Checkpoint Algorithm

```
ALGORITHM: CreateCheckpoint
INPUT: CortexState
OUTPUT: CheckpointId
BOUNDS: Disk I/O

PROCEDURE CreateCheckpoint(state: &CortexState) -> CheckpointId:
    // 1. Generate checkpoint ID
    checkpoint_id ← next_checkpoint_id()

    // 2. Serialize state
    serialized ← serialize_state(state)

    // 3. Compute integrity
    checksum ← compute_file_checksum(serialized)

    // 4. Write checkpoint file
    checkpoint_path ← checkpoint_dir / format!("checkpoint_{:06}.cx", checkpoint_id.0)
    write_file(checkpoint_path, serialized)

    // 5. Record metadata
    metadata ← CheckpointMetadata {
        id: checkpoint_id,
        state_version: state.metadata.architecture_version,
        algorithm_version: state.metadata.algorithm_versions.current(),
        config_hash: state.metadata.config_hash,
        timestamp: Timestamp::now(),
        episode_count: state.metadata.episode_count,
        learning_state: state.learning.clone(),
        integrity_checksum: checksum,
        file_path: checkpoint_path.to_string(),
        file_size_bytes: serialized.len(),
    }

    // 6. Store metadata
    persistence_state.checkpoints.push(metadata)

    // 7. Cleanup old checkpoints (keep max_checkpoints)
    IF persistence_state.checkpoints.len() > max_checkpoints THEN
        oldest ← persistence_state.checkpoints.remove(0)
        delete_file(oldest.file_path)
    END IF

    RETURN checkpoint_id
END PROCEDURE
```

---

## 32. State Transition Algorithms

### 32.1 State Machine Transition Algorithm

```
ALGORITHM: TransitionState
INPUT: Current RuntimeState, RuntimeEvent
OUTPUT: New RuntimeState
BOUNDS: Valid transitions only

PROCEDURE TransitionState(current: &mut RuntimeState, event: RuntimeEvent) -> Result:
    new_state ← MATCH (current, event):
        (Boot, ConfigLoaded) => LoadConfiguration
        (LoadConfiguration, StateLoaded) => LoadState
        (LoadState, StateValidated) => Validate
        (Validate, Initialized) => Initialize
        (Initialize, Ready) => Ready
        (Ready, InputReceived) => Processing
        (Processing, ProcessingComplete) => Learning
        (Learning, LearningComplete) => Consolidating
        (Consolidating, ConsolidationComplete) => Checkpointing
        (Checkpointing, CheckpointComplete) => Ready
        (Ready, ShutdownRequested) => Shutdown
        (_, FatalError(e)) => Fault { error: e }
        (Fault, RecoveryPossible) => Recovery
        (Recovery, RecoveryComplete) => Ready
        (Recovery, RecoveryFailed) => SafeStop
        (Fault, RecoveryImpossible) => SafeStop
        (_, _) => RETURN Error("Invalid state transition")
    END MATCH

    *current ← new_state
    RETURN Ok(())
END PROCEDURE
```

---

## 33. Determinism Rules

### 33.1 Deterministic Operations

| Operation | Deterministic? | Condition |
|---|---|---|
| State serialization | YES | Same state → same bytes |
| Configuration parsing | YES | Same config → same result |
| Vocabulary lookup | YES | Same token → same ID |
| Memory indexing | YES | Same query → same candidates |
| Relevance scoring | YES | Same inputs → same score |
| Verification rules | YES | Same evidence → same status |
| Policy decisions | YES | Same operation → same decision |
| Checksum computation | YES | Same data → same checksum |
| Checkpoint structure | YES | Same state → same structure |
| Error classification | YES | Same error → same kind |

### 33.2 Non-Deterministic Operations

| Operation | Non-Deterministic? | Reason |
|---|---|---|
| Timestamp generation | YES | System clock |
| UUID generation | YES | Random (v4) |
| Learning updates (if configured stochastic) | MAY BE | Explicit configuration |
| Neural activation noise (if configured) | MAY BE | Explicit configuration |
| HashMap iteration order | NO (sorted for serialization) | Deterministic serialization |

### 33.3 Determinism Enforcement Rules

| Rule | Description |
|---|---|
| DET-001 | All HashMap serialization sorts by key |
| DET-002 | All HashSet serialization sorts elements |
| DET-003 | No random ordering in serialized collections |
| DET-004 | Floating-point operations use consistent rounding |
| DET-005 | No dependency on iteration order for correctness |
| DET-006 | Random seed recorded for reproducibility |

---

## 34. Numerical Stability

### 34.1 Floating-Point Rules

| Rule | Description |
|---|---|
| NUM-001 | NaN is NEVER valid in persisted state |
| NUM-002 | Infinity is NEVER valid in persisted state |
| NUM-003 | All Scalar fields validated before persistence |
| NUM-004 | Comparison uses epsilon: `|a - b| < 1e-6` |
| NUM-005 | Division by zero produces defined error, not NaN |
| NUM-006 | Square root of negative produces defined error |
| NUM-007 | Logarithm of non-positive produces defined error |
| NUM-008 | Exponential overflow clamped to max Scalar |

### 34.2 Numerical Guard Algorithm

```
ALGORITHM: NumericalGuard
INPUT: Scalar value
OUTPUT: Validated/clamped Scalar
BOUNDS: None

PROCEDURE NumericalGuard(value: Scalar) -> Scalar:
    // 1. Check for NaN
    IF value.is_nan() THEN
        RETURN 0.0  // Default to zero
    END IF

    // 2. Check for Infinity
    IF value.is_infinite() THEN
        IF value > 0.0 THEN
            RETURN Scalar::MAX
        ELSE
            RETURN Scalar::MIN
        END IF
    END IF

    // 3. Check for subnormal (very small values)
    IF value.abs() < 1e-38 THEN
        RETURN 0.0  // Treat as zero
    END IF

    RETURN value
END PROCEDURE
```

### 34.3 Safe Arithmetic Operations

```
ALGORITHM: SafeDivide
INPUT: numerator, denominator
OUTPUT: Result or error

PROCEDURE SafeDivide(num: Scalar, den: Scalar) -> Scalar:
    IF den.abs() < SCALAR_EPSILON THEN
        RETURN 0.0  // Division by zero → zero
    END IF
    RETURN num / den
END PROCEDURE

ALGORITHM: SafeSqrt
INPUT: value
OUTPUT: Result or error

PROCEDURE SafeSqrt(value: Scalar) -> Scalar:
    IF value < 0.0 THEN
        RETURN 0.0  // Negative → zero
    END IF
    RETURN sqrt(value)
END PROCEDURE

ALGORITHM: SafeExp
INPUT: value
OUTPUT: Clamped result

PROCEDURE SafeExp(value: Scalar) -> Scalar:
    IF value > 88.0 THEN  // ln(f32::MAX) ≈ 88.7
        RETURN Scalar::MAX
    END IF
    RETURN exp(value)
END PROCEDURE
```

---

## 35. Complexity & Performance

### 35.1 Algorithm Complexity Table

| Algorithm | Time Complexity | Space Complexity |
|---|---|---|
| Language encoding | O(n × V) | O(n) |
| Neural processing | O(C × D) | O(C) |
| Column competition | O(C log C) | O(C) |
| Memory retrieval | O(M × R) | O(R) |
| Relevance scoring | O(M) | O(1) |
| World integration | O(E + R) | O(E + R) |
| Hypothesis generation | O(H) | O(H) |
| Hypothesis evaluation | O(H × E) | O(E) |
| Contradiction detection | O(H²) | O(H²) |
| Plan simulation | O(B × D × S) | O(D) |
| Risk evaluation | O(F) | O(F) |
| Verification | O(E) | O(1) |
| Learning application | O(1) | O(1) |
| Replay | O(R × C) | O(R) |
| Consolidation | O(C) | O(C) |
| State serialization | O(S) | O(S) |
| State deserialization | O(S) | O(S) |
| Checksum computation | O(S) | O(1) |

Where:
- n = input tokens
- V = vocabulary size
- C = cell count
- D = dimension
- M = memory size
- R = results/relevance computations
- E = entities/evidence
- H = hypotheses
- B = branches
- S = state size/steps
- F = risk factors

### 35.2 Performance Targets

| Operation | Target | Constraint |
|---|---|---|
| Language encoding | < 10ms per 1000 tokens | CPU-bound |
| Neural processing | < 50ms per cycle | CPU-bound |
| Memory retrieval | < 20ms per query | Memory-bound |
| Reasoning (per step) | < 5ms | CPU-bound |
| Planning (per branch) | < 10ms | CPU-bound |
| Verification | < 5ms per claim | CPU-bound |
| Learning application | < 1ms per signal | CPU-bound |
| State serialization | < 100ms per 10MB | I/O-bound |
| State deserialization | < 100ms per 10MB | I/O-bound |

---

## 36. Algorithm Invariants

### 36.1 Cognitive Pipeline Invariants

| # | Invariant | Enforcement |
|---|---|---|
| ALG-001 | Pipeline steps execute in order | Sequential execution |
| ALG-002 | Each step receives valid input from previous step | Type system |
| ALG-003 | Budget is checked before each bounded operation | Budget enforcement |
| ALG-004 | Policy is checked before each consequential operation | Policy gate |
| ALG-005 | State is consistent between pipeline steps | Single-threaded execution |
| ALG-006 | Error in any step propagates to runtime | Result type |
| ALG-007 | Disabled subsystems produce defined defaults | Configuration check |

### 36.2 Learning Invariants

| # | Invariant | Enforcement |
|---|---|---|
| ALG-008 | Learning signal magnitude ≤ learning_rate | Clamping |
| ALG-009 | Single observation cannot change > 10% of state | Stability guard |
| ALG-010 | All learning is attributed to a subsystem | Attribution algorithm |
| ALG-011 | All learning preserves provenance | Data structure design |
| ALG-012 | Policy can disable learning entirely | Policy gate |
| ALG-013 | Consolidation requires ≥ 3 supporting episodes | Consolidation check |
| ALG-014 | Verification never upgrades UNKNOWN to VERIFIED without evidence | Verification invariant |

### 36.3 Neural Invariants

| # | Invariant | Enforcement |
|---|---|---|
| ALG-015 | Active cells ≤ field_size × sparsity_ratio | Sparsity enforcement |
| ALG-016 | Cell activation ∈ [0.0, 1.0] | Clamping |
| ALG-017 | Plasticity update bounded by plasticity parameter | Bounded update |
| ALG-018 | No NaN in cell state | Numerical guard |

### 36.4 Memory Invariants

| # | Invariant | Enforcement |
|---|---|---|
| ALG-019 | Memory usage ≤ configured budget | Capacity check |
| ALG-020 | All memories have provenance | Data structure design |
| ALG-021 | Forgetting is controlled, not arbitrary | Forgetting policy |
| ALG-022 | Retrieval preserves confidence | Data preservation |

### 36.5 Verification Invariants

| # | Invariant | Enforcement |
|---|---|---|
| ALG-023 | Verification status transitions are valid | State machine |
| ALG-024 | UNKNOWN → VERIFIED requires evidence threshold | Verification rules |
| ALG-025 | Contradictions are preserved, not silently dropped | Contradiction handling |
| ALG-026 | Confidence and verification status are separate | Data structure design |

---

## 37. Failure Modes

### 37.1 Algorithm Failure Modes

| Algorithm | Failure Mode | Detection | Recovery |
|---|---|---|---|
| Language encoding | Invalid UTF-8 | Input validation | Reject with InputError |
| Language encoding | Token overflow | Token count check | Truncate |
| Neural processing | All cells inhibited | Active cell count check | Reset field |
| Neural processing | Activation overflow | Numerical guard | Clamp |
| Memory retrieval | No results | Empty result check | Return empty retrieval |
| Memory storage | Capacity exceeded | Capacity check | Evict |
| World integration | Entity conflict | Identity check | Merge or create new |
| Reasoning | Budget exhausted | Budget check | Return bounded result |
| Reasoning | No hypotheses | Empty hypothesis check | Return uncertain result |
| Planning | No valid plans | Empty plan check | Return None |
| Planning | Depth exceeded | Depth check | Truncate plan |
| Verification | No evidence | Empty evidence check | Return Unknown status |
| Learning | Catastrophic update | Stability guard | Reject or reduce |
| Learning | Policy denied | Policy check | Skip learning |
| Persistence | Disk full | I/O error | Return error |
| Persistence | Corruption | Checksum check | Recover from checkpoint |
| Internet | Timeout | Timeout check | Return NetworkError |
| Internet | Response too large | Size check | Truncate or reject |
| Policy | Ambiguous decision | Policy evaluation | Default to DENY |

### 37.2 Failure Severity Classification

| Severity | Examples | Response |
|---|---|---|
| Recoverable | Network timeout, empty retrieval | Log, continue |
| Degraded | Budget exhausted, capacity exceeded | Bounded result, continue |
| State corruption | Checksum mismatch, invalid state | Recover from checkpoint |
| Fatal | Configuration error, disk full | Safe stop |
| Policy violation | Denied operation | Skip operation, log |

---

## 38. Pseudocode

### 38.1 Complete Cognitive Loop Pseudocode

```
FUNCTION CortexProcess(input: Input) -> Response:
    // PHASE 1: PERCEPTION
    observation ← ParseObservation(input)
    IF observation.is_error() THEN RETURN observation.error()

    // PHASE 2: CONTEXT
    context ← ConstructContext(working_memory, observation)

    // PHASE 3: LANGUAGE ENCODING
    IF config.language.enabled THEN
        language_state ← LanguageEncode(observation.text, context)
    ELSE
        language_state ← LanguageState::raw(observation)
    END IF

    // PHASE 4: NEURAL PROCESSING
    representation ← NeuralProcess(language_state, context)

    // PHASE 5: MEMORY RETRIEVAL
    query ← MemoryQuery::from_representation(representation)
    memories ← MemoryRetrieve(query, context)

    // PHASE 6: WORLD INTEGRATION
    IF config.world.enabled THEN
        world_state ← WorldIntegrate(representation, memories)
    ELSE
        world_state ← WorldState::empty()
    END IF

    // PHASE 7: REASONING
    IF config.reasoning.enabled THEN
        reasoning_result ← ReasoningEvaluate(representation, memories, world_state)
    ELSE
        reasoning_result ← ReasoningResult::from_memory(memories)
    END IF

    // PHASE 8: PLANNING
    IF config.planning.enabled THEN
        plan ← PlanningEvaluate(reasoning_result, world_state)
    ELSE
        plan ← None
    END IF

    // PHASE 9: VERIFICATION
    IF config.verification.enabled THEN
        verified ← VerificationEvaluate(reasoning_result)
    ELSE
        verified ← VerifiedResult::provisional(reasoning_result)
    END IF

    // PHASE 10: GENERATION
    response ← LanguageGenerate(verified)

    // PHASE 11: LEARNING
    IF config.learning.enabled THEN
        experience ← Experience::new(observation, response, world_state, reasoning_result)
        signal ← LearningRecord(experience)
        ApplyLearningSignal(signal, policy.state())
    END IF

    // PHASE 12: PERSISTENCE
    MaybeCheckpoint(state, config.persistence.checkpoint_interval)

    RETURN response
END FUNCTION
```

### 38.2 Learning Cycle Pseudocode

```
FUNCTION LearningCycle(experience: Experience):
    // 1. Generate learning signal
    signal ← GenerateLearningSignal(experience)
    IF signal.is_none() THEN RETURN

    // 2. Attribute error
    attribution ← AttributeError(signal.prediction_error)

    // 3. Stability guard
    decision ← LearningStabilityGuard(signal, state)
    IF decision.is_rejected() THEN
        log_warning("Learning rejected: {}", decision.reason())
        RETURN
    END IF
    IF decision.is_modified() THEN
        signal ← decision.modified_signal()
    END IF

    // 4. Policy check
    policy_decision ← policy.evaluate(ProposedOperation::learning())
    IF policy_decision.is_denied() THEN RETURN

    // 5. Apply learning to appropriate subsystem
    MATCH attribution:
        InputError => language.update(signal)
        MemoryError => memory.update(signal)
        WorldError => world.update(signal)
        ReasoningError => reasoning.update(signal)
        ProcedureError => procedural.update(signal)
        EnvironmentError => ()  // No specific update
    END MATCH

    // 6. Update learning statistics
    learning_state.total_learning_events += 1
    learning_state.update_average_error(signal.magnitude)

    // 7. Check if consolidation needed
    IF learning_state.total_learning_events >= learning_state.next_consolidation_at THEN
        candidates ← learning.consolidation_candidates()
        result ← Consolidate(candidates, policy.state())
        learning_state.next_consolidation_at += config.learning.consolidation_interval
    END IF

    // 8. Check if replay needed
    IF config.learning.replay AND should_replay() THEN
        episodes ← select_replay_episodes()
        budget ← ComputeBudget::for_replay()
        replay_result ← ExecuteReplay(episodes, budget)
    END IF
END FUNCTION
```

### 38.3 Verification Pipeline Pseudocode

```
FUNCTION VerificationPipeline(reasoning_result: ReasoningResult) -> VerifiedResult:
    IF NOT config.verification.enabled THEN
        RETURN VerifiedResult::provisional(reasoning_result)
    END IF

    // 1. Extract primary claim
    claim ← reasoning_result.primary_claim()
    IF claim.is_none() THEN
        RETURN VerifiedResult::unknown()
    END IF

    // 2. Gather evidence
    evidence ← reasoning_result.evidence_set()

    // 3. Evaluate sources
    source_quality ← EvaluateSources(evidence)

    // 4. Check consistency
    consistency ← CheckConsistency(claim, evidence)

    // 5. Count independent sources
    independent_count ← CountIndependentSources(evidence)

    // 6. Find contradictions
    contradictions ← FindContradictions(claim, evidence)

    // 7. Determine status
    status ← DetermineStatus(
        claim, evidence, source_quality, consistency,
        independent_count, contradictions
    )

    // 8. Enforce invariant: no silent UNKNOWN → VERIFIED
    status ← EnforceVerificationInvariant(
        VerificationStatus::Unknown, status, evidence
    )

    // 9. Compute confidence
    confidence ← ConfidenceState {
        belief: claim.confidence,
        evidence_strength: evidence.total_strength(),
        source_quality,
        consistency,
        uncertainty: 1.0 - consistency,
        prediction_reliability: 0.0,
        verification_status: status,
    }

    RETURN VerifiedResult {
        claim,
        verification_status: status,
        confidence,
        evidence,
    }
END FUNCTION
```

---

### 39.1 Complete Parameter Table

| Parameter | Type | Default | Range | Source | Used By |
|---|---|---|---|---|---|
| `learning_rate` | Scalar | 0.001 | (0, 1] | Config | Plasticity, learning |
| `plasticity` | Scalar | 0.01 | [0, 1] | Config | Plasticity bound |
| `sparsity_ratio` | Scalar | 0.05 | (0, 1] | Config | Neural sparsity |
| `max_reasoning_steps` | u32 | 32 | ≥ 1 | Config | Reasoning budget |
| `max_planning_depth` | u32 | 8 | ≥ 1 | Config | Planning budget |
| `max_planning_branches` | u32 | 16 | ≥ 1 | Config | Planning budget |
| `max_generation_length` | u32 | 1024 | ≥ 32 | Config | Generation limit |
| `context_window` | u32 | 4096 | ≥ 64 | Config | Language context |
| `vocabulary_capacity` | u32 | 65536 | ≥ 256 | Config | Vocabulary limit |
| `prediction_horizon` | u32 | 8 | ≥ 1 | Config | World prediction |
| `minimum_confidence` | Scalar | 0.80 | [0, 1] | Config | Verification threshold |
| `consolidation_interval` | u64 | 1000 | ≥ 1 | Config | Consolidation trigger |
| `checkpoint_interval` | u64 | 1000 | ≥ 1 | Config | Checkpoint trigger |
| `timeout_seconds` | u32 | 15 | ≥ 1 | Config | Internet timeout |
| `max_response_mb` | u32 | 4 | ≥ 1 | Config | Internet response limit |
| `SCALAR_EPSILON` | Scalar | 1e-6 | Fixed | Constant | Floating-point comparison |
| `TEMPORAL_HALF_LIFE` | Scalar | 24.0 | Fixed | Constant | Temporal decay (hours) |
| `MAX_HYPOTHESES` | u32 | 10 | Fixed | Constant | Hypothesis generation limit |
| `MAX_CANDIDATES` | u32 | 10 | Fixed | Constant | Language prediction candidates |
| `MIN_CONSOLIDATION_EPISODES` | u32 | 3 | Fixed | Constant | Min episodes for consolidation |
| `CATASTROPHIC_CHANGE_THRESHOLD` | Scalar | 0.10 | Fixed | Constant | Max state change per update |
| `SINGLE_OBS_DISCOUNT` | Scalar | 0.3 | Fixed | Constant | Discount for single-observation updates |
| `REPLAY_PRIORITY_WEIGHTS` | [Scalar; 5] | [0.40, 0.20, 0.20, 0.10, 0.10] | Fixed | Constant | Replay priority factors |
| `FORGET_SCORE_THRESHOLD` | Scalar | 0.7 | Fixed | Constant | Forgetting trigger |
| `MOVING_AVG_ALPHA` | Scalar | 0.1 | Fixed | Constant | Self-model smoothing |
| `RELEVANCE_WEIGHTS` | [Scalar; 6] | [0.30, 0.20, 0.15, 0.15, 0.10, 0.10] | Fixed | Constant | Memory relevance factors |
| `CONFIDENCE_WEIGHTS` | [Scalar; 5] | [0.30, 0.25, 0.15, 0.20, 0.10] | Fixed | Constant | Confidence aggregation |
| `RISK_WEIGHTS` | [Scalar; 5] | [0.30, 0.25, 0.20, 0.15, 0.10] | Fixed | Constant | Risk estimation factors |
| `INTENT_CONFIDENCE` | HashMap | See §8.3 | Fixed | Constant | Intent detection confidence |
| `PREDICTION_UNCERTAINTY_GROWTH` | Scalar | 0.15 | Fixed | Constant | Uncertainty growth per prediction |
| `VERIFICATION_UNCERTAINTY_REDUCTION` | Scalar | 0.20 | Fixed | Constant | Uncertainty reduction per verification |
| `LEARNING_UNCERTAINTY_REDUCTION` | Scalar | 0.05 | Fixed | Constant | Uncertainty reduction per learning |
| `ZSTD_COMPRESSION_LEVEL` | u32 | 3 | Fixed | Constant | .cx compression |
| `MAX_DIAGNOSTIC_ERRORS` | usize | 100 | Fixed | Constant | Diagnostic buffer size |
| `MAX_LEARNING_HISTORY` | usize | 1000 | Fixed | Constant | Learning history buffer |
| `MAX_SELF_MODEL_HISTORY` | usize | 100 | Fixed | Constant | Self-model history buffer |

### 39.2 Parameter Interaction Rules

| Interaction | Rule |
|---|---|
| `learning_rate` × `plasticity` | Effective update bound = `learning_rate × plasticity`; both constrain updates |
| `sparsity_ratio` × `cells` | Max active cells = `cells × sparsity_ratio`; determines neural capacity |
| `max_reasoning_steps` × `max_planning_depth` | Total cognitive budget; both bounded independently |
| `minimum_confidence` × `evidence_strength` | Verification requires BOTH threshold AND evidence conditions |
| `consolidation_interval` × `replay` | Replay budget derived: `max(1, consolidation_interval / 10)` |
| `context_window` × `generation_limit` | Total language budget; input + output bounded by context |
| `vocabulary_capacity` × `learning` | Vocabulary growth bounded by capacity regardless of learning |

### 39.3 Parameter Validation Rules

| Parameter | Validation | Error on Violation |
|---|---|---|
| All Scalar params | `is_finite()`, within documented range | ConfigError |
| All u32 params | ≥ minimum value | ConfigError |
| All bool params | Valid TOML boolean | ConfigError |
| `sparsity_ratio` | (0, 1] | ConfigError |
| `learning_rate` | (0, 1] | ConfigError |
| `plasticity` | [0, 1] | ConfigError |
| `minimum_confidence` | [0, 1] | ConfigError |
| Cross-field | `cells ≥ columns` | ConfigError |
| Cross-field | `context_window ≥ generation_limit` | ConfigError |

---

## 40. Open Technical Parameters

| Parameter | Current Design | Open Question | Resolution Path |
|---|---|---|---|
| Scalar precision | f32 | Should f16/bf16 be supported for neural computation? | Benchmark on target hardware; evaluate accuracy loss |
| Sparsity enforcement | Top-k selection | Should we use k-WTA (Winner-Take-All) circuits? | Neural representation quality experiments |
| Temporal buffer size | Last 3 states | Optimal history depth for temporal encoding? | Sequence learning benchmarks |
| Prediction horizon | 8 steps | Sufficient for target planning complexity? | Planning success rate evaluation |
| Replay priority formula | 5-factor weighted | Optimal weighting for learning efficiency? | Learning curve experiments |
| Consolidation threshold | 0.6 pattern strength | Optimal threshold for knowledge stability? | Long-term stability tests |
| Forgetting policy | 6-factor composite | Optimal forgetting strategy for retention? | Retention benchmarks |
| Relevance scoring | 6-factor weighted | Optimal weighting for retrieval quality? | Retrieval precision/recall tests |
| Confidence aggregation | 5-dimension weighted | Should dimensions be learned rather than fixed? | Confidence calibration experiments |
| Risk estimation | 5-factor weighted | Should risk weights be adaptive? | Risk assessment accuracy evaluation |
| Error attribution | Dimension-based heuristic | Should attribution use learned classifier? | Attribution accuracy evaluation |
| Intent detection | Rule-based with confidence | Should intent use learned model? | Intent classification accuracy |
| Vocabulary expansion | Frequency-based | Should expansion use semantic clustering? | Vocabulary quality evaluation |
| Contradiction detection | Pairwise comparison | Scalability for large hypothesis sets? | Performance benchmarking |
| Plan simulation | Sequential step prediction | Should we use Monte Carlo sampling? | Planning accuracy vs. cost trade-off |
| Verification evidence counting | Independent source count | Definition of "independent" needs refinement? | Verification accuracy evaluation |
| Self-model update | Moving average (α=0.1) | Optimal smoothing factor? | Self-model accuracy evaluation |
| Memory eviction | Composite scoring | Should eviction be learned? | Retention vs. capacity trade-off |
| Neural prediction | Cell-level prediction vectors | Should prediction use field-level model? | Prediction accuracy evaluation |
| Language generation | Greedy with validation | Should we use beam search? | Generation quality evaluation |

These parameters are exposed in configuration or as implementation constants. They represent calibration opportunities, not architectural uncertainty.

---

## 42. Gap Resolution: Additional Algorithm Specifications

### 42.1 Consolidation Algorithm — Complete Specification

```
ALGORITHM: ConsolidateComplete
INPUT: Consolidation candidates, current CortexState, PolicyState
OUTPUT: ConsolidationResult
BOUNDS: consolidation_interval, memory budgets

PRECONDITIONS:
  - learning.enabled = true
  - Candidate list is non-empty
  - Policy allows learning operations

PROCEDURE ConsolidateComplete(
    candidates: &[ConsolidationCandidate],
    state: &CortexState,
    policy: &PolicyState
) -> ConsolidationResult:
    result ← ConsolidationResult::default()
    
    FOR each candidate IN candidates:
        // 1. Policy check
        IF NOT policy.allows_learning() THEN
            result.rejected += 1
            CONTINUE
        END IF
        
        // 2. Minimum supporting episodes check (prevent single-event dominance)
        IF candidate.supporting_episodes.len() < 3 THEN
            result.rejected += 1
            CONTINUE
        END IF
        
        // 3. Candidate evaluation
        evaluation ← evaluate_candidate(candidate)
        IF NOT evaluation.should_consolidate THEN
            result.rejected += 1
            CONTINUE
        END IF
        
        // 4. Confidence threshold check
        IF evaluation.confidence < config.learning.consolidation_threshold THEN
            result.rejected += 1
            CONTINUE
        END IF
        
        // 5. Contradiction risk check
        IF candidate.contradiction_risk > 0.5 THEN
            result.rejected += 1
            CONTINUE
        END IF
        
        // 6. Memory budget check
        IF memory.pressure() == MemoryPressure::Critical THEN
            // During critical pressure, only consolidate highest-confidence candidates
            IF evaluation.confidence < 0.8 THEN
                result.rejected += 1
                CONTINUE
            END IF
        END IF
        
        // 7. Integrate based on target type
        MATCH candidate.target:
            ConsolidationTarget::Semantic =>
                IF candidate.knowledge.is_some() THEN
                    semantic_memory.integrate(candidate.knowledge.unwrap())
                    result.semantic_integrations += 1
                END IF
            
            ConsolidationTarget::Procedural =>
                IF candidate.procedure.is_some() THEN
                    procedural_memory.integrate(candidate.procedure.unwrap())
                    result.procedural_integrations += 1
                END IF
            
            ConsolidationTarget::Associative =>
                IF candidate.association.is_some() THEN
                    associative_memory.integrate(candidate.association.unwrap())
                    result.associative_integrations += 1
                END IF
        END MATCH
        
        result.consolidated += 1
    END FOR
    
    RETURN result
END PROCEDURE

BOUNDS:
  - Maximum candidates per cycle: derived from consolidation_interval
  - Memory budget respected at all times
  - No single cycle can consolidate more than 10% of total state

INVARIANTS:
  - Consolidated items have provenance from ≥ 3 supporting episodes
  - Consolidated items have confidence ≥ consolidation_threshold
  - Memory budgets are not exceeded after consolidation
  - Forgetting may be triggered to make room for consolidated items

FAILURE/RECOVERY:
  - If integration fails → item is skipped, not partially integrated
  - If memory budget exceeded → trigger forgetting before retrying
  - If policy denied → skip and log

PROVENANCE:
  - Consolidated items inherit provenance from supporting episodes
  - Evidence is merged from all supporting episodes
  - New provenance category: Derived (with parent episode references)

TEST REQUIREMENTS:
  - Test: consolidation with < 3 supporting episodes → rejected
  - Test: consolidation with contradiction_risk > 0.5 → rejected
  - Test: consolidation respects memory budgets
  - Test: consolidated items have correct provenance
  - Test: consolidation during critical pressure → high-confidence only
```

### 42.2 Forgetting Algorithm — Complete Specification

```
ALGORITHM: ApplyForgettingComplete
INPUT: ForgettingPolicy, CortexState
OUTPUT: ForgettingResult
BOUNDS: Memory budgets

PRECONDITIONS:
  - Memory pressure ≥ Moderate (or explicit request)
  - Policy allows learning operations

PROCEDURE ApplyForgettingComplete(
    policy: &ForgettingPolicy,
    state: &mut CortexState
) -> ForgettingResult:
    result ← ForgettingResult::default()
    
    // 1. Score all episodic memories for forgetting
    FOR each episode IN state.memory.episodic.episodes:
        forget_score ← compute_forget_score(episode, policy)
        IF forget_score > FORGET_SCORE_THRESHOLD THEN
            state.memory.episodic.remove(episode.id)
            result.episodic_forgotten += 1
            result.bytes_freed += estimate_size(episode)
        END IF
    END FOR
    
    // 2. Score semantic memories for forgetting (lower aggression)
    FOR each knowledge IN state.memory.semantic.knowledge:
        forget_score ← compute_knowledge_forget_score(knowledge, policy)
        // Higher threshold for semantic (more valuable)
        IF forget_score > (FORGET_SCORE_THRESHOLD + 0.1) THEN
            state.memory.semantic.remove(knowledge.id)
            result.semantic_forgotten += 1
            result.bytes_freed += estimate_size(knowledge)
        END IF
    END FOR
    
    // 3. Score associative memories for forgetting
    FOR each association IN state.memory.associative.associations:
        forget_score ← compute_association_forget_score(association, policy)
        IF forget_score > FORGET_SCORE_THRESHOLD THEN
            state.memory.associative.remove(association.id)
            result.associative_forgotten += 1
            result.bytes_freed += estimate_size(association)
        END IF
    END FOR
    
    // 4. Update association index after removals
    state.memory.associative.rebuild_index()
    
    RETURN result
END PROCEDURE

PROCEDURE ComputeForgetScore(episode: &Episode, policy: &ForgettingPolicy) -> Scalar:
    score ← 0.0
    
    // Low importance
    IF episode.importance < policy.min_importance THEN
        score ← score + 0.2
    END IF
    
    // Low confidence
    IF episode.confidence.overall() < policy.min_confidence THEN
        score ← score + 0.2
    END IF
    
    // Age (if max_age configured)
    IF policy.max_age.is_some() THEN
        age ← Timestamp::now().elapsed_since(episode.timestamp)
        IF age > policy.max_age.unwrap() THEN
            score ← score + 0.2
        END IF
    END IF
    
    // Low retrieval frequency
    IF episode.retrieval_count < policy.min_retrieval_count THEN
        score ← score + 0.2
    END IF
    
    // Redundancy (consolidated)
    IF episode.consolidated THEN
        score ← score + 0.1
    END IF
    
    // Contradiction
    IF episode.contradicted() THEN
        score ← score + 0.1
    END IF
    
    RETURN score.clamp(0.0, 1.0)
END PROCEDURE

BOUNDS:
  - Forgetting is bounded by memory budgets
  - No single forgetting cycle can remove > 20% of items
  - Semantic memories have higher retention threshold than episodic

INVARIANTS:
  - Forgotten items are permanently removed from their subsystem
  - Association index is rebuilt after removals
  - Provenance of consolidated items is preserved (they exist in semantic/procedural)
  - Forgetting does not remove items that are currently referenced by working memory

FAILURE/RECOVERY:
  - If removal fails → skip item, continue with next
  - If index rebuild fails → log error, continue (index is recomputed on next access)

PROVENANCE:
  - Forgotten items are logged with reason (low importance, low confidence, age, etc.)
  - Forgetting event is recorded in learning history

TEST REQUIREMENTS:
  - Test: forgetting respects importance threshold
  - Test: forgetting respects confidence threshold
  - Test: forgetting respects age threshold
  - Test: forgetting respects retrieval frequency threshold
  - Test: consolidated items have higher retention
  - Test: semantic memories have higher retention than episodic
  - Test: no more than 20% of items removed per cycle
  - Test: association index is valid after forgetting
```

### 42.3 Selective Learning Gate Algorithm — Complete Specification

```
ALGORITHM: SelectiveLearningGate
INPUT: LearningSignal, CortexState, PolicyState, MemoryPressure
OUTPUT: Approved/Modified/Rejected signal
BOUNDS: learning_rate, plasticity

PRECONDITIONS:
  - learning.enabled = true
  - Signal has non-zero magnitude

PROCEDURE SelectiveLearningGate(
    signal: &LearningSignal,
    state: &CortexState,
    policy: &PolicyState,
    pressure: MemoryPressure
) -> GateDecision:
    
    // 1. Noise filtering: discard signals below threshold
    IF signal.magnitude < (config.learning.learning_rate * 0.01) THEN
        RETURN GateDecision::Rejected("Signal below noise threshold")
    END IF
    
    // 2. Policy check
    IF NOT policy.allows_learning() THEN
        RETURN GateDecision::Rejected("Learning disabled by policy")
    END IF
    
    // 3. Single-observation guard
    IF signal.source_episodes.len() == 1 AND signal.magnitude > 0.5 THEN
        // Discount single-observation high-magnitude signals
        modified_signal ← signal.clone()
        modified_signal.magnitude ← signal.magnitude * SINGLE_OBS_DISCOUNT
        RETURN GateDecision::Modified(modified_signal, "Single-observation discount applied")
    END IF
    
    // 4. Stability guard: check for catastrophic change
    IF signal.would_affect_percentage(state) > CATASTROPHIC_CHANGE_THRESHOLD THEN
        RETURN GateDecision::Rejected("Catastrophic change prevented")
    END IF
    
    // 5. Memory pressure throttling
    IF pressure == MemoryPressure::Critical THEN
        // Only allow high-priority signals during critical pressure
        IF signal.magnitude < config.learning.learning_rate THEN
            RETURN GateDecision::Rejected("Throttled during critical memory pressure")
        END IF
    END IF
    
    // 6. Bounded update
    bounded_magnitude ← signal.magnitude.min(config.learning.learning_rate)
    bounded_signal ← signal.clone()
    bounded_signal.magnitude ← bounded_magnitude
    
    RETURN GateDecision::Approved(bounded_signal)
END PROCEDURE

BOUNDS:
  - Signal magnitude ≤ learning_rate after gating
  - Single-observation signals discounted by 0.3
  - No state change > 10% per update
  - Critical pressure throttles to high-priority only

INVARIANTS:
  - All learning signals pass through the gate
  - Rejected signals are logged but do not affect state
  - Modified signals carry the modification reason
  - Gate decisions are deterministic for same inputs

FAILURE/RECOVERY:
  - Gate failure → signal rejected (fail-safe)
  - Gate timeout → signal rejected (fail-safe)

PROVENANCE:
  - Gate decisions are recorded in learning history
  - Rejection reasons are logged for diagnostics

TEST REQUIREMENTS:
  - Test: noise filtering discards low-magnitude signals
  - Test: single-observation discount applied correctly
  - Test: catastrophic change prevention works
  - Test: critical pressure throttling works
  - Test: bounded update respects learning_rate
  - Test: gate is deterministic
  - Test: rejected signals do not affect state
```

### 42.4 World-State Inference Algorithm — Complete Specification

```
ALGORITHM: InferWorldState
INPUT: NeuralRepresentation, MemoryRetrieval, CurrentWorldState
OUTPUT: InferredWorldState with uncertainty
BOUNDS: max_inference_steps (derived from reasoning.max_steps)

PRECONDITIONS:
  - world.enabled = true (if disabled, return empty state)
  - Neural representation is available

PROCEDURE InferWorldState(
    repr: &NeuralRepresentation,
    memories: &MemoryRetrieval,
    current_world: &WorldState
) -> InferredWorldState:
    
    // 1. Start with current world state
    inferred ← current_world.clone()
    
    // 2. Extract new entities from representation
    new_entities ← extract_entities(repr)
    FOR each entity IN new_entities:
        IF NOT inferred.has_entity(entity.identity) THEN
            // New entity: add with inferred confidence (lower than observed)
            entity.confidence ← entity.confidence * 0.7  // Inference discount
            inferred.add_entity(entity)
        ELSE
            // Existing entity: update with weighted merge
            existing ← inferred.get_entity(entity.identity)
            weight ← 0.5  // Inference weight (lower than direct observation)
            existing.update_weighted(entity, weight)
        END IF
    END FOR
    
    // 3. Extract new relations from representation
    new_relations ← extract_relations(repr)
    FOR each relation IN new_relations:
        IF NOT inferred.has_relation(relation) THEN
            relation.confidence ← relation.confidence * 0.7
            inferred.add_relation(relation)
        END IF
    END FOR
    
    // 4. Integrate semantic memory context
    FOR each knowledge IN memories.semantic:
        inferred.integrate_knowledge(knowledge)
    END FOR
    
    // 5. Compute inference uncertainty
    // Inferred state always has higher uncertainty than observed state
    inference_uncertainty ← 1.0 - (repr.confidence.overall() * 0.7)
    inferred.uncertainty.level ← max(inferred.uncertainty.level, inference_uncertainty)
    
    // 6. Mark as inferred (not observed)
    inferred.source ← WorldStateSource::Inferred
    
    RETURN InferredWorldState {
        state: inferred,
        confidence: repr.confidence.overall() * 0.7,  // Inference discount
        uncertainty: inference_uncertainty,
        source: WorldStateSource::Inferred,
    }
END PROCEDURE

BOUNDS:
  - Inference bounded by max_inference_steps
  - Inferred confidence always ≤ 0.7 × observed confidence
  - Uncertainty always ≥ 1 - (observed_confidence × 0.7)

INVARIANTS:
  - Inferred state is never treated as ground truth
  - Inferred state carries explicit uncertainty
  - Inferred state provenance is Inferred, not Observed
  - Direct observation always overrides inference

FAILURE/RECOVERY:
  - Inference failure → return current world state unchanged
  - Entity conflict → merge with lower confidence

PROVENANCE:
  - Inferred entities carry Inferred provenance
  - Inherited provenance from source memories

TEST REQUIREMENTS:
  - Test: inferred confidence < observed confidence
  - Test: inferred state carries uncertainty
  - Test: direct observation overrides inference
  - Test: inference is bounded by max_inference_steps
  - Test: inferred state provenance is Inferred
```

---

## 43. Algorithm Completeness

### 43.1 Completeness Checklist

| Algorithm Category | Status | Coverage |
|---|---|---|
| Main cognitive pipeline | ✅ Complete | 12-phase pipeline with pseudocode |
| Input processing | ✅ Complete | Parse, validate, construct observation |
| Context construction | ✅ Complete | 7-level hierarchical context assembly |
| Observation processing | ✅ Complete | Policy check, episode creation, state update |
| Language encoding | ✅ Complete | 10-step encoding pipeline |
| Vocabulary management | ✅ Complete | Lookup, create, expand, frequency tracking |
| Intent detection | ✅ Complete | Multi-cue hypothesis-based detection |
| Language generation | ✅ Complete | 8-step generation pipeline |
| Language prediction | ✅ Complete | Multi-factor scoring |
| Neural processing | ✅ Complete | Cell, column, field, temporal, prediction |
| Cell computation | ✅ Complete | Receive, activate, inhibit, predict, adapt |
| Column competition | ✅ Complete | Top-k sparse selection |
| Temporal encoding | ✅ Complete | Sequence, transition, recurrence |
| Neural prediction | ✅ Complete | State-based next-state prediction |
| Prediction error | ✅ Complete | Euclidean distance computation |
| Memory retrieval | ✅ Complete | Multi-type retrieval with relevance scoring |
| Relevance scoring | ✅ Complete | 6-factor weighted scoring |
| Temporal relevance | ✅ Complete | Exponential decay with half-life |
| Memory storage | ✅ Complete | Episode storage with capacity management |
| Memory eviction | ✅ Complete | Value-based eviction |
| World integration | ✅ Complete | Entity/relation extraction and integration |
| State transition prediction | ✅ Complete | S(t)+A(t)→S(t+1) |
| State estimation | ✅ Complete | Multi-observation weighted estimation |
| Inference | ✅ Complete | Gap identification and inference |
| Reasoning evaluation | ✅ Complete | Hypothesis-based with budget |
| Hypothesis generation | ✅ Complete | Memory, world, analogical sources |
| Hypothesis evaluation | ✅ Complete | Evidence gathering, quality, consistency |
| Contradiction detection | ✅ Complete | Pairwise proposition comparison |
| Hypothesis ranking | ✅ Complete | Score-based with contradiction penalty |
| Prediction | ✅ Complete | Neural + world model combination |
| Prediction comparison | ✅ Complete | Error computation and resolution |
| Planning evaluation | ✅ Complete | Goal extraction, simulation, risk, ranking |
| Plan simulation | ✅ Complete | Sequential state prediction |
| Risk evaluation | ✅ Complete | 4-factor risk assessment |
| Action selection | ✅ Complete | Plan-based with policy gate |
| Verification evaluation | ✅ Complete | Claim extraction and verification |
| Claim verification | ✅ Complete | 5-step verification pipeline |
| Verification status determination | ✅ Complete | 6-rule status hierarchy |
| Verification invariant enforcement | ✅ Complete | No silent UNKNOWN→VERIFIED |
| Policy evaluation | ✅ Complete | Classification, risk, decision |
| Risk estimation | ✅ Complete | 5-factor risk scoring |
| Confidence aggregation | ✅ Complete | Weighted multi-source aggregation |
| Overall confidence | ✅ Complete | 5-dimension weighted score |
| Uncertainty propagation | ✅ Complete | Operation-specific uncertainty growth |
| Uncertainty reduction | ✅ Complete | Evidence-based reduction with actions |
| Learning signal generation | ✅ Complete | Experience-based signal |
| Error attribution | ✅ Complete | 6-source attribution |
| Learning application | ✅ Complete | Policy check, routing, statistics |
| Learning stability guard | ✅ Complete | 4-check stability enforcement |
| Plasticity update | ✅ Complete | ΔW = η × A × C × E × V with bounds |
| Cell adaptation | ✅ Complete | Bounded activation update |
| Replay priority | ✅ Complete | 5-factor priority scoring |
| Replay execution | ✅ Complete | Context reconstruction, prediction, learning |
| Consolidation | ✅ Complete | 7-step consolidation with checks |
| Candidate evaluation | ✅ Complete | Pattern, evidence, contradiction evaluation |
| Self-model update | ✅ Complete | 8-component update with moving average |
| Moving average | ✅ Complete | Exponential smoothing |
| Internet fetch | ✅ Complete | Policy, risk, timeout, size enforcement |
| Internet content processing | ✅ Complete | Parse, provenance, observation creation |
| Error recovery | ✅ Complete | Severity classification, strategy selection |
| Checkpoint recovery | ✅ Complete | Sequential checkpoint attempt |
| Memory pressure response | ✅ Complete | 4-level pressure response |
| Forgetting policy | ✅ Complete | Multi-factor forget scoring |
| Compute budget enforcement | ✅ Complete | Per-operation budget check |
| Atomic save | ✅ Complete | Temp→flush→verify→replace |
| State load | ✅ Complete | 9-step load with validation |
| Checkpoint creation | ✅ Complete | Serialize, write, metadata, cleanup |
| State transition | ✅ Complete | State machine with valid transitions |

### 43.2 Traceability to Requirements

| DOC-01 Requirement | DOC-04 Algorithm Coverage |
|---|---|
| FR-LANG-001 through FR-LANG-015 | §8 Language Processing (encode, decode, predict, generate) |
| FR-NEUR-001 through FR-NEUR-009 | §9 Neural Processing (process, predict, error, adapt) |
| FR-MEM-001 through FR-MEM-011 | §10-11 Memory Retrieval & Update |
| FR-WRLD-001 through FR-WRLD-007 | §12 World Model Update |
| FR-RSN-001 through FR-RSN-006 | §15 Reasoning (evaluate, generate, detect, rank) |
| FR-PLN-001 through FR-PLN-004 | §17 Planning (evaluate, simulate, risk) |
| FR-VER-001 through FR-VER-006 | §19 Verification (evaluate, verify, determine, enforce) |
| FR-LRN-001 through FR-LRN-009 | §23-26 Learning, Plasticity, Replay, Consolidation |
| FR-SLF-001 through FR-SLF-004 | §27 Self-Model Update |
| FR-POL-001 through FR-POL-006 | §20 Policy Evaluation |
| FR-INT-001 through FR-INT-005 | §28 Internet Interaction |
| FR-PRS-001 through FR-PRS-006 | §31 Persistence Algorithms |
| FR-API-001 through FR-API-004 | §4 Main Pipeline (API triggers pipeline) |
| FR-CLI-001 | §4 Main Pipeline (CLI triggers pipeline) |
| REL-001 through REL-006 | §29 Error Recovery, §31 Persistence |
| SEC-001 through SEC-008 | §20 Policy Evaluation, §22 Action Selection |
| ERR-001 through ERR-007 | §29 Error Recovery Algorithms |
| AC-* | §35 Complexity, §36 Invariants, §37 Failure Modes |

### 43.3 Algorithm-to-Subsystem Mapping

| Subsystem | Algorithms | Count |
|---|---|---|
| Language Core | Encode, VocabularyLookup, IntentDetect, Generate, Predict | 5 |
| Neural Core | Process, ColumnCompete, TemporalEncode, Predict, ComputeError | 5 |
| Memory System | Retrieve, ScoreRelevance, TemporalRelevance, Store, Evict | 5 |
| World Model | Integrate, PredictTransition, EstimateState | 3 |
| Reasoning Engine | Evaluate, GenerateHypotheses, EvaluateHypothesis, DetectContradictions, RankHypotheses | 5 |
| Planning Engine | Evaluate, SimulatePlan, EvaluateRisk, SelectAction | 4 |
| Verification Engine | Evaluate, VerifyClaim, DetermineStatus, EnforceInvariant | 4 |
| Learning System | GenerateSignal, AttributeError, ApplySignal, StabilityGuard | 4 |
| Plasticity | PlasticityUpdate, CellAdapt | 2 |
| Replay | ComputePriority, ExecuteReplay | 2 |
| Consolidation | Consolidate, EvaluateCandidate | 2 |
| Self Model | UpdateSelfModel, MovingAverage | 2 |
| Policy | PolicyEvaluate, RiskEstimate | 2 |
| Internet | Fetch, ProcessContent | 2 |
| Error Recovery | RecoverFromError, AttemptCheckpointRecovery | 2 |
| Resource Management | HandleMemoryPressure, ApplyForgettingPolicy, EnforceComputeBudget | 3 |
| Persistence | AtomicSave, LoadState, CreateCheckpoint | 3 |
| State Transition | TransitionState | 1 |
| Numerical | NumericalGuard, SafeDivide, SafeSqrt, SafeExp | 4 |
| **Total** | | **58** |

### 43.4 Final Algorithm Contract Statement

> **This document constitutes the computational behavior contract for CORTEX.** It defines every algorithm, every formula, every decision procedure, and every execution semantic that governs CORTEX's cognitive operations.
>
> The algorithm contract ensures:
> - **Bounded execution**: Every algorithm has explicit termination conditions and resource bounds.
> - **Deterministic where practical**: Same input produces same output unless stochasticity is explicitly configured.
> - **Fail-safe**: Invalid input produces defined error, never undefined behavior.
> - **Provenance-preserving**: No algorithm strips provenance from data.
> - **Confidence-aware**: Every result carries confidence; uncertainty is propagated.
> - **Policy-respecting**: No algorithm bypasses the policy gate.
> - **Stability-guarded**: No single observation may catastrophically destabilize state.
> - **Evidence-weighted**: Decisions weight evidence by strength, quality, and recency.
> - **Contradiction-tolerant**: Conflicting information is preserved, not silently dropped.
> - **Graceful degradation**: Disabled subsystems produce defined defaults; budget exhaustion produces bounded results.
>
> **CORTEX algorithm architecture: 58 algorithms, 12 pipeline phases, 36 parameters, 36 invariants, 21 failure modes, 11 pseudocode specifications.**

---

## 44. Appendix: Algorithm Quick Reference

### 44.1 Core Formulas

| Formula | Expression | Usage |
|---|---|---|
| Prediction Error | `E = √(Σ(pᵢ - aᵢ)²)` | Learning signal |
| Plasticity Update | `ΔW = η × A × C × E × V` | Neural adaptation |
| Sparsity Bound | `active ≤ field_size × sparsity_ratio` | Neural activation |
| Overall Confidence | `0.30×belief + 0.25×evidence + 0.15×quality + 0.20×consistency + 0.10×(1-uncertainty)` | Confidence aggregation |
| Temporal Decay | `exp(-0.693 × age_hours / half_life)` | Memory relevance |
| Moving Average | `current × (1-α) + new × α` | Self-model update |
| Language Score | `Lang + Context + Semantic + Memory + World + Verification - Contradiction - Risk` | Prediction scoring |
| State Transition | `C(t+1) = F(C(t), O(t), E(t), P(t))` | Cognitive evolution |

### 44.2 Algorithm Execution Order

```
1.  ParseObservation
2.  ConstructContext
3.  LanguageEncode
4.  NeuralProcess
5.  MemoryRetrieve
6.  WorldIntegrate
7.  ReasoningEvaluate
8.  PlanningEvaluate
9.  VerificationEvaluate
10. LanguageGenerate
11. LearningRecord
12. ApplyLearningSignal
13. MaybeCheckpoint
```

### 44.3 Budget Enforcement Points

| Point | Budget Checked | Action on Exhaustion |
|---|---|---|
| Before reasoning step | `max_reasoning_steps` | Return bounded conclusion |
| Before planning branch | `max_planning_branches` | Stop branching |
| Before planning depth | `max_planning_depth` | Stop deepening |
| Before simulation step | `prediction_horizon` | Stop simulating |
| Before generation token | `generation_limit` | Stop generating |
| Before memory retrieval | `max_memory_retrieval` | Limit results |
| Before replay episode | `max_replay_count` | Stop replaying |

### 44.4 Policy Gate Points

| Point | Operation | Gate Check |
|---|---|---|
| Observation ingestion | Store episode | `policy.learning` |
| Learning application | Mutate state | `policy.learning` |
| Internet fetch | Network access | `policy.internet_learning` |
| Algorithm adaptation | Modify algorithm | `policy.self_modification` |
| Policy modification | Modify policy | `policy.policy_modification` |
| Runtime modification | Modify runtime | `policy.runtime_modification` |
| Action execution | Execute plan step | Risk assessment |
| Checkpoint creation | Write to disk | Always allowed |

---

*End of Document — CORTEX-DOC-04 Algorithm Specification v1.1.0*
