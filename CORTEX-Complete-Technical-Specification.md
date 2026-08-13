# CORTEX — Complete Technical Specification

**Status:** Final Architectural Baseline  
**Role:** End-to-End System Contract  
**Version:** 1.0.0  

| Property | Value |
|---|---|
| Project Type | Native Continual-Learning AI Model |
| Implementation Language | Rust |
| Execution Model | Single Process |
| Deployment Model | Single Binary |
| Persistent Cognitive State | `.cx` |
| Configuration | `cortex.toml` |
| External AI Model | None |
| External Database | None |
| Vector Database | None |
| Agent Framework | None |
| Cognitive Substrate | Native CORTEX Algorithms |
| Language Substrate | Native CORTEX Language Core |
| Primary Target | Linux x86_64 |
| Compute Model | CPU-first |
| Learning Model | Online / Continual / State-Based |
| Autonomy Model | Policy-Bounded |

---

## 1. System Definition

CORTEX is a persistent, state-based, continually learning AI model implemented entirely as a native Rust system. CORTEX is not an orchestration layer around another AI model. The cognitive substrate, language processing, memory, world modeling, reasoning, planning, verification, learning, self-model, persistence, and policy enforcement belong to CORTEX itself.

### 1.1. Component Topology

```
CORTEX
├── Language Core (CLX)
├── Neural Core (CNS)
├── Memory System
│   ├── Working Memory
│   ├── Episodic Memory
│   ├── Semantic Memory
│   ├── Procedural Memory
│   └── Associative Memory
├── World Model
├── Reasoning Engine
├── Planning Engine
├── Verification Engine
├── Continual Learning System
├── Consolidation System
├── Self Model
├── Internet Interface
├── Policy / Risk Gate
├── Persistence Engine
├── Runtime
├── CLI
└── Embedded API
```

### 1.2. Fundamental Cognitive Transformation

```
Observation
    ↓
Language / Perceptual Encoding
    ↓
Internal Representation
    ↓
Neural Processing
    ↓
Memory Retrieval
    ↓
World-State Integration
    ↓
Prediction
    ↓
Reasoning
    ↓
Planning
    ↓
Verification
    ↓
Response / Action
    ↓
Outcome Observation
    ↓
Prediction Error
    ↓
Error Attribution
    ↓
Learning
    ↓
Consolidation
    ↓
Persistent Cognitive State
```

The persistent state is continuously transformed rather than periodically replaced by a separately trained model.

---

## 2. Architectural Principles

CORTEX adheres to the following principles:

| # | Principle |
|---|---|
| 1 | Single executable |
| 2 | Single process |
| 3 | Single configuration |
| 4 | Single persistent cognitive state |
| 5 | Native Rust implementation |
| 6 | Native cognitive algorithms |
| 7 | Native Language Core |
| 8 | No external AI model |
| 9 | No external database |
| 10 | No vector database |
| 11 | No agent framework |
| 12 | Continual learning |
| 13 | Persistent memory |
| 14 | Persistent world model |
| 15 | Persistent learned language state |
| 16 | Inspectable cognitive state |
| 17 | Replaceable algorithm implementations |
| 18 | Versioned state |
| 19 | Provenance-aware knowledge |
| 20 | Resource-bounded execution |
| 21 | Policy-bounded autonomy |
| 22 | Fail-closed security behavior |
| 23 | Deterministic infrastructure where practical |
| 24 | CPU-first execution |
| 25 | No mandatory external runtime |
| 26 | End-to-end operation after deployment |

---

## 3. Architectural Boundary

### 3.1. Internal Components (CORTEX itself)

- Language representation
- Neural representation
- Memory
- World model
- Reasoning
- Planning
- Verification
- Learning
- Consolidation
- Self-model
- Policy
- Persistence
- Runtime
- API
- CLI

### 3.2. External Infrastructure (permitted)

- Operating system
- Filesystem
- Network
- TCP/IP
- Time
- Process scheduling

### 3.3. External Libraries (permitted for infrastructure)

- Serialization
- Compression
- Cryptography
- Networking
- OS interaction

External libraries SHALL NOT constitute the cognitive substrate.

---

## 4. Single-Binary Requirement

The production artifact is the `cortex` binary. It contains the complete runtime. A deployment SHALL NOT require a separate model server, inference server, database server, vector database, embedding server, agent runtime, knowledge graph server, external reasoning engine, or external memory service.

### 4.1. Intended Deployment Structure

```
/opt/cortex/
├── cortex          # executable
├── cortex.toml     # configuration
└── cortex.cx       # persistent cognitive state (auto-created on first boot)
```

Optional:

```
/opt/cortex/
├── cortex
├── cortex.toml
├── cortex.cx
└── checkpoints/    # periodic checkpoint snapshots
```

### 4.2. Deployment Contract

A valid deployment consists of `cortex`, `cortex.toml`, and `cortex.cx` (auto-created on first boot). No external model artifact, external database, or separate service is mandatory.

---

## 5. Configuration

The canonical configuration file is `cortex.toml`. It defines how CORTEX operates. The `.cx` file defines what CORTEX has learned and remembers. This distinction is fundamental and immutable.

### 5.1. Configuration Schema

```toml
[model]
cells = 4096            # uint, minimum 256
columns = 64            # uint, minimum 16
dimension = 256         # uint, minimum 64
precision = "f32"       # "f32" | "f16" | "bf16"
sparsity_ratio = 0.05   # float, (0, 1], fraction of cells active per field

[language]
enabled = true                    # bool
vocabulary_capacity = 65536       # uint, minimum 256
context_window = 4096             # uint, minimum 64
generation_limit = 1024           # uint, minimum 32
learning = true                   # bool

[memory]
working_mb = 128      # uint, minimum 16
episodic_mb = 512     # uint, minimum 32
semantic_mb = 512     # uint, minimum 32
procedural_mb = 256   # uint, minimum 16
associative_mb = 256  # uint, minimum 16

[learning]
enabled = true                   # bool
learning_rate = 0.001            # float, (0, 1]
plasticity = 0.01               # float, [0, 1]
replay = true                    # bool
consolidation_interval = 1000   # uint, minimum 1

[world]
enabled = true               # bool
prediction_horizon = 8       # uint, minimum 1

[reasoning]
enabled = true    # bool
max_steps = 32   # uint, minimum 1

[planning]
enabled = true      # bool
max_depth = 8      # uint, minimum 1
max_branches = 16  # uint, minimum 1

[verification]
enabled = true                # bool
minimum_confidence = 0.80     # float, [0, 1]

[internet]
enabled = true                # bool
timeout_seconds = 15          # uint, minimum 1
max_response_mb = 4           # uint, minimum 1

[policy]
learning = true              # bool
internet_learning = true     # bool
self_modification = false    # bool
policy_modification = false  # bool
runtime_modification = false # bool

[api]
enabled = true                       # bool
bind = "127.0.0.1:8080"             # string, socket address
api_key_env = "CORTEX_API_KEY"      # string, env var name

[persistence]
state = "cortex.cx"            # string, file path
checkpoint_interval = 1000     # uint, minimum 1
```

### 5.2. Configuration Validation

At startup: parse → schema validation → range validation → dependency validation → policy validation → runtime initialization. Invalid configuration SHALL prevent startup.

### 5.3. Configuration Immutability Boundary

Configuration controls architecture limits, resource limits, policy defaults, runtime behavior, persistence, API, and learning parameters. Learning SHALL NOT silently rewrite `cortex.toml`. Runtime state belongs in `.cx`. Administrative configuration belongs in `cortex.toml`. Secrets belong in environment variables or an equivalent external secret mechanism.

### 5.4. Disabled Subsystem Behavior

When a subsystem is disabled via configuration (`enabled = false`), the following behavior applies:

| Subsystem | Disabled Behavior |
|---|---|
| `language.enabled = false` | Input treated as raw observation; no language encoding or generation; output limited to structured responses |
| `world.enabled = false` | World model returns empty state; no transition prediction; reasoning operates without world context |
| `reasoning.enabled = false` | Hypothesis generation skipped; conclusions based on direct memory retrieval and world state |
| `planning.enabled = false` | No goal-directed planning; responses based on immediate reasoning only |
| `verification.enabled = false` | All claims remain in provisional state; no automatic verification; `minimum_confidence` not applied |
| `internet.enabled = false` | No network access; internet observation pipeline disabled |
| `learning.enabled = false` | No state mutation from experience; all learning signals discarded |
| `api.enabled = false` | No embedded API server started; only CLI operational |

A disabled subsystem SHALL return a defined default (empty set, no-op, or passthrough) rather than causing undefined behavior. The cognitive pipeline adapts to skip disabled subsystems while maintaining valid data flow between remaining subsystems.

---

## 6. Cognitive State

### 6.1. State Definition

The complete persistent cognitive state is:

```rust
struct CortexState {
    language: LanguageState,
    neural: NeuralState,
    memory: MemoryState,
    world: WorldState,
    reasoning: ReasoningState,
    planning: PlanningState,
    verification: VerificationState,
    learning: LearningState,
    self_model: SelfModel,
    provenance: ProvenanceState,
    metadata: StateMetadata,
}
```

