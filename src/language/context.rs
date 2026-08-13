use crate::types::*;

pub fn construct(working: &WorkingMemory, _context: &ContextState) -> ContextState {
    let mut ctx = ContextState::initial();
    ctx.conversation_id = Some(working.conversation_context.session_id.0);
    ctx.window_position = working.conversation_context.turn_count as u32;
    ctx.active_concepts = working.active_concepts.clone();
    ctx.world_assumptions = working.world_assumptions.clone();
    ctx
}

pub fn advance_context(ctx: &mut ContextState) {
    ctx.advance_time();
    ctx.temporal_context.prior_states.push(ctx.temporal_context.current_time);
    if ctx.temporal_context.prior_states.len() > 10 {
        ctx.temporal_context.prior_states.remove(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_working() -> WorkingMemory {
        WorkingMemory {
            input: None,
            conversation_context: ConversationContext {
                session_id: SessionId(1),
                turn_count: 5,
                recent_inputs: vec!["hello".into()],
                recent_outputs: vec!["hi".into()],
                started_at: Timestamp::now(),
            },
            active_concepts: vec![ConceptId(1), ConceptId(2)],
            active_hypotheses: Vec::new(),
            goals: Vec::new(),
            reasoning_state: None,
            world_assumptions: vec![EntityId(1)],
            generation_state: None,
        }
    }

    #[test]
    fn test_construct_context() {
        let working = make_working();
        let context = ContextState::initial();
        let ctx = construct(&working, &context);
        assert_eq!(ctx.conversation_id, Some(1));
        assert_eq!(ctx.window_position, 5);
        assert_eq!(ctx.active_concepts.len(), 2);
        assert_eq!(ctx.world_assumptions.len(), 1);
    }

    #[test]
    fn test_advance_context() {
        let mut ctx = ContextState::initial();
        let before = ctx.temporal_context.current_time;
        advance_context(&mut ctx);
        assert!(ctx.temporal_context.current_time.0 >= before.0);
        assert_eq!(ctx.temporal_context.prior_states.len(), 1);
    }
}
