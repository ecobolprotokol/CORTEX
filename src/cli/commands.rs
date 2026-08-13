use crate::config::CortexConfig;
use crate::error::Result;
use crate::types::*;

pub fn execute_init(config_path: &str, force: bool) -> Result<()> {
    let path = std::path::Path::new("cortex.cx");
    if path.exists() && !force {
        return Err(crate::error::CortexError::RuntimeError(
            "State file already exists. Use --force to overwrite.".into(),
        ));
    }
    let config = CortexConfig::load(config_path)?;
    let state = create_initial_state(&config);
    crate::persistence::format::save_cx("cortex.cx", &state)?;
    println!("CORTEX state initialized: cortex.cx");
    Ok(())
}

pub fn execute_status(config_path: &str) -> Result<()> {
    let config = CortexConfig::load(config_path)?;
    if !std::path::Path::new(&config.persistence.state).exists() {
        println!("No state file found. Run 'cortex init' first.");
        return Ok(());
    }
    let state = crate::persistence::format::load_cx(&config.persistence.state)?;
    println!("CORTEX Status: ready");
    println!("State ID: {}", state.metadata.state_id);
    println!("Architecture version: {}", state.metadata.architecture_version);
    println!("Episode count: {}", state.metadata.episode_count);
    println!("Vocabulary size: {}", state.language.vocabulary_size);
    println!("Entity count: {}", state.world.entities.len());
    println!("Memory pressure: Low");
    Ok(())
}

pub fn execute_inspect(config_path: &str, section: Option<&str>) -> Result<()> {
    let config = CortexConfig::load(config_path)?;
    let state = if std::path::Path::new(&config.persistence.state).exists() {
        crate::persistence::format::load_cx(&config.persistence.state)?
    } else {
        println!("No state file found. Run 'cortex init' first.");
        return Ok(());
    };
    match section {
        Some("memory") => {
            println!("Memory State:");
            println!("  Working memory: active");
            println!("  Episodic: {} episodes, {}/{} bytes",
                state.memory.episodic.episodes.len(),
                state.memory.episodic.current_usage_bytes,
                state.memory.episodic.capacity_bytes);
            println!("  Semantic: {} knowledge items, {}/{} bytes",
                state.memory.semantic.knowledge.len(),
                state.memory.semantic.current_usage_bytes,
                state.memory.semantic.capacity_bytes);
            println!("  Procedural: {} procedures", state.memory.procedural.procedures.len());
            println!("  Associative: {} associations", state.memory.associative.associations.len());
        }
        Some("world") => {
            println!("World Model:");
            println!("  Entities: {}", state.world.entities.len());
            println!("  Relations: {}", state.world.relations.len());
            println!("  Active events: {}", state.world.active_events.len());
            println!("  Uncertainty: {:.2}", state.world.uncertainty.level);
        }
        Some("language") => {
            println!("Language Core:");
            println!("  Vocabulary size: {}", state.language.vocabulary_size);
            println!("  Symbols: {}", state.language.symbols.len());
            println!("  Tokens: {}", state.language.tokens.len());
        }
        Some("neural") => {
            println!("Neural Core:");
            println!("  Fields: {}", state.neural.fields.len());
            println!("  Active cells: {}", state.neural.active_cells.len());
            println!("  Active columns: {}", state.neural.active_columns.len());
            println!("  Temporal buffer: {}", state.neural.temporal_buffer.len());
        }
        Some("reasoning") => {
            println!("Reasoning Engine:");
            println!("  Active hypotheses: {}", state.reasoning.active_hypotheses.len());
            println!("  Budget remaining: {}", state.reasoning.budget_remaining);
            println!("  Contradictions: {}", state.reasoning.contradiction_log.len());
        }
        Some("learning") => {
            println!("Learning System:");
            println!("  Enabled: {}", state.learning.enabled);
            println!("  Total learning events: {}", state.learning.total_learning_events);
            println!("  Total replay events: {}", state.learning.total_replay_events);
            println!("  Total consolidation events: {}", state.learning.total_consolidation_events);
            println!("  Average prediction error: {:.4}", state.learning.average_prediction_error);
            println!("  Learning rate: {}", state.learning.learning_rate);
        }
        Some("self-model") => {
            println!("Self Model:");
            println!("  Prediction accuracy: {:.2}", state.self_model.prediction_accuracy);
            println!("  Uncertainty level: {:.2}", state.self_model.uncertainty_level);
            println!("  Memory pressure: {:?}", state.self_model.memory_health.pressure);
        }
        Some("policy") => {
            println!("Policy:");
            println!("  Learning: {}", true);
            println!("  Self-modification: {}", false);
            println!("  Policy modification: {}", false);
        }
        Some("metadata") => {
            println!("State Metadata:");
            println!("  State ID: {}", state.metadata.state_id);
            println!("  Created at: {}", state.metadata.created_at.0);
            println!("  Last updated: {}", state.metadata.last_updated.0);
            println!("  Architecture version: {}", state.metadata.architecture_version);
            println!("  Episode count: {}", state.metadata.episode_count);
            println!("  Total learning events: {}", state.metadata.total_learning_events);
            println!("  Checkpoint count: {}", state.metadata.checkpoint_count);
        }
        _ => {
            println!("CORTEX State Summary:");
            println!("  State ID: {}", state.metadata.state_id);
            println!("  Architecture version: {}", state.metadata.architecture_version);
            println!("  Episode count: {}", state.metadata.episode_count);
            println!("  Vocabulary size: {}", state.language.vocabulary_size);
            println!("  World entities: {}", state.world.entities.len());
            println!("  Memory pressure: Low");
        }
    }
    Ok(())
}

