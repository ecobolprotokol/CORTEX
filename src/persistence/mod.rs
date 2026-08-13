pub mod checkpoint;
pub mod format;
pub mod migration;

pub use checkpoint::CheckpointManager;
pub use format::FormatHandler;
pub use migration::MigrationHandler;

use crate::error::CortexError;

pub struct PersistenceManager {
    pub format: FormatHandler,
    pub checkpoint_manager: CheckpointManager,
    pub migration_handler: MigrationHandler,
}

impl PersistenceManager {
    pub fn new(max_checkpoints: usize) -> Self {
        Self {
            format: FormatHandler::new(),
            checkpoint_manager: CheckpointManager::new(max_checkpoints),
            migration_handler: MigrationHandler::new(),
        }
    }

    pub fn save_state(&self, data: &[u8]) -> Result<Vec<u8>, CortexError> {
        self.format.serialize(data)
    }

    pub fn load_state(&self, data: &[u8]) -> Result<Vec<u8>, CortexError> {
        self.format.deserialize(data)
    }

    pub fn verify_integrity(&self, data: &[u8], expected_checksum: &[u8; 32]) -> bool {
        let actual = FormatHandler::compute_checksum(data);
        actual == *expected_checksum
    }
}