### 6.2. State Transition Function

```
C(t+1) = F(C(t), O(t), E(t), P(t))
```

Where:
- `C(t)` = current cognitive state
- `O(t)` = observation
- `E(t)` = experience / feedback / prediction error
- `P(t)` = active policy

The policy is an external constraint on adaptation, not an ordinary learned memory.

---

## 7. Language Core (CLX)

CLX — CORTEX Language Core — is the native language-processing substrate of CORTEX. It handles text ingestion, symbol recognition, tokenization, vocabulary management, lexical representation, syntax representation, semantic representation, context modeling, language prediction, meaning construction, response planning, language realization, and text generation. CLX SHALL NOT depend on an external LLM.

### 7.1. Architecture

```
LANGUAGE CORE
                     │
          ┌──────────┴──────────┐
          │                     │
     INPUT PATH            OUTPUT PATH
          │                     │
    normalization         response meaning
          │                     │
     segmentation          response planning
          │                     │
   symbol encoding         lexical selection
          │                     │
    token sequence         syntax generation
          │                     │
     lexical state         semantic validation
          │                     │
    syntax analysis         decoding
          │                     │
   semantic analysis             │
          │                     │
     context model               │
          └──────────┬──────────┘
                     │
              COGNITIVE STATE
```

### 7.2. Language State

```rust
struct LanguageState {
    symbols: SymbolSequence,
    tokens: TokenSequence,
    concepts: ConceptSet,
    entities: EntitySet,
    relations: RelationSet,
    syntax: SyntaxGraph,
    semantics: SemanticGraph,
    context: ContextState,
    intent: IntentHypotheses,
    confidence: ConfidenceState,
}
```

### 7.3. Representation Hierarchy

```
Raw Text
  ↓
Character / Symbol
  ↓
Token
  ↓
Lexical Unit
  ↓
Phrase / Structure
  ↓
Concept
  ↓
Relation
  ↓
Semantic State
  ↓
Cognitive Representation
```

### 7.4. Symbol System

```rust
struct Symbol {
    id: u32,
    kind: SymbolKind,
    frequency: Scalar,
    activation: Scalar,
    confidence: Scalar,
    associations: AssociationSet,
}

enum SymbolKind {
    Character,
    Subword,
    Word,
    Concept,
    Entity,
    Relation,
    Operator,
    Punctuation,
    StructuralMarker,
    SpecialToken,
}
```

The representation SHALL be extensible without requiring a complete model rebuild.

### 7.5. Dynamic Vocabulary

The vocabulary supports continual expansion. Unknown input follows:

```
Unknown Symbol
      ↓
Context Observation
      ↓
Frequency Tracking
      ↓
Association Discovery
      ↓
Semantic Hypothesis
      ↓
Evidence
      ↓
Vocabulary Update
```

Vocabulary membership and semantic understanding are separate states. A new symbol SHALL NOT automatically be considered semantically understood.

### 7.6. Lexical Learning

Lexical associations are learned from: co-occurrence, sequence, context, repetition, user feedback, verified information, prediction error, and semantic association. The system maintains confidence and provenance for learned lexical meanings.

### 7.7. Syntax System

Syntax is represented internally as relationships rather than only token positions. Example: "Ali gives the book to Budi" produces:

```
ACTION:    GIVE
AGENT:     ALI
OBJECT:    BOOK
RECIPIENT: BUDI
```

The syntax subsystem supports: dependency, ordering, roles, nesting, scope, agreement, and structural context.

### 7.8. Semantic System

Semantic representation maps linguistic structures into concepts and relations. Example: "Water boils at approximately 100°C at standard atmospheric pressure" produces:

```
ENTITY:     water
PROPERTY:   boiling_temperature
VALUE:      approximately 100°C
CONDITION:  standard atmospheric pressure
```

Semantic representations are connected to semantic memory, world model, reasoning, verification, and language generation.

### 7.9. Context Model

CORTEX maintains hierarchical context:

| Level | Scope |
|---|---|
| Symbol Context | Individual token/symbol level |
| Sentence Context | Single utterance |
| Conversation Context | Current interaction session |
| Episode Context | Related historical experiences |
| Semantic Context | Active conceptual frame |
| World Context | Current world-state assumptions |
| Long-Term Context | Persistent background state |

Context influences interpretation, memory retrieval, prediction, reasoning, generation, and confidence.

### 7.10. Intent Representation

Input intent is represented as hypotheses rather than absolute classification when ambiguity exists:

```rust
struct IntentHypothesis {
    intent: Intent,
    evidence: EvidenceSet,
    confidence: Scalar,
    alternatives: Vec<IntentHypothesis>,
}

enum Intent {
    Question,
    Instruction,
    Statement,
    Correction,
    Feedback,
    RequestForAction,
    RequestForReasoning,
    RequestForGeneration,
    Observation,
    Conversation,
}
```

### 7.11. Language Prediction

CORTEX predicts candidate continuations and meanings. Prediction scoring combines:

```
Score(candidate) =
    LanguageScore
  + ContextScore
  + SemanticScore
  + MemoryScore
  + WorldScore
  + VerificationScore
  - ContradictionPenalty
  - RiskPenalty
```

### 7.12. Language Generation

Generation proceeds from internal meaning toward language:

```
Cognitive Result
  ↓
Response Intent
  ↓
Meaning Representation
  ↓
Response Structure
  ↓
Candidate Expressions
  ↓
Semantic Validation
  ↓
Syntax Realization
  ↓
Token Selection
  ↓
Output
```

This separates what CORTEX intends to communicate from how CORTEX expresses it.

### 7.13. CLX Interface Contract

```rust
trait LanguageCore {
    fn encode(&self, input: &str, context: &ContextState) -> Result<LanguageState>;
    fn decode(&self, state: &LanguageState, meaning: &MeaningRepresentation) -> Result<String>;
    fn predict(&self, state: &LanguageState) -> Result<Vec<CandidateContinuation>>;
    fn generate(&self, meaning: &VerifiedResult) -> Result<GeneratedResponse>;
    fn update(&mut self, learning_signal: &LearningSignal) -> Result<()>;
    fn vocabulary_size(&self) -> usize;
    fn context_window_size(&self) -> usize;
}
```

---

## 8. Neural Core (CNS)

CNS — CORTEX Neural Substrate — is the native neural processing substrate. It transforms language and perceptual representations into sparse, temporally-aware neural representations and generates predictions.

### 8.1. Architecture

```
Input Representation
        ↓
Cell Field
        ↓
Column Field
        ↓
Sparse Representation
        ↓
Temporal Representation
        ↓
Prediction
        ↓
Prediction Error
        ↓
Plasticity
```

### 8.2. Cell

Cell is the fundamental computational unit:

```rust
struct Cell {
    id: CellId,
    state: CellState,
    activation: Scalar,
    context: ContextVector,
    prediction: PredictionVector,
    confidence: Scalar,
    plasticity: Scalar,
    connections: Connections,
}

enum CellState {
    Resting,
    Active,
    Inhibited,
    Learning,
    Predicting,
}
```

Cell operations: receive, activate, inhibit, associate, predict, adapt, decay, reset.

### 8.3. Column

Columns organize cells into local computational structures:

```rust
struct Column {
    id: ColumnId,
    cells: CellSet,
    context: ContextState,
    prediction: Prediction,
    activation: Scalar,
    competition: CompetitionState,
    routing: RoutingState,
}
```

Column processing: Input → Cell activation → Competition → Sparse selection → Column representation.

### 8.4. Neural Field

Columns are grouped into fields:

```rust
struct Field {
    id: FieldId,
    columns: ColumnSet,
    global_context: ContextVector,
    local_context: ContextVector,
    routing: RoutingState,
    competition: CompetitionState,
    temporal_state: TemporalState,
    prediction_state: PredictionState,
}
```

Fields represent different learned structures (e.g., language, concepts, temporal patterns, world states, procedures).

### 8.5. Sparse Representation

CORTEX uses sparse activation as its baseline representation strategy. For a field of 4096 cells, only a bounded subset may be active simultaneously. Sparsity controls memory efficiency, computational efficiency, representation separation, and interference reduction.

Sparsity bound: `active_cells = min(configured_max_active, field_size * model.sparsity_ratio)` where `model.sparsity_ratio` is configured in `cortex.toml` (default: 0.05).

### 8.6. Temporal Representation

Given:

```
X(t-2), X(t-1), X(t)
```

CNS produces temporal representation `T(t)` encoding sequence, transition, recurrence, context, event order, and temporal dependency.

### 8.7. Neural Prediction

Neural prediction is a first-class operation:

```
Current State
  ↓
Context
  ↓
Active Representation
  ↓
Predicted Next State
```

The prediction becomes the principal learning signal when compared with subsequent observation.

### 8.8. Plasticity

The baseline local plasticity mechanism is:

```
ΔW = η × A × C × E × V
```

Where:
- `η` = learning rate
- `A` = activation relationship
- `C` = context factor
- `E` = prediction error
- `V` = evidence/confidence

All updates are bounded. No single observation may arbitrarily destabilize the complete neural state.

