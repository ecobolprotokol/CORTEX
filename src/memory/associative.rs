use crate::error::Result;
use crate::types::*;

pub fn store(memory: &mut AssociativeMemory, association: Association) -> Result<()> {
    memory.current_usage_bytes += estimate_size(&association);
    memory.associations.push(association);
    Ok(())
}

fn estimate_size(association: &Association) -> u64 {
    128
}
