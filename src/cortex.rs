//! Global orchestration and CortexRuntime.

use crate::config::CortexConfig;
use crate::error::CortexError;
use crate::runtime::{Runtime, RuntimeState};
use crate::transaction::mutation::{MutationKind, MutationLog, RecordParams};
use crate::transaction::state_tx::StateTransaction;
use crate::transaction::invariant::StateInvariant;
use crate::types::state::{
    AlgorithmVersions, CortexState, LanguageState, LearningState, MemoryState, NeuralState,
    PlanningState, ProvenanceState, ReasoningState, SelfModel, StateMetadata, VerificationState,
    WorldState, ARCHITECTURE_VERSION, SCHEMA_VERSION,
};
use crate::types::common::Timestamp;
use crate::language::tokenizer::Tokenizer;
use crate::language::vocabulary::Vocabulary;
use crate::neural::field::Field;
use crate::memory::episodic::EpisodicMemory;
use crate::memory::semantic::SemanticMemory;
use crate::memory::associative::AssociativeMemory;
use crate::memory::working::WorkingMemory;
use crate::world::entity::{EntityManager, EntityKind};
use crate::reasoning::hypothesis::HypothesisGenerator;
use crate::policy::gate::{PolicyDecision, PolicyGate};
use crate::learning::stability::StabilityGuard;
use crate::persistence::checkpoint::CheckpointManager;
use crate::persistence::format::FormatHandler;
use crate::memory::consolidation::ConsolidationEngine;
use crate::verification::VerificationPipeline;
use crate::self_model::SelfModelManager;
use crate::self_model::capability::SelfModel as CapabilitySelfModel;
use crate::observability::diagnostics::Diagnostics;

pub struct CortexRuntime {
    pub state: CortexState,
    pub config: CortexConfig,
    pub runtime_state: RuntimeState,
    pub state_version: u64,
    pub mutation_log: MutationLog,
    pub language_tokenizer: Tokenizer,
    pub language_vocabulary: Vocabulary,
    pub neural_field: Field,
    pub memory_episodic: EpisodicMemory,
    pub memory_semantic: SemanticMemory,
    pub memory_associative: AssociativeMemory,
    pub memory_working: WorkingMemory,
    pub world_entity_manager: EntityManager,
    pub reasoning_generator: HypothesisGenerator,
    pub policy_gate: PolicyGate,
    pub learning_stability: StabilityGuard,
    pub persistence_checkpoint: CheckpointManager,
    pub format_handler: FormatHandler,
    pub consolidation: ConsolidationEngine,
    pub verification_pipeline: VerificationPipeline,
    pub self_model_manager: SelfModelManager,
    pub self_model: CapabilitySelfModel,
    pub diagnostics: Diagnostics,
    observation_count: u64,
}

impl Default for CortexState {
    fn default() -> Self {
        Self {
            metadata: StateMetadata {
                state_id: [0u8; 16],
                created_at: Timestamp::now(),
                last_updated: Timestamp::now(),
                architecture_version: ARCHITECTURE_VERSION,
                schema_version: SCHEMA_VERSION,
                algorithm_versions: AlgorithmVersions {
                    cell_algorithm: "1.0.0".into(),
                    column_algorithm: "1.0.0".into(),
                    plasticity_algorithm: "1.0.0".into(),
                    memory_algorithm: "1.0.0".into(),
                    language_algorithm: "1.0.0".into(),
                    reasoning_algorithm: "1.0.0".into(),
                    planning_algorithm: "1.0.0".into(),
                    verification_algorithm: "1.0.0".into(),
                    consolidation_algorithm: "1.0.0".into(),
                },
                config_hash: [0u8; 32],
                episode_count: 0,
                total_learning_events: 0,
                checkpoint_count: 0,
            },
            language: LanguageState::default(),
            neural: NeuralState::default(),
            memory: MemoryState::default(),
            world: WorldState::default(),
            reasoning: ReasoningState::default(),
            planning: PlanningState::default(),
            verification: VerificationState::default(),
            learning: LearningState::default(),
            self_model: SelfModel::default(),
            provenance: ProvenanceState::default(),
        }
    }
}

