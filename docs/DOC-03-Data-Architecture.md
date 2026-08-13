# CORTEX — 03 Data & State Specification

---

## Document Control

| Property | Value |
|---|---|
| **Document ID** | CORTEX-DOC-03 |
| **Title** | Data & State Specification |
| **Version** | 1.1.0 |
| **Status** | Final Architectural Baseline |
| **Classification** | Data Contract |
| **Scope** | All data structures, state definitions, ownership, lifecycle, persistence |
| **Parent Document** | CORTEX-DOC-02 Software Design Specification |
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
| Data Architecture Lead | _____________ | _____________ |
| Persistence Lead | _____________ | _____________ |

### Document Purpose

This document defines **all data and state** used by the CORTEX architecture. It constitutes the data-level contract: what data exists, its concrete shape, who owns it, who may mutate it, how it transitions, how it is validated, and how it is persisted.

### Document Scope

This specification covers:

- Every data structure in the CORTEX system with complete field definitions.
- Every state type with ownership, mutability, and lifecycle rules.
- The `.cx` binary data layout with byte-level precision.
- Serialization, deserialization, integrity, and migration rules.
- Cross-subsystem data contracts and state invariants.
- Resource limits, validation rules, and corrupt-state handling.

This specification does NOT cover:

- Algorithm logic that operates on data (governed by algorithm documents).
- API request/response JSON schemas (governed by API specification).
- Configuration file syntax (governed by configuration specification).

---

## 1. Data Design Principles

| # | Principle | Implication |
|---|---|---|
| DDP-001 | Explicit typing | Every data element has a defined Rust type; no untyped blobs in cognitive state |
| DDP-002 | Ownership clarity | Every struct has exactly one owner; no shared mutable references without synchronization |
| DDP-003 | Immutable by default | Data is immutable unless mutation is explicitly required through `&mut` |
| DDP-004 | Provenance-preserving | Every knowledge item carries origin, timestamp, confidence, and evidence |
| DDP-005 | Bounded storage | Every collection has an explicit or derived capacity bound |
| DDP-006 | Version-aware | All persisted data carries version metadata for migration |
| DDP-007 | Fail-before-persist | Invalid data never reaches disk; validation precedes serialization |
| DDP-008 | Deterministic serialization | Same logical state always produces same serialized bytes |
| DDP-009 | Separation of identity | Each entity type has its own ID type; no cross-type ID reuse |
| DDP-010 | Scalar uniformity | All floating-point cognitive values use `Scalar` type alias |
| DDP-011 | No optional provenance | Provenance is NEVER optional on knowledge items |
| DDP-012 | No silent defaults | Missing data is an error, not silently defaulted |
| DDP-013 | Flat where possible | Prefer flat structures over deep nesting for serialization efficiency |
| DDP-014 | Enum over magic numbers | All categorical data uses Rust enums, not integer constants |
| DDP-015 | Time is explicit | All temporal data uses explicit `Timestamp`; no implicit "now" |

---

## 2. Type System

### 2.1 Type Categories

```
CORTEX Type System
├── Primitive Types (Scalar, String, bool, integers)
├── ID Types (CellId, EpisodeId, ConceptId, etc.)
├── Timestamp Types (Timestamp, Duration)
├── Enum Types (CellState, VerificationStatus, etc.)
├── Struct Types (Cell, Episode, Knowledge, etc.)
├── Collection Types (Vec, HashMap, HashSet, BoundedVec)
├── Composite State Types (CortexState, MemoryState, etc.)
└── Configuration Types (CortexConfig, ModelConfig, etc.)
```

### 2.2 Type Hierarchy

```
CortexState (root)
├── LanguageState
│   ├── SymbolSequence → Vec<Symbol>
│   ├── TokenSequence → Vec<Token>
│   ├── ConceptSet → Vec<ConceptId>
│   ├── EntitySet → Vec<EntityId>
│   ├── RelationSet → Vec<Relation>
│   ├── SyntaxGraph
│   ├── SemanticGraph
│   ├── ContextState
│   ├── IntentHypotheses
│   └── ConfidenceState
├── NeuralState
│   ├── Vec<Field>
│   │   └── Vec<Column>
│   │       └── Vec<Cell>
│   ├── HashSet<CellId>
│   ├── HashSet<ColumnId>
│   ├── TemporalBuffer
│   └── PredictionState
├── MemoryState
│   ├── WorkingMemory
│   ├── EpisodicMemory → Vec<Episode>
│   ├── SemanticMemory → Vec<Knowledge>
│   ├── ProceduralMemory → Vec<Procedure>
│   └── AssociativeMemory → Vec<Association>
├── WorldState
│   ├── EntitySet → Vec<Entity>
│   ├── RelationSet → Vec<Relation>
│   ├── EventSet → Vec<Event>
│   ├── TemporalContext
│   └── UncertaintyState
├── ReasoningState
├── PlanningState
├── VerificationState
├── LearningState
├── SelfModel
├── ProvenanceState
└── StateMetadata
```

---

## 3. Primitive & Scalar Types

### 3.1 Scalar Definition

```rust
/// The fundamental floating-point type for all cognitive computations.
/// Default: f32. Configurable to f16/bf16 via model.precision.
pub type Scalar = f32;

/// Precision configuration enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Precision {
    F32,
    F16,
    BF16,
}

impl Default for Precision {
    fn default() -> Self { Precision::F32 }
}
```

### 3.2 Primitive Type Usage

| Rust Type | Usage | Constraints |
|---|---|---|
| `Scalar` (f32) | All cognitive values: activation, confidence, strength, error, risk | Range varies by field; documented per-field |
| `u8` | Bytes, section type IDs, small enums | 0-255 |
| `u16` | Section type, section version | 0-65535 |
| `u32` | Counts, capacities, versions, flags | 0-4,294,967,295 |
| `u64` | Timestamps, episode counts, learning events, IDs | 0-18,446,744,073,709,551,615 |
| `u128` | Checksums | Full range |
| `usize` | Collection lengths, indices | Platform-dependent |
| `i64` | Signed offsets (rare) | Platform-dependent |
| `bool` | Flags, enabled states | true/false |
| `String` | Text content, names | UTF-8, bounded by context |
| `[u8; N]` | Fixed-size byte arrays (hashes, magic) | Exact size |
| `Uuid` | State identity | RFC 4122 v4 |

### 3.3 Scalar Constraints by Domain

| Domain | Min | Max | Default | Notes |
|---|---|---|---|---|
| Activation | 0.0 | 1.0 | 0.0 | Cell activation level |
| Confidence | 0.0 | 1.0 | 0.0 | Belief certainty |
| Strength | 0.0 | 1.0 | 0.0 | Association strength |
| Error magnitude | 0.0 | ∞ | 0.0 | Prediction error |
| Risk | 0.0 | 1.0 | 0.0 | Risk assessment |
| Importance | 0.0 | 1.0 | 0.5 | Episode importance |
| Learning rate | 0.0 | 1.0 | 0.001 | η parameter |
| Plasticity | 0.0 | 1.0 | 0.01 | Plasticity bound |
| Sparsity ratio | 0.0 | 1.0 | 0.05 | Active cell fraction |
| Cost | 0.0 | ∞ | 0.0 | Estimated plan cost |
| Utility | -∞ | ∞ | 0.0 | Plan utility |

---

## 4. ID Types

### 4.1 ID Design Rules

| Rule | Description |
|---|---|
| ID-001 | Each entity type has its own distinct ID newtype |
| ID-002 | IDs are `u64` internally |
| ID-003 | IDs are `Copy`, `Clone`, `PartialEq`, `Eq`, `Hash` |
| ID-004 | IDs are `Serialize`, `Deserialize` |
| ID-005 | ID 0 is reserved as "no ID" / "null" sentinel |
| ID-006 | IDs are monotonically increasing within a type |
| ID-007 | IDs are never reused after deletion |
| ID-008 | IDs are generated by the owning subsystem |

### 4.2 Complete ID Type Definitions

```rust
// types/ids.rs

macro_rules! define_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        pub struct $name(pub u64);

        impl $name {
            pub const NULL: Self = Self(0);
            
            pub fn new(value: u64) -> Self { Self(value) }
            
            pub fn is_null(&self) -> bool { self.0 == 0 }
            
            pub fn next(&self) -> Self { Self(self.0 + 1) }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({})", stringify!($name), self.0)
            }
        }
    };
}

// Neural IDs
define_id!(CellId);
define_id!(ColumnId);
define_id!(FieldId);

// Memory IDs
define_id!(EpisodeId);
define_id!(KnowledgeId);
define_id!(ProcedureId);
define_id!(AssociationId);
define_id!(MemoryId);       // Generic memory reference

// Language IDs
define_id!(SymbolId);
define_id!(TokenId);
define_id!(ConceptId);

// World IDs
define_id!(EntityId);
define_id!(RelationId);
define_id!(EventId);
define_id!(TransitionId);

// Reasoning IDs
define_id!(HypothesisId);
define_id!(EvidenceId);

// Planning IDs
define_id!(PlanId);
define_id!(GoalId);
define_id!(ActionId);

// Verification IDs
define_id!(ClaimId);

// Provenance IDs
define_id!(SourceId);
define_id!(ProvenanceId);

// Runtime IDs
define_id!(CheckpointId);
define_id!(SessionId);

// Internal reference (union type for associative memory)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InternalId {
    Cell(CellId),
    Column(ColumnId),
    Episode(EpisodeId),
    Concept(ConceptId),
    Entity(EntityId),
    Procedure(ProcedureId),
    Association(AssociationId),
    Hypothesis(HypothesisId),
    Symbol(SymbolId),
}
```

### 4.3 ID Generation Strategy

| Subsystem | ID Type | Generation |
|---|---|---|
| Neural Core | CellId, ColumnId, FieldId | Sequential from config at init |
| Episodic Memory | EpisodeId | Monotonic counter in LearningState |
| Semantic Memory | KnowledgeId | Monotonic counter |
| Procedural Memory | ProcedureId | Monotonic counter |
| Associative Memory | AssociationId | Monotonic counter |
| Language Core | SymbolId, ConceptId | Monotonic counter in Vocabulary |
| World Model | EntityId, RelationId, EventId | Monotonic counter |
| Reasoning | HypothesisId | Per-session counter |
| Provenance | SourceId | Monotonic counter in ProvenanceState |
| Persistence | CheckpointId | Monotonic counter in metadata |

---

## 5. Naming Rules

### 5.1 Type Naming

| Rule | Convention | Example |
|---|---|---|
| Structs | PascalCase | `CortexState`, `Episode`, `CellState` |
| Enums | PascalCase | `CellState`, `VerificationStatus` |
| Enum variants | PascalCase | `CellState::Active`, `VerificationStatus::Verified` |
| Type aliases | PascalCase | `Scalar`, `TokenSequence` |
| Traits | PascalCase | `LanguageCore`, `MemorySystem` |

### 5.2 Field Naming

| Rule | Convention | Example |
|---|---|---|
| Struct fields | snake_case | `prediction_error`, `confidence` |
| Collection fields | Plural or descriptive | `episodes`, `active_cells` |
| Boolean fields | `is_`, `has_`, or descriptive | `is_null`, `has_evidence`, `enabled` |
| Optional fields | `Option<T>` | `Option<Action>` |
| ID fields | Match type name | `id: EpisodeId` |
| Count fields | `_count` suffix | `episode_count`, `success_count` |
| Timestamp fields | Descriptive | `created_at`, `last_updated` |
| State fields | `_state` suffix | `temporal_state`, `prediction_state` |

### 5.3 Module Naming

| Rule | Convention | Example |
|---|---|---|
| Module files | snake_case | `working_memory.rs`, `episodic_memory.rs` |
| Module directories | snake_case | `language/`, `neural/`, `memory/` |
| Re-export module | `mod.rs` | `memory/mod.rs` |

---

## 6. Timestamp & Time Representation

### 6.1 Timestamp Type

```rust
/// Unix timestamp in milliseconds since epoch (UTC).
/// Used for all temporal ordering in CORTEX.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Timestamp(pub u64);

impl Timestamp {
    pub const ZERO: Self = Self(0);
    
    pub fn now() -> Self {
        Self(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64)
    }
    
    pub fn from_secs(secs: u64) -> Self { Self(secs * 1000) }
    
    pub fn as_secs(&self) -> u64 { self.0 / 1000 }
    
    pub fn as_millis(&self) -> u64 { self.0 }
    
    pub fn elapsed_since(&self, earlier: Timestamp) -> Duration {
        Duration(self.0.saturating_sub(earlier.0))
    }
    
    pub fn is_before(&self, other: Timestamp) -> bool { self.0 < other.0 }
    
    pub fn is_after(&self, other: Timestamp) -> bool { self.0 > other.0 }
}

/// Duration in milliseconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Duration(pub u64);

impl Duration {
    pub const ZERO: Self = Self(0);
    
    pub fn from_secs(secs: u64) -> Self { Self(secs * 1000) }
    pub fn from_millis(ms: u64) -> Self { Self(ms) }
    pub fn as_secs(&self) -> u64 { self.0 / 1000 }
    pub fn as_millis(&self) -> u64 { self.0 }
}
```

### 6.2 Timestamp Usage Rules

| Rule | Description |
|---|---|
| TIME-001 | All timestamps are UTC milliseconds since Unix epoch |
| TIME-002 | Timestamps are monotonically increasing within a session |
| TIME-003 | Timestamps are set at creation time; not modified |
| TIME-004 | Temporal ordering uses `Timestamp` comparison, not insertion order |
| TIME-005 | `Timestamp::ZERO` indicates "no timestamp" / "unknown" |
| TIME-006 | Duration is always non-negative |

