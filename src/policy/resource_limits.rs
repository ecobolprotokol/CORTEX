use crate::error::CortexError;

#[derive(Debug, Clone)]
pub struct ResourceLimits {
    pub max_memory_bytes: usize,
    pub max_episodes: usize,
    pub max_knowledge: usize,
    pub max_entities: usize,
    pub max_hypotheses: usize,
    pub max_associations: usize,
    pub max_reasoning_steps: u32,
    pub max_planning_depth: u32,
    pub max_operations_per_minute: u32,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_bytes: 256 * 1024 * 1024,
            max_episodes: 10_000,
            max_knowledge: 50_000,
            max_entities: 10_000,
            max_hypotheses: 1_000,
            max_associations: 50_000,
            max_reasoning_steps: 32,
            max_planning_depth: 8,
            max_operations_per_minute: 120,
        }
    }
}

impl ResourceLimits {
    pub fn check_memory_usage(&self, current_bytes: usize) -> Result<(), CortexError> {
        if current_bytes >= self.max_memory_bytes {
            Err(CortexError::ResourceError(format!(
                "Memory usage {} exceeds limit {}",
                current_bytes, self.max_memory_bytes
            )))
        } else {
            Ok(())
        }
    }

    pub fn check_episode_count(&self, count: usize) -> Result<(), CortexError> {
        if count >= self.max_episodes {
            Err(CortexError::ResourceError(format!(
                "Episode count {} exceeds limit {}",
                count, self.max_episodes
            )))
        } else {
            Ok(())
        }
    }

    pub fn check_entity_count(&self, count: usize) -> Result<(), CortexError> {
        if count >= self.max_entities {
            Err(CortexError::ResourceError(format!(
                "Entity count {} exceeds limit {}",
                count, self.max_entities
            )))
        } else {
            Ok(())
        }
    }

    pub fn check_operation_rate(&self, operations_in_minute: u32) -> Result<(), CortexError> {
        if operations_in_minute >= self.max_operations_per_minute {
            Err(CortexError::ResourceError(format!(
                "Operation rate {} exceeds limit {}",
                operations_in_minute, self.max_operations_per_minute
            )))
        } else {
            Ok(())
        }
    }
}
