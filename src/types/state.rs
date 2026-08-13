use super::common::*;
use super::evidence::*;
use super::ids::*;
use super::scalars::Scalar;
use super::observation::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateMetadata {
    pub state_id: uuid::Uuid,
    pub created_at: Timestamp,
    pub last_updated: Timestamp,
    pub architecture_version: u32,
    pub algorithm_versions: AlgorithmVersions,
    pub config_hash: [u8; 32],
    pub episode_count: u64,
    pub total_learning_events: u64,
    pub checkpoint_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageState {
    pub symbols: Vec<Symbol>,
    pub tokens: Vec<Token>,
    pub vocabulary_size: u32,
    pub next_symbol_id: SymbolId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub id: SymbolId,
    pub text: String,
    pub kind: SymbolKind,
    pub frequency: u64,
    pub activation: Scalar,
    pub confidence: Scalar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SymbolKind {
    Word,
    Subword,
    Punctuation,
    Number,
    Unknown,
    Special,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub id: TokenId,
    pub symbol_id: SymbolId,
    pub position: u32,
    pub weight: Scalar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralState {
    pub fields: Vec<NeuralField>,
    pub active_cells: Vec<CellId>,
    pub active_columns: Vec<ColumnId>,
    pub temporal_buffer: Vec<NeuralField>,
    pub prediction: Option<super::observation::Prediction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralField {
    pub id: FieldId,
    pub columns: Vec<Column>,
    pub average_activation: Scalar,
    pub coherence: Scalar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {
    pub id: ColumnId,
    pub cells: Vec<Cell>,
    pub active_cells: Vec<CellId>,
    pub activation_threshold: Scalar,
    pub learned_pattern: Vec<Scalar>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cell {
    pub id: CellId,
    pub state: CellState,
    pub activation: Scalar,
    pub prediction_vector: Vec<Scalar>,
    pub refractory_steps: u32,
    pub adaptation_level: Scalar,
    pub burst_counter: u32,
    pub eligibility_trace: Scalar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CellState {
    Resting,
    Active,
    Inhibited,
    Learning,
    Predicting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingMemory {
    pub input: Option<CurrentInput>,
    pub conversation_context: ConversationContext,
    pub active_concepts: Vec<ConceptId>,
    pub active_hypotheses: Vec<HypothesisId>,
    pub goals: Vec<GoalId>,
    pub reasoning_state: Option<ReasoningSnapshot>,
    pub world_assumptions: Vec<EntityId>,
    pub generation_state: Option<GenerationState>,
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
pub struct EpisodicMemory {
    pub episodes: Vec<Episode>,
    pub capacity_bytes: u64,
    pub current_usage_bytes: u64,
    pub next_id: EpisodeId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    pub id: EpisodeId,
    pub observation: super::observation::Observation,
    pub context: ContextState,
    pub action: Option<super::observation::Action>,
    pub outcome: Option<super::observation::Outcome>,
    pub timestamp: Timestamp,
    pub prediction: Option<super::observation::Prediction>,
    pub prediction_error: super::observation::PredictionError,
    pub confidence: ConfidenceState,
    pub source: Provenance,
    pub importance: Scalar,
    pub retrieval_count: u64,
    pub last_retrieved: Option<Timestamp>,
    pub consolidated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticMemory {
    pub knowledge: Vec<Knowledge>,
    pub capacity_bytes: u64,
    pub current_usage_bytes: u64,
    pub next_id: KnowledgeId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Knowledge {
    pub id: KnowledgeId,
    pub concept: ConceptId,
    pub properties: Vec<Property>,
    pub relations: Vec<Relation>,
    pub evidence: EvidenceSet,
    pub confidence: ConfidenceState,
    pub provenance: Vec<Provenance>,
    pub verification_status: VerificationStatus,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub confirmation_count: u64,
    pub contradiction_count: u64,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralMemory {
    pub procedures: Vec<Procedure>,
    pub capacity_bytes: u64,
    pub current_usage_bytes: u64,
    pub next_id: ProcedureId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Procedure {
    pub id: ProcedureId,
    pub condition: Condition,
    pub steps: Vec<super::observation::Action>,
    pub expected_outcome: super::observation::Outcome,
    pub success_count: u64,
    pub failure_count: u64,
    pub confidence: Scalar,
    pub context_requirements: ContextRequirements,
    pub risk: RiskAssessment,
    pub provenance: Provenance,
    pub created_at: Timestamp,
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
pub struct AssociativeMemory {
    pub associations: Vec<Association>,
    pub capacity_bytes: u64,
    pub current_usage_bytes: u64,
    pub next_id: AssociationId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Association {
    pub id: AssociationId,
    pub source: InternalId,
    pub target: InternalId,
    pub kind: AssociationKind,
    pub strength: Scalar,
    pub confidence: Scalar,
    pub context: ContextState,
    pub provenance: Provenance,
    pub created_at: Timestamp,
    pub last_strengthened: Timestamp,
    pub activation_count: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AssociationKind {
    Semantic,
    Temporal,
    Contextual,
    Causal,
    Episodic,
    Procedural,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryState {
    pub working: WorkingMemory,
    pub episodic: EpisodicMemory,
    pub semantic: SemanticMemory,
    pub procedural: ProceduralMemory,
    pub associative: AssociativeMemory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldState {
    pub entities: Vec<Entity>,
    pub relations: Vec<Relation>,
    pub active_events: Vec<WorldEvent>,
    pub temporal_context: TemporalContext,
    pub uncertainty: UncertaintyState,
    pub next_entity_id: EntityId,
    pub next_relation_id: RelationId,
    pub next_event_id: EventId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: EntityId,
    pub kind: EntityKind,
    pub identity: IdentityState,
    pub properties: Vec<Property>,
    pub state: EntityState,
    pub relations: Vec<RelationId>,
    pub confidence: Scalar,
    pub provenance: Vec<Provenance>,
    pub created_at: Timestamp,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldEvent {
    pub id: EventId,
    pub description: String,
    pub participants: Vec<EntityId>,
    pub timestamp: Timestamp,
    pub duration: Option<Duration>,
    pub outcome: Option<super::observation::Outcome>,
    pub provenance: Provenance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictedState {
    pub predicted_entities: Vec<Entity>,
    pub predicted_relations: Vec<Relation>,
    pub confidence: Scalar,
    pub uncertainty: Scalar,
    pub prediction_horizon: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulatedTrajectory {
    pub steps: Vec<WorldState>,
    pub actions: Vec<super::observation::Action>,
    pub confidence: Scalar,
    pub uncertainty: Scalar,
    pub is_counterfactual: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningState {
    pub active_hypotheses: Vec<Hypothesis>,
    pub conclusion: Option<Conclusion>,
    pub premises: Vec<Proposition>,
    pub evidence_index: HashMap<HypothesisId, Vec<EvidenceId>>,
    pub contradiction_log: Vec<Contradiction>,
    pub budget_remaining: u32,
    pub next_hypothesis_id: HypothesisId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hypothesis {
    pub id: HypothesisId,
    pub proposition: Proposition,
    pub evidence: EvidenceSet,
    pub counter_evidence: EvidenceSet,
    pub confidence: Scalar,
    pub dependencies: Vec<HypothesisId>,
    pub contradictions: Vec<Contradiction>,
    pub provenance: Vec<Provenance>,
    pub reasoning_type: ReasoningType,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningResult {
    pub hypotheses: Vec<Hypothesis>,
    pub contradictions: Vec<Contradiction>,
    pub budget_remaining: u32,
    pub conclusion: Option<Conclusion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanningState {
    pub active_goals: Vec<Goal>,
    pub candidate_plans: Vec<Plan>,
    pub selected_plan: Option<Plan>,
    pub budget_remaining: u32,
    pub simulation_count: u32,
    pub next_plan_id: PlanId,
    pub next_goal_id: GoalId,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub id: PlanId,
    pub goal: GoalId,
    pub steps: Vec<super::observation::Action>,
    pub estimated_cost: Scalar,
    pub estimated_risk: Scalar,
    pub uncertainty: Scalar,
    pub confidence: Scalar,
    pub predicted_outcomes: Vec<super::observation::Outcome>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationState {
    pub pending_claims: Vec<Claim>,
    pub verified_claims: u64,
    pub contradicted_claims: u64,
    pub confidence_threshold: Scalar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claim {
    pub id: ClaimId,
    pub text: String,
    pub status: VerificationStatus,
    pub confidence: ConfidenceState,
    pub evidence: EvidenceSet,
    pub created_at: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiedResult {
    pub claims: Vec<Claim>,
    pub overall_confidence: ConfidenceState,
    pub verification_status: VerificationStatus,
    pub reasoning_result: Option<ReasoningResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningState {
    pub enabled: bool,
    pub total_learning_events: u64,
    pub total_replay_events: u64,
    pub total_consolidation_events: u64,
    pub average_prediction_error: Scalar,
    pub learning_rate: Scalar,
    pub plasticity_rate: Scalar,
    pub next_consolidation_at: u64,
    pub pending_experiences: Vec<Experience>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub observation: super::observation::Observation,
    pub internal_state: StateSnapshot,
    pub prediction: super::observation::Prediction,
    pub action: Option<super::observation::Action>,
    pub outcome: Option<super::observation::Outcome>,
    pub error: super::observation::PredictionError,
    pub attribution: ErrorAttribution,
    pub evidence: EvidenceSet,
    pub provenance: Provenance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSnapshot {
    pub language_vocabulary_size: u32,
    pub neural_active_cells: usize,
    pub memory_episode_count: usize,
    pub world_entity_count: usize,
    pub reasoning_hypothesis_count: usize,
    pub timestamp: Timestamp,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfModel {
    pub capabilities: CapabilityEstimate,
    pub limitations: Limitations,
    pub prediction_accuracy: Scalar,
    pub uncertainty_level: Scalar,
    pub memory_health: MemoryHealth,
    pub last_updated: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityEstimate {
    pub language_accuracy: Scalar,
    pub prediction_accuracy: Scalar,
    pub verification_reliability: Scalar,
    pub planning_success: Scalar,
    pub memory_retrieval_success: Scalar,
    pub reasoning_consistency: Scalar,
    pub resource_availability: Scalar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Limitations {
    pub known_limitations: Vec<String>,
    pub resource_constraints: Vec<String>,
    pub capability_gaps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryHealth {
    pub pressure: MemoryPressure,
    pub fragmentation: Scalar,
    pub consolidation_backlog: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryPressure {
    Low,
    Moderate,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceState {
    pub sources: Vec<Provenance>,
    pub next_source_id: SourceId,
}
