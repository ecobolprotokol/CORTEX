pub mod working;
pub mod episodic;
pub mod semantic;
pub mod procedural;
pub mod associative;
pub mod retrieval;
pub mod consolidation;

use crate::config::MemoryConfig;
use crate::error::Result;
use crate::types::*;

pub trait MemorySystem {
    fn retrieve(&self, query: &MemoryQuery, context: &ContextState) -> Result<MemoryRetrieval>;
    fn store_episode(&mut self, episode: Episode) -> Result<()>;
    fn store_knowledge(&mut self, knowledge: Knowledge) -> Result<()>;
    fn state(&self) -> &MemoryState;
    fn working_memory(&self) -> &WorkingMemory;
    fn working_memory_mut(&mut self) -> &mut WorkingMemory;
}

pub struct MemorySystemImpl {
    config: MemoryConfig,
    state: MemoryState,
}

impl MemorySystemImpl {
    pub fn new(config: &MemoryConfig) -> Result<Self> {
        Ok(Self {
            config: config.clone(),
            state: MemoryState {
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
                    capacity_bytes: config.episodic_mb as u64 * 1024 * 1024,
                    current_usage_bytes: 0,
                    next_id: EpisodeId(1),
                },
                semantic: SemanticMemory {
                    knowledge: Vec::new(),
                    capacity_bytes: config.semantic_mb as u64 * 1024 * 1024,
                    current_usage_bytes: 0,
                    next_id: KnowledgeId(1),
                },
                procedural: ProceduralMemory {
                    procedures: Vec::new(),
                    capacity_bytes: config.procedural_mb as u64 * 1024 * 1024,
                    current_usage_bytes: 0,
                    next_id: ProcedureId(1),
                },
                associative: AssociativeMemory {
                    associations: Vec::new(),
                    capacity_bytes: config.associative_mb as u64 * 1024 * 1024,
                    current_usage_bytes: 0,
                    next_id: AssociationId(1),
                },
            },
        })
    }
}

impl MemorySystem for MemorySystemImpl {
    fn retrieve(&self, query: &MemoryQuery, context: &ContextState) -> Result<MemoryRetrieval> {
        retrieval::retrieve(&self.state, query, context)
    }

    fn store_episode(&mut self, episode: Episode) -> Result<()> {
        episodic::store(&mut self.state.episodic, episode)
    }

    fn store_knowledge(&mut self, knowledge: Knowledge) -> Result<()> {
        semantic::store(&mut self.state.semantic, knowledge)
    }

    fn state(&self) -> &MemoryState {
        &self.state
    }

    fn working_memory(&self) -> &WorkingMemory {
        &self.state.working
    }

    fn working_memory_mut(&mut self) -> &mut WorkingMemory {
        &mut self.state.working
    }
}
