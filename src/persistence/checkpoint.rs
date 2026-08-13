use crate::types::common::Timestamp;
use crate::types::ids::CheckpointId;

#[derive(Debug, Clone)]
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
}

impl CheckpointManager {
    pub fn new(max_checkpoints: usize) -> Self {
        Self {
            checkpoints: Vec::new(),
            max_checkpoints,
            next_id: 1,
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
            self.checkpoints.remove(0);
        }
    }
}

impl Default for CheckpointManager {
    fn default() -> Self {
        Self::new(10)
    }
}
