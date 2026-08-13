use crate::types::*;

pub fn update_input(working: &mut WorkingMemory, text: &str) {
    working.input = Some(CurrentInput {
        text: text.to_string(),
        timestamp: Timestamp::now(),
        kind: ObservationKind::UserInput,
    });
    working.conversation_context.turn_count += 1;
    working.conversation_context.recent_inputs.push(text.to_string());
    if working.conversation_context.recent_inputs.len() > 10 {
        working.conversation_context.recent_inputs.remove(0);
    }
}

pub fn update_output(working: &mut WorkingMemory, text: &str) {
    working.conversation_context.recent_outputs.push(text.to_string());
    if working.conversation_context.recent_outputs.len() > 10 {
        working.conversation_context.recent_outputs.remove(0);
    }
}

pub fn add_active_concept(working: &mut WorkingMemory, concept: ConceptId) {
    if !working.active_concepts.contains(&concept) {
        working.active_concepts.push(concept);
    }
}

pub fn remove_active_concept(working: &mut WorkingMemory, concept: &ConceptId) {
    working.active_concepts.retain(|c| c != concept);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_working() -> WorkingMemory {
        WorkingMemory {
            input: None,
            conversation_context: ConversationContext {
                session_id: SessionId(1),
                turn_count: 0,
                recent_inputs: Vec::new(),
                recent_outputs: Vec::new(),
                started_at: Timestamp::now(),
            },
            active_concepts: Vec::new(),
            active_hypotheses: Vec::new(),
            goals: Vec::new(),
            reasoning_state: None,
            world_assumptions: Vec::new(),
            generation_state: None,
        }
    }

    #[test]
    fn test_update_input() {
        let mut working = make_working();
        update_input(&mut working, "hello");
        assert!(working.input.is_some());
        assert_eq!(working.conversation_context.turn_count, 1);
        assert_eq!(working.conversation_context.recent_inputs.len(), 1);
    }

    #[test]
    fn test_update_output() {
        let mut working = make_working();
        update_output(&mut working, "response");
        assert_eq!(working.conversation_context.recent_outputs.len(), 1);
    }

    #[test]
    fn test_add_concept() {
        let mut working = make_working();
        add_active_concept(&mut working, ConceptId(1));
        add_active_concept(&mut working, ConceptId(1));
        assert_eq!(working.active_concepts.len(), 1);
        add_active_concept(&mut working, ConceptId(2));
        assert_eq!(working.active_concepts.len(), 2);
    }

    #[test]
    fn test_remove_concept() {
        let mut working = make_working();
        add_active_concept(&mut working, ConceptId(1));
        remove_active_concept(&mut working, &ConceptId(1));
        assert!(working.active_concepts.is_empty());
    }
}