### 8.9. CNS Interface Contract

```rust
trait NeuralCore {
    fn process(&self, input: &LanguageState, context: &ContextState) -> Result<NeuralRepresentation>;
    fn predict(&self, state: &NeuralState) -> Result<Prediction>;
    fn compute_error(&self, predicted: &Prediction, actual: &Observation) -> Result<PredictionError>;
    fn adapt(&mut self, error: &PredictionError, signal: &LearningSignal) -> Result<()>;
    fn field_count(&self) -> usize;
    fn active_cells(&self) -> usize;
    fn active_columns(&self) -> usize;
}
```

---

## 9. Memory System

CORTEX maintains five memory subsystems. Memory is a cognitive subsystem rather than an external database.

### 9.1. Working Memory

Working memory contains: current input, current conversation context, active concepts, active hypotheses, current goals, temporary reasoning state, current world-state assumptions, and generation state. Working memory is bounded by `memory.working_mb`.

```rust
struct WorkingMemory {
    input: Option<CurrentInput>,
    conversation_context: ConversationContext,
    active_concepts: ConceptSet,
    active_hypotheses: HypothesisSet,
    goals: GoalSet,
    reasoning_state: Option<ReasoningSnapshot>,
    world_assumptions: WorldStateSnapshot,
    generation_state: Option<GenerationState>,
}
```

### 9.2. Episodic Memory

```rust
struct Episode {
    id: EpisodeId,
    observation: Observation,
    context: ContextState,
    action: Option<Action>,
    outcome: Option<Outcome>,
    timestamp: Timestamp,
    prediction: Option<Prediction>,
    prediction_error: PredictionError,
    confidence: ConfidenceState,
    source: Provenance,
    importance: Scalar,
}
```

Episodes preserve experience. Bounded by `memory.episodic_mb`.

### 9.3. Semantic Memory

```rust
struct Knowledge {
    concept: ConceptId,
    properties: PropertySet,
    relations: RelationSet,
    evidence: EvidenceSet,
    confidence: ConfidenceState,
    provenance: ProvenanceSet,
}
```

Semantic knowledge is revisable. Bounded by `memory.semantic_mb`.

### 9.4. Procedural Memory

```rust
struct Procedure {
    id: ProcedureId,
    condition: Condition,
    steps: Vec<Action>,
    expected_outcome: Outcome,
    success_count: u64,
    failure_count: u64,
    confidence: Scalar,
    context_requirements: ContextRequirements,
    risk: RiskAssessment,
    provenance: Provenance,
}
```

Bounded by `memory.procedural_mb`.

### 9.5. Associative Memory

Associative memory represents relationships among internal structures:

```rust
struct Association {
    id: AssociationId,
    source: InternalId,
    target: InternalId,
    kind: AssociationKind,
    strength: Scalar,
    confidence: Scalar,
    context: ContextState,
    provenance: Provenance,
}

enum AssociationKind {
    Semantic,
    Temporal,
    Contextual,
    Causal,
    Episodic,
    Procedural,
}
```

Retrieval score considers: semantic relevance, context relevance, temporal relevance, association strength, importance, prediction relevance, confidence, and recency. Bounded by `memory.associative_mb`.

### 9.6. Memory Retrieval

```
Query
  ↓
Context Analysis
  ↓
Candidate Retrieval
  ↓
Relevance Scoring
  ↓
Confidence Filtering
  ↓
Contradiction Detection
  ↓
Ranked Memory Set
```

Retrieved memories preserve provenance and confidence.

### 9.7. Memory Consolidation

```
Working Memory
      ↓
Episode Formation
      ↓
Pattern Extraction
      ↓
Semantic / Procedural Candidate
      ↓
Evidence Evaluation
      ↓
Consolidation
      ↓
Long-Term Memory
```

Consolidation may: merge, compress, strengthen, generalize, decay, forget.

### 9.8. Forgetting

Forgetting is controlled rather than arbitrary. Candidate forgetting factors: low importance, low retrieval frequency, low confidence, redundancy, age, memory pressure, contradiction. High-value knowledge receives stronger retention.

### 9.9. Memory Query and Retrieval Types

```rust
struct MemoryQuery {
    query_type: MemoryQueryType,
    text: Option<String>,
    concept_ids: Vec<ConceptId>,
    time_range: Option<(Timestamp, Timestamp)>,
    max_results: usize,
    min_confidence: Scalar,
}

enum MemoryQueryType {
    Semantic,
    Episodic,
    Procedural,
    Associative,
    All,
}

struct MemoryRetrieval {
    episodic: Vec<Episode>,
    semantic: Vec<Knowledge>,
    procedural: Vec<Procedure>,
    associative: Vec<Association>,
    relevance_scores: HashMap<MemoryId, Scalar>,
    confidence_filter_applied: bool,
}
```

### 9.10. Memory Interface Contract

```rust
trait MemorySystem {
    fn store(&mut self, episode: Episode) -> Result<()>;
    fn retrieve(&self, query: &MemoryQuery, context: &ContextState) -> Result<MemoryRetrieval>;
    fn consolidate(&mut self) -> Result<ConsolidationResult>;
    fn forget(&mut self, policy: &ForgettingPolicy) -> Result<ForgettingResult>;
    fn working_memory(&self) -> &WorkingMemory;
    fn working_memory_mut(&mut self) -> &mut WorkingMemory;
    fn episode_count(&self) -> usize;
    fn knowledge_count(&self) -> usize;
    fn memory_usage(&self) -> MemoryUsage;
}
```

---

## 10. World Model

The World Model represents CORTEX's current internal model of external reality. It is explicitly distinguished from raw memory.

### 10.1. World Model Structure

```rust
struct WorldModel {
    entities: EntitySet,
    properties: PropertyMap,
    states: StateMap,
    relations: RelationSet,
    events: EventSet,
    transitions: TransitionSet,
    temporal_patterns: TemporalPatternSet,
    causal_hypotheses: CausalHypothesisSet,
    uncertainty: UncertaintyState,
}
```

### 10.2. Entity Model

Entities may represent: person, object, place, organization, conceptual object, event, system, process.

```rust
struct Entity {
    id: EntityId,
    kind: EntityKind,
    identity: IdentityState,
    properties: PropertySet,
    state: EntityState,
    relations: RelationSet,
    confidence: Scalar,
    provenance: ProvenanceSet,
}

enum EntityKind {
    Person,
    Object,
    Place,
    Organization,
    ConceptualObject,
    Event,
    System,
    Process,
}
```

### 10.3. World State

```rust
struct WorldState {
    entities: EntitySet,
    relations: RelationSet,
    active_events: EventSet,
    temporal_context: TemporalContext,
    uncertainty: UncertaintyState,
}
```

World state changes over time. `WorldState` is the persistent snapshot stored in `.cx`. `WorldModel` (section 10.1) is the full runtime representation including transition models, causal hypotheses, and temporal patterns used for simulation and prediction. `WorldModel` is reconstructed from `WorldState` at startup and is not directly serialized.

### 10.4. Transition Model

```
S(t) + A(t)
    ↓
Transition Model
    ↓
Predicted S(t+1)
```

Actual observation: `Actual S(t+1)`. Comparison yields prediction error.

### 10.5. Causal Hypotheses

CORTEX distinguishes correlation, association, temporal relationship, causal hypothesis, and verified causal relationship:

```rust
struct CausalHypothesis {
    cause: ConceptId,
    effect: ConceptId,
    confidence: Scalar,
    evidence: EvidenceSet,
    contradictions: EvidenceSet,
    conditions: ConditionSet,
}
```

### 10.6. Counterfactual Model

CORTEX supports hypothetical world trajectories. A counterfactual result carries uncertainty:

```
Current World State
       │
       ├── Actual trajectory
       │
       └── Hypothetical trajectory
```

### 10.7. World Model Interface Contract

```rust
trait WorldModelInterface {
    fn integrate(&mut self, representation: &NeuralRepresentation, memories: &MemoryRetrieval) -> Result<WorldState>;
    fn predict_transition(&self, state: &WorldState, action: &Action) -> Result<PredictedState>;
    fn observe(&mut self, observation: &Observation, provenance: &Provenance) -> Result<()>;
    fn simulate(&self, state: &WorldState, actions: &[Action]) -> Result<SimulatedTrajectory>;
    fn entity_count(&self) -> usize;
    fn relation_count(&self) -> usize;
}
```

---

## 11. Reasoning Engine

The reasoning system is based on a Hypothesis Workspace.

### 11.1. Reasoning Pipeline

```
Observation
  ↓
Problem Representation
  ↓
Memory Retrieval
  ↓
Hypothesis Generation
  ↓
Evidence Evaluation
  ↓
World Simulation
  ↓
Counter-Evidence Search
  ↓
Contradiction Detection
  ↓
Hypothesis Ranking
  ↓
Conclusion
```

### 11.2. Hypothesis State

```rust
struct Hypothesis {
    id: HypothesisId,
    proposition: Proposition,
    evidence: EvidenceSet,
    counter_evidence: EvidenceSet,
    confidence: Scalar,
    dependencies: DependencySet,
    contradictions: ContradictionSet,
    provenance: ProvenanceSet,
}
```