impl CortexRuntime {
    pub fn new(config: CortexConfig) -> Result<Self, CortexError> {
        config.validate()?;

        let language_tokenizer = Tokenizer::new();
        let language_vocabulary = Vocabulary::new(config.language.vocabulary_capacity);
        let neural_field = Field::new(config.model.columns, config.model.cells);
        let memory_episodic = EpisodicMemory::new(config.memory.episodic_mb as usize);
        let memory_semantic = SemanticMemory::new(config.memory.semantic_mb as usize);
        let memory_associative = AssociativeMemory::new();
        let memory_working = WorkingMemory::new(config.memory.working_mb as usize);
        let world_entity_manager = EntityManager::new();
        let reasoning_generator =
            HypothesisGenerator::new(config.reasoning.max_steps as usize);
        let policy_gate = PolicyGate::new();
        let learning_stability = StabilityGuard::new(
            config.learning.learning_rate,
            config.learning.plasticity,
        );
        let persistence_checkpoint = CheckpointManager::new(10);
        let format_handler = FormatHandler::new();
        let consolidation = ConsolidationEngine::new(config.learning.consolidation_interval);
        let verification_pipeline = VerificationPipeline::new(config.verification.minimum_confidence);
        let self_model_manager = SelfModelManager::new();
        let self_model = CapabilitySelfModel::new();
        let diagnostics = Diagnostics::new();
        let mutation_log = MutationLog::new(10_000);

        Ok(Self {
            state: CortexState::default(),
            config,
            runtime_state: RuntimeState::Booting,
            state_version: 0,
            mutation_log,
            language_tokenizer,
            language_vocabulary,
            neural_field,
            memory_episodic,
            memory_semantic,
            memory_associative,
            memory_working,
            world_entity_manager,
            reasoning_generator,
            policy_gate,
            learning_stability,
            persistence_checkpoint,
            format_handler,
            consolidation,
            verification_pipeline,
            self_model_manager,
            self_model,
            diagnostics,
            observation_count: 0,
        })
    }

    fn transition_to(&mut self, target: RuntimeState) -> Result<(), CortexError> {
        if !self.runtime_state.can_transition_to(&target) {
            return Err(CortexError::RuntimeError(format!(
                "Invalid transition: {:?} -> {:?}",
                self.runtime_state, target
            )));
        }
        tracing::info!(from = ?self.runtime_state, to = ?target, "Runtime state transition");
        self.runtime_state = target;
        Ok(())
    }

    fn attempt_recovery(&mut self) -> Result<(), CortexError> {
        tracing::warn!("Attempting recovery");
        self.transition_to(RuntimeState::Recovering)?;

        self.observation_count = 0;
        self.memory_working.clear();

        if self.diagnostics.is_healthy() {
            self.transition_to(RuntimeState::Ready)?;
            tracing::info!("Recovery successful");
            Ok(())
        } else {
            tracing::error!("Recovery failed, stopping");
            self.transition_to(RuntimeState::Stopped)?;
            Err(CortexError::RuntimeError("Recovery failed".into()))
        }
    }

    pub fn save_state(&self) -> Result<(), CortexError> {
        let data = bincode::serialize(&self.state)
            .map_err(|e| CortexError::SerializationError(format!("Failed to serialize state: {}", e)))?;
        self.format_handler.save_to_file(&self.config.persistence.state, &data)
    }

