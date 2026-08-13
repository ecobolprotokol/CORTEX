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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_procedure() {
        let mut memory = ProceduralMemory {
            procedures: Vec::new(),
            capacity_bytes: 1024 * 1024,
            current_usage_bytes: 0,
            next_id: ProcedureId(1),
        };
        let procedure = Procedure {
            id: ProcedureId(1),
            condition: Condition {
                description: "test".into(),
                required_concepts: Vec::new(),
                required_entities: Vec::new(),
                required_context: None,
            },
            steps: Vec::new(),
            expected_outcome: Outcome {
                success: true,
                description: "test".into(),
                result: None,
                timestamp: Timestamp::now(),
                confidence: 0.8,
            },
            success_count: 0,
            failure_count: 0,
            confidence: 0.5,
            context_requirements: ContextRequirements {
                requires_world_model: false,
                requires_memory: false,
                requires_reasoning: false,
                max_context_tokens: 1024,
            },
            risk: RiskAssessment::default(),
            provenance: Provenance::user_provided(),
            created_at: Timestamp::now(),
            last_used: None,
        };
        store(&mut memory, procedure).unwrap();
        assert_eq!(memory.procedures.len(), 1);
    }
}