### 11.3. Reasoning Types

CORTEX supports: deductive reasoning, inductive reasoning, abductive reasoning, analogical reasoning, temporal reasoning, causal reasoning, counterfactual reasoning, constraint reasoning, and consistency reasoning. No reasoning result automatically becomes verified knowledge.

### 11.4. Contradiction Handling

When knowledge A conflicts with knowledge B, CORTEX retains the conflict until resolved. The system evaluates: source quality, recency, independent confirmation, logical consistency, context, and verification status.

### 11.5. Reasoning Interface Contract

```rust
trait ReasoningEngine {
    fn evaluate(&self, representation: &NeuralRepresentation, memories: &MemoryRetrieval, world: &WorldState) -> Result<ReasoningResult>;
    fn generate_hypotheses(&self, problem: &ProblemRepresentation) -> Result<Vec<Hypothesis>>;
    fn evaluate_hypothesis(&self, hypothesis: &Hypothesis, evidence: &EvidenceSet) -> Result<HypothesisEvaluation>;
    fn detect_contradictions(&self, claims: &[KnowledgeClaim]) -> Result<Vec<Contradiction>>;
    fn bounded_conclusion(&self, hypotheses: &[Hypothesis], budget: &ComputeBudget) -> Result<BoundedConclusion>;
}
```

---

## 12. Planning Engine

Planning operates on the world model.

### 12.1. Planning Pipeline

```
Current State
  ↓
Goal
  ↓
Candidate Actions
  ↓
World Simulation
  ↓
Predicted Outcomes
  ↓
Risk Evaluation
  ↓
Utility Evaluation
  ↓
Plan Ranking
  ↓
Selected Plan
```

### 12.2. Plan Representation

```rust
struct Plan {
    goal: Goal,
    steps: Vec<Action>,
    predicted_outcomes: OutcomeSet,
    estimated_cost: Scalar,
    estimated_risk: Scalar,
    uncertainty: Scalar,
    confidence: Scalar,
}
```

Planning SHALL be resource-bounded by `planning.max_depth` and `planning.max_branches`.

### 12.3. Planning Interface Contract

```rust
trait PlanningEngine {
    fn evaluate(&self, reasoning: &ReasoningResult, world: &WorldState) -> Result<Option<Plan>>;
    fn simulate_plan(&self, plan: &Plan, world: &WorldState) -> Result<SimulatedOutcome>;
    fn evaluate_risk(&self, plan: &Plan, world: &WorldState) -> Result<RiskAssessment>;
}
```

---

## 13. Verification Engine

Verification separates observed, inferred, supported, provisional, verified, unknown, and contradicted.

### 13.1. Verification Pipeline

```
Claim
  ↓
Evidence Retrieval
  ↓
Source Evaluation
  ↓
Consistency Analysis
  ↓
Independent Evidence
  ↓
Contradiction Analysis
  ↓
Confidence Update
  ↓
Verification Status
```

### 13.2. Verification Status

```rust
enum VerificationStatus {
    Observed,
    Inferred,
    Supported,
    Provisional,
    Verified,
    Unknown,
    Contradicted,
}
```

### 13.3. Verification Is Not Confidence

Confidence and verification are separate. A claim may have high confidence without `verified = true` because verification requires defined evidence conditions. The `verification.minimum_confidence` configuration parameter (default: 0.80) sets the confidence threshold that evidence must meet for a claim to transition from `Supported` or `Provisional` to `Verified`.

### 13.4. Confidence Model

CORTEX tracks multiple dimensions:

```rust
struct ConfidenceState {
    belief: Scalar,
    evidence_strength: Scalar,
    source_quality: Scalar,
    consistency: Scalar,
    uncertainty: Scalar,
    prediction_reliability: Scalar,
    verification_status: VerificationStatus,
}
```

### 13.5. Verification Interface Contract

```rust
trait VerificationEngine {
    fn evaluate(&self, reasoning: &ReasoningResult) -> Result<VerifiedResult>;
    fn verify_claim(&self, claim: &KnowledgeClaim, evidence: &EvidenceSet) -> Result<VerificationResult>;
    fn confidence_dimensions(&self, claim: &KnowledgeClaim) -> Result<ConfidenceState>;
}
```

---

## 14. Continual Learning System

CORTEX learns from experience without requiring complete retraining.

### 14.1. Learning Hierarchy

```
FAST
│
├── Working state
├── Temporary adaptation
└── Active context

MEDIUM
│
├── Episodic patterns
├── Semantic knowledge
├── Procedural knowledge
└── World model

SLOW
│
├── Neural adaptation
├── Language adaptation
└── Long-term consolidation
```

### 14.2. Learning Sources

CORTEX may learn from: conversation, user-provided information, environment observations, internet information, feedback, prediction errors, verified information, successful procedures, and failed procedures. Learning is filtered through attribution and policy.

### 14.3. Prediction Error

The principal learning signal:

```
Prediction
  ↓
Observation
  ↓
Difference
  ↓
Prediction Error
```

### 14.4. Error Attribution

```
Prediction Error
      ↓
 ┌────┼────┬────┬────┬────┐
 ▼    ▼    ▼    ▼    ▼    ▼
Input Memory World Reasoning Procedure Environment
Error Error  Error   Error     Error     Error
```

The attribution mechanism determines which subsystem receives the learning signal.

### 14.5. Replay

```
Episode
  ↓
Context Reconstruction
  ↓
Prediction
  ↓
Counterfactual Evaluation
  ↓
Error Analysis
  ↓
Learning
```

Replay priority is based on: prediction error, novelty, importance, uncertainty, recurrence, and learning value.

### 14.6. Consolidation

```
Experience
  ↓
Episode
  ↓
Pattern
  ↓
Generalization
  ↓
Knowledge Candidate
  ↓
Evidence Evaluation
  ↓
Consolidation
```

Consolidation avoids allowing a single anomalous event to dominate long-term state.

### 14.6.1. Consolidation Interface Contract

```rust
trait ConsolidationEngine {
    fn consolidate(&mut self, candidates: &[ConsolidationCandidate], policy: &PolicyState) -> Result<ConsolidationResult>;
    fn evaluate_candidate(&self, candidate: &ConsolidationCandidate) -> Result<EvaluationResult>;
    fn merge_knowledge(&self, existing: &Knowledge, candidate: &Knowledge) -> Result<Knowledge>;
    fn should_consolidate(&self, candidate: &ConsolidationCandidate, budget: &ComputeBudget) -> bool;
    fn consolidation_stats(&self) -> ConsolidationStats;
}
```

### 14.7. Language Continual Learning

The Language Core learns: new symbols, new vocabulary, new semantic associations, new syntactic patterns, new terminology, new domain concepts, new discourse patterns. Language learning uses the same persistent learning infrastructure as the rest of CORTEX.

### 14.8. Learning Interface Contract

```rust
trait LearningSystem {
    fn record(&mut self, experience: &Experience) -> Result<LearningSignal>;
    fn attribute_error(&self, error: &PredictionError) -> Result<ErrorAttribution>;
    fn apply_signal(&mut self, signal: &LearningSignal, policy: &PolicyState) -> Result<LearningResult>;
    fn replay(&mut self, episodes: &[Episode], budget: &ComputeBudget) -> Result<ReplayResult>;
    fn consolidation_candidates(&self) -> Result<Vec<ConsolidationCandidate>>;
    fn learning_events(&self) -> u64;
}
```

---

## 15. Self Model

The Self Model describes CORTEX's own operational state. It is a computational representation. It SHALL NOT be interpreted by the architecture as proof of consciousness or subjective experience.

### 15.1. Self Model Structure

```rust
struct SelfModel {
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
```

### 15.2. Capability Estimation

CORTEX maintains estimates of: language accuracy, prediction accuracy, verification reliability, planning success, memory retrieval success, reasoning consistency, and resource availability. These estimates influence confidence and planning.

### 15.3. Self Model Interface Contract

```rust
trait SelfModelInterface {
    fn estimate_capability(&self, capability: Capability) -> Result<CapabilityEstimate>;
    fn health_status(&self) -> Result<HealthStatus>;
    fn update(&mut self, metrics: &PerformanceMetrics) -> Result<()>;
}
```

---

## 16. Policy / Risk Gate

All potentially consequential operations pass through the Policy / Risk Gate.

### 16.1. Gate Pipeline

```
Proposed Operation
       ↓
Operation Classification
       ↓
Risk Estimation
       ↓
Policy Evaluation
       ↓
┌──────┼──────┐
▼      ▼      ▼
ALLOW  LIMIT  DENY
```

### 16.2. Risk Model

Risk considers: potential impact, uncertainty, confidence, reversibility, scope, resource consumption, policy constraints, and external side effects. Risk evaluation is separate from task reasoning.

### 16.3. Default Policy

```toml
[policy]
learning = true
internet_learning = true
self_modification = false
policy_modification = false
runtime_modification = false
```