    fn execute_pipeline(&mut self, input: &str) -> Result<String, CortexError> {
        let pre_version = self.state_version;

        StateInvariant::pre_mutation_check(&self.state, pre_version)?;

        // Stage 1: Parse observation
        let mut txn = StateTransaction::begin(
            MutationKind::LanguageEncode,
            "parse_observation",
            pre_version,
        );
        txn.apply("set_input")?;
        self.memory_working.set_input(input.to_string());
        self.mutation_log.record(RecordParams {
            kind: MutationKind::LanguageEncode,
            description: "parse observation",
            subsystem: "working_memory",
            pre_version,
            post_version: self.state_version,
            success: true,
            error: None,
        });

        // Stage 2: Encode language (tokenize)
        let tokens = self.language_tokenizer.tokenize(input)?;
        let mut symbol_ids = Vec::new();
        for token in &tokens {
            let id = self.language_vocabulary.lookup_or_create(token);
            symbol_ids.push(id);
        }
        self.state.language.symbols = symbol_ids;
        txn.commit(&mut self.mutation_log, self.state_version);

        // Stage 3: Process neural representation
        let gate_result = self.policy_gate.evaluate("neural_process");
        if gate_result.decision == PolicyDecision::Deny {
            return Err(CortexError::PolicyError(
                "Neural processing denied by policy".into(),
            ));
        }
        let max_active =
            (self.neural_field.columns.len() as f32 * self.config.model.sparsity_ratio) as usize;
        self.neural_field.enforce_sparsity(max_active);
        self.state.neural.active_cells = self
            .neural_field
            .columns
            .iter()
            .flat_map(|c| c.active_cells.iter().cloned())
            .collect();
        self.state.neural.active_columns = self
            .neural_field
            .columns
            .iter()
            .enumerate()
            .filter(|(_, c)| !c.active_cells.is_empty())
            .map(|(i, _)| crate::types::ids::ColumnId::from(i as u64))
            .collect();
        self.mutation_log.record(RecordParams {
            kind: MutationKind::NeuralProcess,
            description: "neural processing",
            subsystem: "neural",
            pre_version,
            post_version: self.state_version,
            success: true,
            error: None,
        });

        // Stage 4: Retrieve memories
        let recent_episodes = self.memory_episodic.recent(5);
        let context_strings: Vec<String> = recent_episodes
            .iter()
            .map(|e| e.observation.text.clone())
            .collect();
        self.state.memory.episodic.episodes.clear();
        for ep in &self.memory_episodic.episodes {
            self.state.memory.episodic.episodes.push(
                crate::types::state::EpisodeRecord {
                    id: ep.id,
                    observation: ep.observation.clone(),
                    timestamp: ep.timestamp,
                    importance: ep.importance,
                    consolidated: ep.consolidated,
                    retrieval_count: ep.retrieval_count,
                },
            );
        }
        self.state.memory.episodic.next_id =
            crate::types::ids::EpisodeId::from(self.memory_episodic.next_id);
        self.mutation_log.record(RecordParams {
            kind: MutationKind::MemoryStore,
            description: "memory retrieval and sync",
            subsystem: "memory",
            pre_version,
            post_version: self.state_version,
            success: true,
            error: None,
        });

        // Stage 5: Integrate world state
        let gate_result = self.policy_gate.evaluate("world_integrate");
        if gate_result.decision == PolicyDecision::Deny {
            return Err(CortexError::PolicyError(
                "World integration denied by policy".into(),
            ));
        }
        let words: Vec<&str> = input.split_whitespace().collect();
        for word in words.iter().take(3) {
            if word.len() > 3 {
                self.world_entity_manager
                    .create(word, EntityKind::ConceptualObject);
            }
        }
        self.state.world.entities = self
            .world_entity_manager
            .entities
            .iter()
            .map(|e| crate::types::state::Entity {
                id: e.id,
                name: e.name.clone(),
                confidence: e.confidence,
                created_at: e.created_at,
                updated_at: e.updated_at,
            })
            .collect();
        self.mutation_log.record(RecordParams {
            kind: MutationKind::WorldIntegrate,
            description: "world integration",
            subsystem: "world",
            pre_version,
            post_version: self.state_version,
            success: true,
            error: None,
        });

        // Stage 6: Evaluate reasoning
        let gate_result = self.policy_gate.evaluate("reasoning_evaluate");
        if gate_result.decision == PolicyDecision::Deny {
            return Err(CortexError::PolicyError(
                "Reasoning denied by policy".into(),
            ));
        }
        let hypotheses = self.reasoning_generator.generate(input, &context_strings);
        self.state.reasoning.active_hypotheses = hypotheses
            .iter()
            .map(|h| crate::types::state::Hypothesis {
                id: h.id,
                proposition: crate::types::state::Proposition {
                    subject: h.proposition.clone(),
                    predicate: "suggests".into(),
                    object: None,
                    negated: false,
                },
                confidence: h.confidence,
                evidence: crate::types::evidence::EvidenceSet::new(),
                counter_evidence: crate::types::evidence::EvidenceSet::new(),
                created_at: Timestamp::now(),
            })
            .collect();
        self.state.reasoning.budget_remaining = self.config.reasoning.max_steps;
        self.mutation_log.record(RecordParams {
            kind: MutationKind::ReasoningEvaluate,
            description: "reasoning evaluation",
            subsystem: "reasoning",
            pre_version,
            post_version: self.state_version,
            success: true,
            error: None,
        });

        // Stage 7: Evaluate planning (optional)
        if self.config.planning.enabled {
            let gate_result = self.policy_gate.evaluate("planning_evaluate");
            if gate_result.decision != PolicyDecision::Deny {
                let _plan = crate::planning::plan::PlanBuilder::new().build(input);
                self.state.planning.simulation_count += 1;
                self.mutation_log.record(RecordParams {
                    kind: MutationKind::PlanningEvaluate,
                    description: "planning evaluation",
                    subsystem: "planning",
                    pre_version,
                    post_version: self.state_version,
                    success: true,
                    error: None,
                });
            }
        }

        // Stage 8: Verify claims
        let top_confidence = hypotheses
            .first()
            .map(|h| h.confidence)
            .unwrap_or(0.0);
        let verified = top_confidence >= self.config.verification.minimum_confidence;

        // Stage 9: Generate response
        let response = if let Some(conclusion) = hypotheses.iter().find(|h| h.confidence > 0.3) {
            format!(
                "{} (confidence: {:.2})",
                conclusion.proposition, conclusion.confidence
            )
        } else {
            format!("Processed: {}", input)
        };

        // Stage 10: Record experience
        let importance = if hypotheses.is_empty() {
            0.5
        } else {
            hypotheses.iter().map(|h| h.confidence).sum::<f32>() / hypotheses.len() as f32
        };
        self.memory_episodic.store(crate::types::observation::Observation::user_provided(input));
        self.state.metadata.episode_count += 1;
        self.mutation_log.record(RecordParams {
            kind: MutationKind::MemoryStore,
            description: "experience recorded",
            subsystem: "episodic_memory",
            pre_version,
            post_version: self.state_version,
            success: true,
            error: None,
        });

        // Self-model update from experience
        if self.observation_count > 0 && !hypotheses.is_empty() {
            let prediction_correct = verified;
            self.self_model_manager.update_from_experience(prediction_correct, input);
            let assessment = self.self_model_manager.get_model().assess();
            self.state.self_model.prediction_accuracy = assessment.prediction_accuracy;
            self.state.self_model.uncertainty_level = 1.0 - assessment.overall;
        }

        // Stage 11: Apply learning
        if self.config.learning.enabled {
            let gate_result = self.policy_gate.evaluate("learning");
            if gate_result.decision == PolicyDecision::Allow {
                let change = (importance * self.config.learning.learning_rate).abs();
                if self.learning_stability.check_stability(change) {
                    self.state.learning.total_learning_events += 1;
                    self.state.learning.learning_rate = self.config.learning.learning_rate;
                    self.state.learning.plasticity_rate = self.config.learning.plasticity;
                    self.mutation_log.record(RecordParams {
                        kind: MutationKind::LearningApply,
                        description: "learning applied",
                        subsystem: "learning",
                        pre_version,
                        post_version: self.state_version,
                        success: true,
                        error: None,
                    });
                }
            }
        }

        // Stage 12: Checkpoint (if interval reached)
        if self.config.persistence.checkpoint_interval > 0
            && self.observation_count > 0
            && self.observation_count % self.config.persistence.checkpoint_interval == 0
        {
            self.persistence_checkpoint.create_checkpoint(
                self.memory_episodic.episodes.len() as u64,
                self.memory_episodic.next_id,
            );
            self.state.metadata.checkpoint_count += 1;
            let _ = self.save_state();
            self.mutation_log.record(RecordParams {
                kind: MutationKind::CheckpointCreate,
                description: "periodic checkpoint",
                subsystem: "persistence",
                pre_version,
                post_version: self.state_version,
                success: true,
                error: None,
            });
        }

        // Consolidation check
        if self.config.learning.consolidation_interval > 0
            && self.observation_count > 0
            && self.observation_count % self.config.learning.consolidation_interval == 0
        {
            self.state.learning.total_consolidation_events += 1;
            tracing::info!(count = self.observation_count, "Memory consolidation triggered");
            self.consolidation.record_episode();
            if self.consolidation.should_consolidate() {
                tracing::info!("Running memory consolidation");
                let report = self.consolidation.consolidate(
                    &mut self.memory_episodic,
                    &mut self.memory_semantic,
                    &mut self.memory_associative,
                );
                tracing::info!(
                    episodes_merged = report.episodes_merged,
                    knowledge_created = report.knowledge_extracted,
                    associations_strengthened = report.patterns_strengthened,
                    memories_decayed = report.memories_decayed,
                    "Consolidation complete"
                );
                self.consolidation.reset_counter();
            }
        }

        self.state_version += 1;
        self.state.metadata.last_updated = Timestamp::now();

        StateInvariant::post_mutation_check(&self.state, self.state_version)?;

        Ok(response)
    }

