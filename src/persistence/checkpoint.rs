use crate::error::CortexError;
use crate::types::common::Timestamp;
use crate::types::ids::CheckpointId;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Checkpoint {
    pub id: CheckpointId,
    pub timestamp: Timestamp,
    pub state_size: u64,
    pub episode_count: u64,
    pub checksum: [u8; 32],
    pub description: String,
}

pub struct CheckpointManager {
    pub checkpoints: Vec<Checkpoint>,
    pub max_checkpoints: usize,
    pub next_id: u64,
    checkpoint_dir: String,
}

impl CheckpointManager {
    pub fn new(max_checkpoints: usize) -> Self {
        Self {
            checkpoints: Vec::new(),
            max_checkpoints,
            next_id: 1,
            checkpoint_dir: String::new(),
        }
    }

    pub fn with_dir(max_checkpoints: usize, checkpoint_dir: &str) -> Self {
        Self {
            checkpoints: Vec::new(),
            max_checkpoints,
            next_id: 1,
            checkpoint_dir: checkpoint_dir.to_string(),
        }
    }

    pub fn create_checkpoint(&mut self, state_size: u64, episode_count: u64) -> Checkpoint {
        let data = format!("checkpoint_{}", self.next_id);
        let checksum = crate::persistence::format::FormatHandler::compute_checksum(data.as_bytes());

        let cp = Checkpoint {
            id: CheckpointId::from(self.next_id),
            timestamp: Timestamp::now(),
            state_size,
            episode_count,
            checksum,
            description: format!("Checkpoint #{}", self.next_id),
        };
        self.next_id += 1;

        if self.checkpoints.len() >= self.max_checkpoints {
            self.checkpoints.remove(0);
        }

        self.checkpoints.push(cp.clone());
        cp
    }

    pub fn create_checkpoint_with_data(
        &mut self,
        state_data: &[u8],
        episode_count: u64,
    ) -> Checkpoint {
        let checksum = crate::persistence::format::FormatHandler::compute_checksum(state_data);

        let cp = Checkpoint {
            id: CheckpointId::from(self.next_id),
            timestamp: Timestamp::now(),
            state_size: state_data.len() as u64,
            episode_count,
            checksum,
            description: format!("Checkpoint #{}", self.next_id),
        };
        self.next_id += 1;

        if self.checkpoints.len() >= self.max_checkpoints {
            self.checkpoints.remove(0);
        }

        self.checkpoints.push(cp.clone());
        cp
    }

    pub fn save_checkpoint_to_disk(
        &self,
        checkpoint: &Checkpoint,
        state_data: &[u8],
    ) -> Result<(), CortexError> {
        if self.checkpoint_dir.is_empty() {
            return Ok(());
        }
        let dir = std::path::Path::new(&self.checkpoint_dir);
        if !dir.exists() {
            std::fs::create_dir_all(dir).map_err(|e| {
                CortexError::PersistenceError(format!("Failed to create checkpoint dir: {}", e))
            })?;
        }
        let path = dir.join(format!("checkpoint_{}.cx", checkpoint.id));
        let handler = crate::persistence::format::FormatHandler::new();
        handler.save_to_file(path.to_str().unwrap_or(""), state_data)?;
        tracing::debug!(checkpoint_id = ?checkpoint.id, "Checkpoint saved to disk");
        Ok(())
    }

    pub fn load_checkpoint_from_disk(&self, id: CheckpointId) -> Result<Vec<u8>, CortexError> {
        if self.checkpoint_dir.is_empty() {
            return Err(CortexError::PersistenceError(
                "No checkpoint directory configured".into(),
            ));
        }
        let path = std::path::Path::new(&self.checkpoint_dir).join(format!("checkpoint_{}.cx", id));
        if !path.exists() {
            return Err(CortexError::PersistenceError(format!(
                "Checkpoint file not found: {}",
                path.display()
            )));
        }
        let handler = crate::persistence::format::FormatHandler::new();
        handler.load_from_file(path.to_str().unwrap_or(""))
    }

    pub fn latest(&self) -> Option<&Checkpoint> {
        self.checkpoints.last()
    }

    pub fn find_by_id(&self, id: CheckpointId) -> Option<&Checkpoint> {
        self.checkpoints.iter().find(|c| c.id == id)
    }

    pub fn list_checkpoints(&self) -> &[Checkpoint] {
        &self.checkpoints
    }

    pub fn remove_checkpoint(&mut self, id: CheckpointId) -> bool {
        let pos = self.checkpoints.iter().position(|c| c.id == id);
        if let Some(pos) = pos {
            self.checkpoints.remove(pos);
            if !self.checkpoint_dir.is_empty() {
                let path = std::path::Path::new(&self.checkpoint_dir)
                    .join(format!("checkpoint_{}.cx", id));
                let _ = std::fs::remove_file(path);
            }
            true
        } else {
            false
        }
    }

    pub fn latest_episode_count(&self) -> u64 {
        self.checkpoints
            .last()
            .map(|c| c.episode_count)
            .unwrap_or(0)
    }

    pub fn total_state_size(&self) -> u64 {
        self.checkpoints.iter().map(|c| c.state_size).sum()
    }

    pub fn checkpoint_count(&self) -> usize {
        self.checkpoints.len()
    }

    pub fn prune_old(&mut self, keep_count: usize) {
        while self.checkpoints.len() > keep_count {
            let removed = self.checkpoints.remove(0);
            if !self.checkpoint_dir.is_empty() {
                let path = std::path::Path::new(&self.checkpoint_dir)
                    .join(format!("checkpoint_{}.cx", removed.id));
                let _ = std::fs::remove_file(path);
            }
        }
    }

    pub fn find_latest_valid_checkpoint(&self) -> Option<&Checkpoint> {
        self.checkpoints.iter().rev().find(|cp| {
            if self.checkpoint_dir.is_empty() {
                return true;
            }
            let path = std::path::Path::new(&self.checkpoint_dir)
                .join(format!("checkpoint_{}.cx", cp.id));
            path.exists()
        })
    }
}

impl Default for CheckpointManager {
    fn default() -> Self {
        Self::new(10)
    }
}