Learning may modify: memory, knowledge, world model, language state, learned parameters. Learning SHALL NOT modify: root policy, authorization boundary, security credentials, runtime executable, policy enforcement code — unless an explicitly authorized external administrative operation permits it.

### 16.4. Self-Modification Levels

| Level | Scope | Default |
|---|---|---|
| 1 — Cognitive State Adaptation | memory, language state, world model, learned parameters, procedures, associations | Allowed |
| 2 — Algorithm Adaptation | learning algorithm, reasoning algorithm, language algorithm, runtime algorithm | Restricted |
| 3 — Security / Policy Modification | policy, authorization, risk boundary, security enforcement | Restricted (highest) |

Normal continual learning SHALL NOT modify Level 3.

### 16.5. Policy Interface Contract

```rust
trait PolicyEngine {
    fn evaluate(&self, operation: &ProposedOperation) -> Result<PolicyDecision>;
    fn risk_estimate(&self, operation: &ProposedOperation) -> Result<RiskEstimate>;
    fn is_allowed(&self, operation: &ProposedOperation) -> bool;
}

enum PolicyDecision {
    Allowed,
    Limited { constraints: OperationConstraints },
    Denied { reason: DenialReason },
}
```

---

## 17. Internet Interface

The Internet Interface treats external information as observation.

### 17.1. Internet Pipeline

```
URL / Request
  ↓
Policy Gate
  ↓
Network Access
  ↓
Response
  ↓
Parsing
  ↓
Content Extraction
  ↓
Provenance
  ↓
Evidence
  ↓
Verification
  ↓
Memory
  ↓
World Model
  ↓
Learning
```

Internet information SHALL NOT automatically be treated as ground truth.

### 17.2. Internet Safety Boundary

```
Intent
  ↓
Proposed Network Operation
  ↓
Risk Assessment
  ↓
Policy
  ↓
Network
```

Network results return as observations:

```
Network Result
  ↓
Evidence
  ↓
Verification
  ↓
Memory
```

### 17.3. Internet Interface Contract

```rust
trait InternetInterface {
    fn fetch(&self, request: &NetworkRequest, policy: &PolicyState) -> Result<NetworkObservation>;
    fn parse(&self, response: &NetworkResponse) -> Result<ExtractedContent>;
    fn to_observation(&self, content: &ExtractedContent, provenance: &Provenance) -> Result<Observation>;
}
```

---

## 18. Persistence Engine

CORTEX persistence is implemented through the `.cx` binary format.

### 18.1. Save Path

```
Runtime State
  ↓
Serialization
  ↓
Integrity Calculation
  ↓
Atomic Write
  ↓
cortex.cx
```

### 18.2. Load Path

```
cortex.cx
  ↓
Integrity Check
  ↓
Version Check
  ↓
Migration if required
  ↓
State Validation
  ↓
Runtime Reconstruction
```

### 18.3. Atomic Persistence

State writes use an atomic strategy:

```
Current State
  ↓
Write Temporary State
  ↓
Flush
  ↓
Verify
  ↓
Atomic Replace
```

A failed write SHALL NOT silently destroy the last valid state.

### 18.4. Persistence Interface Contract

```rust
trait PersistenceEngine {
    fn save(&self, state: &CortexState, path: &Path) -> Result<SaveResult>;
    fn load(&self, path: &Path) -> Result<CortexState>;
    fn maybe_checkpoint(&self, state: &CortexState, interval: u64) -> Result<Option<CheckpointId>>;
    fn validate(&self, path: &Path) -> Result<ValidationResult>;
    fn recover(&self, checkpoints: &[Path]) -> Result<CortexState>;
}
```

---

## 19. `.cx` State Format

The `.cx` format is a binary, versioned, section-oriented cognitive state container.

### 19.1. Format Structure

```
CORTEX.CX
│
├── HEADER
├── ARCHITECTURE
├── LANGUAGE
├── NEURAL
├── CELLS
├── COLUMNS
├── FIELDS
├── WORKING_MEMORY
├── EPISODIC_MEMORY
├── SEMANTIC_MEMORY
├── PROCEDURAL_MEMORY
├── ASSOCIATIVE_MEMORY
├── WORLD_MODEL
├── REASONING
├── PLANNING
├── VERIFICATION
├── LEARNING
├── SELF_MODEL
├── PROVENANCE
├── CHECKPOINT_METADATA
└── INTEGRITY
```

### 19.2. Section Header

Each section contains:

| Field | Type | Description |
|---|---|---|
| TYPE | u16 | Section type identifier |
| VERSION | u16 | Section format version |
| FLAGS | u32 | Section flags |
| OFFSET | u64 | Byte offset to data |
| LENGTH | u64 | Byte length of data |
| CHECKSUM | u128 | Integrity checksum of data |
| DATA | bytes | Serialized section data |

This enables partial loading, validation, migration, recovery, and checkpointing.

### 19.3. File Header

The header identifies: format magic, format version, architecture version, algorithm version, configuration hash, state identifier, creation timestamp, last checkpoint, and integrity metadata.

```rust
struct CxHeader {
    magic: [u8; 8],           // b"CORTEX\0\0"
    format_version: u32,
    architecture_version: u32,
    algorithm_version: u32,
    config_hash: [u8; 32],    // SHA-256 of cortex.toml
    state_id: Uuid,
    created_at: Timestamp,
    last_checkpoint: Timestamp,
    integrity: IntegrityMetadata,
}
```

### 19.4. State Versioning

Migration: old state → version detection → compatibility check → migration → validation → new state. Migration SHALL preserve semantic state whenever technically possible.

### 19.5. Algorithm Versioning

`.cx` records: cell_algorithm, column_algorithm, plasticity_algorithm, memory_algorithm, language_algorithm, reasoning_algorithm, planning_algorithm, verification_algorithm, consolidation_algorithm. Changing an algorithm SHALL create a detectable architectural state transition.

### 19.6. Integrity

CORTEX verifies persistent state before loading: checksum/integrity verification → structural validation → semantic validation → load. Invalid critical state SHALL trigger STOP or recovery from valid checkpoint rather than silent continuation.

### 19.7. Checkpointing

Checkpoint metadata includes: state version, algorithm version, configuration hash, timestamp, episode count, learning state, and integrity information.

---

## 20. Provenance

Every externally derived knowledge item retains: source, source identity, timestamp, retrieval context, content identity, evidence, verification status, and confidence.

```rust
enum ProvenanceCategory {
    Observed,
    UserProvided,
    Internet,
    Derived,
    Inferred,
    Replayed,
    Verified,
}

struct Provenance {
    category: ProvenanceCategory,
    source: Source,
    source_identity: SourceIdentity,
    timestamp: Timestamp,
    retrieval_context: Option<RetrievalContext>,
    content_hash: [u8; 32],
    evidence: EvidenceSet,
    verification_status: VerificationStatus,
    confidence: ConfidenceState,
}
```

---

## 21. Runtime

### 21.1. Runtime State Machine

```
BOOT
  ↓
LOAD_CONFIGURATION
  ↓
LOAD_STATE
  ↓
VALIDATE
  ↓
INITIALIZE
  ↓
READY
  ↓
PROCESSING
  ↓
LEARNING
  ↓
CONSOLIDATING
  ↓
CHECKPOINTING
  ↓
READY
```

Failure path:

```
ANY STATE
   ↓
FAULT
   ↓
RECOVERY
   ↓
READY
```

or:

```
FAULT
  ↓
SAFE STOP
```

### 21.2. First Boot

If `cortex.cx` does not exist:

```
Read cortex.toml
  ↓
Validate configuration
  ↓
Initialize Language Core
  ↓
Initialize Vocabulary
  ↓
Initialize Neural Core
  ↓
Initialize Memory
  ↓
Initialize World Model
  ↓
Initialize Reasoning
  ↓
Initialize Planning
  ↓
Initialize Verification
  ↓
Initialize Learning
  ↓
Initialize Self Model
  ↓
Initialize Policy
  ↓
Create initial cognitive state
  ↓
Persist cortex.cx
  ↓
Start runtime
  ↓
READY
```

The system is operational without an externally supplied trained model. The initial state may have limited knowledge and language competence; capability is expected to grow through the defined learning mechanisms.

### 21.3. Runtime Modes

```bash
cortex run           # normal cognitive runtime
cortex serve         # embedded API interface
cortex observe       # observation-only mode
cortex experience    # experience ingestion mode
cortex learn         # learning-focused mode
cortex query         # query-only mode
cortex inspect       # state inspection
cortex verify        # verification mode
cortex checkpoint    # manual checkpoint
cortex status        # status display
```

### 21.4. Main Cognitive Operation

```rust
fn process(input: Input) -> Result<Response> {
    let observation = observe(input)?;
    let context = working_memory.context();
    let language_state = language.encode(&observation.text, &context)?;
    let representation = neural.process(&language_state, &context)?;
    let query = MemoryQuery::from_representation(&representation);
    let memories = memory.retrieve(&query, &context)?;
    let world_state = world.integrate(&representation, &memories)?;
    let reasoning_state = reasoning.evaluate(&representation, &memories, &world_state)?;
    let plan = planning.evaluate(&reasoning_state, &world_state)?;
    let verified = verification.evaluate(&reasoning_state)?;
    let response = language.generate(&verified)?;
    let experience = Experience::new(observation, &response, world_state, reasoning_state);
    learning.record(&experience)?;
    persistence.maybe_checkpoint()?;
    Ok(response)
}
```

