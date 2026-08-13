use crate::error::Result;
use crate::types::*;

pub fn migrate(state: &mut CortexState, from_version: u32, to_version: u32) -> Result<()> {
    let mut current = from_version;
    while current < to_version {
        current += 1;
        migrate_step(state, current)?;
    }
    state.metadata.architecture_version = to_version;
    Ok(())
}

fn migrate_step(state: &mut CortexState, target_version: u32) -> Result<()> {
    match target_version {
        1 => {}
        _ => {}
    }
    Ok(())
}
