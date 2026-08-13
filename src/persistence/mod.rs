pub mod format;
pub mod checkpoint;
pub mod migration;

use crate::config::PersistenceConfig;
use crate::error::Result;
use crate::types::*;

pub trait PersistenceEngine {
    fn save(&self, state: &CortexState) -> Result<()>;
    fn load(&self) -> Result<CortexState>;
    fn checkpoint(&self, state: &CortexState) -> Result<CheckpointId>;
    fn exists(&self) -> bool;
}

pub struct PersistenceEngineImpl {
    config: PersistenceConfig,
}

impl PersistenceEngineImpl {
    pub fn new(config: &PersistenceConfig) -> Result<Self> {
        Ok(Self { config: config.clone() })
    }
}

impl PersistenceEngine for PersistenceEngineImpl {
    fn save(&self, state: &CortexState) -> Result<()> {
        format::save_cx(&self.config.state, state)
    }

    fn load(&self) -> Result<CortexState> {
        format::load_cx(&self.config.state)
    }

    fn checkpoint(&self, state: &CortexState) -> Result<CheckpointId> {
        checkpoint::create(&self.config, state)
    }

    fn exists(&self) -> bool {
        std::path::Path::new(&self.config.state).exists()
    }
}