This is an architectural contract rather than a requirement that the implementation use exactly this function structure. The types and method signatures align with the interface contracts defined in each subsystem's section.

### 21.5. Cognitive Feedback Loop

```
Response
  ↓
Outcome
  ↓
Observation
  ↓
Prediction Comparison
  ↓
Prediction Error
  ↓
Attribution
  ↓
Learning Signal
```

The outcome may come from: user feedback, subsequent observation, environment, verification, later evidence, or task result.

### 21.6. Experience Representation

```rust
struct Experience {
    observation: Observation,
    internal_state: StateSnapshot,
    prediction: Prediction,
    action: Option<Action>,
    outcome: Option<Outcome>,
    error: PredictionError,
    attribution: ErrorAttribution,
    evidence: EvidenceSet,
    provenance: Provenance,
}
```

### 21.7. Complete Cognitive Loop

```
┌──────────────────────────────────────────────────────┐
│                    OBSERVATION                       │
└────────────────────────┬─────────────────────────────┘
                         ↓
                  LANGUAGE ENCODING
                         ↓
                   REPRESENTATION
                         ↓
                   NEURAL PROCESS
                         ↓
                   MEMORY RETRIEVAL
                         ↓
                   WORLD INTEGRATION
                         ↓
                      PREDICTION
                         ↓
                      REASONING
                         ↓
                      PLANNING
                         ↓
                    VERIFICATION
                         ↓
                 RESPONSE / ACTION
                         ↓
                    ENVIRONMENT
                         ↓
                    OBSERVATION
                         ↓
                 PREDICTION ERROR
                         ↓
                  ERROR ATTRIBUTION
                         ↓
                       LEARNING
                         ↓
                      REPLAY
                         ↓
                   CONSOLIDATION
                         ↓
                    PERSISTENCE
                         │
                         └───────────────↺
```

### 21.8. Runtime Interface Contract

```rust
trait Runtime {
    fn boot(config: CortexConfig) -> Result<Self>;
    fn ready(&self) -> bool;
    fn process(&mut self, input: Input) -> Result<Response>;
    fn observe(&mut self, observation: Observation) -> Result<()>;
    fn experience(&mut self, experience: Experience) -> Result<()>;
    fn query(&self, query: CognitiveQuery) -> Result<CognitiveResponse>;
    fn checkpoint(&self) -> Result<CheckpointId>;
    fn status(&self) -> Result<RuntimeStatus>;
    fn shutdown(&mut self) -> Result<()>;
}
```

---

## 22. Resource Management

CORTEX operates under explicit resource budgets.

### 22.1. RAM Budget

```
RAM
│
├── Language Core
├── Neural Core
├── Working Memory
├── Episodic Memory
├── Semantic Memory
├── Procedural Memory
├── Associative Memory
├── World Model
├── Reasoning
└── Runtime Cache
```

When memory pressure increases: compress → consolidate → evict → forget. The system SHALL NOT assume unlimited memory.

### 22.2. Compute Budget

Reasoning and planning have bounded execution. Configuration:

```rust
struct ComputeBudget {
    max_reasoning_steps: u32,      // from reasoning.max_steps
    max_planning_depth: u32,       // from planning.max_depth
    max_planning_branches: u32,    // from planning.max_branches
    max_simulation_steps: u32,     // from world.prediction_horizon
    max_generation_length: u32,    // from language.generation_limit
    max_memory_retrieval: u32,     // min(episodic_count, semantic_count, procedural_count, associative_count) / 4
    max_replay_count: u32,         // max(1, learning.consolidation_interval / 10)
}
```

A cognitive operation that reaches its budget SHALL terminate with an explicit bounded result rather than consuming unlimited resources.

### 22.3. Resource-Aware Cognition

High uncertainty + high reasoning cost + low available compute may result in bounded reasoning, lower plan depth, and explicit uncertainty rather than unbounded execution.

---

## 23. Concurrency Model

The system remains a single process. Internal concurrency may be used for: I/O, network access, background persistence, replay, maintenance, and non-conflicting computation. Cognitive state mutation uses explicit synchronization or an ownership-based state transition model. The architecture prevents concurrent updates from corrupting `.cx` state.

---

## 24. API

### 24.1. Endpoints

| Method | Path | Description |
|---|---|---|
| POST | `/v1/inference` | Process input and return response |
| POST | `/v1/observe` | Submit observation without response |
| POST | `/v1/experience` | Submit explicit learning experience |
| POST | `/v1/learn` | Trigger learning operation |
| POST | `/v1/query` | Query memory, world model, knowledge |
| GET | `/v1/status` | Runtime status |
| POST | `/v1/checkpoint` | Manual checkpoint |

### 24.2. Inference API

```http
POST /v1/inference
Authorization: Bearer <API_KEY>
Content-Type: application/json

{
  "input": "Explain what gravity is.",
  "context": {},
  "options": {
    "max_tokens": 1024,
    "verify": true
  }
}
```

Response:

```json
{
  "output": "...",
  "confidence": 0.84,
  "verification_status": "SUPPORTED",
  "state_updated": true
}
```

### 24.3. Observation API

```http
POST /v1/observe
Authorization: Bearer <API_KEY>
Content-Type: application/json

{
  "observation": "...",
  "source": "user",
  "context": {}
}
```

The observation enters the cognitive pipeline without necessarily requiring an immediate response.

### 24.4. Experience API

```http
POST /v1/experience
Authorization: Bearer <API_KEY>
Content-Type: application/json

{
  "observation": "...",
  "action": "...",
  "outcome": "...",
  "feedback": "...",
  "source": "user"
}
```

This supplies an explicit learning experience.

### 24.5. Query API

```http
POST /v1/query
Authorization: Bearer <API_KEY>
Content-Type: application/json

{
  "target": "memory",
  "query": "...",
  "parameters": {}
}
```

May query: memory, world model, knowledge, episodes, procedures, verification state, self model.

### 24.6. Status API

```http
GET /v1/status
Authorization: Bearer <API_KEY>
```

Response:

```json
{
  "status": "ready",
  "uptime": 1234,
  "memory_usage": 0,
  "episode_count": 0,
  "prediction_error": 0.0,
  "learning_enabled": true,
  "world_model_size": 0,
  "language_vocabulary_size": 0
}
```

### 24.7. API Authentication

Authentication uses `CORTEX_API_KEY` environment variable. Configuration:

```toml
[api]
enabled = true
bind = "127.0.0.1:8080"
api_key_env = "CORTEX_API_KEY"
```

The API key SHALL NOT be persisted inside `.cx`.

### 24.8. API Safety Boundary

External API requests SHALL NOT directly mutate arbitrary internal memory structures. Instead:

```
API Request
  ↓
Validated Command
  ↓
Policy
  ↓
Cognitive Operation
  ↓
State Transition
```

This preserves state invariants.

---

## 25. CLI

### 25.1. Commands

```bash
cortex run                  # Start normal cognitive runtime
cortex serve                # Start embedded API server
cortex observe <text>       # Submit observation
cortex experience <json>    # Submit experience
cortex learn                # Trigger learning cycle
cortex query <text>         # Query cognitive state
cortex inspect              # Inspect state
cortex verify <claim>       # Verify claim
cortex checkpoint           # Create checkpoint
cortex status               # Show status
cortex init                 # Initialize new state
cortex migrate              # Migrate state format
```

---

## 26. Knowledge Lifecycle

```
Observation
  ↓
Representation
  ↓
Candidate Memory
  ↓
Evidence
  ↓
Verification
  ↓
Generalization
  ↓
Semantic Knowledge
  ↓
World Model
  ↓
Prediction
  ↓
New Observation
  ↓
Belief Update
```

Knowledge is dynamic.

---

## 27. Learning Stability

CORTEX prevents catastrophic state changes through: bounded updates, confidence weighting, evidence weighting, experience replay, consolidation thresholds, memory protection, policy constraints, and contradiction detection. No individual observation should automatically rewrite the complete cognitive architecture.

### 27.1. Knowledge Conflict

Conflicting information remains represented as competing hypotheses until resolved:

```
Claim A: confidence = 0.81
Claim B: confidence = 0.57
```

The system may prefer A while preserving B as contradictory evidence.

---

## 28. Separation of Concerns

### 28.1. Knowledge and Language Separation

CORTEX distinguishes knowing a word from understanding a concept, and understanding a concept from having verified knowledge about the concept. This prevents vocabulary expansion from being mistaken for cognitive learning.

### 28.2. Reasoning and Generation Separation

The architecture separates reasoning result from language expression. A language-generation error does not necessarily imply a reasoning error. A correct-looking sentence does not prove correct reasoning.

### 28.3. Verification and Generation Separation

The system supports: generate → verify → revise → generate again when required. This enables response correction before output.

### 28.4. Planning and Policy Separation

