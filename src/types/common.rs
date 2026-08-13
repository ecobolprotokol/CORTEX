use super::ids::*;
use super::scalars::Scalar;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Timestamp(pub u64);

impl Timestamp {
    pub const ZERO: Self = Self(0);

    pub fn now() -> Self {
        Self(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        )
    }

    pub fn from_secs(secs: u64) -> Self {
        Self(secs * 1000)
    }

    pub fn as_secs(&self) -> u64 {
        self.0 / 1000
    }

    pub fn as_millis(&self) -> u64 {
        self.0
    }

    pub fn elapsed_since(&self, earlier: Timestamp) -> Duration {
        Duration(self.0.saturating_sub(earlier.0))
    }

    pub fn is_before(&self, other: Timestamp) -> bool {
        self.0 < other.0
    }

    pub fn is_after(&self, other: Timestamp) -> bool {
        self.0 > other.0
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Duration(pub u64);

impl Duration {
    pub const ZERO: Self = Self(0);
    pub fn from_secs(secs: u64) -> Self {
        Self(secs * 1000)
    }
    pub fn from_millis(ms: u64) -> Self {
        Self(ms)
    }
    pub fn as_secs(&self) -> u64 {
        self.0 / 1000
    }
    pub fn as_millis(&self) -> u64 {
        self.0
    }
}

impl Default for Duration {
    fn default() -> Self {
        Self::ZERO
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalContext {
    pub current_time: Timestamp,
    pub sequence_position: u64,
    pub prior_states: Vec<Timestamp>,
    pub temporal_horizon: Duration,
}

impl Default for TemporalContext {
    fn default() -> Self {
        Self {
            current_time: Timestamp::now(),
            sequence_position: 0,
            prior_states: Vec::new(),
            temporal_horizon: Duration::from_secs(3600),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextState {
    pub conversation_id: Option<u64>,
    pub episode_context: Vec<EpisodeId>,
    pub active_concepts: Vec<ConceptId>,
    pub world_assumptions: Vec<EntityId>,
    pub temporal_context: TemporalContext,
    pub active_intents: Vec<IntentHypothesis>,
    pub window_position: u32,
    pub tokens_used: u32,
}

impl Default for ContextState {
    fn default() -> Self {
        Self::initial()
    }
}

impl ContextState {
    pub fn initial() -> Self {
        Self {
            conversation_id: None,
            episode_context: Vec::new(),
            active_concepts: Vec::new(),
            world_assumptions: Vec::new(),
            temporal_context: TemporalContext::default(),
            active_intents: Vec::new(),
            window_position: 0,
            tokens_used: 0,
        }
    }

    pub fn advance_time(&mut self) {
        self.temporal_context.current_time = Timestamp::now();
        self.temporal_context.sequence_position += 1;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentHypothesis {
    pub intent: Intent,
    pub confidence: Scalar,
    pub alternatives: Vec<IntentHypothesis>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Intent {
    Question,
    Statement,
    Instruction,
    Correction,
    Conversation,
    Exclamation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeBudget {
    pub max_reasoning_steps: u32,
    pub max_planning_depth: u32,
    pub max_planning_branches: u32,
    pub max_simulation_steps: u32,
    pub max_generation_length: u32,
    pub max_memory_retrieval: u32,
    pub max_replay_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub score: Scalar,
    pub level: RiskLevel,
    pub factors: Vec<RiskFactor>,
    pub reversibility: Scalar,
}

impl Default for RiskAssessment {
    fn default() -> Self {
        Self {
            score: 0.0,
            level: RiskLevel::None,
            factors: Vec::new(),
            reversibility: 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    None,
    Low,
    Moderate,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    pub description: String,
    pub severity: Scalar,
    pub likelihood: Scalar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandidateContinuation {
    pub token: SymbolId,
    pub score: Scalar,
}