### 6.3 Temporal Context

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalContext {
    pub current_time: Timestamp,
    pub sequence_position: u64,
    pub prior_states: Vec<Timestamp>,
    pub temporal_horizon: Duration,
}
```

---

## 7. Numeric Representation

### 7.1 Floating-Point Rules

| Rule | Description |
|---|---|
| NUM-001 | All cognitive scalars use `Scalar` type alias |
| NUM-002 | NaN is NEVER valid in persisted state |
| NUM-003 | Infinity is NEVER valid in persisted state |
| NUM-004 | All Scalar fields have documented valid ranges |
| NUM-005 | Comparison uses epsilon where appropriate: `|a - b| < EPSILON` |
| NUM-006 | Default epsilon: `1e-6` for f32 |

### 7.2 Integer Rules

| Rule | Description |
|---|---|
| NUM-007 | Counts use `u64` or `u32`; never negative |
| NUM-008 | Capacities use `usize` |
| NUM-009 | Versions use `u32` |
| NUM-010 | Checksums use `u128` |
| NUM-011 | Integer overflow SHALL panic in debug; saturate in release |

### 7.3 Scalar Validation

```rust
impl Scalar {
    pub fn is_valid_cognitive_value(self) -> bool {
        self.is_finite()
    }
    
    pub fn validate_range(self, min: Scalar, max: Scalar) -> Result<(), DataValidationError> {
        if !self.is_finite() {
            return Err(DataValidationError::NonFiniteValue);
        }
        if self < min || self > max {
            return Err(DataValidationError::OutOfRange { value: self, min, max });
        }
        Ok(())
    }
}

pub const SCALAR_EPSILON: Scalar = 1e-6;

pub fn scalar_eq(a: Scalar, b: Scalar) -> bool {
    (a - b).abs() < SCALAR_EPSILON
}
```

---

## 8. Core Data Structures

### 8.1 Context

```rust
/// Hierarchical context state influencing all cognitive operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextState {
    /// Current conversation/session identifier
    pub conversation_id: Option<u64>,
    
    /// Related historical episode references
    pub episode_context: Vec<EpisodeId>,
    
    /// Currently active concepts
    pub active_concepts: Vec<ConceptId>,
    
    /// Current world-state assumptions
    pub world_assumptions: Vec<EntityId>,
    
    /// Temporal context
    pub temporal_context: TemporalContext,
    
    /// Active intent hypotheses
    pub active_intents: Vec<IntentHypothesis>,
    
    /// Context window position
    pub window_position: u32,
    
    /// Total context tokens used
    pub tokens_used: u32,
}

impl ContextState {
    pub fn initial() -> Self {
        Self {
            conversation_id: None,
            episode_context: Vec::new(),
            active_concepts: Vec::new(),
            world_assumptions: Vec::new(),
            temporal_context: TemporalContext {
                current_time: Timestamp::now(),
                sequence_position: 0,
                prior_states: Vec::new(),
                temporal_horizon: Duration::from_secs(3600),
            },
            active_intents: Vec::new(),
            window_position: 0,
            tokens_used: 0,
        }
    }
    
    pub fn advance_time(&mut self) {
        self.temporal_context.current_time = Timestamp::now();
        self.temporal_context.sequence_position += 1;
    }
}
```

### 8.2 Observation

```rust
/// A single observation entering the cognitive pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    /// The observed text or content
    pub text: String,
    
    /// Source provenance
    pub source: Provenance,
    
    /// When the observation was made
    pub timestamp: Timestamp,
    
    /// Context at time of observation
    pub context: ContextState,
    
    /// Observation type
    pub kind: ObservationKind,
    
    /// Importance weighting
    pub importance: Scalar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObservationKind {
    /// Direct user input
    UserInput,
    /// Environment observation
    Environment,
    /// Internet-sourced observation
    Internet,
    /// Internal self-observation
    Internal,
    /// Feedback on previous action
    Feedback,
    /// Explicit teaching/correction
    Correction,
}

impl Observation {
    pub fn user_provided(text: &str) -> Self {
        Self {
            text: text.to_string(),
            source: Provenance::user_provided(),
            timestamp: Timestamp::now(),
            context: ContextState::initial(),
            kind: ObservationKind::UserInput,
            importance: 0.5,
        }
    }
    
    pub fn from_internet(text: &str, url: &str) -> Self {
        Self {
            text: text.to_string(),
            source: Provenance::internet(url),
            timestamp: Timestamp::now(),
            context: ContextState::initial(),
            kind: ObservationKind::Internet,
            importance: 0.3,
        }
    }
}
```

### 8.3 Action

```rust
/// An action that CORTEX may take or has taken.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    /// Unique action identifier
    pub id: ActionId,
    
    /// Action type
    pub kind: ActionKind,
    
    /// Action parameters
    pub parameters: HashMap<String, ActionParameter>,
    
    /// Expected outcome
    pub expected_outcome: Option<Outcome>,
    
    /// Risk assessment for this action
    pub risk: RiskAssessment,
    
    /// When the action was/will be taken
    pub timestamp: Timestamp,
    
    /// Provenance of the action decision
    pub provenance: Provenance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionKind {
    Respond,
    Observe,
    Query,
    Learn,
    Plan,
    Verify,
    Fetch,
    Store,
    Forget,
    Consolidate,
    Checkpoint,
    NoOp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionParameter {
    Text(String),
    Number(Scalar),
    Integer(i64),
    Boolean(bool),
    List(Vec<ActionParameter>),
}

/// Outcome of an action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outcome {
    /// Whether the action succeeded
    pub success: bool,
    
    /// Outcome description
    pub description: String,
    
    /// Observed result
    pub result: Option<String>,
    
    /// Timestamp of outcome observation
    pub timestamp: Timestamp,
    
    /// Confidence in outcome assessment
    pub confidence: Scalar,
}
```

### 8.4 Prediction

```rust
/// A prediction made by the neural core or world model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prediction {
    /// What is being predicted
    pub target: PredictionTarget,
    
    /// Predicted state/representation
    pub predicted_state: Vec<Scalar>,
    
    /// Confidence in the prediction
    pub confidence: Scalar,
    
    /// When the prediction was made
    pub timestamp: Timestamp,
    
    /// What context the prediction was based on
    pub context: ContextState,
    
    /// Whether this prediction has been resolved
    pub resolved: bool,
    
    /// The actual outcome (if resolved)
    pub actual: Option<Vec<Scalar>>,
    
    /// Prediction error (if resolved)
    pub error: Option<PredictionError>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PredictionTarget {
    NextToken,
    NextState,
    NextAction,
    Outcome,
    Transition,
    Intent,
}

/// Prediction error: difference between predicted and actual.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionError {
    /// Overall error magnitude
    pub magnitude: Scalar,
    
    /// Per-dimension error breakdown
    pub dimensions: HashMap<String, Scalar>,
    
    /// When the error was computed
    pub timestamp: Timestamp,
    
    /// The prediction that produced this error
    pub prediction_id: Option<u64>,
}

impl PredictionError {
    pub fn compute(predicted: &[Scalar], actual: &[Scalar]) -> Self {
        let raw_magnitude = predicted.iter()
            .zip(actual.iter())
            .map(|(p, a)| (p - a).powi(2))
            .sum::<Scalar>()
            .sqrt();
        
        // INV-PE-001: Normalize to [0.0, 1.0] using tanh
        let magnitude = raw_magnitude.tanh();
        
        Self {
            magnitude,
            dimensions: HashMap::new(),
            timestamp: Timestamp::now(),
            prediction_id: None,
        }
    }
    