Planning generates candidate actions. Policy determines whether those actions are permitted. The planner cannot bypass the policy layer.

### 28.5. Self Model and Policy Separation

The Self Model may estimate "I am uncertain" computationally. It does not gain authority to change policy merely because it estimates a different capability.

### 28.6. Policy as a Non-Learned Boundary

Policy is represented separately from learned knowledge. The learned model cannot redefine the gate simply by generating a different internal belief.

---

## 29. Failure Handling

### 29.1. Error Taxonomy

```rust
enum ErrorKind {
    InputError,
    EncodingError,
    LanguageError,
    MemoryError,
    WorldModelError,
    ReasoningError,
    PlanningError,
    VerificationError,
    LearningError,
    PersistenceError,
    PolicyError,
    ResourceError,
    NetworkError,
    RuntimeError,
}
```

This taxonomy feeds diagnostics and learning attribution.

### 29.2. Failure Categories

CORTEX distinguishes: recoverable error, cognitive error, input error, network error, state corruption, configuration error, policy violation, resource exhaustion, and fatal runtime error.

### 29.3. Failure Response

| Error | Response |
|---|---|
| Network failure | Record failed observation, continue |
| Corrupt `.cx` | Validate checkpoint, recover |
| Invalid policy | Restricted mode |

### 29.4. Safe State Recovery

Recovery priority:

1. Current Valid State
2. Latest Valid Checkpoint
3. Previous Valid Checkpoint
4. Initial State
5. Safe Stop

A corrupt state SHALL never be silently treated as valid.

---

## 30. State Invariants

The runtime preserves:

- Valid memory references
- Valid neural topology
- Valid vocabulary references
- Valid world-model relationships
- Valid provenance
- Valid algorithm versions
- Valid policy state
- Valid `.cx` structure

Any invalid transition SHALL fail before persistence.

### 30.1. Memory Invariants

Memory entries have: identity, type, content, confidence, timestamp, provenance, and retention metadata. Semantic memory SHALL NOT contain unverifiable claims without their verification status.

### 30.2. World Model Invariants

Every world-model assertion has: source, confidence, temporal context, and verification state. The system distinguishes: world observation, world hypothesis, world inference, and world prediction.

### 30.3. Reasoning Invariants

Reasoning retains: premises, hypotheses, evidence, counter-evidence, dependencies, confidence, and conclusion. This prevents a conclusion from becoming detached from its basis.

### 30.4. Verification Invariants

Verification SHALL never silently upgrade UNKNOWN to VERIFIED without satisfying the configured evidence conditions.

### 30.5. Learning Invariants

Learning SHALL be: bounded, attributable, policy-respecting, resource-limit-respecting, provenance-preserving, and change-recording.

---

## 31. Persistent Learning

After restart:

```
Previous Cognitive State
        ↓
Load cortex.cx
        ↓
Restore Language State
        ↓
Restore Neural State
        ↓
Restore Memory
        ↓
Restore World Model
        ↓
Restore Learning State
        ↓
Continue
```

CORTEX SHALL NOT reset to an empty model unless explicitly instructed to initialize a new state.

### 31.1. Cognitive State Growth

State may grow through: new vocabulary, new concepts, new relations, new episodes, new procedures, new world states, new hypotheses, new learned associations. Resource limits determine when state is compressed or forgotten.

### 31.2. Model Identity

A CORTEX instance is identified by: state identifier, architecture version, algorithm version, and configuration identity. The `.cx` state is the persistent identity-bearing computational state of the instance.

### 31.3. Architectural Change Boundary

Changing algorithm, representation, memory format, language representation, or neural topology requires an explicit architecture or algorithm version transition. The state format makes such transitions detectable.

---

## 32. Operational Lifecycle

### 32.1. Operational Sequence

```
Install binary
      ↓
Provide cortex.toml
      ↓
Provide API key if API enabled
      ↓
Run cortex
      ↓
Initialize / load cortex.cx
      ↓
READY
      ↓
Accept language input
      ↓
Process
      ↓
Respond
      ↓
Learn
      ↓
Persist
```

### 32.2. Shutdown

Graceful shutdown:

```
STOP ACCEPTING NEW WORK
  ↓
FINISH SAFE OPERATIONS
  ↓
CONSOLIDATE IF REQUIRED
  ↓
CHECKPOINT
  ↓
FLUSH
  ↓
VERIFY
  ↓
EXIT
```

Emergency shutdown may skip non-critical consolidation but SHALL attempt to preserve the last valid state.

### 32.3. Restart

```
Load Configuration
  ↓
Load .cx
  ↓
Verify Integrity
  ↓
Restore State
  ↓
Restore Algorithm Versions
  ↓
Restore Learning State
  ↓
Restore Memory
  ↓
Restore World Model
  ↓
READY
```

---

## 33. Observability

### 33.1. Runtime Observability

CORTEX exposes: status, uptime, memory utilization, neural utilization, language vocabulary size, episode count, knowledge count, world-model size, prediction error, learning status, consolidation status, and checkpoint status.

### 33.2. Internal Observability

Internal subsystem state is inspectable through controlled interfaces. Possible inspection: language statistics, memory statistics, world-model statistics, learning statistics, prediction error, reasoning statistics, verification statistics, and resource statistics. Sensitive internal structures SHALL NOT automatically become publicly writable.

### 33.3. Cognitive Metrics

Core metrics: prediction error, memory retrieval success, knowledge stability, verification confidence, reasoning consistency, planning success, language prediction quality, learning rate, forgetting rate, and consolidation rate. These metrics become part of the Self Model.

### 33.4. State Statistics

CORTEX can calculate: active cells, active columns, vocabulary size, memory occupancy, episode count, semantic concept count, procedural count, world entities, world relations, hypothesis count, verification count, and learning events.

### 33.5. Diagnostic State

The runtime maintains bounded diagnostics: last errors, error frequency, subsystem source, severity, recovery action, and timestamp. Diagnostics SHALL NOT become uncontrolled persistent memory.

---

## 34. Reproducibility

CORTEX records: random seed, architecture version, algorithm versions, configuration hash, state version, and runtime version. This allows experiments to identify why two runs diverged.

Where practical, the following are deterministic under identical conditions: state serialization, configuration interpretation, algorithm selection, memory indexing, verification rules, policy decisions, and checkpoint structure. Learning may remain stochastic if explicitly configured.

---

## 35. Algorithm Replacement

### 35.1. Algorithm Boundaries

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

Implementations can change without requiring architectural replacement of the entire system.

### 35.2. Algorithm Contract

Each algorithm defines: input state, output state, parameters, resource bounds, error conditions, version, determinism characteristics, and state compatibility.

---

## 36. Security Boundary

### 36.1. Security-Sensitive Resources

- API keys
- Policy configuration
- Runtime executable
- Filesystem access
- Network access
- Persistent state

These SHALL NOT be controlled solely by learned model output.

### 36.2. API Secret Handling

Secrets remain external to cognitive state: Environment → Runtime → Authentication. NOT: Secret → Memory → `.cx`.

### 36.3. Persistent State Security

`.cx` integrity is verified. Where configured, state may additionally use authenticated integrity, encryption, and access control without changing the cognitive architecture.

---

## 37. Testing Contract

### 37.1. Test Coverage Requirements

Testing SHALL cover: cell computation, column computation, temporal processing, language encoding, language generation, vocabulary learning, memory retrieval, memory consolidation, world transitions, reasoning, counterfactuals, planning, verification, learning stability, replay, persistence, corruption recovery, policy enforcement, API authentication, resource limits, and configuration validation.

### 37.2. Persistence Testing

The following invariant SHALL hold: `Save(State)` → `Load(State)` must produce a semantically equivalent cognitive state within defined serialization tolerances.

### 37.3. Learning Testing

Learning tests verify bounded behavior. Testing measures: learning direction, stability, retention, interference, and error reduction where applicable.

### 37.4. Regression Testing

Changing an algorithm SHALL test: state compatibility, memory compatibility, language behavior, reasoning behavior, verification behavior, learning stability, and `.cx` migration.

---

## 38. Repository Architecture

```
cortex/
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
│
├── src/
│   ├── main.rs
│   ├── cortex.rs
│   ├── config.rs
│   ├── error.rs
│   ├── runtime.rs
│   │
│   ├── language.rs
│   ├── tokenizer.rs
│   ├── vocabulary.rs
│   ├── syntax.rs
│   ├── semantics.rs
│   ├── language_model.rs
│   ├── decoder.rs
│   │
│   ├── neural.rs
│   ├── cell.rs
│   ├── column.rs
│   ├── field.rs
│   │
│   ├── memory.rs
│   ├── working_memory.rs
│   ├── episodic_memory.rs
│   ├── semantic_memory.rs
│   ├── procedural_memory.rs
│   ├── associative_memory.rs
│   │
│   ├── world.rs
│   ├── reasoning.rs
│   ├── planning.rs
│   ├── verification.rs
│   │
│   ├── learning.rs
│   ├── plasticity.rs
│   ├── replay.rs
│   ├── consolidation.rs
│   │
│   ├── self_model.rs
│   ├── internet.rs
│   ├── policy.rs
│   │
│   ├── format.rs
│   ├── persistence.rs
│   ├── checkpoint.rs
│   │
│   └── api.rs
│
├── cortex.toml
├── cortex.cx
└── README.md
```

