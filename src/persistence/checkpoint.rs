use crate::config::PersistenceConfig;
use crate::error::{CortexError, Result};
use crate::persistence::format;
use crate::types::*;

pub fn create(config: &PersistenceConfig, state: &CortexState) -> Result<CheckpointId> {
    let checkpoints_dir = std::path::Path::new(&config.state)
        .parent()
        .unwrap_or(std::path::Path::new("."))
        .join("checkpoints");
    std::fs::create_dir_all(&checkpoints_dir)
        .map_err(|e| CortexError::PersistenceError(format!("Failed to create checkpoints dir: {}", e)))?;
    let checkpoint_id = state.metadata.checkpoint_count + 1;
    let path = checkpoints_dir.join(format!("checkpoint_{:06}.cx", checkpoint_id));
    format::save_cx(path.to_str().unwrap_or("checkpoint.cx"), state)?;
    Ok(CheckpointId(checkpoint_id as u64))
}