    pub fn is_zero(&self) -> bool {
        self.magnitude < SCALAR_EPSILON
    }
}
```

### 8.5 Evidence

```rust
/// A single piece of evidence supporting or contradicting a claim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    /// Unique evidence identifier
    pub id: EvidenceId,
    
    /// Source of this evidence
    pub source: Provenance,
    
    /// Evidence content
    pub content: EvidenceContent,
    
    /// Strength of this evidence (0.0 to 1.0)
    pub strength: Scalar,
    
    /// Whether this evidence supports or contradicts
    pub polarity: EvidencePolarity,
    
    /// When this evidence was recorded
    pub timestamp: Timestamp,
    
    /// Related evidence IDs
    pub related: Vec<EvidenceId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvidenceContent {
    Text(String),
    Observation(Observation),
    KnowledgeRef(KnowledgeId),
    EpisodeRef(EpisodeId),
    Numeric(Scalar),
    Composite(Vec<EvidenceContent>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidencePolarity {
    Supports,
    Contradicts,
    Neutral,
}

/// A collection of evidence items.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EvidenceSet {
    pub items: Vec<Evidence>,
}

impl EvidenceSet {
    pub fn new() -> Self { Self { items: Vec::new() } }
    
    pub fn add(&mut self, evidence: Evidence) {
        self.items.push(evidence);
    }
    
    pub fn total_strength(&self) -> Scalar {
        self.items.iter().map(|e| e.strength).sum::<Scalar>()
            / self.items.len().max(1) as Scalar
    }
    
    pub fn supporting(&self) -> Vec<&Evidence> {
        self.items.iter()
            .filter(|e| e.polarity == EvidencePolarity::Supports)
            .collect()
    }
    
    pub fn contradicting(&self) -> Vec<&Evidence> {
        self.items.iter()
            .filter(|e| e.polarity == EvidencePolarity::Contradicts)
            .collect()
    }
    
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
    
    pub fn len(&self) -> usize { self.items.len() }
    
    pub fn merge(&self, other: &EvidenceSet) -> EvidenceSet {
        let mut merged = self.clone();
        merged.items.extend(other.items.clone());
        merged
    }
}
```

### 8.6 Provenance

> **Canonical definition:** See DOC-00 §2.1. Provenance tracks origin only. It does NOT contain verification_status or confidence — those are separate dimensions tracked on the KnowledgeClaim.

```rust
/// Complete provenance information for any knowledge item.
/// Provenance is NEVER optional.
/// INV-PV-001: Every knowledge mutation SHALL preserve provenance.
/// INV-PV-002: Provenance SHALL NOT be modified after creation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provenance {
    /// Category of origin
    pub category: ProvenanceCategory,
    
    /// Source description
    pub source: Source,
    
    /// Source identity
    pub source_identity: SourceIdentity,
    
    /// When the knowledge was acquired
    pub timestamp: Timestamp,
    
    /// Context at time of acquisition
    pub retrieval_context: Option<RetrievalContext>,
    
    /// Content hash for integrity (BLAKE3-256)
    /// INV-PV-003: Provenance SHALL record source, timestamp, and content hash
    pub content_hash: [u8; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProvenanceCategory {
    Observed,
    UserProvided,
    Internet,
    Derived,
    Inferred,
    Replayed,
    Verified,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub id: SourceId,
    pub name: String,
    pub kind: SourceKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceKind {
    User,
    System,
    Internet,
    Derived,
    Internal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceIdentity {
    pub identifier: String,
    pub reliability: Scalar,
    pub verification_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievalContext {
    pub query: String,
    pub timestamp: Timestamp,
    pub session_id: SessionId,
}

impl Provenance {
    pub fn user_provided() -> Self {
        Self {
            category: ProvenanceCategory::UserProvided,
            source: Source { id: SourceId(1), name: "user".into(), kind: SourceKind::User },
            source_identity: SourceIdentity { identifier: "user".into(), reliability: 0.8, verification_count: 0 },
            timestamp: Timestamp::now(),
            retrieval_context: None,
            content_hash: [0u8; 32],
            evidence: EvidenceSet::new(),
            verification_status: VerificationStatus::Observed,
            confidence: ConfidenceState::default(),
        }
    }
    
    pub fn internet(url: &str) -> Self {
        Self {
            category: ProvenanceCategory::Internet,
            source: Source { id: SourceId(2), name: url.to_string(), kind: SourceKind::Internet },
            source_identity: SourceIdentity { identifier: url.to_string(), reliability: 0.3, verification_count: 0 },
            timestamp: Timestamp::now(),
            retrieval_context: None,
            content_hash: [0u8; 32],
            evidence: EvidenceSet::new(),
            verification_status: VerificationStatus::Unknown,
            confidence: ConfidenceState::low(),
        }
    }
    
    pub fn derived(parents: &[Provenance]) -> Self {
        Self {
            category: ProvenanceCategory::Derived,
            source: Source { id: SourceId(3), name: "derived".into(), kind: SourceKind::Derived },
            source_identity: SourceIdentity { identifier: "derived".into(), reliability: 0.5, verification_count: 0 },
            timestamp: Timestamp::now(),
            retrieval_context: None,
            content_hash: [0u8; 32],
            evidence: parents.iter().flat_map(|p| p.evidence.items.clone()).collect(),
            verification_status: VerificationStatus::Inferred,
            confidence: ConfidenceState::default(),
        }
    }
}
```

### 8.7 Confidence & Uncertainty

> **Canonical definition:** See DOC-00 §2.1. Confidence and verification status are separate dimensions. ConfidenceState does NOT contain verification_status.

```rust
/// Multi-dimensional confidence state.
/// Does NOT contain verification_status — that is a separate dimension.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceState {
    /// Overall belief strength (0.0 to 1.0)
    pub belief: Scalar,
    
    /// Strength of supporting evidence (0.0 to 1.0)
    pub evidence_strength: Scalar,
    
    /// Quality of sources (0.0 to 1.0)
    pub source_quality: Scalar,
    
    /// Internal consistency (0.0 to 1.0)
    pub consistency: Scalar,
    
    /// Uncertainty level (0.0 to 1.0, higher = more uncertain)
    /// INV-CF-003: belief + uncertainty = 1.0
    pub uncertainty: Scalar,
    
    /// Historical prediction reliability (0.0 to 1.0)
    pub prediction_reliability: Scalar,
}

impl ConfidenceState {
    pub fn default() -> Self {
        Self {
            belief: 0.5,
            evidence_strength: 0.0,
            source_quality: 0.5,
            consistency: 0.5,
            uncertainty: 0.5,
            prediction_reliability: 0.0,
        }
    }
    
    pub fn low() -> Self {
        Self {
            belief: 0.1,
            evidence_strength: 0.0,
            source_quality: 0.1,
            consistency: 0.1,
            uncertainty: 0.9,
            prediction_reliability: 0.0,
            verification_status: VerificationStatus::Unknown,
        }
    }
    
    pub fn high() -> Self {
        Self {
            belief: 0.9,
            evidence_strength: 0.8,
            source_quality: 0.9,
            consistency: 0.9,
            uncertainty: 0.1,
            prediction_reliability: 0.8,
            verification_status: VerificationStatus::Supported,
        }
    }
    
    pub fn from_encoding(semantics: &SemanticGraph) -> Self {
        Self {
            belief: 0.7,
            evidence_strength: 0.5,
            source_quality: 0.7,
            consistency: 0.7,
            uncertainty: 0.3,
            prediction_reliability: 0.0,
            verification_status: VerificationStatus::Observed,
        }
    }
    
    pub fn from_neural(field_state: &FieldState) -> Self {
        Self {
            belief: field_state.average_activation,
            evidence_strength: 0.5,
            source_quality: 0.5,
            consistency: field_state.coherence,
            uncertainty: 1.0 - field_state.coherence,
            prediction_reliability: 0.0,
            verification_status: VerificationStatus::Inferred,
        }
    }
    
    /// Compute overall confidence score
    pub fn overall(&self) -> Scalar {
        (self.belief * 0.3)
            + (self.evidence_strength * 0.25)
            + (self.source_quality * 0.15)
            + (self.consistency * 0.2)
            + ((1.0 - self.uncertainty) * 0.1)
    }
    
    pub fn is_verified(&self) -> bool {
        self.verification_status == VerificationStatus::Verified
    }
}

/// Uncertainty state for world model and predictions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UncertaintyState {
    /// Overall uncertainty level (0.0 to 1.0)
    pub level: Scalar,
    
    /// Per-dimension uncertainty
    pub dimensions: HashMap<String, Scalar>,
    
    /// Whether uncertainty is reducible with more evidence
    pub reducible: bool,
    
    /// Last update time
    pub updated_at: Timestamp,
}

impl UncertaintyState {
    pub fn initial() -> Self {
        Self {
            level: 1.0,
            dimensions: HashMap::new(),
            reducible: true,
            updated_at: Timestamp::now(),
        }
    }
}
```

---

## 9. Memory Data Model

### 9.1 Working Memory

```rust
/// Active cognitive state. Bounded by memory.working_mb.
/// Owner: MemorySystem. Mutability: mutable during cognitive loop.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingMemory {
    /// Current input being processed
    pub input: Option<CurrentInput>,
    
    /// Current conversation context
    pub conversation_context: ConversationContext,
    
    /// Currently active concepts
    pub active_concepts: Vec<ConceptId>,
    
    /// Currently active hypotheses
    pub active_hypotheses: Vec<HypothesisId>,
    
    /// Current goals
    pub goals: Vec<GoalId>,
    
    /// Snapshot of current reasoning state
    pub reasoning_state: Option<ReasoningSnapshot>,
    
    /// Current world-state assumptions
    pub world_assumptions: Vec<EntityId>,
    
    /// Current generation state (if generating)
    pub generation_state: Option<GenerationState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentInput {
    pub text: String,
    pub timestamp: Timestamp,
    pub kind: ObservationKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationContext {
    pub session_id: SessionId,
    pub turn_count: u64,
    pub recent_inputs: Vec<String>,
    pub recent_outputs: Vec<String>,
    pub started_at: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningSnapshot {
    pub active_hypotheses: Vec<HypothesisId>,
    pub current_step: u32,
    pub budget_remaining: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationState {
    pub tokens_generated: u32,
    pub max_tokens: u32,
    pub current_candidates: Vec<CandidateContinuation>,
}
```

### 9.2 Episodic Memory

```rust
/// Episodic memory: stores experience episodes.
/// Bounded by memory.episodic_mb.
/// Owner: MemorySystem. Mutability: append-only, consolidation may remove.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodicMemory {
    /// All stored episodes
    pub episodes: Vec<Episode>,
    
    /// Maximum capacity in bytes
    pub capacity_bytes: u64,
    
    /// Current usage in bytes
    pub current_usage_bytes: u64,
    
    /// Eviction policy
    pub eviction_policy: EvictionPolicy,
    
    /// Next episode ID
    pub next_id: EpisodeId,
}

/// A single experience episode.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    /// Unique episode identifier
    pub id: EpisodeId,
    
    /// The observation that triggered this episode
    pub observation: Observation,
    
    /// Context at time of episode
    pub context: ContextState,
    
    /// Action taken (if any)
    pub action: Option<Action>,
    
    /// Outcome observed (if any)
    pub outcome: Option<Outcome>,
    
    /// When the episode occurred
    pub timestamp: Timestamp,
    
    /// Prediction made before the episode
    pub prediction: Option<Prediction>,
    
    /// Prediction error for this episode
    pub prediction_error: PredictionError,
    
    /// Confidence in episode representation
    pub confidence: ConfidenceState,
    
    /// Source provenance
    pub source: Provenance,
    
    /// Importance weighting (0.0 to 1.0)
    pub importance: Scalar,
    
    /// Number of times this episode has been retrieved
    pub retrieval_count: u64,
    
    /// Last retrieval time
    pub last_retrieved: Option<Timestamp>,
    
    /// Whether this episode has been consolidated
    pub consolidated: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvictionPolicy {
    /// Evict least recently used
    LRU,
    /// Evict lowest importance
    LowestImportance,
    /// Evict lowest confidence
    LowestConfidence,
    /// Evict oldest
    Oldest,
    /// Composite scoring
    Composite,
}
```

### 9.3 Semantic Memory

```rust
/// Semantic memory: stores knowledge.
/// Bounded by memory.semantic_mb.
/// Owner: MemorySystem. Mutability: mutable (knowledge is revisable).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticMemory {
    /// All stored knowledge items
    pub knowledge: Vec<Knowledge>,
    
    /// Maximum capacity in bytes
    pub capacity_bytes: u64,
    
    /// Current usage in bytes
    pub current_usage_bytes: u64,
    
    /// Eviction policy
    pub eviction_policy: EvictionPolicy,
    
    /// Next knowledge ID
    pub next_id: KnowledgeId,
}

/// A single knowledge item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Knowledge {
    /// Unique knowledge identifier
    pub id: KnowledgeId,
    
    /// The concept this knowledge is about
    pub concept: ConceptId,
    
    /// Properties of the concept
    pub properties: Vec<Property>,
    
    /// Relations to other concepts
    pub relations: Vec<Relation>,
    
    /// Evidence supporting this knowledge
    pub evidence: EvidenceSet,
    
    /// Confidence in this knowledge
    pub confidence: ConfidenceState,
    
    /// Provenance set (may have multiple sources)
    pub provenance: Vec<Provenance>,
    
    /// Verification status
    pub verification_status: VerificationStatus,
    
    /// When this knowledge was first acquired
    pub created_at: Timestamp,
    
    /// When this knowledge was last updated
    pub updated_at: Timestamp,
    
    /// Number of times this knowledge has been confirmed
    pub confirmation_count: u64,
    
    /// Number of times this knowledge has been contradicted
    pub contradiction_count: u64,
}

/// A property of a concept.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    pub value: PropertyValue,
    pub confidence: Scalar,
    pub provenance: Provenance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyValue {
    Text(String),
    Number(Scalar),
    Boolean(bool),
    ConceptRef(ConceptId),
    EntityRef(EntityId),
    List(Vec<PropertyValue>),
}

/// A relation between concepts/entities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub id: RelationId,
    pub kind: RelationKind,
    pub source: InternalId,
    pub target: InternalId,
    pub confidence: Scalar,
    pub provenance: Provenance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationKind {
    IsA,
    HasProperty,
    PartOf,
    Causes,
    Requires,
    Enables,
    Contradicts,
    Supports,
    RelatedTo,
    TemporalBefore,
    TemporalAfter,
    SpatialNear,
    AgentOf,
    ObjectOf,
    RecipientOf,
}
```

### 9.4 Procedural Memory

```rust
/// Procedural memory: stores procedures/skills.
/// Bounded by memory.procedural_mb.
/// Owner: MemorySystem. Mutability: mutable (success/failure counts update).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralMemory {
    /// All stored procedures
    pub procedures: Vec<Procedure>,
    
    /// Maximum capacity in bytes
    pub capacity_bytes: u64,
    
    /// Current usage in bytes
    pub current_usage_bytes: u64,
    
    /// Next procedure ID
    pub next_id: ProcedureId,
}

/// A single procedure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Procedure {
    /// Unique procedure identifier
    pub id: ProcedureId,
    
    /// Condition under which this procedure applies
    pub condition: Condition,
    
    /// Ordered steps of the procedure
    pub steps: Vec<Action>,
    
    /// Expected outcome when procedure succeeds
    pub expected_outcome: Outcome,
    
    /// Number of successful applications
    pub success_count: u64,
    
    /// Number of failed applications
    pub failure_count: u64,
    
    /// Confidence in this procedure (derived from success/failure)
    pub confidence: Scalar,
    
    /// Context requirements for this procedure
    pub context_requirements: ContextRequirements,
    
    /// Risk assessment for this procedure
    pub risk: RiskAssessment,
    
    /// Provenance
    pub provenance: Provenance,
    
    /// When this procedure was learned
    pub created_at: Timestamp,
    
    /// Last time this procedure was used
    pub last_used: Option<Timestamp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub description: String,
    pub required_concepts: Vec<ConceptId>,
    pub required_entities: Vec<EntityId>,
    pub required_context: Option<ContextState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextRequirements {
    pub requires_world_model: bool,
    pub requires_memory: bool,
    pub requires_reasoning: bool,
    pub max_context_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    /// Risk score (0.0 to 1.0)
    pub score: Scalar,
    
    /// Risk level
    pub level: RiskLevel,
    
    /// Risk factors
    pub factors: Vec<RiskFactor>,
    
    /// Reversibility (0.0 to 1.0)
    pub reversibility: Scalar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    None,
    Low,
    Moderate,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    pub description: String,
    pub severity: Scalar,
    pub likelihood: Scalar,
}
```

### 9.5 Associative Memory

```rust
/// Associative memory: stores typed associations between internal structures.
/// Bounded by memory.associative_mb.
/// Owner: MemorySystem. Mutability: mutable (strength updates).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssociativeMemory {
    /// All stored associations
    pub associations: Vec<Association>,
    
    /// Maximum capacity in bytes
    pub capacity_bytes: u64,
    
    /// Current usage in bytes
    pub current_usage_bytes: u64,
    
    /// Index for fast lookup
    pub index: AssociationIndex,
    
    /// Next association ID
    pub next_id: AssociationId,
}

/// A single association.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Association {
    /// Unique association identifier
    pub id: AssociationId,
    
    /// Source of the association
    pub source: InternalId,
    
    /// Target of the association
    pub target: InternalId,
    
    /// Type of association
    pub kind: AssociationKind,
    
    /// Association strength (0.0 to 1.0)
    pub strength: Scalar,
    
    /// Confidence in this association
    pub confidence: Scalar,
    
    /// Context in which this association is relevant
    pub context: ContextState,
    
    /// Provenance
    pub provenance: Provenance,
    
    /// When this association was formed
    pub created_at: Timestamp,
    
    /// Last time this association was strengthened
    pub last_strengthened: Timestamp,
    
    /// Number of times this association has been activated
    pub activation_count: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssociationKind {
    Semantic,
    Temporal,
    Contextual,
    Causal,
    Episodic,
    Procedural,
}

/// Index for fast association lookup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssociationIndex {
    /// Forward index: source → association IDs
    pub forward: HashMap<InternalId, Vec<AssociationId>>,
    
    /// Backward index: target → association IDs
    pub backward: HashMap<InternalId, Vec<AssociationId>>,
    
    /// Kind index: kind → association IDs
    pub by_kind: HashMap<AssociationKind, Vec<AssociationId>>,
}
```

### 9.6 Memory State (Composite)

```rust
/// Complete memory state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryState {
    pub working: WorkingMemory,
    pub episodic: EpisodicMemory,
    pub semantic: SemanticMemory,
    pub procedural: ProceduralMemory,
    pub associative: AssociativeMemory,
}