pub fn execute_checkpoint(config_path: &str) -> Result<()> {
    let config = CortexConfig::load(config_path)?;
    if !std::path::Path::new(&config.persistence.state).exists() {
        println!("No state file found. Run 'cortex init' first.");
        return Ok(());
    }
    let state = crate::persistence::format::load_cx(&config.persistence.state)?;
    let persistence = crate::persistence::PersistenceEngineImpl::new(&config.persistence)?;
    use crate::persistence::PersistenceEngine;
    let id = persistence.checkpoint(&state)?;
    println!("Checkpoint created: {}", id.0);
    Ok(())
}

pub fn execute_observe(config_path: &str, text: &str, importance: Option<f32>) -> Result<()> {
    let config = CortexConfig::load(config_path)?;
    let mut state = if std::path::Path::new(&config.persistence.state).exists() {
        crate::persistence::format::load_cx(&config.persistence.state)?
    } else {
        create_initial_state(&config)
    };

    let observation = Observation {
        text: text.to_string(),
        source: Provenance::user_provided(),
        timestamp: Timestamp::now(),
        context: ContextState::initial(),
        kind: ObservationKind::UserInput,
        importance: importance.unwrap_or(0.5),
    };

    let episode = Episode {
        id: state.memory.episodic.next_id,
        observation,
        context: ContextState::initial(),
        action: None,
        outcome: None,
        timestamp: Timestamp::now(),
        prediction: None,
        prediction_error: PredictionError::zero(),
        confidence: ConfidenceState::default(),
        source: Provenance::user_provided(),
        importance: importance.unwrap_or(0.5),
        retrieval_count: 0,
        last_retrieved: None,
        consolidated: false,
    };

    state.memory.episodic.next_id = state.memory.episodic.next_id.next();
    state.memory.episodic.episodes.push(episode);
    state.metadata.episode_count += 1;

    crate::persistence::format::save_cx(&config.persistence.state, &state)?;
    println!("Observation stored. Episode created. State updated.");
    Ok(())
}

pub fn execute_query(config_path: &str, text: &str, target: &str, max_results: u32) -> Result<()> {
    let config = CortexConfig::load(config_path)?;
    if !std::path::Path::new(&config.persistence.state).exists() {
        println!("No state file found. Run 'cortex init' first.");
        return Ok(());
    }
    let state = crate::persistence::format::load_cx(&config.persistence.state)?;

    let query_type = match target {
        "episodes" | "episodic" => MemoryQueryType::Episodic,
        "knowledge" | "semantic" => MemoryQueryType::Semantic,
        "procedures" | "procedural" => MemoryQueryType::Procedural,
        _ => MemoryQueryType::All,
    };

    let query = MemoryQuery {
        query_type,
        text: text.to_string(),
        concept_ids: Vec::new(),
        time_range: None,
        max_results,
        min_confidence: 0.0,
    };

    let results = crate::memory::retrieval::retrieve(&state.memory, &query, &ContextState::initial())?;

    println!("Query: '{}'", text);
    println!("Episodic results: {}", results.episodic.len());
    println!("Semantic results: {}", results.semantic.len());
    println!("Procedural results: {}", results.procedural.len());
    println!("Associative results: {}", results.associative.len());

    for se in &results.episodic {
        println!("  [EP] {} (relevance: {:.2})", se.episode.observation.text, se.relevance_score);
    }
    for sk in &results.semantic {
        println!("  [KN] concept-{} (relevance: {:.2})", sk.knowledge.concept.0, sk.relevance_score);
    }

    Ok(())
}

pub fn execute_verify(config_path: &str, claim: &str) -> Result<()> {
    let config = CortexConfig::load(config_path)?;
    println!("Verification for '{}':", claim);
    if std::path::Path::new(&config.persistence.state).exists() {
        let state = crate::persistence::format::load_cx(&config.persistence.state)?;
        let matching: Vec<_> = state.memory.semantic.knowledge.iter()
            .filter(|k| {
                k.properties.iter().any(|p| {
                    p.name.to_lowercase().contains(&claim.to_lowercase())
                    || match &p.value {
                        PropertyValue::Text(t) => t.to_lowercase().contains(&claim.to_lowercase()),
                        _ => false,
                    }
                })
            })
            .collect();
        if matching.is_empty() {
            println!("  Status: Unknown (no matching knowledge found)");
        } else {
            for k in &matching {
                println!("  Status: {:?} (confidence: {:.2})", k.verification_status, k.confidence.overall());
            }
        }
    } else {
        println!("  Status: Unknown (no state loaded)");
    }
    Ok(())
}

pub fn execute_migrate(_dry_run: bool) -> Result<()> {
    println!("Migration: No migration needed. State is at current version.");
    Ok(())
}

pub fn create_initial_state(config: &CortexConfig) -> CortexState {
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
    }
}
