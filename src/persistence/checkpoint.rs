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

pub fn list_checkpoints(config: &PersistenceConfig) -> Vec<String> {
    let checkpoints_dir = std::path::Path::new(&config.state)
        .parent()
        .unwrap_or(std::path::Path::new("."))
        .join("checkpoints");
    if !checkpoints_dir.exists() {
        return Vec::new();
    }
    let mut paths: Vec<String> = match std::fs::read_dir(&checkpoints_dir) {
        Ok(rd) => rd
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.path().extension().map_or(false, |ext| ext == "cx")
            })
            .map(|entry| entry.path().to_string_lossy().to_string())
            .collect(),
        Err(_) => Vec::new(),
    };
    paths.sort();
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_checkpoints_empty() {
        let config = PersistenceConfig {
            state: "/tmp/nonexistent_cortex_test/cortex.cx".into(),
            checkpoint_interval: 1000,
        };
        let paths = list_checkpoints(&config);
        assert!(paths.is_empty());
    }
}
