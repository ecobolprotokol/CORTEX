use crate::types::ids::CheckpointId;
use crate::types::common::Timestamp;

#[derive(Debug, Clone)]
pub struct Checkpoint {
    pub id: CheckpointId,
    pub timestamp: Timestamp,
    pub state_size: u64,
    pub episode_count: u64,
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
        let cp = Checkpoint {
            id: CheckpointId::from(self.next_id),
            timestamp: Timestamp::now(),
            state_size,
            episode_count,
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
}
