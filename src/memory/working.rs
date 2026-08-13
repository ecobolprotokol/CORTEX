use crate::types::*;

pub fn update_context(working: &mut WorkingMemory, text: &str) {
    working.conversation_context.turn_count += 1;
    working.conversation_context.recent_inputs.push(text.to_string());
    if working.conversation_context.recent_inputs.len() > 10 {
        working.conversation_context.recent_inputs.remove(0);
    }
}
