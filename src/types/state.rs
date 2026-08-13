use serde::{Deserialize, Serialize};

use crate::types::common::{Duration, Timestamp};
use crate::types::ids::SessionId;

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
    pub version: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub session_id: SessionId,
    pub algorithm_versions: AlgorithmVersions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmVersions {
    pub attention: String,
    pub consolidation: String,
    pub inference: String,
    pub planning: String,
    pub learning: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LanguageState {
    pub vocabulary_size: u32,
    pub active_symbols: u32,
    pub embedding_dim: u32,
    pub total_tokens_processed: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NeuralState {
    pub column_count: u32,
    pub active_columns: u32,
    pub total_synapses: u64,
    pub average_sparsity: f32,
    pub global_activity: f32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemoryState {
    pub episode_count: u32,
    pub knowledge_count: u32,
    pub procedure_count: u32,
    pub total_memories: u64,
    pub working_memory_size: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorldState {
    pub entity_count: u32,
    pub relation_count: u32,
    pub event_count: u32,
    pub active_model_version: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReasoningState {
    pub active_hypotheses: u32,
    pub evidence_count: u32,
    pub total_inferences: u64,
    pub average_confidence: f32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlanningState {
    pub active_plans: u32,
    pub completed_goals: u32,
    pub total_actions: u64,
    pub average_plan_depth: f32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VerificationState {
    pub claims_checked: u64,
    pub claims_verified: u64,
    pub claims_rejected: u64,
    pub pending_verifications: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LearningState {
    pub total_updates: u64,
    pub learning_rate: f32,
    pub recent_loss: f32,
    pub episodes_trained: u64,
    pub total_training_duration: Duration,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SelfModel {
    pub capabilities: Vec<String>,
    pub limitations: Vec<String>,
    pub confidence_in_self: f32,
    pub total_tasks_attempted: u64,
    pub total_tasks_succeeded: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProvenanceState {
    pub total_sources: u32,
    pub total_provenance_records: u64,
    pub total_evidence: u64,
    pub average_confidence: f32,
}
