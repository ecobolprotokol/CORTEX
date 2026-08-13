use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::common::{ContextState, TemporalContext, Timestamp};
use crate::types::evidence::{ConfidenceState, EvidenceSet, UncertaintyState, VerificationStatus};
use crate::types::ids::*;
use crate::types::observation::Observation;
use crate::types::scalars::Scalar;

pub const ARCHITECTURE_VERSION: u32 = 1;
pub const SCHEMA_VERSION: u32 = 1;
pub const API_VERSION: &str = "v1";
pub const DOCUMENT_VERSION: &str = "1.0.0";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CortexState {
    pub metadata: StateMetadata,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateMetadata {
    pub state_id: [u8; 16],
    pub created_at: Timestamp,
    pub last_updated: Timestamp,
    pub architecture_version: u32,
    pub schema_version: u32,
    pub algorithm_versions: AlgorithmVersions,
    pub config_hash: [u8; 32],
    pub episode_count: u64,
    pub total_learning_events: u64,
    pub checkpoint_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmVersions {
    pub cell_algorithm: String,
    pub column_algorithm: String,
    pub plasticity_algorithm: String,
    pub memory_algorithm: String,
    pub language_algorithm: String,
    pub reasoning_algorithm: String,
    pub planning_algorithm: String,
    pub verification_algorithm: String,
    pub consolidation_algorithm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LanguageState {
    pub symbols: Vec<SymbolId>,
    pub tokens: Vec<TokenId>,
    pub concepts: Vec<ConceptId>,
    pub entities: Vec<EntityId>,
    pub relations: Vec<RelationId>,
    pub syntax: SyntaxState,
    pub semantics: SemanticState,
    pub context: ContextState,
    pub intent: Vec<IntentHypothesis>,
    pub confidence: ConfidenceState,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SyntaxState {
    pub rules_applied: u64,
    pub parse_depth: u32,
    pub active_patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticState {
    pub active_frames: Vec<String>,
    pub slot_bindings: HashMap<String, String>,
    pub coherence: Scalar,
}

impl Default for SemanticState {
    fn default() -> Self {
        Self {
            active_frames: Vec::new(),
            slot_bindings: HashMap::new(),
            coherence: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentHypothesis {
    pub id: u64,
    pub description: String,
    pub confidence: Scalar,
    pub supporting_evidence: Vec<EvidenceId>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NeuralState {
    pub fields: Vec<FieldState>,
    pub active_cells: Vec<CellId>,
    pub active_columns: Vec<ColumnId>,
    pub field_activations: HashMap<FieldId, Scalar>,
    pub temporal_encoding: Vec<Scalar>,
    pub prediction: Option<NeuralPredictionState>,
    pub confidence: ConfidenceState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldState {
    pub id: FieldId,
    pub column_count: u32,
    pub active_columns: u32,
    pub average_activation: Scalar,
    pub coherence: Scalar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralPredictionState {
    pub predicted_next: Vec<Scalar>,
    pub confidence: Scalar,
    pub context: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MemoryState {
    pub working: WorkingMemory,
    pub episodic: EpisodicMemory,
    pub semantic: SemanticMemory,
    pub procedural: ProceduralMemory,
    pub associative: AssociativeMemory,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkingMemory {
    pub input: Option<String>,
    pub active_concepts: Vec<ConceptId>,
    pub active_hypotheses: Vec<HypothesisId>,
    pub goals: Vec<GoalId>,
    pub world_assumptions: Vec<EntityId>,
    pub turn_count: u64,
    pub recent_inputs: Vec<String>,
    pub recent_outputs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodicMemory {
    pub episodes: Vec<EpisodeRecord>,
    pub capacity_bytes: u64,
    pub current_usage_bytes: u64,
    pub next_id: EpisodeId,
}

impl Default for EpisodicMemory {
    fn default() -> Self {
        Self {
            episodes: Vec::new(),
            capacity_bytes: 0,
            current_usage_bytes: 0,
            next_id: EpisodeId::NULL,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeRecord {
    pub id: EpisodeId,
    pub observation: Observation,
    pub timestamp: Timestamp,
    pub importance: Scalar,
    pub consolidated: bool,
    pub retrieval_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticMemory {
    pub knowledge: Vec<KnowledgeRecord>,
    pub capacity_bytes: u64,
    pub current_usage_bytes: u64,
    pub next_id: KnowledgeId,
}

impl Default for SemanticMemory {
    fn default() -> Self {
        Self {
            knowledge: Vec::new(),
            capacity_bytes: 0,
            current_usage_bytes: 0,
            next_id: KnowledgeId::NULL,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeRecord {
    pub id: KnowledgeId,
    pub concept: ConceptId,
    pub confidence: ConfidenceState,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub confirmation_count: u64,
    pub contradiction_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralMemory {
    pub procedures: Vec<ProcedureRecord>,
    pub capacity_bytes: u64,
    pub current_usage_bytes: u64,
    pub next_id: ProcedureId,
}

impl Default for ProceduralMemory {
    fn default() -> Self {
        Self {
            procedures: Vec::new(),
            capacity_bytes: 0,
            current_usage_bytes: 0,
            next_id: ProcedureId::NULL,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureRecord {
    pub id: ProcedureId,
    pub description: String,
    pub success_count: u64,
    pub failure_count: u64,
    pub confidence: Scalar,
    pub created_at: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssociativeMemory {
    pub associations: Vec<AssociationRecord>,
    pub capacity_bytes: u64,
    pub current_usage_bytes: u64,
    pub next_id: AssociationId,
}

impl Default for AssociativeMemory {
    fn default() -> Self {
        Self {
            associations: Vec::new(),
            capacity_bytes: 0,
            current_usage_bytes: 0,
            next_id: AssociationId::NULL,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssociationRecord {
    pub id: AssociationId,
    pub source: InternalId,
    pub target: InternalId,
    pub strength: Scalar,
    pub created_at: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorldState {
    pub entities: Vec<Entity>,
    pub relations: Vec<Relation>,
    pub active_events: Vec<Event>,
    pub temporal_context: TemporalContext,
    pub uncertainty: UncertaintyState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: EntityId,
    pub name: String,
    pub confidence: Scalar,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub id: RelationId,
    pub source: InternalId,
    pub target: InternalId,
    pub kind: String,
    pub confidence: Scalar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: EventId,
    pub description: String,
    pub participants: Vec<EntityId>,
    pub timestamp: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReasoningState {
    pub active_hypotheses: Vec<Hypothesis>,
    pub conclusion: Option<Conclusion>,
    pub premises: Vec<Proposition>,
    pub evidence_index: HashMap<HypothesisId, Vec<EvidenceId>>,
    pub contradiction_log: Vec<Contradiction>,
    pub budget_remaining: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hypothesis {
    pub id: HypothesisId,
    pub proposition: Proposition,
    pub confidence: Scalar,
    pub evidence: EvidenceSet,
    pub counter_evidence: EvidenceSet,
    pub created_at: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposition {
    pub subject: String,
    pub predicate: String,
    pub object: Option<String>,
    pub negated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conclusion {
    pub hypothesis_id: HypothesisId,
    pub proposition: Proposition,
    pub confidence: Scalar,
    pub evidence_strength: Scalar,
    pub reasoning_steps: u32,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlanningState {
    pub active_goals: Vec<Goal>,
    pub candidate_plans: Vec<Plan>,
    pub selected_plan: Option<Plan>,
    pub budget_remaining: u32,
    pub simulation_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: GoalId,
    pub description: String,
    pub priority: Scalar,
    pub deadline: Option<Timestamp>,
    pub status: GoalStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GoalStatus {
    Active,
    Achieved,
    Failed,
    Abandoned,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub id: PlanId,
    pub goal: GoalId,
    pub steps: Vec<String>,
    pub estimated_cost: Scalar,
    pub estimated_risk: Scalar,
    pub confidence: Scalar,
    pub created_at: Timestamp,
    pub status: PlanStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlanStatus {
    Candidate,
    Selected,
    Executing,
    Completed,
    Failed,
    Abandoned,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationState {
    pub pending_claims: Vec<KnowledgeClaim>,
    pub verified_claims: Vec<KnowledgeClaim>,
    pub contradiction_log: Vec<Contradiction>,
    pub confidence_threshold: Scalar,
}

impl Default for VerificationState {
    fn default() -> Self {
        Self {
            pending_claims: Vec::new(),
            verified_claims: Vec::new(),
            contradiction_log: Vec::new(),
            confidence_threshold: 0.7,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeClaim {
    pub id: ClaimId,
    pub proposition: Proposition,
    pub evidence: EvidenceSet,
    pub counter_evidence: EvidenceSet,
    pub status: VerificationStatus,
    pub confidence: ConfidenceState,
    pub claimed_at: Timestamp,
    pub last_verified: Option<Timestamp>,
    pub verification_attempts: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningState {
    pub total_learning_events: u64,
    pub total_replay_events: u64,
    pub total_consolidation_events: u64,
    pub average_prediction_error: Scalar,
    pub learning_rate: Scalar,
    pub plasticity_rate: Scalar,
    pub next_consolidation_at: u64,
}

impl Default for LearningState {
    fn default() -> Self {
        Self {
            total_learning_events: 0,
            total_replay_events: 0,
            total_consolidation_events: 0,
            average_prediction_error: 0.0,
            learning_rate: 0.001,
            plasticity_rate: 0.01,
            next_consolidation_at: 100,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfModel {
    pub capabilities: CapabilitySet,
    pub limitations: LimitationSet,
    pub prediction_accuracy: Scalar,
    pub uncertainty_level: Scalar,
    pub memory_health: MemoryHealth,
}

impl Default for SelfModel {
    fn default() -> Self {
        Self {
            capabilities: CapabilitySet::default(),
            limitations: LimitationSet::default(),
            prediction_accuracy: 0.0,
            uncertainty_level: 1.0,
            memory_health: MemoryHealth::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilitySet {
    pub language_accuracy: Scalar,
    pub prediction_accuracy: Scalar,
    pub verification_reliability: Scalar,
    pub planning_success: Scalar,
    pub memory_retrieval_success: Scalar,
    pub reasoning_consistency: Scalar,
}

impl Default for CapabilitySet {
    fn default() -> Self {
        Self {
            language_accuracy: 0.0,
            prediction_accuracy: 0.0,
            verification_reliability: 0.0,
            planning_success: 0.0,
            memory_retrieval_success: 0.0,
            reasoning_consistency: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LimitationSet {
    pub known_limitations: Vec<String>,
    pub resource_constraints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryHealth {
    pub fragmentation: Scalar,
    pub consolidation_backlog: u64,
    pub eviction_rate: Scalar,
}

impl Default for MemoryHealth {
    fn default() -> Self {
        Self {
            fragmentation: 0.0,
            consolidation_backlog: 0,
            eviction_rate: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceState {
    pub total_sources: u64,
    pub total_provenance_records: u64,
    pub total_evidence: u64,
    pub average_confidence: Scalar,
}

impl Default for ProvenanceState {
    fn default() -> Self {
        Self {
            total_sources: 0,
            total_provenance_records: 0,
            total_evidence: 0,
            average_confidence: 0.0,
        }
    }
}
