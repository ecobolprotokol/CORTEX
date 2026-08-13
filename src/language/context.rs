use crate::types::*;

pub fn construct(working: &WorkingMemory, _context: &ContextState) -> ContextState {
    let mut ctx = ContextState::initial();
    ctx.conversation_id = Some(working.conversation_context.session_id.0);
    ctx.window_position = working.conversation_context.turn_count as u32;
    ctx.episode_context = working.conversation_context.recent_inputs.iter().enumerate().map(|_| EpisodeId(0)).collect();
    ctx.active_concepts = working.active_concepts.clone();
    ctx.world_assumptions = working.world_assumptions.clone();
    ctx
}
