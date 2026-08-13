use crate::config::CortexConfig;
use crate::error::Result;
use crate::language::LanguageCore;
use crate::neural::NeuralCore;
use crate::memory::MemorySystem;
use crate::memory::working;
use crate::world::WorldModelInterface;
use crate::reasoning::ReasoningEngine;
use crate::planning::PlanningEngine;
use crate::verification::VerificationEngine;
use crate::learning::LearningSystem;
use crate::persistence::PersistenceEngine;
use crate::self_model::SelfModelInterface;
use crate::runtime::{Runtime, RuntimeState};
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
    pub runtime: Runtime,
}

impl CortexRuntime {
    pub fn boot(config: CortexConfig) -> Result<Self> {
        let mut rt = Runtime::new();
        let _ = rt.transition(RuntimeState::LoadingConfig);
        let persistence = crate::persistence::PersistenceEngineImpl::new(&config.persistence)?;
        let _ = rt.transition(RuntimeState::LoadingState);
        let state = if persistence.exists() {
            persistence.load()?
        } else {
            Self::initialize_state(&config)?
        };
        let _ = rt.transition(RuntimeState::Validating);
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
        let _ = rt.transition(RuntimeState::Ready);
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
            runtime: rt,
        })
    }

    pub fn process(&mut self, input: &str) -> Result<String> {
        let _ = self.runtime.transition(RuntimeState::Processing);

        let observation = Observation::user_provided(input);
        let episode_count = self.state.metadata.episode_count;

        let mut ctx = ContextState::initial();
        ctx.advance_time();

        working::update_input(&mut self.memory.working_memory_mut(), input);

        let language_state = self.language.encode(input, &ctx)?;
        let vocab_size = language_state.vocabulary_size;

        let neural_repr = self.neural.process(&language_state, &ctx)?;

        let query = MemoryQuery {
            query_type: MemoryQueryType::All,
            text: input.to_string(),
            concept_ids: Vec::new(),
            time_range: None,
            max_results: self.budget.max_memory_retrieval,
            min_confidence: 0.0,
        };
        let memories = self.memory.retrieve(&query, &ctx)?;

        let _retrieval_success = if memories.episodic.is_empty() && memories.semantic.is_empty() {
            0.3
        } else {
            0.8
        };

        let world_state = self.world.integrate(&neural_repr, &memories)?;

        let reasoning_result = self.reasoning.evaluate(&neural_repr, &memories, &world_state)?;

        let plan = self.planning.evaluate(&reasoning_result, &world_state)?;

        let verified = self.verification.evaluate(&reasoning_result)?;

        let response = self.language.generate(&verified)?;

        let _ = self.runtime.transition(RuntimeState::Learning);

        let previous_prediction = self.state.neural.prediction.clone();
        let prediction_error = if let Some(ref prev_pred) = previous_prediction {
            let actual_encoding: Vec<f32> = neural_repr.field_activations.clone();
            if prev_pred.predicted_state.is_empty() || actual_encoding.is_empty() {
                PredictionError::zero()
            } else {
                let min_len = prev_pred.predicted_state.len().min(actual_encoding.len());
                PredictionError::compute(
                    &prev_pred.predicted_state[..min_len],
                    &actual_encoding[..min_len],
                )
            }
        } else {
            PredictionError::zero()
        };

        let attribution = self.determine_attribution(&prediction_error, &neural_repr);

        let experience = Experience {
            observation: observation.clone(),
            internal_state: StateSnapshot {
                language_vocabulary_size: vocab_size,
                neural_active_cells: neural_repr.active_cells.len(),
                memory_episode_count: episode_count as usize,
                world_entity_count: world_state.entities.len(),
                reasoning_hypothesis_count: reasoning_result.hypotheses.len(),
                timestamp: Timestamp::now(),
            },
            prediction: neural_repr.prediction.clone().unwrap_or(Prediction {
                target: PredictionTarget::NextState,
                predicted_state: neural_repr.field_activations.clone(),
                confidence: 0.5,
                timestamp: Timestamp::now(),
                context: ctx.clone(),
                resolved: false,
                actual: None,
                error: None,
            }),
            action: plan.map(|_p| Action {
                id: ActionId(0),
                kind: ActionKind::Respond,
                parameters: std::collections::HashMap::new(),
                expected_outcome: None,
                risk: RiskAssessment::default(),
                timestamp: Timestamp::now(),
                provenance: Provenance::system("cortex"),
            }),
            outcome: Some(Outcome {
                success: true,
                description: "cognitive_cycle_complete".into(),
                result: Some(response.text.clone()),
                timestamp: Timestamp::now(),
                confidence: response.confidence,
            }),
            error: prediction_error.clone(),
            attribution,
            evidence: {
                let mut es = EvidenceSet::new();
                if verified.overall_confidence.evidence_strength > 0.0 {
                    es.add(Evidence {
                        id: EvidenceId(episode_count),
                        source: Provenance::system("cortex"),
                        content: EvidenceContent::Text(response.text.clone()),
                        strength: response.confidence,
                        polarity: EvidencePolarity::Supports,
                        timestamp: Timestamp::now(),
                        related: Vec::new(),
                    });
                }
                es
            },
            provenance: Provenance::derived(&[observation.source.clone()]),
        };

        let learning_signal = self.learning.record(experience.clone())?;

        self.learning.apply_signal(&learning_signal)?;

        let episode = Episode {
            id: self.memory.state().episodic.next_id,
            observation: observation.clone(),
            context: ctx.clone(),
            action: None,
            outcome: experience.outcome.clone(),
            timestamp: Timestamp::now(),
            prediction: Some(experience.prediction.clone()),
            prediction_error: prediction_error.clone(),
            confidence: verified.overall_confidence.clone(),
            source: experience.provenance.clone(),
            importance: observation.importance,
            retrieval_count: 0,
            last_retrieved: None,
            consolidated: false,
        };
        self.memory.store_episode(episode)?;

        if self.state.learning.total_learning_events % 10 == 0 && self.state.learning.total_learning_events > 0 {
            let current_pressure = self.compute_memory_pressure();
            self.self_model.update(&crate::self_model::ModelMetrics {
                prediction_error: prediction_error.magnitude,
                memory_pressure: current_pressure,
                episode_count: self.state.metadata.episode_count,
            });
        }

        let _ = self.runtime.transition(RuntimeState::Consolidating);
        self.maybe_consolidate()?;

        if self.config.learning.replay && self.state.learning.total_learning_events > 0
            && self.state.learning.total_learning_events % 50 == 0
        {
            self.run_replay()?;
        }

        self.state.metadata.episode_count += 1;
        self.state.learning.total_learning_events += 1;
        self.state.metadata.last_updated = Timestamp::now();

        working::update_output(&mut self.memory.working_memory_mut(), &response.text);

        if self.state.metadata.episode_count % self.config.persistence.checkpoint_interval == 0
            && self.state.metadata.episode_count > 0
        {
            let _ = self.runtime.transition(RuntimeState::Checkpointing);
            let _ = self.checkpoint();
        }

        let _ = self.runtime.transition(RuntimeState::Ready);
        Ok(response.text)
    }

    fn determine_attribution(
        &self,
        error: &PredictionError,
        repr: &crate::neural::NeuralRepresentation,
    ) -> ErrorAttribution {
        if error.magnitude > 0.5 {
            ErrorAttribution::EnvironmentError
        } else if repr.confidence.belief < 0.3 {
            ErrorAttribution::InputError
        } else if self.memory.state().episodic.episodes.len() < 5 {
            ErrorAttribution::MemoryError
        } else if self.memory.state().working.active_concepts.is_empty() {
            ErrorAttribution::ReasoningError
        } else {
            ErrorAttribution::InputError
        }
    }

    fn compute_memory_pressure(&self) -> f32 {
        let episodic_usage = self.memory.state().episodic.current_usage_bytes as f32
            / self.memory.state().episodic.capacity_bytes.max(1) as f32;
        let semantic_usage = self.memory.state().semantic.current_usage_bytes as f32
            / self.memory.state().semantic.capacity_bytes.max(1) as f32;
        let procedural_usage = self.memory.state().procedural.current_usage_bytes as f32
            / self.memory.state().procedural.capacity_bytes.max(1) as f32;
        let associative_usage = self.memory.state().associative.current_usage_bytes as f32
            / self.memory.state().associative.capacity_bytes.max(1) as f32;
        (episodic_usage + semantic_usage + procedural_usage + associative_usage) / 4.0
    }

    fn maybe_consolidate(&mut self) -> Result<()> {
        if !self.config.learning.enabled {
            return Ok(());
        }
        let episode_count = self.state.metadata.episode_count;
        if episode_count >= self.state.learning.next_consolidation_at {
            let consolidated = {
                let mem = self.memory.state_mut();
                crate::memory::consolidation::consolidate(
                    &mut mem.episodic,
                    &mut mem.semantic,
                )
            };
            if consolidated > 0 {
                self.state.learning.total_consolidation_events += consolidated as u64;
            }
            self.state.learning.next_consolidation_at =
                episode_count + self.config.learning.consolidation_interval;
        }
        Ok(())
    }

    fn run_replay(&mut self) -> Result<()> {
        if !self.config.learning.enabled || !self.config.learning.replay {
            return Ok(());
        }
        let pending = self.state.learning.pending_experiences.clone();
        if pending.is_empty() {
            return Ok(());
        }
        let max_replay = self.budget.max_replay_count as usize;
        let candidates = crate::learning::replay::select_replay_candidates(&pending, max_replay);
        for experience in candidates {
            let signal = self.learning.record(experience.clone())?;
            self.learning.apply_signal(&signal)?;
            self.state.learning.total_replay_events += 1;
        }
        self.state.learning.pending_experiences.clear();
        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        self.persistence.save(&self.state)
    }

    pub fn checkpoint(&mut self) -> Result<CheckpointId> {
        let id = self.persistence.checkpoint(&self.state)?;
        self.state.metadata.checkpoint_count += 1;
        Ok(id)
    }

    pub fn observe(&mut self, text: &str) -> Result<String> {
        let observation = Observation::user_provided(text);
        let ctx = ContextState::initial();

        let language_state = self.language.encode(text, &ctx)?;
        let neural_repr = self.neural.process(&language_state, &ctx)?;

        let episode = Episode {
            id: self.memory.state().episodic.next_id,
            observation: observation.clone(),
            context: ctx,
            action: None,
            outcome: None,
            timestamp: Timestamp::now(),
            prediction: neural_repr.prediction.clone(),
            prediction_error: PredictionError::zero(),
            confidence: ConfidenceState {
                belief: 0.5,
                evidence_strength: 0.3,
                source_quality: 0.8,
                consistency: 0.5,
                uncertainty: 0.5,
                prediction_reliability: 0.0,
                verification_status: VerificationStatus::Observed,
            },
            source: observation.source.clone(),
            importance: observation.importance,
            retrieval_count: 0,
            last_retrieved: None,
            consolidated: false,
        };
        self.memory.store_episode(episode)?;
        self.state.metadata.episode_count += 1;

        let experience = Experience {
            observation,
            internal_state: StateSnapshot {
                language_vocabulary_size: self.language.vocabulary_size(),
                neural_active_cells: neural_repr.active_cells.len(),
                memory_episode_count: self.state.metadata.episode_count as usize,
                world_entity_count: 0,
                reasoning_hypothesis_count: 0,
                timestamp: Timestamp::now(),
            },
            prediction: neural_repr.prediction.unwrap_or(Prediction {
                target: PredictionTarget::NextState,
                predicted_state: Vec::new(),
                confidence: 0.5,
                timestamp: Timestamp::now(),
                context: ContextState::initial(),
                resolved: false,
                actual: None,
                error: None,
            }),
            action: None,
            outcome: None,
            error: PredictionError::zero(),
            attribution: ErrorAttribution::InputError,
            evidence: EvidenceSet::new(),
            provenance: Provenance::system("cortex"),
        };
        self.state.learning.pending_experiences.push(experience);

        Ok(format!("Observation stored. Episode count: {}", self.state.metadata.episode_count))
    }

    pub fn learn(&mut self, text: &str) -> Result<String> {
        if !self.config.learning.enabled {
            return Err(crate::error::CortexError::LearningError("Learning is disabled".into()));
        }
        if !self.config.policy.learning {
            return Err(crate::error::CortexError::PolicyError("Learning is not permitted by policy".into()));
        }

        let observation = Observation::user_provided(text);
        let ctx = ContextState::initial();
        let language_state = self.language.encode(text, &ctx)?;
        let neural_repr = self.neural.process(&language_state, &ctx)?;

        let experience = Experience {
            observation: observation.clone(),
            internal_state: StateSnapshot {
                language_vocabulary_size: self.language.vocabulary_size(),
                neural_active_cells: neural_repr.active_cells.len(),
                memory_episode_count: self.state.metadata.episode_count as usize,
                world_entity_count: 0,
                reasoning_hypothesis_count: 0,
                timestamp: Timestamp::now(),
            },
            prediction: neural_repr.prediction.unwrap_or(Prediction {
                target: PredictionTarget::NextState,
                predicted_state: neural_repr.field_activations.clone(),
                confidence: 0.5,
                timestamp: Timestamp::now(),
                context: ctx,
                resolved: false,
                actual: None,
                error: None,
            }),
            action: None,
            outcome: None,
            error: PredictionError::zero(),
            attribution: ErrorAttribution::InputError,
            evidence: EvidenceSet::new(),
            provenance: Provenance::derived(&[observation.source.clone()]),
        };

        let signal = self.learning.record(experience)?;
        self.learning.apply_signal(&signal)?;
        self.state.learning.total_learning_events += 1;

        let episode = Episode {
            id: self.memory.state().episodic.next_id,
            observation: observation.clone(),
            context: ContextState::initial(),
            action: None,
            outcome: None,
            timestamp: Timestamp::now(),
            prediction: None,
            prediction_error: PredictionError::zero(),
            confidence: ConfidenceState::default(),
            source: observation.source.clone(),
            importance: observation.importance,
            retrieval_count: 0,
            last_retrieved: None,
            consolidated: false,
        };
        self.memory.store_episode(episode)?;
        self.state.metadata.episode_count += 1;

        Ok(format!(
            "Learning applied. Signal magnitude: {:.4}, Events: {}",
            signal.magnitude,
            self.state.learning.total_learning_events
        ))
    }

    pub fn query(&self, text: &str) -> Result<MemoryRetrieval> {
        let query = MemoryQuery {
            query_type: MemoryQueryType::All,
            text: text.to_string(),
            concept_ids: Vec::new(),
            time_range: None,
            max_results: 10,
            min_confidence: 0.0,
        };
        let ctx = ContextState::initial();
        self.memory.retrieve(&query, &ctx)
    }

    pub fn inspect(&self) -> Result<serde_json::Value> {
        let health = self.runtime.health_check();
        Ok(serde_json::json!({
            "runtime": {
                "state": health.state.name(),
                "healthy": health.healthy,
                "uptime_secs": health.uptime_secs,
                "error_count": health.error_count,
                "recoverable_errors": health.recoverable_errors,
                "critical_errors": health.critical_errors,
            },
            "language": {
                "vocabulary_size": self.language.vocabulary_size(),
                "symbols_count": self.language.state().symbols.len(),
            },
            "neural": {
                "active_cells": self.neural.state().active_cells.len(),
                "active_columns": self.neural.state().active_columns.len(),
                "fields_count": self.neural.state().fields.len(),
                "has_prediction": self.neural.state().prediction.is_some(),
            },
            "memory": {
                "episodes": self.memory.state().episodic.episodes.len(),
                "knowledge": self.memory.state().semantic.knowledge.len(),
                "procedures": self.memory.state().procedural.procedures.len(),
                "associations": self.memory.state().associative.associations.len(),
                "working_memory_concepts": self.memory.working_memory().active_concepts.len(),
            },
            "world": {
                "entities": self.world.current_state().entities.len(),
                "relations": self.world.current_state().relations.len(),
                "events": self.world.current_state().active_events.len(),
            },
            "reasoning": {
                "hypotheses": self.reasoning.state().active_hypotheses.len(),
                "budget_remaining": self.reasoning.state().budget_remaining,
                "contradictions": self.reasoning.state().contradiction_log.len(),
            },
            "planning": {
                "goals": self.planning.state().active_goals.len(),
                "plans": self.planning.state().candidate_plans.len(),
                "budget_remaining": self.planning.state().budget_remaining,
            },
            "verification": {
                "pending": self.verification.state().pending_claims.len(),
                "verified": self.verification.state().verified_claims,
                "contradicted": self.verification.state().contradicted_claims,
                "threshold": self.verification.state().confidence_threshold,
            },
            "learning": {
                "enabled": self.learning.state().enabled,
                "total_events": self.learning.state().total_learning_events,
                "replay_events": self.learning.state().total_replay_events,
                "consolidation_events": self.learning.state().total_consolidation_events,
                "avg_prediction_error": self.learning.state().average_prediction_error,
                "pending_experiences": self.learning.state().pending_experiences.len(),
            },
            "self_model": {
                "prediction_accuracy": self.self_model.estimate().prediction_accuracy,
                "uncertainty_level": self.self_model.estimate().uncertainty_level,
                "memory_pressure": self.self_model.estimate().memory_health.pressure,
            },
            "policy": {
                "learning_allowed": self.config.policy.learning,
                "self_modification_allowed": self.config.policy.self_modification,
            },
            "metadata": {
                "episode_count": self.state.metadata.episode_count,
                "learning_events": self.state.metadata.total_learning_events,
                "checkpoint_count": self.state.metadata.checkpoint_count,
                "architecture_version": self.state.metadata.architecture_version,
            }
        }))
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
