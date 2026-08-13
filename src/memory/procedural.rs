use crate::error::Result;
use crate::types::*;

pub fn store(memory: &mut ProceduralMemory, procedure: Procedure) -> Result<()> {
    memory.current_usage_bytes += estimate_size(&procedure);
    memory.procedures.push(procedure);
    Ok(())
}

fn estimate_size(procedure: &Procedure) -> u64 {
    (procedure.steps.len() * 128 + 256) as u64
}
