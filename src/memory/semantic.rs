use crate::error::Result;
use crate::types::*;

pub fn store(memory: &mut SemanticMemory, knowledge: Knowledge) -> Result<()> {
    if memory.current_usage_bytes >= memory.capacity_bytes {
        evict_lowest(memory);
    }
    memory.current_usage_bytes += estimate_size(&knowledge);
    memory.knowledge.push(knowledge);
    Ok(())
}

fn evict_lowest(memory: &mut SemanticMemory) {
    if memory.knowledge.is_empty() {
        return;
    }
    let mut min_idx = 0;
    let mut min_conf = f32::MAX;
    for (i, k) in memory.knowledge.iter().enumerate() {
        let conf = k.confidence.overall();
        if conf < min_conf {
            min_conf = conf;
            min_idx = i;
        }
    }
    memory.knowledge.remove(min_idx);
}

fn estimate_size(knowledge: &Knowledge) -> u64 {
    (knowledge.properties.len() * 64 + knowledge.relations.len() * 64 + 128) as u64
}