### 38.1. Module Responsibility Contract

| Module | Responsibility |
|---|---|
| `cortex.rs` | Global orchestration |
| `config.rs` | Configuration parsing and validation |
| `error.rs` | Error taxonomy and handling |
| `runtime.rs` | Runtime lifecycle management |
| `language.rs` | Language Core orchestration |
| `tokenizer.rs` | Symbol and token encoding |
| `vocabulary.rs` | Dynamic vocabulary management |
| `syntax.rs` | Syntax representation |
| `semantics.rs` | Semantic representation |
| `language_model.rs` | Language prediction |
| `decoder.rs` | Language realization |
| `neural.rs` | Neural substrate orchestration |
| `cell.rs` | Cell computation |
| `column.rs` | Column computation |
| `field.rs` | Neural field management |
| `memory.rs` | Memory orchestration |
| `working_memory.rs` | Active state management |
| `episodic_memory.rs` | Experience storage |
| `semantic_memory.rs` | Knowledge storage |
| `procedural_memory.rs` | Procedure storage |
| `associative_memory.rs` | Association management |
| `world.rs` | World model |
| `reasoning.rs` | Hypothesis reasoning |
| `planning.rs` | Goal-directed planning |
| `verification.rs` | Evidence verification |
| `learning.rs` | Continual learning orchestration |
| `plasticity.rs` | Neural adaptation |
| `replay.rs` | Experience replay |
| `consolidation.rs` | Long-term adaptation |
| `self_model.rs` | Capability model |
| `internet.rs` | External observation |
| `policy.rs` | Risk and policy enforcement |
| `format.rs` | `.cx` format handling |
| `persistence.rs` | State persistence |
| `checkpoint.rs` | Checkpoint lifecycle |
| `api.rs` | Embedded API |

---

## 39. Core Data Model

### 39.1. Top-Level Structures

```rust
struct CortexState {
    language: LanguageState,
    neural: NeuralState,
    memory: MemoryState,
    world: WorldState,
    reasoning: ReasoningState,
    planning: PlanningState,
    verification: VerificationState,
    learning: LearningState,
    self_model: SelfModel,
    provenance: ProvenanceState,
    metadata: StateMetadata,
}

struct CortexRuntime {
    state: CortexState,
    policy: PolicyEngine,
    persistence: PersistenceEngine,
    configuration: CortexConfig,
}
```

### 39.2. Memory State

```rust
struct MemoryState {
    working: WorkingMemory,
    episodic: EpisodicMemory,
    semantic: SemanticMemory,
    procedural: ProceduralMemory,
    associative: AssociativeMemory,
}

struct EpisodicMemory {
    episodes: Vec<Episode>,
    capacity: usize,
    eviction_policy: EvictionPolicy,
}

struct SemanticMemory {
    knowledge: Vec<Knowledge>,
    capacity: usize,
    eviction_policy: EvictionPolicy,
}

struct ProceduralMemory {
    procedures: Vec<Procedure>,
    capacity: usize,
    eviction_policy: EvictionPolicy,
}

struct AssociativeMemory {
    associations: Vec<Association>,
    capacity: usize,
    index: AssociationIndex,
}
```

### 39.2.1. Neural Representation

```rust
struct NeuralRepresentation {
    active_cells: HashSet<CellId>,
    active_columns: HashSet<ColumnId>,
    field_activations: HashMap<FieldId, FieldActivation>,
    temporal_encoding: TemporalEncoding,
    prediction: Prediction,
    confidence: ConfidenceState,
}
```

### 39.2.2. Reasoning State

```rust
struct ReasoningState {
    active_hypotheses: Vec<Hypothesis>,
    conclusion: Option<Conclusion>,
    premises: Vec<Proposition>,
    evidence_index: EvidenceIndex,
    contradiction_log: Vec<Contradiction>,
    budget_remaining: u32,
}
```

### 39.2.3. Planning State

```rust
struct PlanningState {
    active_goals: Vec<Goal>,
    candidate_plans: Vec<Plan>,
    selected_plan: Option<Plan>,
    budget_remaining: u32,
    simulation_count: u32,
}
```

### 39.2.4. Verification State

```rust
struct VerificationState {
    pending_claims: Vec<KnowledgeClaim>,
    verified_claims: Vec<KnowledgeClaim>,
    contradicted_claims: Vec<KnowledgeClaim>,
    confidence_threshold: Scalar,
    evidence_requirements: EvidenceRequirements,
}
```

### 39.2.5. Provenance State

```rust
struct ProvenanceState {
    provenance_records: Vec<Provenance>,
    source_registry: HashMap<SourceId, SourceInfo>,
    total_observations: u64,
    total_inferences: u64,
}
```

### 39.2.6. State Metadata

```rust
struct StateMetadata {
    state_id: Uuid,
    created_at: Timestamp,
    last_updated: Timestamp,
    architecture_version: u32,
    algorithm_versions: AlgorithmVersions,
    config_hash: [u8; 32],
    episode_count: u64,
    total_learning_events: u64,
    checkpoint_count: u32,
}
```

### 39.3. Neural State

```rust
struct NeuralState {
    fields: Vec<Field>,
    active_cells: HashSet<CellId>,
    active_columns: HashSet<ColumnId>,
    temporal_buffer: TemporalBuffer,
    prediction_state: PredictionState,
}

struct Field {
    id: FieldId,
    columns: Vec<Column>,
    global_context: ContextVector,
    local_context: ContextVector,
    routing: RoutingState,
    competition: CompetitionState,
    temporal_state: TemporalState,
    prediction_state: PredictionState,
}

struct Column {
    id: ColumnId,
    cells: Vec<Cell>,
    context: ContextState,
    prediction: Prediction,
    activation: Scalar,
    competition: CompetitionState,
    routing: RoutingState,
}

struct Cell {
    id: CellId,
    state: CellState,
    activation: Scalar,
    context: ContextVector,
    prediction: PredictionVector,
    confidence: Scalar,
    plasticity: Scalar,
    connections: Connections,
}
```

### 39.4. Learning State

```rust
struct LearningState {
    total_learning_events: u64,
    total_replay_events: u64,
    total_consolidation_events: u64,
    average_prediction_error: Scalar,
    learning_rate: Scalar,
    plasticity_rate: Scalar,
    consolidation_threshold: Scalar,
}
```

---

## 40. Final Architectural Contract

CORTEX is defined as:

A native Rust, single-binary, persistent, continually learning AI model whose cognitive state consists of a native Language Core, Neural Core, Memory System, World Model, Reasoning Engine, Planning Engine, Verification Engine, Learning System, Consolidation System, Self Model, Policy/Risk Gate, and persistent `.cx` state.

### 40.1. Fundamental Architecture

```
LANGUAGE
    ↕
NEURAL REPRESENTATION
    ↕
MEMORY
    ↕
WORLD MODEL
    ↕
REASONING
    ↕
PLANNING
    ↕
VERIFICATION
    ↕
RESPONSE / ACTION
    ↕
EXPERIENCE
    ↕
PREDICTION ERROR
    ↕
LEARNING
    ↕
CONSOLIDATION
    ↕
PERSISTENT COGNITIVE STATE
```

### 40.2. Deployment Model

```
ONE BINARY
+
ONE CONFIGURATION
+
ONE COGNITIVE STATE
=
CORTEX
```

### 40.3. Learning Model

```
NO REQUIRED STATIC RETRAINING LOOP
        ↓
OBSERVATION
        ↓
EXPERIENCE
        ↓
PREDICTION
        ↓
ERROR
        ↓
ATTRIBUTION
        ↓
LOCAL ADAPTATION
        ↓
MEMORY
        ↓
WORLD MODEL
        ↓
LANGUAGE / REASONING ADAPTATION
        ↓
CONSOLIDATION
        ↓
.cx
```

### 40.4. Security Boundary

```
LEARNED COGNITIVE STATE
          ↓
     DECISION
          ↓
   POLICY / RISK GATE
          ↓
   ALLOW / LIMIT / DENY
```

### 40.5. Persistence Boundary

| Artifact | Role |
|---|---|
| `cortex.toml` | Operational configuration |
| `cortex.cx` | Persistent cognitive state |
| `cortex` | Complete executable system |

### 40.6. Ready-for-Use Definition

After deployment, the `cortex` binary is ready for use when it can:

1. Start and load configuration from `cortex.toml`
2. Load or initialize cognitive state from `cortex.cx`
3. Accept input through its CLI or embedded API
4. Process information through the full cognitive pipeline (Language Core → Neural Core → Memory → World Model → Reasoning → Planning → Verification)
5. Generate responses
6. Learn from experience and feedback
7. Persist learned state to `.cx`
8. Continue operation across restarts without requiring another AI model or external cognitive service

---

This specification constitutes the final architectural baseline for CORTEX. It defines the complete target system and the contracts between its subsystems; it is not a roadmap, phased development plan, MVP definition, or proposal for a later architecture.
