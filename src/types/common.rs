use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::types::ids::SessionId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Timestamp(u64);

impl Timestamp {
    pub fn now() -> Self {
        let d = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock before UNIX epoch");
        Self(d.as_millis() as u64)
    }

    pub fn from_secs(secs: u64) -> Self {
        Self(secs * 1000)
    }

    pub fn as_millis(self) -> u64 {
        self.0
    }

    pub fn elapsed_since(self, earlier: Timestamp) -> Duration {
        Duration::from_millis(self.0.saturating_sub(earlier.0))
    }

    pub fn is_before(self, other: Timestamp) -> bool {
        self.0 < other.0
    }

    pub fn is_after(self, other: Timestamp) -> bool {
        self.0 > other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Duration(u64);

impl Default for Duration {
    fn default() -> Self {
        Self(0)
    }
}

impl Duration {
    pub fn from_secs(secs: u64) -> Self {
        Self(secs * 1000)
    }

    pub fn from_millis(millis: u64) -> Self {
        Self(millis)
    }

    pub fn as_secs(self) -> u64 {
        self.0 / 1000
    }

    pub fn as_millis(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalContext {
    pub now: Timestamp,
    pub episode_start: Option<Timestamp>,
    pub last_action: Option<Timestamp>,
    pub deadline: Option<Timestamp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeBudget {
    pub max_tokens: u32,
    pub max_tool_calls: u32,
    pub max_duration: Duration,
    pub tokens_used: u32,
    pub tool_calls_used: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextState {
    pub session_id: SessionId,
    pub temporal: TemporalContext,
    pub compute: ComputeBudget,
    pub active: bool,
}
