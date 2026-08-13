pub mod working;
pub mod episodic;
pub mod semantic;
pub mod procedural;
pub mod associative;
pub mod retrieval;
pub mod consolidation;

use crate::error::CortexError;

pub trait MemorySystem {
    fn store_episode(&mut self, episode: episodic::Episode) -> Result<(), CortexError>;
    fn retrieve(&self, query: &str, max_results: usize) -> Vec<Box<dyn std::any::Any>>;
    fn consolidation_interval(&self) -> u64;
}
