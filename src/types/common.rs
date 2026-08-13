use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::types::ids::{ConceptId, EntityId, EpisodeId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Timestamp(pub u64);

impl Timestamp {
    pub const ZERO: Self = Self(0);

    pub fn now() -> Self {
        let d = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock before UNIX epoch");
        Self(d.as_millis() as u64)
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
pub struct ComputeBudget {
    pub max_reasoning_steps: u32,
    pub max_planning_depth: u32,
    pub max_planning_branches: u32,
    pub max_simulation_steps: u32,
    pub max_generation_length: u32,
}

impl Default for ComputeBudget {
    fn default() -> Self {
        Self {
            max_reasoning_steps: 100,
            max_planning_depth: 10,
            max_planning_branches: 5,
            max_simulation_steps: 20,
            max_generation_length: 2048,
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
}

impl Default for ContextState {
    fn default() -> Self {
        Self {
            conversation_id: None,
            episode_context: Vec::new(),
            active_concepts: Vec::new(),
            world_assumptions: Vec::new(),
            temporal_context: TemporalContext::default(),
        }
    }
}

impl ContextState {
    pub fn initial() -> Self {
        Self::default()
    }

    pub fn advance_time(&mut self) {
        self.temporal_context.current_time = Timestamp::now();
        self.temporal_context.sequence_position += 1;
    }
}
