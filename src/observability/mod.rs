pub mod diagnostics;

use crate::error::Result;
use crate::types::*;

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