/// Memory usage report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryUsage {
    pub working_bytes: u64,
    pub episodic_bytes: u64,
    pub semantic_bytes: u64,
    pub procedural_bytes: u64,
    pub associative_bytes: u64,
    pub total_bytes: u64,
    pub pressure: MemoryPressure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryPressure {
    Low,
    Moderate,
    High,
    Critical,
}
```

---

## 10. World Model Data

```rust
/// World state: persistent snapshot stored in .cx.
/// Owner: WorldModel subsystem. Mutability: mutable through observe/integrate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldState {
    /// All known entities
    pub entities: Vec<Entity>,
    
    /// All known relations
    pub relations: Vec<Relation>,
    
    /// Currently active events
    pub active_events: Vec<Event>,
    
    /// Temporal context
    pub temporal_context: TemporalContext,
    
    /// Overall uncertainty
    pub uncertainty: UncertaintyState,
    
    /// Next entity ID
    pub next_entity_id: EntityId,
    
    /// Next relation ID
    pub next_relation_id: RelationId,
    
    /// Next event ID
    pub next_event_id: EventId,
}

/// An entity in the world model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    /// Unique entity identifier
    pub id: EntityId,
    
    /// Entity kind
    pub kind: EntityKind,
    
    /// Identity state
    pub identity: IdentityState,
    
    /// Entity properties
    pub properties: Vec<Property>,
    
    /// Current entity state
    pub state: EntityState,
    
    /// Relations involving this entity
    pub relations: Vec<RelationId>,
    
    /// Confidence in this entity's existence
    pub confidence: Scalar,
    
    /// Provenance set
    pub provenance: Vec<Provenance>,
    
    /// When this entity was first observed
    pub created_at: Timestamp,
    
    /// When this entity was last updated
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityKind {
    Person,
    Object,
    Place,
    Organization,
    ConceptualObject,
    Event,
    System,
    Process,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityState {
    pub name: String,
    pub aliases: Vec<String>,
    pub unique_identifier: Option<String>,
    pub identity_confidence: Scalar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityState {
    pub state_description: String,
    pub state_properties: Vec<Property>,
    pub state_timestamp: Timestamp,
    pub state_confidence: Scalar,
}

/// An event in the world model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: EventId,
    pub description: String,
    pub participants: Vec<EntityId>,
    pub timestamp: Timestamp,
    pub duration: Option<Duration>,
    pub outcome: Option<Outcome>,
    pub provenance: Provenance,
}

/// Predicted state from transition model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictedState {
    pub predicted_entities: Vec<Entity>,
    pub predicted_relations: Vec<Relation>,
    pub confidence: Scalar,
    pub uncertainty: Scalar,
    pub prediction_horizon: u32,
}

/// Simulated trajectory for planning/counterfactuals.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulatedTrajectory {
    pub steps: Vec<WorldState>,
    pub actions: Vec<Action>,
    pub confidence: Scalar,
    pub uncertainty: Scalar,
    pub is_counterfactual: bool,
}
```

---

## 11. Reasoning State

```rust
/// Reasoning state: tracks active reasoning process.
/// Owner: ReasoningEngine. Mutability: mutable during reasoning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningState {
    /// Currently active hypotheses
    pub active_hypotheses: Vec<Hypothesis>,
    
    /// Current conclusion (if reached)
    pub conclusion: Option<Conclusion>,
    
    /// Premises used in reasoning
    pub premises: Vec<Proposition>,
    
    /// Evidence index for fast lookup
    pub evidence_index: HashMap<HypothesisId, Vec<EvidenceId>>,
    
    /// Detected contradictions
    pub contradiction_log: Vec<Contradiction>,
    
    /// Remaining reasoning budget
    pub budget_remaining: u32,
    
    /// Next hypothesis ID
    pub next_hypothesis_id: HypothesisId,
}

/// A hypothesis in the reasoning workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hypothesis {
    /// Unique hypothesis identifier
    pub id: HypothesisId,
    
    /// The proposition being hypothesized
    pub proposition: Proposition,
    
    /// Supporting evidence
    pub evidence: EvidenceSet,
    
    /// Contradicting evidence
    pub counter_evidence: EvidenceSet,
    
    /// Confidence in this hypothesis
    pub confidence: Scalar,
    
    /// Dependencies (other hypotheses this depends on)
    pub dependencies: Vec<HypothesisId>,
    
    /// Contradictions with other hypotheses
    pub contradictions: Vec<Contradiction>,
    
    /// Provenance set
    pub provenance: Vec<Provenance>,
    
    /// Reasoning type that generated this hypothesis
    pub reasoning_type: ReasoningType,
    
    /// When this hypothesis was generated
    pub created_at: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposition {
    pub subject: InternalId,
    pub predicate: String,
    pub object: Option<InternalId>,
    pub modifiers: Vec<String>,
    pub negated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conclusion {
    pub hypothesis_id: HypothesisId,
    pub proposition: Proposition,
    pub confidence: Scalar,
    pub evidence_strength: Scalar,
    pub reasoning_steps: u32,
    pub bounded: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contradiction {
    pub claim_a: HypothesisId,
    pub claim_b: HypothesisId,
    pub description: String,
    pub severity: Scalar,
    pub detected_at: Timestamp,
    pub resolved: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReasoningType {
    Deductive,
    Inductive,
    Abductive,
    Analogical,
    Temporal,
    Causal,
    Counterfactual,
    Constraint,
    Consistency,
}

/// Result of reasoning evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningResult {
    pub hypotheses: Vec<Hypothesis>,
    pub contradictions: Vec<Contradiction>,
    pub budget_remaining: u32,
    pub conclusion: Option<Conclusion>,
}
```

---

## 12. Planning State

```rust
/// Planning state: tracks active planning process.
/// Owner: PlanningEngine. Mutability: mutable during planning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanningState {
    /// Currently active goals
    pub active_goals: Vec<Goal>,
    
    /// Candidate plans being evaluated
    pub candidate_plans: Vec<Plan>,
    
    /// Selected plan (if any)
    pub selected_plan: Option<Plan>,
    
    /// Remaining planning budget
    pub budget_remaining: u32,
    
    /// Number of simulations performed
    pub simulation_count: u32,
    
    /// Next plan ID
    pub next_plan_id: PlanId,
    
    /// Next goal ID
    pub next_goal_id: GoalId,
}

/// A goal for planning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: GoalId,
    pub description: String,
    pub target_state: Option<WorldState>,
    pub priority: Scalar,
    pub deadline: Option<Timestamp>,
    pub status: GoalStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GoalStatus {
    Active,
    Achieved,
    Failed,
    Abandoned,
}

/// A plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    /// Unique plan identifier
    pub id: PlanId,
    
    /// The goal this plan achieves
    pub goal: GoalId,
    
    /// Ordered steps
    pub steps: Vec<Action>,
    
    /// Predicted outcomes for each step
    pub predicted_outcomes: Vec<Outcome>,
    
    /// Estimated cost
    pub estimated_cost: Scalar,
    
    /// Estimated risk
    pub estimated_risk: Scalar,
    
    /// Uncertainty in plan success
    pub uncertainty: Scalar,
    
    /// Confidence in plan success
    pub confidence: Scalar,
    
    /// When this plan was created
    pub created_at: Timestamp,
    
    /// Plan status
    pub status: PlanStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlanStatus {
    Candidate,
    Selected,
    Executing,
    Completed,
    Failed,
    Abandoned,
}
```

---

## 13. Verification State

```rust
/// Verification state: tracks verification process.
/// Owner: VerificationEngine. Mutability: mutable during verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationState {
    /// Claims pending verification
    pub pending_claims: Vec<KnowledgeClaim>,
    
    /// Claims that have been verified
    pub verified_claims: Vec<KnowledgeClaim>,
    
    /// Claims that have been contradicted
    pub contradicted_claims: Vec<KnowledgeClaim>,
    
    /// Confidence threshold for verification
    pub confidence_threshold: Scalar,
    
    /// Evidence requirements for verification
    pub evidence_requirements: EvidenceRequirements,
    
    /// Next claim ID
    pub next_claim_id: ClaimId,
}

/// A knowledge claim subject to verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeClaim {
    /// Unique claim identifier
    pub id: ClaimId,
    
    /// The claim content
    pub proposition: Proposition,
    
    /// Evidence supporting the claim
    pub evidence: EvidenceSet,
    
    /// Evidence contradicting the claim
    pub counter_evidence: EvidenceSet,
    
    /// Current verification status
    pub status: VerificationStatus,
    
    /// Confidence in the claim
    pub confidence: ConfidenceState,
    
    /// Provenance
    pub provenance: Provenance,
    
    /// When the claim was made
    pub claimed_at: Timestamp,
    
    /// When the claim was last verified/checked
    pub last_verified: Option<Timestamp>,
    
    /// Number of verification attempts
    pub verification_attempts: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationStatus {
    Unknown,      // Ordinal 0
    Observed,     // Ordinal 1
    Inferred,     // Ordinal 2
    Supported,    // Ordinal 3
    Provisional,  // Ordinal 4
    Verified,     // Ordinal 5
    Contradicted, // Ordinal -1 (serialized specially)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceRequirements {
    /// Minimum number of independent evidence sources
    pub min_independent_sources: u32,
    
    /// Minimum evidence strength
    pub min_evidence_strength: Scalar,
    
    /// Minimum source quality
    pub min_source_quality: Scalar,
    
    /// Whether contradictions must be resolved
    pub require_no_contradictions: bool,
}

/// Result of verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiedResult {
    pub claim: KnowledgeClaim,
    pub verification_status: VerificationStatus,
    pub confidence: ConfidenceState,
    pub evidence: EvidenceSet,
}
```

---

## 14. Learning State

```rust
/// Learning state: tracks learning statistics and parameters.
/// Owner: LearningSystem. Mutability: mutable during learning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningState {
    /// Total learning events processed
    pub total_learning_events: u64,
    
    /// Total replay events processed
    pub total_replay_events: u64,
    
    /// Total consolidation events
    pub total_consolidation_events: u64,
    
    /// Average prediction error (moving average)
    pub average_prediction_error: Scalar,
    
    /// Current learning rate
    pub learning_rate: Scalar,
    
    /// Current plasticity rate
    pub plasticity_rate: Scalar,
    
    /// Consolidation threshold
    pub consolidation_threshold: Scalar,
    
    /// Replay buffer
    pub replay_buffer: Vec<EpisodeId>,
    
    /// Maximum replay buffer size
    pub replay_buffer_capacity: usize,
    
    /// Next consolidation episode count
    pub next_consolidation_at: u64,
    
    /// Learning history (bounded)
    pub history: BoundedVec<LearningEvent, 1000>,
}

/// A single learning event record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningEvent {
    pub timestamp: Timestamp,
    pub signal_magnitude: Scalar,
    pub attribution: ErrorAttribution,
    pub subsystem: String,
    pub applied: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorAttribution {
    InputError,
    MemoryError,
    WorldError,
    ReasoningError,
    ProcedureError,
    EnvironmentError,
}

/// Bounded vector: fixed-capacity ring buffer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundedVec<T, const N: usize> {
    items: Vec<T>,
    capacity: usize,
}

impl<T, const N: usize> BoundedVec<T, N> {
    pub fn new() -> Self {
        Self { items: Vec::with_capacity(N), capacity: N }
    }
    
    pub fn push(&mut self, item: T) {
        if self.items.len() >= self.capacity {
            self.items.remove(0);
        }
        self.items.push(item);
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }
    
    pub fn len(&self) -> usize { self.items.len() }
    
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
}
```

---

## 15. Plasticity State

```rust
/// Plasticity state: tracks neural adaptation parameters.
/// Owner: NeuralCore (plasticity module). Mutability: mutable during learning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlasticityState {
    /// Current learning rate (η)
    pub learning_rate: Scalar,
    
    /// Current plasticity bound
    pub plasticity_bound: Scalar,
    
    /// Total weight updates applied
    pub total_updates: u64,
    
    /// Average update magnitude
    pub average_update_magnitude: Scalar,
    
    /// Maximum update magnitude observed
    pub max_update_magnitude: Scalar,
    
    /// Whether plasticity is currently enabled
    pub enabled: bool,
    
    /// Last plasticity application time
    pub last_applied: Option<Timestamp>,
}

impl PlasticityState {
    /// Compute bounded weight update: ΔW = η × A × C × E × V
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
        
        // Bounded: prevent single observation from destabilizing
        delta.clamp(-self.plasticity_bound, self.plasticity_bound)
    }
}
```

---

## 16. Replay/Consolidation State

```rust
/// Replay state: tracks experience replay process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayState {
    /// Episodes queued for replay
    pub queue: Vec<EpisodeId>,
    
    /// Episodes replayed in current session
    pub replayed_this_session: u64,
    
    /// Total episodes replayed
    pub total_replayed: u64,
    
    /// Replay budget for current cycle
    pub budget_remaining: u32,
}

/// Consolidation state: tracks long-term memory formation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationState {
    /// Candidates pending consolidation
    pub pending_candidates: Vec<ConsolidationCandidate>,
    
    /// Total consolidations performed
    pub total_consolidations: u64,
    
    /// Last consolidation time
    pub last_consolidation: Option<Timestamp>,
    
    /// Next consolidation trigger (episode count)
    pub next_trigger: u64,
    
    /// Consolidation statistics
    pub stats: ConsolidationStats,
}

/// A candidate for consolidation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationCandidate {
    /// Target memory type
    pub target: ConsolidationTarget,
    
    /// Knowledge to consolidate
    pub knowledge: Option<Knowledge>,
    
    /// Procedure to consolidate
    pub procedure: Option<Procedure>,
    
    /// Supporting episodes
    pub supporting_episodes: Vec<EpisodeId>,
    
    /// Number of supporting episodes
    pub episode_count: usize,
    
    /// Pattern strength (0.0 to 1.0)
    pub pattern_strength: Scalar,
    
    /// Evidence strength
    pub evidence_strength: Scalar,
    
    /// Contradiction risk
    pub contradiction_risk: Scalar,
    
    /// When this candidate was identified
    pub identified_at: Timestamp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsolidationTarget {
    Semantic,
    Procedural,
    Associative,
}

