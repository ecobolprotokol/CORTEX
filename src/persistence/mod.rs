pub mod format;
pub mod checkpoint;
pub mod migration;

use crate::error::CortexError;

pub trait PersistenceEngine {
    fn save(&self, state: &[u8]) -> Result<(), CortexError>;
    fn load(&self) -> Result<Vec<u8>, CortexError>;
    fn checkpoint(&self) -> Result<u64, CortexError>;
}