    pub fn process(&mut self, input: &str) -> Result<String, CortexError> {
        if !self.ready() {
            return Err(CortexError::RuntimeError(
                "Runtime not in Ready state".into(),
            ));
        }

        self.transition_to(RuntimeState::Processing)?;

        match self.execute_pipeline(input) {
            Ok(response) => {
                self.observation_count += 1;
                self.state.metadata.last_updated = Timestamp::now();
                self.transition_to(RuntimeState::Ready)?;
                Ok(response)
            }
            Err(e) if e.is_recoverable() => {
                let _ = self.transition_to(RuntimeState::Fault);
                self.attempt_recovery()?;
                Err(e)
            }
            Err(e) => {
                let _ = self.transition_to(RuntimeState::Fault);
                let _ = self.transition_to(RuntimeState::Stopped);
                Err(e)
            }
        }
    }
}

impl Runtime for CortexRuntime {
    fn boot(&mut self) -> Result<(), CortexError> {
        tracing::info!("Boot sequence initiated");

        self.transition_to(RuntimeState::LoadingConfig)?;
        self.config.validate()?;
        tracing::info!("Configuration validated");

        self.transition_to(RuntimeState::LoadingState)?;
        let state_path = &self.config.persistence.state;
        if std::path::Path::new(state_path).exists() {
            match self.format_handler.load_from_file(state_path) {
                Ok(data) => {
                    match bincode::deserialize::<CortexState>(&data) {
                        Ok(loaded_state) => {
                            if loaded_state.metadata.architecture_version == ARCHITECTURE_VERSION {
                                self.state = loaded_state;
                                tracing::info!(path = %state_path, "State loaded from disk");
                            } else {
                                tracing::warn!("Architecture version mismatch, using fresh state");
                            }
                        }
                        Err(e) => {
                            tracing::warn!(error = %e, "Failed to deserialize state, using fresh state");
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!(error = %e, "Failed to load state file, using fresh state");
                }
            }
        }
        tracing::info!(version = self.state_version, "State loaded");

        self.transition_to(RuntimeState::Validating)?;
        StateInvariant::validate_state(&self.state)?;
        self.state.metadata.last_updated = Timestamp::now();
        tracing::info!("State validated");

        self.transition_to(RuntimeState::Initializing)?;
        self.state.metadata.architecture_version = ARCHITECTURE_VERSION;
        self.state.metadata.schema_version = SCHEMA_VERSION;
        self.state.learning.learning_rate = self.config.learning.learning_rate;
        self.state.learning.plasticity_rate = self.config.learning.plasticity;
        self.state_version = 1;
        self.mutation_log.record(RecordParams {
            kind: MutationKind::StateInitialize,
            description: "runtime initialized",
            subsystem: "runtime",
            pre_version: 0,
            post_version: self.state_version,
            success: true,
            error: None,
        });
        tracing::info!(
            architecture_version = ARCHITECTURE_VERSION,
            schema_version = SCHEMA_VERSION,
            "Subsystems initialized"
        );

        self.transition_to(RuntimeState::Ready)?;
        tracing::info!("Boot complete - Ready");

        Ok(())
    }

    fn ready(&self) -> bool {
        matches!(self.runtime_state, RuntimeState::Ready)
    }

    fn run(&mut self) -> Result<(), CortexError> {
        if !self.ready() {
            return Err(CortexError::RuntimeError("Runtime not ready".into()));
        }
        tracing::info!("Runtime entering main loop");
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), CortexError> {
        tracing::info!("Shutdown initiated");

        self.transition_to(RuntimeState::ShuttingDown)?;

        StateInvariant::validate_state(&self.state)?;

        self.state.metadata.last_updated = Timestamp::now();
        tracing::info!("State validated for shutdown");

        match self.save_state() {
            Ok(()) => tracing::info!("State saved to disk"),
            Err(e) => tracing::warn!(error = %e, "Failed to save state to disk"),
        }
        self.persistence_checkpoint.create_checkpoint(
            self.memory_episodic.episodes.len() as u64,
            self.memory_episodic.next_id,
        );
        self.state.metadata.checkpoint_count += 1;
        self.mutation_log.record(RecordParams {
            kind: MutationKind::CheckpointCreate,
            description: "shutdown checkpoint",
            subsystem: "persistence",
            pre_version: self.state_version,
            post_version: self.state_version,
            success: true,
            error: None,
        });
        tracing::info!(checkpoint_count = self.state.metadata.checkpoint_count, "Final checkpoint created");

        self.transition_to(RuntimeState::Stopped)?;
        tracing::info!(
            version = self.state_version,
            mutations = self.mutation_log.records.len(),
            "Shutdown complete"
        );

        Ok(())
    }
}
