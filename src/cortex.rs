use crate::config::CortexConfig;
use crate::error::Result;
use crate::language::LanguageCore;
use crate::neural::NeuralCore;
use crate::memory::MemorySystem;
use crate::world::WorldModelInterface;
use crate::reasoning::ReasoningEngine;
use crate::planning::PlanningEngine;
use crate::verification::VerificationEngine;
use crate::learning::LearningSystem;
use crate::persistence::PersistenceEngine;
use crate::types::*;

pub struct CortexRuntime {
    pub state: CortexState,
    pub config: CortexConfig,
    pub language: crate::language::LanguageCoreImpl,
    pub neural: crate::neural::NeuralCoreImpl,
    pub memory: crate::memory::MemorySystemImpl,
    pub world: crate::world::WorldModelImpl,
    pub reasoning: crate::reasoning::ReasoningEngineImpl,
    pub planning: crate::planning::PlanningEngineImpl,
    pub verification: crate::verification::VerificationEngineImpl,
    pub learning: crate::learning::LearningSystemImpl,
    pub self_model: crate::self_model::SelfModelImpl,
    pub policy: crate::policy::PolicyEngineImpl,
    pub persistence: crate::persistence::PersistenceEngineImpl,
    pub budget: ComputeBudget,
}

impl CortexRuntime {
    pub fn boot(config: CortexConfig) -> Result<Self> {
        let persistence = crate::persistence::PersistenceEngineImpl::new(&config.persistence)?;
        let state = if persistence.exists() {
            persistence.load()?
        } else {
            Self::initialize_state(&config)?
        };
        let budget = config.compute_budget();
        let language = crate::language::LanguageCoreImpl::new(&config.language)?;
        let neural = crate::neural::NeuralCoreImpl::new(&config.model)?;
        let memory = crate::memory::MemorySystemImpl::new(&config.memory)?;
        let world = crate::world::WorldModelImpl::new(&config.world)?;
        let reasoning = crate::reasoning::ReasoningEngineImpl::new(&config.reasoning)?;
        let planning = crate::planning::PlanningEngineImpl::new(&config.planning)?;
        let verification = crate::verification::VerificationEngineImpl::new(&config.verification)?;
        let learning = crate::learning::LearningSystemImpl::new(&config.learning)?;
        let self_model = crate::self_model::SelfModelImpl::new()?;
        let policy = crate::policy::PolicyEngineImpl::new(&config.policy)?;
        Ok(Self {
            state,
            config,
            language,
            neural,
            memory,
            world,
            reasoning,
            planning,
            verification,
            learning,
            self_model,
            policy,
            persistence,
            budget,
        })
    }

    pub fn process(&mut self, input: &str) -> Result<String> {
        let _observation = Observation::user_provided(input);
        let _context = self.memory.working_memory().conversation_context.clone();
        let ctx = ContextState::initial();
        let language_state = self.language.encode(input, &ctx)?;
        let neural_repr = self.neural.process(&language_state, &ctx)?;
        let query = MemoryQuery {
            query_type: MemoryQueryType::All,
            text: input.to_string(),
            concept_ids: Vec::new(),
            time_range: None,
            max_results: 10,
            min_confidence: 0.0,
        };
        let memories = self.memory.retrieve(&query, &ctx)?;
        let world_state = self.world.integrate(&neural_repr, &memories)?;
        let reasoning_result = self.reasoning.evaluate(&neural_repr, &memories, &world_state)?;
        let _plan = self.planning.evaluate(&reasoning_result, &world_state)?;
        let verified = self.verification.evaluate(&reasoning_result)?;
        let response = self.language.generate(&verified)?;
        self.memory.working_memory_mut().conversation_context.turn_count += 1;
        self.memory.working_memory_mut().conversation_context.recent_inputs.push(input.to_string());
        self.state.metadata.episode_count += 1;
        Ok(response.text)
    }

    pub fn save(&self) -> Result<()> {
        self.persistence.save(&self.state)
    }

    pub fn checkpoint(&mut self) -> Result<CheckpointId> {
        let id = self.persistence.checkpoint(&self.state)?;
        self.state.metadata.checkpoint_count += 1;
        Ok(id)
    }

    fn initialize_state(config: &CortexConfig) -> Result<CortexState> {
        Ok(CortexState {
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
                    capacity_bytes: config.memory.episodic_mb as u64 * 1024 * 1024,
                    current_usage_bytes: 0,
                    next_id: EpisodeId(1),
                },
                semantic: SemanticMemory {
                    knowledge: Vec::new(),
                    capacity_bytes: config.memory.semantic_mb as u64 * 1024 * 1024,
                    current_usage_bytes: 0,
                    next_id: KnowledgeId(1),
                },
                procedural: ProceduralMemory {
                    procedures: Vec::new(),
                    capacity_bytes: config.memory.procedural_mb as u64 * 1024 * 1024,
                    current_usage_bytes: 0,
                    next_id: ProcedureId(1),
                },
                associative: AssociativeMemory {
                    associations: Vec::new(),
                    capacity_bytes: config.memory.associative_mb as u64 * 1024 * 1024,
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
                budget_remaining: config.reasoning.max_steps,
                next_hypothesis_id: HypothesisId(1),
            },
            planning: PlanningState {
                active_goals: Vec::new(),
                candidate_plans: Vec::new(),
                selected_plan: None,
                budget_remaining: config.planning.max_depth,
                simulation_count: 0,
                next_plan_id: PlanId(1),
                next_goal_id: GoalId(1),
            },
            verification: VerificationState {
                pending_claims: Vec::new(),
                verified_claims: 0,
                contradicted_claims: 0,
                confidence_threshold: config.verification.minimum_confidence,
            },
            learning: LearningState {
                enabled: config.learning.enabled,
                total_learning_events: 0,
                total_replay_events: 0,
                total_consolidation_events: 0,
                average_prediction_error: 0.0,
                learning_rate: config.learning.learning_rate,
                plasticity_rate: config.learning.plasticity,
                next_consolidation_at: config.learning.consolidation_interval,
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
                config_hash: config.config_hash(),
                episode_count: 0,
                total_learning_events: 0,
                checkpoint_count: 0,
            },
        })
    }
}
