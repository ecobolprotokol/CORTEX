pub mod diagnostics;

use crate::types::*;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct RuntimeStatus {
    pub status: String,
    pub uptime_seconds: u64,
    pub episode_count: u64,
    pub vocabulary_size: u32,
    pub entity_count: usize,
    pub memory_pressure: MemoryPressure,
}

pub fn compute_status(state: &CortexState) -> RuntimeStatus {
    RuntimeStatus {
        status: "ready".into(),
        uptime_seconds: 0,
        episode_count: state.metadata.episode_count,
        vocabulary_size: state.language.vocabulary_size,
        entity_count: state.world.entities.len(),
        memory_pressure: MemoryPressure::Low,
    }
}

pub fn format_status(status: &RuntimeStatus) -> String {
    format!(
        "Status: {}\nEpisodes: {}\nVocabulary: {}\nEntities: {}",
        status.status, status.episode_count, status.vocabulary_size, status.entity_count
    )
}

#[derive(Debug, Clone)]
pub struct MetricsSnapshot {
    pub timestamp: Timestamp,
    pub episode_count: u64,
    pub vocabulary_size: u32,
    pub memory_usage_bytes: u64,
    pub memory_capacity_bytes: u64,
    pub prediction_error: f32,
    pub learning_events: u64,
    pub active_cells: usize,
    pub entity_count: usize,
    pub hypothesis_count: usize,
}

pub struct MetricsCollector {
    snapshots: VecDeque<MetricsSnapshot>,
    max_snapshots: usize,
}

impl MetricsCollector {
    pub fn new(max_snapshots: usize) -> Self {
        Self {
            snapshots: VecDeque::new(),
            max_snapshots,
        }
    }

    pub fn record(&mut self, snapshot: MetricsSnapshot) {
        if self.snapshots.len() >= self.max_snapshots {
            self.snapshots.pop_front();
        }
        self.snapshots.push_back(snapshot);
    }

    pub fn latest(&self) -> Option<&MetricsSnapshot> {
        self.snapshots.back()
    }

    pub fn history(&self) -> Vec<&MetricsSnapshot> {
        self.snapshots.iter().collect()
    }

    pub fn count(&self) -> usize {
        self.snapshots.len()
    }

    pub fn average_prediction_error(&self) -> f32 {
        if self.snapshots.is_empty() {
            return 0.0;
        }
        let sum: f32 = self.snapshots.iter().map(|s| s.prediction_error).sum();
        sum / self.snapshots.len() as f32
    }

    pub fn memory_usage_trend(&self) -> Vec<f32> {
        self.snapshots.iter().map(|s| {
            if s.memory_capacity_bytes == 0 {
                0.0
            } else {
                s.memory_usage_bytes as f32 / s.memory_capacity_bytes as f32
            }
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_status() {
        let state = CortexState {
            language: LanguageState {
                symbols: Vec::new(),
                tokens: Vec::new(),
                vocabulary_size: 100,
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
                    capacity_bytes: 1024,
                    current_usage_bytes: 0,
                    next_id: EpisodeId(1),
                },
                semantic: SemanticMemory {
                    knowledge: Vec::new(),
                    capacity_bytes: 1024,
                    current_usage_bytes: 0,
                    next_id: KnowledgeId(1),
                },
                procedural: ProceduralMemory {
                    procedures: Vec::new(),
                    capacity_bytes: 1024,
                    current_usage_bytes: 0,
                    next_id: ProcedureId(1),
                },
                associative: AssociativeMemory {
                    associations: Vec::new(),
                    capacity_bytes: 1024,
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
                episode_count: 42,
                total_learning_events: 100,
                checkpoint_count: 5,
            },
        };
        let status = compute_status(&state);
        assert_eq!(status.episode_count, 42);
        assert_eq!(status.vocabulary_size, 100);
        assert_eq!(status.status, "ready");
    }

    #[test]
    fn test_metrics_collector() {
        let mut collector = MetricsCollector::new(3);
        for i in 0..5 {
            collector.record(MetricsSnapshot {
                timestamp: Timestamp::now(),
                episode_count: i,
                vocabulary_size: 100,
                memory_usage_bytes: 1000,
                memory_capacity_bytes: 10000,
                prediction_error: i as f32 * 0.1,
                learning_events: i * 10,
                active_cells: 50,
                entity_count: 5,
                hypothesis_count: 3,
            });
        }
        assert_eq!(collector.count(), 3);
        assert!(collector.latest().is_some());
        assert_eq!(collector.latest().unwrap().episode_count, 4);
        let avg = collector.average_prediction_error();
        assert!((avg - 0.3).abs() < 0.01);
    }
}