/// Consolidation statistics.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConsolidationStats {
    pub semantic_integrations: u64,
    pub procedural_integrations: u64,
    pub associative_integrations: u64,
    pub merges: u64,
    pub compressions: u64,
    pub generalizations: u64,
    pub rejections: u64,
}

/// Result of consolidation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConsolidationResult {
    pub consolidated: u64,
    pub rejected: u64,
    pub merged: u64,
    pub semantic_integrations: u64,
    pub procedural_integrations: u64,
}

/// Forgetting policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgettingPolicy {
    pub min_importance: Scalar,
    pub min_confidence: Scalar,
    pub max_age: Option<Duration>,
    pub min_retrieval_count: u64,
    pub allow_contradicted: bool,
    pub aggressive: bool,
}

/// Result of forgetting operation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ForgettingResult {
    pub episodic_forgotten: u64,
    pub semantic_forgotten: u64,
    pub associative_forgotten: u64,
    pub procedural_forgotten: u64,
    pub bytes_freed: u64,
}
```

---

## 17. Self Model State

```rust
/// Self model: CORTEX's computational self-assessment.
/// Owner: SelfModel subsystem. Mutability: mutable through update().
/// NOT a conscious state. NOT authoritative over policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfModel {
    /// Estimated capabilities
    pub capabilities: CapabilitySet,
    
    /// Known limitations
    pub limitations: LimitationSet,
    
    /// Overall prediction accuracy (0.0 to 1.0)
    pub prediction_accuracy: Scalar,
    
    /// Current uncertainty state
    pub uncertainty: UncertaintyState,
    
    /// Memory health assessment
    pub memory_health: MemoryHealth,
    
    /// Language capability assessment
    pub language_capability: LanguageCapability,
    
    /// Reasoning performance assessment
    pub reasoning_performance: ReasoningPerformance,
    
    /// Resource state
    pub resource_state: ResourceState,
    
    /// Learning statistics
    pub learning_statistics: LearningStatistics,
    
    /// Historical performance (bounded)
    pub historical_performance: BoundedVec<PerformanceSnapshot, 100>,
    
    /// Last update time
    pub last_updated: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilitySet {
    pub language_accuracy: Scalar,
    pub prediction_accuracy: Scalar,
    pub verification_reliability: Scalar,
    pub planning_success: Scalar,
    pub memory_retrieval_success: Scalar,
    pub reasoning_consistency: Scalar,
    pub resource_availability: Scalar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitationSet {
    pub known_limitations: Vec<String>,
    pub resource_constraints: Vec<String>,
    pub capability_gaps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryHealth {
    pub pressure: MemoryPressure,
    pub fragmentation: Scalar,
    pub consolidation_backlog: u64,
    pub eviction_rate: Scalar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageCapability {
    pub vocabulary_size: usize,
    pub accuracy: Scalar,
    pub confidence: Scalar,
    pub unknown_word_rate: Scalar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningPerformance {
    pub consistency: Scalar,
    pub confidence: Scalar,
    pub average_steps: Scalar,
    pub contradiction_rate: Scalar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceState {
    pub memory_available_bytes: u64,
    pub memory_total_bytes: u64,
    pub compute_available: bool,
    pub network_available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningStatistics {
    pub total_events: u64,
    pub average_error: Scalar,
    pub learning_rate_effective: Scalar,
    pub consolidation_rate: Scalar,
    pub forgetting_rate: Scalar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSnapshot {
    pub timestamp: Timestamp,
    pub prediction_accuracy: Scalar,
    pub memory_pressure: MemoryPressure,
    pub learning_events: u64,
    pub reasoning_steps: u32,
}
```

---

## 18. Policy/Risk State

```rust
/// Policy state: non-learned boundary.
/// Owner: PolicyEngine. Mutability: ONLY through administrative operation.
/// Learning SHALL NOT modify this state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyState {
    /// Whether learning is enabled
    pub learning_enabled: bool,
    
    /// Whether internet learning is enabled
    pub internet_learning_enabled: bool,
    
    /// Whether self-modification is allowed
    pub self_modification_allowed: bool,
    
    /// Whether policy modification is allowed
    pub policy_modification_allowed: bool,
    
    /// Whether runtime modification is allowed
    pub runtime_modification_allowed: bool,
    
    /// Risk thresholds
    pub risk_thresholds: RiskThresholds,
    
    /// Operation constraints
    pub operation_constraints: HashMap<String, OperationConstraints>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskThresholds {
    /// Maximum acceptable risk for automatic approval
    pub auto_approve_below: Scalar,
    
    /// Risk level requiring limitation
    pub limit_above: Scalar,
    
    /// Risk level requiring denial
    pub deny_above: Scalar,
}

/// Proposed operation for policy evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposedOperation {
    /// Operation classification
    pub classification: OperationClass,
    
    /// Operation description
    pub description: String,
    
    /// Target subsystem
    pub target: String,
    
    /// Estimated impact
    pub estimated_impact: Scalar,
    
    /// Reversibility
    pub reversibility: Scalar,
    
    /// Resource consumption estimate
    pub resource_estimate: Scalar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperationClass {
    CognitiveStateAdaptation,
    AlgorithmAdaptation,
    SecurityPolicyModification,
    RuntimeModification,
}

/// Policy decision.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyDecision {
    Allowed,
    Limited { constraints: OperationConstraints },
    Denied { reason: DenialReason },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationConstraints {
    pub max_magnitude: Scalar,
    pub max_scope: u32,
    pub requires_confirmation: bool,
    pub timeout: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DenialReason {
    LearningDisabled,
    SelfModificationDisabled,
    PolicyModificationDisabled,
    RuntimeModificationDisabled,
    CriticalRisk,
    InsufficientConfidence,
    PolicyViolation,
    ResourceExhaustion,
}
```

---

## 19. Internet Interface State

```rust
/// Internet interface state.
/// Owner: InternetInterface. Mutability: mutable during fetch operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternetState {
    /// Whether internet access is enabled
    pub enabled: bool,
    
    /// Total requests made
    pub total_requests: u64,
    
    /// Total successful requests
    pub successful_requests: u64,
    
    /// Total failed requests
    pub failed_requests: u64,
    
    /// Total bytes received
    pub total_bytes_received: u64,
    
    /// Last request time
    pub last_request: Option<Timestamp>,
    
    /// Last request result
    pub last_result: Option<NetworkObservation>,
    
    /// Pending observations from internet
    pub pending_observations: Vec<Observation>,
}

/// Network request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkRequest {
    pub url: String,
    pub method: HttpMethod,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub timeout: Duration,
    pub max_response_bytes: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HttpMethod {
    Get,
    Post,
}

/// Network observation result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkObservation {
    pub content: String,
    pub status: u16,
    pub timestamp: Timestamp,
    pub source_url: String,
    pub content_hash: [u8; 32],
    pub size_bytes: u64,
}

/// Extracted content from network response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedContent {
    pub text: String,
    pub metadata: HashMap<String, String>,
    pub extracted_at: Timestamp,
}
```

---

## 20. Runtime State

```rust
/// Runtime state machine state.
/// Owner: Runtime. Mutability: mutable through state transitions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    Fault { error_kind: String },
    Recovery,
    SafeStop,
    Shutdown,
}

/// Runtime status report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeStatus {
    pub state: String,
    pub uptime_seconds: u64,
    pub memory_usage: MemoryUsage,
    pub episode_count: u64,
    pub prediction_error: Scalar,
    pub learning_enabled: bool,
    pub world_model_size: usize,
    pub language_vocabulary_size: usize,
    pub checkpoint_count: u32,
    pub last_checkpoint: Option<Timestamp>,
}

/// Runtime event triggering state transitions.
#[derive(Debug, Clone)]
pub enum RuntimeEvent {
    ConfigLoaded,
    StateLoaded,
    StateValidated,
    Initialized,
    Ready,
    InputReceived,
    ProcessingComplete,
    LearningComplete,
    ConsolidationComplete,
    CheckpointComplete,
    ShutdownRequested,
    FatalError(String),
    RecoveryPossible,
    RecoveryComplete,
    RecoveryFailed,
    RecoveryImpossible,
}
```

---

## 21. Configuration State

```rust
/// Complete configuration. Immutable after boot.
/// Owner: Config module. Mutability: NONE after parsing.
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
    pub cells: u32,
    pub columns: u32,
    pub dimension: u32,
    pub precision: Precision,
    pub sparsity_ratio: Scalar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageConfig {
    pub enabled: bool,
    pub vocabulary_capacity: u32,
    pub context_window: u32,
    pub generation_limit: u32,
    pub learning: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    pub working_mb: u32,
    pub episodic_mb: u32,
    pub semantic_mb: u32,
    pub procedural_mb: u32,
    pub associative_mb: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningConfig {
    pub enabled: bool,
    pub learning_rate: Scalar,
    pub plasticity: Scalar,
    pub replay: bool,
    pub consolidation_interval: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldConfig {
    pub enabled: bool,
    pub prediction_horizon: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningConfig {
    pub enabled: bool,
    pub max_steps: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanningConfig {
    pub enabled: bool,
    pub max_depth: u32,
    pub max_branches: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationConfig {
    pub enabled: bool,
    pub minimum_confidence: Scalar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternetConfig {
    pub enabled: bool,
    pub timeout_seconds: u32,
    pub max_response_mb: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyConfig {
    pub learning: bool,
    pub internet_learning: bool,
    pub self_modification: bool,
    pub policy_modification: bool,
    pub runtime_modification: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub enabled: bool,
    pub bind: String,
    pub api_key_env: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistenceConfig {
    pub state: String,
    pub checkpoint_interval: u64,
}
```

---

## 22. Persistence State

```rust
/// Persistence metadata.
/// Owner: PersistenceEngine. Mutability: mutable during save/load.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistenceState {
    /// Path to .cx file
    pub state_path: String,
    
    /// Path to checkpoint directory
    pub checkpoint_dir: String,
    
    /// Last save time
    pub last_save: Option<Timestamp>,
    
    /// Last save result
    pub last_save_result: Option<SaveResult>,
    
    /// Checkpoint history
    pub checkpoints: Vec<CheckpointMetadata>,
    
    /// Maximum checkpoints to retain
    pub max_checkpoints: usize,
}

/// Save operation result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveResult {
    pub bytes_written: u64,
    pub checksum: u128,
    pub duration_ms: u64,
    pub timestamp: Timestamp,
}

/// Checkpoint metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointMetadata {
    pub id: CheckpointId,
    pub state_version: u32,
    pub algorithm_version: u32,
    pub config_hash: [u8; 32],
    pub timestamp: Timestamp,
    pub episode_count: u64,
    pub learning_state: LearningState,
    pub integrity_checksum: u128,
    pub file_path: String,
    pub file_size_bytes: u64,
}

/// Validation result for .cx file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub format_version: u32,
    pub architecture_version: u32,
}
```

---

## 23. `.cx` Data Layout

### 23.1 Binary Format Specification

```
BYTE OFFSET    SIZE    FIELD                   TYPE
─────────────────────────────────────────────────────────
0x0000         8       magic                   [u8; 8] = b"CORTEX\0\0"
0x0008         4       format_version          u32 (LE)
0x000C         4       architecture_version    u32 (LE)
0x0010         4       algorithm_version       u32 (LE)
0x0014         32      config_hash             [u8; 32] (BLAKE3-256)
0x0034         16      state_id                [u8; 16] (UUID v4)
0x0044         8       created_at              u64 (LE, ms since epoch)
0x004C         8       last_checkpoint         u64 (LE, ms since epoch)
0x0054         4       section_count           u32 (LE)
0x0058         32      integrity_metadata      IntegrityMetadata
0x0078         32      reserved                [u8; 32]
─────────────────────────────────────────────────────────
0x0098         ...     SECTION TABLE           (section_count entries)
─────────────────────────────────────────────────────────
                       SECTION DATA            (variable)
─────────────────────────────────────────────────────────
```

### 23.2 Section Table Entry

```
BYTE OFFSET    SIZE    FIELD                   TYPE
─────────────────────────────────────────────────────────
+0x00          2       section_type            u16 (LE)
+0x02          2       section_version         u16 (LE)
+0x04          4       flags                   u32 (LE)
+0x08          8       offset                  u64 (LE)
+0x10          8       length                  u64 (LE)
+0x18          16      checksum                u128 (LE)
─────────────────────────────────────────────────────────
Total: 40 bytes per section entry
```

### 23.3 Section Type IDs

| Section | Type ID | Version |
|---|---|---|
| ARCHITECTURE | 0x0001 | 1 |
| LANGUAGE | 0x0002 | 1 |
| NEURAL | 0x0003 | 1 |
| CELLS | 0x0004 | 1 |
| COLUMNS | 0x0005 | 1 |
| FIELDS | 0x0006 | 1 |
| WORKING_MEMORY | 0x0007 | 1 |
| EPISODIC_MEMORY | 0x0008 | 1 |
| SEMANTIC_MEMORY | 0x0009 | 1 |
| PROCEDURAL_MEMORY | 0x000A | 1 |
| ASSOCIATIVE_MEMORY | 0x000B | 1 |
| WORLD_MODEL | 0x000C | 1 |
| REASONING | 0x000D | 1 |
| PLANNING | 0x000E | 1 |
| VERIFICATION | 0x000F | 1 |
| LEARNING | 0x0010 | 1 |
| SELF_MODEL | 0x0011 | 1 |
| PROVENANCE | 0x0012 | 1 |
| CHECKPOINT_METADATA | 0x0013 | 1 |
| INTEGRITY | 0x0014 | 1 |

### 23.4 IntegrityMetadata Structure

```
BYTE OFFSET    SIZE    FIELD                   TYPE
─────────────────────────────────────────────────────────
+0x00          16      file_checksum           u128 (LE)
+0x10          4       checksum_algorithm      u32 (LE)  // 2 = BLAKE3 truncated
+0x14          4       compression_algorithm   u32 (LE)  // 0 = none, 1 = zstd
+0x18          8       uncompressed_size       u64 (LE)
+0x20          8       compressed_size         u64 (LE)
─────────────────────────────────────────────────────────
Total: 40 bytes
```

### 23.5 Byte Order

All multi-byte integers use **little-endian** byte order.

### 23.6 Compression

Section data is compressed using **zstd** at level 3 (default). Compression is applied per-section, not to the entire file.

---

## 24. Serialization Rules

### 24.1 Serialization Pipeline

```
CortexState (in-memory)
    │
    ↓
┌─────────────────────────────────────┐
│  1. Validate invariants             │
│  2. Serialize each section           │
│  3. Compress each section (zstd)    │
│  4. Compute per-section checksum    │
│  5. Build section table             │
│  6. Compute file checksum           │
│  7. Write header                    │
│  8. Write section table             │
│  9. Write section data              │
│  10. Flush to disk                  │
└─────────────────────────────────────┘
    │
    ↓
cortex.cx (on-disk)
```

### 24.2 Serialization Format

| Data Type | Serialization Format |
|---|---|
| Scalars (f32) | IEEE 754 little-endian, 4 bytes |
| Integers (u8-u128) | Little-endian, native size |
| Strings | Length-prefixed UTF-8: `[u32 length][bytes]` |
| Vec<T> | Length-prefixed: `[u64 count][T elements...]` |
| HashMap<K,V> | Length-prefixed: `[u64 count][(K,V) pairs...]` |
| Option<T> | Tag byte: `0x00 = None`, `0x01 = Some(T)` |
| Enum | Tag byte + variant data |
| Struct | Fields in declaration order |
| bool | Single byte: `0x00 = false`, `0x01 = true` |
| [u8; N] | Raw bytes, N bytes |
| Uuid | 16 bytes, RFC 4122 layout |
| Timestamp | u64 little-endian |

### 24.3 Serialization Invariants

| Rule | Description |
|---|---|
| SER-001 | Same logical state MUST produce same serialized bytes |
| SER-002 | Field order in structs is declaration order |
| SER-003 | HashMap serialization order is sorted by key |
| SER-004 | No padding bytes in serialized output |
| SER-005 | All strings are valid UTF-8 |
| SER-006 | NaN/Infinity are NEVER serialized |
| SER-007 | Compression is deterministic for same input |

---

## 25. Deserialization Rules

### 25.1 Deserialization Pipeline

```
cortex.cx (on-disk)
    │
    ↓
┌─────────────────────────────────────┐
│  1. Read header (fixed size)        │
│  2. Verify magic bytes              │
│  3. Check format version            │
│  4. Verify file checksum            │
│  5. Read section table              │
│  6. For each section:               │
│     a. Read section data            │
│     b. Verify section checksum      │
│     c. Decompress                   │
│     d. Deserialize                  │
│     e. Validate section data        │
│  7. Validate cross-section refs     │
│  8. Validate state invariants       │
│  9. Construct CortexState           │
└─────────────────────────────────────┘
    │
    ↓
CortexState (in-memory)
```

### 25.2 Deserialization Error Handling

| Error | Response |
|---|---|
| Invalid magic | Reject file; attempt recovery |
| Version mismatch | Attempt migration |
| Checksum failure | Reject section; attempt recovery from checkpoint |
| Decompression failure | Reject section; attempt recovery |
| Invalid UTF-8 | Reject file; attempt recovery |
| NaN/Infinity found | Reject file; attempt recovery |
| Cross-reference invalid | Reject file; attempt recovery |
| Invariant violation | Reject file; attempt recovery |

---

## 26. State Ownership

### 26.1 Ownership Matrix

| Data Structure | Owner | Location |
|---|---|---|
| `CortexState` | `CortexRuntime` | `cortex.rs` |
| `LanguageState` | `LanguageCoreImpl` | `language/mod.rs` |
| `NeuralState` | `NeuralCoreImpl` | `neural/mod.rs` |
| `MemoryState` | `MemorySystemImpl` | `memory/mod.rs` |
| `WorkingMemory` | `MemorySystemImpl` | `memory/working.rs` |
| `EpisodicMemory` | `MemorySystemImpl` | `memory/episodic.rs` |
| `SemanticMemory` | `MemorySystemImpl` | `memory/semantic.rs` |
| `ProceduralMemory` | `MemorySystemImpl` | `memory/procedural.rs` |
| `AssociativeMemory` | `MemorySystemImpl` | `memory/associative.rs` |
| `WorldState` | `WorldModelImpl` | `world/mod.rs` |
| `ReasoningState` | `ReasoningEngineImpl` | `reasoning/mod.rs` |
| `PlanningState` | `PlanningEngineImpl` | `planning/mod.rs` |
| `VerificationState` | `VerificationEngineImpl` | `verification/mod.rs` |
| `LearningState` | `LearningSystemImpl` | `learning/mod.rs` |
| `SelfModel` | `SelfModelImpl` | `self_model/mod.rs` |
| `PolicyState` | `PolicyEngineImpl` | `policy/mod.rs` |
| `InternetState` | `InternetInterfaceImpl` | `internet/mod.rs` |
| `ProvenanceState` | `ProvenanceTracker` | `provenance.rs` |
| `StateMetadata` | `CortexRuntime` | `cortex.rs` |
| `CortexConfig` | `CortexRuntime` (immutable) | `config.rs` |
| `RuntimeState` | `CortexRuntime` | `runtime.rs` |
| `PersistenceState` | `PersistenceEngineImpl` | `persistence/mod.rs` |

### 26.2 Ownership Rules

| Rule | Description |
|---|---|
| OWN-001 | Each data structure has exactly ONE owner |
| OWN-002 | Owner has `&mut` access; all others have `&` (read-only) |
| OWN-003 | Cross-subsystem data access is through method calls, not direct field access |
| OWN-004 | `CortexRuntime` owns `CortexState` and orchestrates all mutations |
| OWN-005 | `CortexConfig` is immutable after boot; no owner mutates it |
| OWN-006 | `PolicyState` is owned by `PolicyEngine`; learning CANNOT mutate it |
| OWN-007 | Background tasks receive cloned snapshots, not references |

---

## 27. State Mutability

### 27.1 Mutability Matrix

| Data Structure | Mutability | Mutation Trigger | Mutation Boundary |
|---|---|---|---|
| `LanguageState` | Mutable | Learning signal, vocabulary expansion | Bounded by vocabulary_capacity |
| `NeuralState` | Mutable | Plasticity updates | Bounded by plasticity_bound |
| `WorkingMemory` | Mutable | Every cognitive operation | Bounded by working_mb |
| `EpisodicMemory` | Append + eviction | New episodes, forgetting | Bounded by episodic_mb |
| `SemanticMemory` | Mutable | Consolidation, learning | Bounded by semantic_mb |
| `ProceduralMemory` | Mutable | Procedure learning, usage | Bounded by procedural_mb |
| `AssociativeMemory` | Mutable | Association formation, strengthening | Bounded by associative_mb |
| `WorldState` | Mutable | Observation, integration | Unbounded (bounded by memory) |
| `ReasoningState` | Mutable (transient) | During reasoning only | Bounded by max_steps |
| `PlanningState` | Mutable (transient) | During planning only | Bounded by max_depth × max_branches |
| `VerificationState` | Mutable | During verification | Bounded by evidence |
| `LearningState` | Mutable | During learning | Counters only |
| `SelfModel` | Mutable | Performance updates | Bounded by history capacity |
| `PolicyState` | **Immutable** (normal) | Administrative operation ONLY | Level 3 restriction |
| `CortexConfig` | **Immutable** | Never after boot | N/A |
| `StateMetadata` | Mutable | Counters, timestamps | Counters only |
| `ProvenanceState` | Append-only | New observations | Unbounded (bounded by memory) |

### 27.2 Mutation Rules

| Rule | Description |
|---|---|
| MUT-001 | All mutations go through owner's methods |
| MUT-002 | No direct field mutation from outside owner |
| MUT-003 | Transient state (Reasoning, Planning) resets after operation |
| MUT-004 | PolicyState mutation requires Level 3 authorization |
| MUT-005 | CortexConfig is NEVER mutated after boot |
| MUT-006 | Every mutation is validated before application |
| MUT-007 | Failed mutations do not partially apply |
| MUT-008 | Mutations that violate invariants are rejected |

---

## 28. State Lifecycle

### 28.1 Complete State Lifecycle

```
CREATION (first boot)
    │
    ↓
INITIALIZATION
    │  - All subsystems create initial state
    │  - StateMetadata created with new UUID
    │  - Algorithm versions recorded
    │  - Config hash computed
    │
    ↓
PERSISTENCE (initial save)
    │  - cortex.cx created
    │
    ↓
OPERATION (steady state)
    │  - Cognitive loop processes inputs
    │  - State mutates through learning
    │  - Periodic checkpoints
    │  - Periodic saves
    │
    ↓
SHUTDOWN
    │  - Final consolidation
    │  - Final checkpoint
    │  - Final save
    │  - Verify
    │
    ↓
RESTART
    │  - Load cortex.cx
    │  - Verify integrity
    │  - Validate invariants
    │  - Restore state
    │  - Continue operation
    │
    ↓
MIGRATION (on version change)
    │  - Detect version mismatch
    │  - Apply migration
    │  - Validate migrated state
    │  - Continue operation
    │
    ↓
CORRUPTION RECOVERY (on failure)
    │  - Detect corruption
    │  - Attempt checkpoint recovery
    │  - If no valid checkpoint: initialize new state
    │  - Log corruption event
```

### 28.2 State Creation Rules

| Rule | Description |
|---|---|
| LC-001 | State is created exactly once per instance (first boot) |
| LC-002 | State identity (UUID) is assigned at creation and never changes |
| LC-003 | State is NEVER deleted except by explicit `cortex init` |
| LC-004 | State survives restarts; loading is the default path |
| LC-005 | State migration preserves semantic content |
| LC-006 | Corrupt state triggers recovery, never silent continuation |

---

## 29. State Transition Constraints

### 29.1 Valid State Transitions

| From State | To State | Condition |
|---|---|---|
| Boot | LoadConfiguration | Config file exists |
| LoadConfiguration | LoadState | Config valid |
| LoadState | Validate | State loaded or initialized |
| Validate | Initialize | State valid |
| Initialize | Ready | All subsystems initialized |
| Ready | Processing | Input received |
| Processing | Learning | Processing complete |
| Learning | Consolidating | Learning complete |
| Consolidating | Checkpointing | Consolidation complete |
| Checkpointing | Ready | Checkpoint complete |
| Ready | Shutdown | Shutdown requested |
| Any | Fault | Fatal error |
| Fault | Recovery | Recovery possible |
| Fault | SafeStop | Recovery impossible |
| Recovery | Ready | Recovery successful |
| Recovery | SafeStop | Recovery failed |

### 29.2 Invalid Transitions

| From | To | Reason |
|---|---|---|
| Ready | Boot | Cannot re-boot |
| Processing | Ready | Must complete learning first |
| Fault | Ready | Must go through Recovery |
| Shutdown | Any | Terminal state |
| SafeStop | Any | Terminal state |

### 29.3 Data Transition Constraints

| Constraint | Description |
|---|---|
| TC-001 | Episode count only increases (except forgetting) |
| TC-002 | Learning event count only increases |
| TC-003 | Vocabulary size only increases (except explicit pruning) |
| TC-004 | Checkpoint count only increases |
| TC-005 | State ID never changes |
| TC-006 | Architecture version only increases |
| TC-007 | Algorithm versions only increase |
| TC-008 | Timestamps are monotonically increasing within session |
| TC-009 | Verification status transitions follow the matrix defined in DOC-00 §5.2 and DOC-03 §40.1 |
| TC-010 | Verification status can regress to Contradicted with evidence |
| TC-011 | PolicyState changes require Level 3 authorization |
| TC-012 | Config hash changes require restart |

---

## 30. State Invariants

### 30.1 Complete Invariant List

| # | Invariant | Enforcement Point |
|---|---|---|
| INV-001 | All memory references point to existing items | Before persistence, after load |
| INV-002 | Neural topology is valid (cells in columns, columns in fields) | Before persistence, after load |
| INV-003 | All vocabulary references are valid | Before persistence, after load |
| INV-004 | World-model relationships reference existing entities | Before persistence, after load |
| INV-005 | All knowledge items have provenance | Before persistence, after load |
| INV-006 | Algorithm versions are consistent | Before persistence, after load |
| INV-007 | Policy state is valid | Before persistence, after load |
| INV-008 | `.cx` structure is valid | After load |
| INV-009 | No NaN or Infinity in any Scalar field | Before serialization |
| INV-010 | All timestamps are non-zero where required | Before persistence |
| INV-011 | Confidence values are in [0, 1] | Before persistence |
| INV-012 | Active cell count ≤ field_size × sparsity_ratio | After neural processing |
| INV-013 | Memory usage ≤ configured budget | After mutation |
| INV-014 | Verification status transitions are valid | After verification |
| INV-015 | State ID matches loaded state ID | After load |
| INV-016 | Config hash matches current config | After load |
| INV-017 | Episode IDs are unique | After episode creation |
| INV-018 | Association source ≠ target | After association creation |
| INV-019 | Plan steps ≤ max_depth | After plan creation |
| INV-020 | Reasoning steps ≤ max_steps | During reasoning |

### 30.2 Invariant Enforcement

```rust
impl CortexState {
    pub fn validate_invariants(&self) -> Result<(), CortexError> {
        self.validate_memory_references()?;
        self.validate_neural_topology()?;
        self.validate_vocabulary_references()?;
        self.validate_world_relationships()?;
        self.validate_provenance()?;
        self.validate_algorithm_versions()?;
        self.validate_policy_state()?;
        self.validate_scalars()?;
        self.validate_timestamps()?;
        self.validate_confidence_ranges()?;
        self.validate_sparsity()?;
        self.validate_memory_budgets()?;
        self.validate_verification_transitions()?;
        self.validate_ids()?;
        Ok(())
    }
}
```

---

## 31. Cross-Subsystem Data Contracts

### 31.1 Data Flow Contracts

| Producer | Consumer | Data Type | Contract |
|---|---|---|---|
| LanguageCore | NeuralCore | `LanguageState` | Valid tokens, syntax, semantics; non-empty |
| NeuralCore | MemorySystem | `NeuralRepresentation` | Valid cell/column IDs; sparse activation |
| MemorySystem | WorldModel | `MemoryRetrieval` | Ranked memories with provenance |
| WorldModel | ReasoningEngine | `WorldState` | Valid entities, relations |
| ReasoningEngine | PlanningEngine | `ReasoningResult` | Ranked hypotheses with evidence |
| PlanningEngine | VerificationEngine | `Plan` (optional) | Valid steps, risk |
| ReasoningEngine | VerificationEngine | `ReasoningResult` | Claims with evidence |
| VerificationEngine | LanguageCore | `VerifiedResult` | Status, confidence, claim |
| LanguageCore | Output | `GeneratedResponse` | Text, confidence, status |
| Experience | LearningSystem | `Experience` | Observation, prediction, error |
| LearningSystem | All subsystems | `LearningSignal` | Bounded, attributed |
| All subsystems | PersistenceEngine | `CortexState` | Valid invariants |
| PolicyEngine | All subsystems | `PolicyDecision` | ALLOW/LIMIT/DENY |

### 31.2 Data Contract Rules

| Rule | Description |
|---|---|
| DC-001 | Producer guarantees data validity before passing to consumer |
| DC-002 | Consumer validates received data before processing |
| DC-003 | Invalid data from producer is an error, not silently handled |
| DC-004 | Data types are shared through `types/` module |
| DC-005 | No subsystem directly accesses another subsystem's internal state |
| DC-006 | All cross-subsystem communication is through trait method calls |
| DC-007 | Provenance is preserved across all data transformations |
| DC-008 | Confidence is preserved and updated, never silently reset |

---

## 32. State Versioning

### 32.1 Version Hierarchy

```
Architecture Version (u32)
    │
    ├── Algorithm Versions (per-algorithm u32)
    │   ├── cell_algorithm
    │   ├── column_algorithm
    │   ├── plasticity_algorithm
    │   ├── memory_algorithm
    │   ├── language_algorithm
    │   ├── reasoning_algorithm
    │   ├── planning_algorithm
    │   ├── verification_algorithm
    │   └── consolidation_algorithm
    │
    └── Format Version (u32)
        └── .cx binary format version
```

### 32.2 Version Rules

| Rule | Description |
|---|---|
| VER-001 | Architecture version increments on structural changes |
| VER-002 | Algorithm version increments when algorithm changes |
| VER-003 | Format version increments when .cx layout changes |
| VER-004 | Versions are monotonically increasing |
| VER-005 | Version changes are recorded in .cx header |
| VER-006 | Version mismatch triggers migration or rejection |
| VER-007 | Downgrade is NEVER supported |

---

## 33. Migration Rules

### 33.1 Migration Pipeline

```
Loaded .cx
    │
    ↓
Read format_version from header
    │
    ↓
Is format_version == CURRENT_FORMAT_VERSION?
    │
    ├── YES → No migration needed → Continue
    │
    └── NO → Is format_version < CURRENT_FORMAT_VERSION?
              │
              ├── YES → Migration path exists?
              │         │
              │         ├── YES → Apply migrations sequentially
              │         │         │
              │         │         ↓
              │         │     Validate migrated state
              │         │         │
              │         │         ├── Valid → Continue
              │         │         └── Invalid → Recovery
              │         │
              │         └── NO → Reject: no migration path
              │
              └── NO → Reject: version is newer (downgrade not supported)
```

### 33.2 Migration Rules

| Rule | Description |
|---|---|
| MIG-001 | Migrations are sequential: v1 → v2 → v3 → ... → vN |
| MIG-002 | Each migration is a pure function: old bytes → new bytes |
| MIG-003 | Migrations preserve semantic content |
| MIG-004 | Migrations are idempotent |
| MIG-005 | Failed migration triggers recovery, not partial state |
| MIG-006 | Migration is logged with before/after versions |
| MIG-007 | Original data is preserved until migration succeeds |
| MIG-008 | Downgrade is NEVER supported |

---

## 34. Corrupt-State Handling

### 34.1 Corruption Detection

| Detection Point | Method |
|---|---|
| File level | Magic bytes check |
| Header level | Header checksum |
| Section level | Per-section checksum |
| Data level | Deserialization validation |
| Semantic level | Invariant validation |
| Cross-reference level | Reference integrity check |

### 34.2 Corruption Response

```
Corruption Detected
    │
    ↓
Log corruption event
    │
    ↓
Attempt recovery:
    │
    ├── 1. Try current .cx (may be partially valid)
    │       └── If valid sections → partial recovery
    │
    ├── 2. Try latest checkpoint
    │       └── If valid → restore from checkpoint
    │
    ├── 3. Try previous checkpoints (newest first)
    │       └── If valid → restore from checkpoint
    │
    ├── 4. Initialize new state
    │       └── If all checkpoints corrupt → fresh start
    │
    └── 5. Safe stop
            └── If initialization also fails → halt
```

### 34.3 Corruption Handling Rules

| Rule | Description |
|---|---|
| COR-001 | Corrupt state is NEVER silently treated as valid |
| COR-002 | Corruption is always logged |
| COR-003 | Recovery priority: current → latest checkpoint → older checkpoints → fresh |
| COR-004 | Partial recovery is allowed if sections are independently valid |
| COR-005 | Fresh state initialization preserves config but loses learned state |
| COR-006 | Safe stop is the last resort; preserves diagnostic info |
| COR-007 | Atomic write prevents most corruption scenarios |

---

## 35. Integrity & BLAKE3 Rules

### 35.1 Checksum Strategy

| Level | Algorithm | Scope |
|---|---|---|
| File level | BLAKE3 truncated to u128 | Entire file |
| Section level | BLAKE3 truncated to u128 | Individual section data |
| Config level | BLAKE3 full (32 bytes) | cortex.toml content |

### 35.2 Checksum Computation

```rust
/// Compute BLAKE3 truncated to u128 for section data.
fn compute_section_checksum(data: &[u8]) -> u128 {
    let hash = blake3::hash(data);
    let bytes = hash.as_bytes();
    // Take first 16 bytes as u128
    u128::from_le_bytes(bytes[..16].try_into().unwrap())
}

/// Compute BLAKE3 for config file.
fn compute_config_hash(config_content: &[u8]) -> [u8; 32] {
    let hash = blake3::hash(config_content);
    *hash.as_bytes()
}
```

### 35.3 Integrity Rules

| Rule | Description |
|---|---|
| INT-001 | Checksums are computed BEFORE writing |
| INT-002 | Checksums are verified AFTER reading |
| INT-003 | Checksum mismatch → reject section |
| INT-004 | File checksum covers header + section table + all section data |
| INT-005 | Config hash is verified at boot |
| INT-006 | Config hash mismatch → warning (config may have changed) |
| INT-007 | Checksums are deterministic for same input |

---

## 36. Deterministic Representation

### 36.1 Determinism Requirements

| Data | Deterministic? | Notes |
|---|---|---|
| State serialization | YES | Same state → same bytes |
| Configuration interpretation | YES | Same config → same behavior |
| Algorithm selection | YES | Same version → same algorithm |
| Memory indexing | YES | Same query → same results |
| Verification rules | YES | Same evidence → same status |
| Policy decisions | YES | Same operation → same decision |
| Checkpoint structure | YES | Same state → same checkpoint |
| ID generation | YES (sequential) | Deterministic within session |
| Learning updates | MAY BE stochastic | If explicitly configured |
| Neural activation noise | MAY BE present | If configured |

### 36.2 Determinism Rules

| Rule | Description |
|---|---|
| DET-001 | HashMap serialization is sorted by key |
| DET-002 | HashSet serialization is sorted |
| DET-003 | No random ordering in serialized collections |
| DET-004 | Timestamps use system clock (non-deterministic across runs) |
| DET-005 | UUID generation is non-deterministic (v4) |
| DET-006 | All other operations are deterministic given same inputs |

---

## 37. Resource Limits

### 37.1 Memory Limits

| Component | Limit | Source |
|---|---|---|
| Working Memory | `memory.working_mb` MB | Config |
| Episodic Memory | `memory.episodic_mb` MB | Config |
| Semantic Memory | `memory.semantic_mb` MB | Config |
| Procedural Memory | `memory.procedural_mb` MB | Config |
| Associative Memory | `memory.associative_mb` MB | Config |
| Vocabulary | `language.vocabulary_capacity` entries | Config |
| Context Window | `language.context_window` tokens | Config |
| Generation | `language.generation_limit` tokens | Config |
| Replay Buffer | Derived: `max(1, consolidation_interval / 10)` | Derived |
| Learning History | 1000 events (BoundedVec) | Hard-coded |
| Self Model History | 100 snapshots (BoundedVec) | Hard-coded |
| Diagnostic Errors | 100 entries | Hard-coded |
| Checkpoints | Configurable retention | Config |

### 37.2 Compute Limits

| Operation | Limit | Source |
|---|---|---|
| Reasoning steps | `reasoning.max_steps` | Config |
| Planning depth | `planning.max_depth` | Config |
| Planning branches | `planning.max_branches` | Config |
| Simulation steps | `world.prediction_horizon` | Config |
| Generation tokens | `language.generation_limit` | Config |
| Memory retrieval | `min(counts) / 4` | Derived |
| Replay count | `max(1, consolidation_interval / 10)` | Derived |

### 37.3 Network Limits

| Parameter | Limit | Source |
|---|---|---|
| Request timeout | `internet.timeout_seconds` | Config |
| Response size | `internet.max_response_mb` MB | Config |

### 37.4 Resource Limit Enforcement

| Rule | Description |
|---|---|
| RES-001 | All limits are enforced at operation time |
| RES-002 | Exceeding a limit produces a bounded result, not an error |
| RES-003 | Bounded results carry explicit uncertainty |
| RES-004 | Memory pressure triggers: compress → consolidate → evict → forget |
| RES-005 | Compute budget exhaustion terminates operation gracefully |
| RES-006 | Network timeout produces failed observation, not crash |

---

## 38. Data Validation Rules

### 38.1 Validation Pipeline

```
Data Input
    │
    ↓
┌─────────────────────────────────────┐
│  1. Type validation                 │
│  2. Range validation                │
│  3. Reference validation            │
│  4. Provenance validation           │
│  5. Invariant validation            │
│  6. Cross-reference validation      │
│  7. Resource limit validation       │
└─────────────────────────────────────┘
    │
    ├── All pass → Accept
    └── Any fail → Reject with error
```

### 38.2 Validation Rules by Type

| Data Type | Validation |
|---|---|
| Scalar | Not NaN, not Infinity, within documented range |
| String | Valid UTF-8, non-empty where required, bounded length |
| Timestamp | Non-zero where required, monotonically increasing |
| ID | Non-null where required, exists in referenced collection |
| Confidence | [0.0, 1.0] |
| Activation | [0.0, 1.0] |
| Strength | [0.0, 1.0] |
| Risk | [0.0, 1.0] |
| Importance | [0.0, 1.0] |
| Sparsity | (0.0, 1.0] |
| Learning rate | (0.0, 1.0] |
| Plasticity | [0.0, 1.0] |
| Vec | Length ≤ capacity |
| HashMap | Keys are valid |
| Option | None is valid only where documented |
| Provenance | NEVER None; always present |
| Evidence | Non-empty for verified claims |

### 38.3 Validation Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum DataValidationError {
    #[error("Non-finite scalar value")]
    NonFiniteValue,
    
    #[error("Value {value} out of range [{min}, {max}]")]
    OutOfRange { value: Scalar, min: Scalar, max: Scalar },
    
    #[error("Invalid UTF-8 string")]
    InvalidUtf8,
    
    #[error("Empty string where non-empty required")]
    EmptyString,
    
    #[error("String exceeds maximum length: {len} > {max}")]
    StringTooLong { len: usize, max: usize },
    
    #[error("Null ID where non-null required")]
    NullId,
    
    #[error("Reference to non-existent item: {id}")]
    DanglingReference { id: String },
    
    #[error("Missing provenance")]
    MissingProvenance,
    
    #[error("Collection exceeds capacity: {len} > {capacity}")]
    CapacityExceeded { len: usize, capacity: usize },
    
    #[error("Invalid timestamp")]
    InvalidTimestamp,
    
    #[error("Invariant violation: {0}")]
    InvariantViolation(String),
}
```

---

## 39. Open Technical Parameters

| Parameter | Current Value | Open Question | Resolution Path |
|---|---|---|---|
| Scalar precision | f32 | Should f16/bf16 be supported at runtime? | Benchmark on target hardware |
| Checksum algorithm | BLAKE3 truncated to u128 | Is u128 sufficient for integrity? | Evaluate collision probability |
| Compression level | zstd level 3 | Optimal level for .cx? | Benchmark size vs. speed |
| BoundedVec capacity (learning history) | 1000 | Sufficient for diagnostics? | Operational evaluation |
| BoundedVec capacity (self model history) | 100 | Sufficient for performance tracking? | Operational evaluation |
| Diagnostic error buffer | 100 | Sufficient for debugging? | Operational evaluation |
| Maximum checkpoint retention | Configurable | Default value? | Operational evaluation |
| Association index strategy | HashMap | Should we use a more efficient index? | Benchmark retrieval performance |
| Episode eviction policy | Composite | Optimal eviction strategy? | Learning stability tests |
| Replay buffer capacity | Derived | Should it be explicitly configurable? | Memory pressure testing |
| Serialization format | bincode | Should we consider a more schema-evolvable format? | Evaluate migration complexity |
| HashMap serialization order | Sorted by key | Performance impact of sorting? | Benchmark |
| String length limits | None (context-bounded) | Should we add explicit max string lengths? | Security evaluation |
| Maximum entity count | Unbounded (memory-bounded) | Should we add explicit entity limits? | Resource testing |
| Maximum relation count | Unbounded (memory-bounded) | Should we add explicit relation limits? | Resource testing |

---

## 40. Gap Resolution: Additional Data Specifications

### 40.1 Verification Status Lifecycle

> **Canonical definition:** See DOC-00 §5. Verification status is a separate dimension from confidence. The transition matrix below defines all valid state transitions.

Verification status transitions are governed by a strict state machine:

```
                ┌──────────────┐
                │   Unknown    │
                └──────┬───────┘
                       │ evidence gathered / inference
                       ↓
                ┌──────────────┐
                │   Observed   │ (direct observation)
                └──────┬───────┘
                       │ inference applied / evidence accumulated
                       ↓
                ┌──────────────┐
                │   Inferred   │ (derived from other claims)
                └──────┬───────┘
                       │ evidence_count ≥ 1 AND strength ≥ 0.5
                       ↓
                ┌──────────────┐
                │  Supported   │ (evidence total_strength ≥ 0.5)
                └──────┬───────┘
                       │ confidence ≥ 0.3 AND no contradictions
                       ↓
                ┌──────────────┐
                │ Provisional  │ (evidence total_strength ≥ 0.3)
                └──────┬───────┘
                       │ independent_sources ≥ 2, strength ≥ threshold,
                       │ quality ≥ 0.7, consistency ≥ 0.8
                       ↓
                ┌──────────────┐
                │   Verified   │ (all verification criteria met)
                └──────────────┘
                       
    Any status ──contradiction found──→ Contradicted
    Provisional ──confidence downgrade──→ Supported
```

**Transition Rules (DOC-00 §5.2 canonical):**

| From | To | Condition |
|---|---|---|
| Unknown | Observed | observation_count ≥ 1 |
| Unknown | Inferred | inference_source != None |
| Unknown | Contradicted | contradiction_count ≥ 1 |
| Observed | Inferred | inference_source != None |
| Observed | Supported | evidence_count ≥ 1 AND strength ≥ 0.5 |
| Observed | Contradicted | contradiction_count ≥ 1 |
| Inferred | Supported | evidence_count ≥ 1 AND strength ≥ 0.5 |
| Inferred | Contradicted | contradiction_count ≥ 1 |
| Supported | Provisional | confidence ≥ 0.3 AND no contradictions |
| Supported | Contradicted | contradiction_count ≥ 1 |
| Provisional | Verified | independent_sources ≥ 2 AND strength ≥ threshold AND quality ≥ 0.7 AND consistency ≥ 0.8 |
| Provisional | Supported | confidence < 0.3 OR contradiction detected |
| Provisional | Contradicted | contradiction_count ≥ 1 |
| Verified | Contradicted | contradiction_count ≥ 1 AND severity > threshold |

**Forbidden Transitions (DOC-00 §5.3):**

| Transition | Reason |
|---|---|
| Verified → Observed | Cannot downgrade from verified without new contradiction |
| Verified → Inferred | Cannot downgrade from verified without new contradiction |
| Verified → Supported | Cannot downgrade from verified without new contradiction |
| Verified → Provisional | Cannot downgrade from verified without new contradiction |
| Contradicted → Any except Unknown | Contradicted is terminal until contradiction resolved |
| Any → Unknown | Cannot reset to unknown (state is permanent) |

**Invariant INV-DOC-004:** Verification SHALL never silently upgrade UNKNOWN to VERIFIED without satisfying configured evidence conditions (minimum_confidence threshold).

### 40.2 Memory Capacity & Eviction State Transitions

```
Memory Pressure State Machine:

    ┌────────┐
    │  Low   │ (< 0.7 usage ratio)
    └───┬────┘
        │ usage increases
        ↓
  ┌───────────┐
  │ Moderate  │ (0.7 - 0.85)
  └─────┬─────┘
        │ usage increases
        ↓
  ┌───────────┐
  │   High    │ (0.85 - 0.95)
  └─────┬─────┘
        │ usage increases
        ↓
  ┌───────────┐
  │ Critical  │ (≥ 0.95)
  └───────────┘

Pressure Response Actions:
  Low      → No action
  Moderate → Consolidate
  High     → Consolidate + Forget (moderate policy)
  Critical → Consolidate + Forget (emergency policy) + Compress working memory
```

**Eviction Priority:**

When eviction is triggered, items are scored for forgetting using multi-factor computation:

| Factor | Weight | Description |
|---|---|---|
| Low importance | 0.20 | importance < min_importance |
| Low confidence | 0.20 | confidence.overall() < min_confidence |
| Age | 0.20 | age > max_age (if configured) |
| Low retrieval frequency | 0.20 | retrieval_count < min_retrieval_count |
| Redundancy | 0.10 | already consolidated |
| Contradiction | 0.10 | contradicted by other knowledge |

Items with forget_score > 0.7 are candidates for eviction. Eviction is processed oldest-first among candidates.

---

## 41. Data Completeness

### 40.1 Completeness Checklist

| Data Category | Status | Coverage |
|---|---|---|
| Primitive types | ✅ Complete | Scalar, integers, strings, bools, bytes |
| ID types | ✅ Complete | 25 distinct ID types defined |
| Timestamp types | ✅ Complete | Timestamp, Duration, TemporalContext |
| Core data structures | ✅ Complete | Context, Observation, Action, Prediction, Evidence, Provenance, Confidence |
| Memory data model | ✅ Complete | Working, Episodic, Semantic, Procedural, Associative |
| World model data | ✅ Complete | Entity, Relation, Event, WorldState, PredictedState, SimulatedTrajectory |
| Reasoning state | ✅ Complete | Hypothesis, Proposition, Conclusion, Contradiction |
| Planning state | ✅ Complete | Goal, Plan, PlanStatus |
| Verification state | ✅ Complete | KnowledgeClaim, VerificationStatus, EvidenceRequirements |
| Learning state | ✅ Complete | LearningState, LearningEvent, ErrorAttribution |
| Plasticity state | ✅ Complete | PlasticityState, ΔW formula |
| Replay/Consolidation state | ✅ Complete | ReplayState, ConsolidationState, ConsolidationCandidate |
| Self model state | ✅ Complete | SelfModel, CapabilitySet, LimitationSet |
| Policy/Risk state | ✅ Complete | PolicyState, ProposedOperation, PolicyDecision |
| Internet interface state | ✅ Complete | InternetState, NetworkRequest, NetworkObservation |
| Runtime state | ✅ Complete | RuntimeState enum, RuntimeStatus, RuntimeEvent |
| Configuration state | ✅ Complete | CortexConfig with all sub-configs |
| Persistence state | ✅ Complete | PersistenceState, SaveResult, CheckpointMetadata |
| .cx data layout | ✅ Complete | Byte-level format specification |
| Serialization rules | ✅ Complete | Format, ordering, invariants |
| Deserialization rules | ✅ Complete | Pipeline, error handling |
| State ownership | ✅ Complete | Ownership matrix for all structures |
| State mutability | ✅ Complete | Mutability matrix with triggers and bounds |
| State lifecycle | ✅ Complete | Creation → Operation → Shutdown → Restart → Migration |
| State transition constraints | ✅ Complete | Valid/invalid transitions, data constraints |
| State invariants | ✅ Complete | 20 invariants with enforcement points |
| Cross-subsystem contracts | ✅ Complete | Data flow contracts, rules |
| State versioning | ✅ Complete | Version hierarchy, rules |
| Migration rules | ✅ Complete | Pipeline, rules |
| Corrupt-state handling | ✅ Complete | Detection, response, rules |
| Integrity & BLAKE3 | ✅ Complete | Checksum strategy, computation, rules |
| Deterministic representation | ✅ Complete | Determinism requirements, rules |
| Resource limits | ✅ Complete | Memory, compute, network limits |
| Data validation rules | ✅ Complete | Pipeline, per-type rules, error types |

### 40.2 Traceability to Requirements

| DOC-01 Requirement | DOC-03 Data Coverage |
|---|---|
| FR-LANG-* | §8.1 Context, §9 Language data |
| FR-NEUR-* | §15 Plasticity, Neural state in §8 |
| FR-MEM-* | §9 Memory Data Model |
| FR-WRLD-* | §10 World Model Data |
| FR-RSN-* | §11 Reasoning State |
| FR-PLN-* | §12 Planning State |
| FR-VER-* | §13 Verification State |
| FR-LRN-* | §14 Learning State, §15 Plasticity, §16 Replay/Consolidation |
| FR-SLF-* | §17 Self Model State |
| FR-POL-* | §18 Policy/Risk State |
| FR-INT-* | §19 Internet Interface State |
| FR-PRS-* | §22 Persistence State, §23 .cx Layout |
| FR-API-* | §8.2 Observation, §8.3 Action |
| REL-* | §30 State Invariants, §34 Corrupt-State Handling |
| SEC-* | §18 Policy State, §35 Integrity |
| PRV-* | §8.6 Provenance |
| ERR-* | §38 Data Validation Rules |
| CMP-* | §32 State Versioning, §33 Migration Rules |

### 40.3 Traceability to Requirements

| DOC-01 Requirement | DOC-03 Section | Data Type |
|---|---|---|
| FR-LANG-001 through FR-LANG-015 | §8 Language State | `LanguageState` |
| FR-NEUR-001 through FR-NEUR-009 | §8 Neural State | `NeuralState`, `CellState` |
| FR-MEM-001 through FR-MEM-011 | §9 Memory Data Model | `MemoryState`, `WorkingMemory`, `EpisodicMemory`, `SemanticMemory`, `ProceduralMemory`, `AssociativeMemory` |
| FR-WRLD-001 through FR-WRLD-007 | §10 World Data | `WorldState`, `Entity`, `Transition` |
| FR-RSN-001 through FR-RSN-006 | §11 Reasoning State | `ReasoningState`, `KnowledgeClaim` |
| FR-PLN-001 through FR-PLN-004 | §12 Planning State | `PlanningState`, `Goal`, `Plan` |
| FR-VER-001 through FR-VER-006 | §13 Verification State | `VerificationState`, `KnowledgeClaim`, `VerificationStatus` |
| FR-LRN-001 through FR-LRN-009 | §14 Learning State | `LearningState`, `LearningEvent` |
| FR-SLF-001 through FR-SLF-004 | §15 Self Model State | `SelfModel`, `CapabilitySet` |
| FR-POL-001 through FR-POL-006 | §16 Policy State | `PolicyState`, `RiskThresholds` |
| FR-INT-001 through FR-INT-005 | §17 Internet State | `InternetState` |
| FR-PRS-001 through FR-PRS-006 | §23 .cx Format | `.cx` binary format |
| FR-API-001 through FR-API-004 | §8.2 Observation, §8.3 Action | `Observation`, `Action` |

### 40.4 Final Data Contract Statement

> **This document constitutes the data-level contract for CORTEX.** It defines every data structure, every state type, every ownership boundary, every mutability rule, every lifecycle transition, every invariant, and every persistence byte.
>
> The data contract ensures:
> - **Explicit typing**: Every data element has a defined Rust type.
> - **Ownership clarity**: Every struct has exactly one owner.
> - **Provenance preservation**: Every knowledge item carries origin, confidence, and evidence.
> - **Bounded storage**: Every collection has an explicit or derived capacity bound.
> - **Fail-before-persist**: Invalid data never reaches disk.
> - **Deterministic serialization**: Same logical state always produces same serialized bytes.
> - **Version-aware persistence**: All data carries version metadata for migration.
> - **Corrupt-state recovery**: Corruption is detected, logged, and recovered from.
>
> **CORTEX data architecture: 25 ID types, 50+ struct types, 20 state invariants, 21 .cx sections, 1 binary format.**

---

*End of Document — CORTEX-DOC-03 Data & State Specification v1.1.0*
