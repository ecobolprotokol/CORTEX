use crate::error::{CortexError, Result};
use crate::types::*;

pub const MAGIC: &[u8; 8] = b"CORTEX\0\0";
pub const FORMAT_VERSION: u32 = 1;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CxHeader {
    pub magic: [u8; 8],
    pub format_version: u32,
    pub architecture_version: u32,
    pub state_id: uuid::Uuid,
    pub created_at: Timestamp,
    pub last_checkpoint: Timestamp,
    pub state_checksum: [u8; 32],
    pub section_count: u32,
}

impl CxHeader {
    pub fn new(state_id: uuid::Uuid) -> Self {
        Self {
            magic: *MAGIC,
            format_version: FORMAT_VERSION,
            architecture_version: 1,
            state_id,
            created_at: Timestamp::now(),
            last_checkpoint: Timestamp::now(),
            state_checksum: [0u8; 32],
            section_count: 0,
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.magic != *MAGIC {
            return Err(CortexError::PersistenceError("Invalid magic bytes".into()));
        }
        if self.format_version != FORMAT_VERSION {
            return Err(CortexError::PersistenceError(format!(
                "Unsupported format version: {}",
                self.format_version
            )));
        }
        Ok(())
    }
}

pub fn save_cx(path: &str, state: &CortexState) -> Result<()> {
    let data = bincode::serialize(state)
        .map_err(|e| CortexError::PersistenceError(format!("Serialization failed: {}", e)))?;

    let checksum = *blake3::hash(&data).as_bytes();

    let header = CxHeader {
        state_checksum: checksum,
        last_checkpoint: Timestamp::now(),
        ..CxHeader::new(state.metadata.state_id)
    };

    let header_bytes = bincode::serialize(&header)
        .map_err(|e| CortexError::PersistenceError(format!("Header serialization failed: {}", e)))?;

    let header_len = header_bytes.len() as u64;
    let mut file_data = Vec::with_capacity(8 + 8 + header_bytes.len() + data.len());
    file_data.extend_from_slice(&header_len.to_le_bytes());
    file_data.extend_from_slice(&(data.len() as u64).to_le_bytes());
    file_data.extend_from_slice(&header_bytes);
    file_data.extend_from_slice(&data);

    let temp_path = format!("{}.tmp", path);
    std::fs::write(&temp_path, &file_data)
        .map_err(|e| CortexError::PersistenceError(format!("Write failed: {}", e)))?;
    std::fs::rename(&temp_path, path)
        .map_err(|e| CortexError::PersistenceError(format!("Atomic replace failed: {}", e)))?;
    Ok(())
}

pub fn load_cx(path: &str) -> Result<CortexState> {
    let file_data = std::fs::read(path)
        .map_err(|e| CortexError::PersistenceError(format!("Read failed: {}", e)))?;

    if file_data.len() < 16 {
        return Err(CortexError::PersistenceError("File too small for header".into()));
    }

    let header_len = u64::from_le_bytes(file_data[0..8].try_into().unwrap()) as usize;
    let data_len = u64::from_le_bytes(file_data[8..16].try_into().unwrap()) as usize;

    if file_data.len() < 16 + header_len + data_len {
        return Err(CortexError::PersistenceError("File truncated".into()));
    }

    let header_bytes = &file_data[16..16 + header_len];
    let data_bytes = &file_data[16 + header_len..16 + header_len + data_len];

    let header: CxHeader = bincode::deserialize(header_bytes)
        .map_err(|e| CortexError::PersistenceError(format!("Header deserialization failed: {}", e)))?;
    header.validate()?;

    let computed_checksum = *blake3::hash(data_bytes).as_bytes();
    if computed_checksum != header.state_checksum {
        return Err(CortexError::PersistenceError(
            "Checksum mismatch - state may be corrupted".into(),
        ));
    }

    let state: CortexState = bincode::deserialize(data_bytes)
        .map_err(|e| CortexError::PersistenceError(format!("Deserialization failed: {}", e)))?;
    Ok(state)
}

pub fn compute_checksum(data: &[u8]) -> [u8; 32] {
    *blake3::hash(data).as_bytes()
}

pub fn verify_integrity(path: &str) -> Result<bool> {
    let file_data = std::fs::read(path)
        .map_err(|e| CortexError::PersistenceError(format!("Read failed: {}", e)))?;

    if file_data.len() < 16 {
        return Ok(false);
    }

    let header_len = u64::from_le_bytes(file_data[0..8].try_into().unwrap()) as usize;
    let data_len = u64::from_le_bytes(file_data[8..16].try_into().unwrap()) as usize;

    if file_data.len() < 16 + header_len + data_len {
        return Ok(false);
    }

    let header_bytes = &file_data[16..16 + header_len];
    let data_bytes = &file_data[16 + header_len..16 + header_len + data_len];

    let header: CxHeader = match bincode::deserialize(header_bytes) {
        Ok(h) => h,
        Err(_) => return Ok(false),
    };

    if header.validate().is_err() {
        return Ok(false);
    }

    let computed_checksum = *blake3::hash(data_bytes).as_bytes();
    Ok(computed_checksum == header.state_checksum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    fn make_test_state() -> CortexState {
        CortexState {
            language: LanguageState {
                symbols: Vec::new(),
                tokens: Vec::new(),
                vocabulary_size: 0,
                next_symbol_id: SymbolId(1),
            },
            neural: NeuralState {
                fields: Vec::new(),
                active_cells: Vec::new(),
                active_columns: Vec::new(),
                temporal_buffer: Vec::new(),
                prediction: None,
            },
            memory: MemoryState {
                working: WorkingMemory {
                    input: None,
                    conversation_context: ConversationContext {
                        session_id: SessionId(1),
                        turn_count: 0,
                        recent_inputs: Vec::new(),
                        recent_outputs: Vec::new(),
                        started_at: Timestamp::now(),
                    },
                    active_concepts: Vec::new(),
                    active_hypotheses: Vec::new(),
                    goals: Vec::new(),
                    reasoning_state: None,
                    world_assumptions: Vec::new(),
                    generation_state: None,
                },
                episodic: EpisodicMemory {
                    episodes: Vec::new(),
                    capacity_bytes: 1024 * 1024,
                    current_usage_bytes: 0,
                    next_id: EpisodeId(1),
                },
                semantic: SemanticMemory {
                    knowledge: Vec::new(),
                    capacity_bytes: 1024 * 1024,
                    current_usage_bytes: 0,
                    next_id: KnowledgeId(1),
                },
                procedural: ProceduralMemory {
                    procedures: Vec::new(),
                    capacity_bytes: 1024 * 1024,
                    current_usage_bytes: 0,
                    next_id: ProcedureId(1),
                },
                associative: AssociativeMemory {
                    associations: Vec::new(),
                    capacity_bytes: 1024 * 1024,
                    current_usage_bytes: 0,
                    next_id: AssociationId(1),
                },
            },
            world: WorldState {
                entities: Vec::new(),
                relations: Vec::new(),
                active_events: Vec::new(),
                temporal_context: TemporalContext::default(),
                uncertainty: UncertaintyState::initial(),
                next_entity_id: EntityId(1),
                next_relation_id: RelationId(1),
                next_event_id: EventId(1),
            },
            reasoning: ReasoningState {
                active_hypotheses: Vec::new(),
                conclusion: None,
                premises: Vec::new(),
                evidence_index: std::collections::HashMap::new(),
                contradiction_log: Vec::new(),
                budget_remaining: 32,
                next_hypothesis_id: HypothesisId(1),
            },
            planning: PlanningState {
                active_goals: Vec::new(),
                candidate_plans: Vec::new(),
                selected_plan: None,
                budget_remaining: 8,
                simulation_count: 0,
                next_plan_id: PlanId(1),
                next_goal_id: GoalId(1),
            },
            verification: VerificationState {
                pending_claims: Vec::new(),
                verified_claims: 0,
                contradicted_claims: 0,
                confidence_threshold: 0.8,
            },
            learning: LearningState {
                enabled: true,
                total_learning_events: 0,
                total_replay_events: 0,
                total_consolidation_events: 0,
                average_prediction_error: 0.0,
                learning_rate: 0.001,
                plasticity_rate: 0.01,
                next_consolidation_at: 1000,
                pending_experiences: Vec::new(),
            },
            self_model: SelfModel {
                capabilities: CapabilityEstimate {
                    language_accuracy: 0.5,
                    prediction_accuracy: 0.5,
                    verification_reliability: 0.5,
                    planning_success: 0.5,
                    memory_retrieval_success: 0.5,
                    reasoning_consistency: 0.5,
                    resource_availability: 1.0,
                },
                limitations: Limitations {
                    known_limitations: Vec::new(),
                    resource_constraints: Vec::new(),
                    capability_gaps: Vec::new(),
                },
                prediction_accuracy: 0.5,
                uncertainty_level: 0.5,
                memory_health: MemoryHealth {
                    pressure: MemoryPressure::Low,
                    fragmentation: 0.0,
                    consolidation_backlog: 0,
                },
                last_updated: Timestamp::now(),
            },
            provenance: ProvenanceState {
                sources: Vec::new(),
                next_source_id: SourceId(1),
            },
            metadata: StateMetadata {
                state_id: uuid::Uuid::new_v4(),
                created_at: Timestamp::now(),
                last_updated: Timestamp::now(),
                architecture_version: 1,
                algorithm_versions: AlgorithmVersions {
                    cell_algorithm: 1,
                    column_algorithm: 1,
                    plasticity_algorithm: 1,
                    memory_algorithm: 1,
                    language_algorithm: 1,
                    reasoning_algorithm: 1,
                    planning_algorithm: 1,
                    verification_algorithm: 1,
                    consolidation_algorithm: 1,
                },
                config_hash: [0u8; 32],
                episode_count: 0,
                total_learning_events: 0,
                checkpoint_count: 0,
            },
        }
    }

    #[test]
    fn test_save_load_roundtrip() {
        let tmp = NamedTempFile::new().unwrap();
        let path = tmp.path().to_str().unwrap();
        let state = make_test_state();
        save_cx(path, &state).unwrap();
        let loaded = load_cx(path).unwrap();
        assert_eq!(state.metadata.state_id, loaded.metadata.state_id);
        assert_eq!(state.metadata.episode_count, loaded.metadata.episode_count);
    }

    #[test]
    fn test_integrity_check() {
        let tmp = NamedTempFile::new().unwrap();
        let path = tmp.path().to_str().unwrap();
        let state = make_test_state();
        save_cx(path, &state).unwrap();
        assert!(verify_integrity(path).unwrap());
    }
}
