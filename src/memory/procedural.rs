use crate::error::{CortexError, Result};
use crate::types::*;

pub fn store(memory: &mut ProceduralMemory, procedure: Procedure) -> Result<()> {
    let size = estimate_size(&procedure);
    while memory.current_usage_bytes + size > memory.capacity_bytes && !memory.procedures.is_empty() {
        evict_lowest(memory);
    }
    memory.current_usage_bytes += size;
    memory.procedures.push(procedure);
    Ok(())
}

pub fn find_by_condition<'a>(memory: &'a ProceduralMemory, description: &str) -> Vec<&'a Procedure> {
    let desc_lower = description.to_lowercase();
    memory.procedures.iter().filter(|p| {
        p.condition.description.to_lowercase().contains(&desc_lower)
    }).collect()
}

pub fn update_success_rate(memory: &mut ProceduralMemory, id: ProcedureId, success: bool) -> Result<()> {
    let procedure = memory.procedures.iter_mut().find(|p| p.id == id)
        .ok_or_else(|| CortexError::MemoryError(format!("Procedure {} not found", id)))?;

    if success {
        procedure.success_count += 1;
        let total = procedure.success_count + procedure.failure_count;
        let success_rate = procedure.success_count as f32 / total as f32;
        procedure.confidence = (success_rate * 0.6 + procedure.confidence * 0.4).min(1.0);
    } else {
        procedure.failure_count += 1;
        let total = procedure.success_count + procedure.failure_count;
        let success_rate = procedure.success_count as f32 / total as f32;
        procedure.confidence = (success_rate * 0.4 + procedure.confidence * 0.3).min(1.0);
    }

    procedure.last_used = Some(Timestamp::now());
    Ok(())
}

pub fn select_best<'a>(memory: &'a ProceduralMemory, description: &str) -> Option<&'a Procedure> {
    let desc_lower = description.to_lowercase();
    memory.procedures.iter()
        .filter(|p| p.condition.description.to_lowercase().contains(&desc_lower))
        .max_by(|a, b| {
            let score_a = procedure_score(a);
            let score_b = procedure_score(b);
            score_a.partial_cmp(&score_b).unwrap_or(std::cmp::Ordering::Equal)
        })
}

fn procedure_score(procedure: &Procedure) -> f32 {
    let success_rate = if procedure.success_count + procedure.failure_count > 0 {
        procedure.success_count as f32 / (procedure.success_count + procedure.failure_count) as f32
    } else {
        0.5
    };
    success_rate * 0.6 + procedure.confidence * 0.4
}

fn evict_lowest(memory: &mut ProceduralMemory) {
    if memory.procedures.is_empty() {
        return;
    }
    let mut min_idx = 0;
    let mut min_score = f32::MAX;
    for (i, p) in memory.procedures.iter().enumerate() {
        let score = procedure_score(p);
        if score < min_score {
            min_score = score;
            min_idx = i;
        }
    }
    let removed = memory.procedures.remove(min_idx);
    memory.current_usage_bytes = memory.current_usage_bytes.saturating_sub(estimate_size(&removed));
}

fn estimate_size(procedure: &Procedure) -> u64 {
    let steps_size = procedure.steps.len() as u64 * 128;
    let condition_size = procedure.condition.description.len() as u64 + 64;
    let concepts_size = procedure.condition.required_concepts.len() as u64 * 8;
    let entities_size = procedure.condition.required_entities.len() as u64 * 8;
    256 + steps_size + condition_size + concepts_size + entities_size
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_procedure(id: u64, description: &str, success: u64, failure: u64, confidence: f32) -> Procedure {
        Procedure {
            id: ProcedureId(id),
            condition: Condition {
                description: description.into(),
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
            success_count: success,
            failure_count: failure,
            confidence,
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
        }
    }

    fn make_memory(capacity: u64) -> ProceduralMemory {
        ProceduralMemory {
            procedures: Vec::new(),
            capacity_bytes: capacity,
            current_usage_bytes: 0,
            next_id: ProcedureId(1),
        }
    }

    #[test]
    fn test_store_and_find() {
        let mut memory = make_memory(1024 * 1024);
        store(&mut memory, make_procedure(1, "how to cook rice", 10, 2, 0.8)).unwrap();
        store(&mut memory, make_procedure(2, "how to bake bread", 5, 1, 0.7)).unwrap();

        let found = find_by_condition(&memory, "cook rice");
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].id, ProcedureId(1));
    }

    #[test]
    fn test_find_by_condition_case_insensitive() {
        let mut memory = make_memory(1024 * 1024);
        store(&mut memory, make_procedure(1, "How To Cook Rice", 10, 2, 0.8)).unwrap();

        let found = find_by_condition(&memory, "cook rice");
        assert_eq!(found.len(), 1);
    }

    #[test]
    fn test_find_by_condition_no_match() {
        let mut memory = make_memory(1024 * 1024);
        store(&mut memory, make_procedure(1, "how to cook rice", 10, 2, 0.8)).unwrap();

        let found = find_by_condition(&memory, "bake bread");
        assert!(found.is_empty());
    }

    #[test]
    fn test_update_success_rate_success() {
        let mut memory = make_memory(1024 * 1024);
        store(&mut memory, make_procedure(1, "test", 0, 0, 0.5)).unwrap();

        update_success_rate(&mut memory, ProcedureId(1), true).unwrap();
        let p = &memory.procedures[0];
        assert_eq!(p.success_count, 1);
        assert_eq!(p.failure_count, 0);
        assert!(p.last_used.is_some());
    }

    #[test]
    fn test_update_success_rate_failure() {
        let mut memory = make_memory(1024 * 1024);
        store(&mut memory, make_procedure(1, "test", 5, 0, 0.8)).unwrap();

        update_success_rate(&mut memory, ProcedureId(1), false).unwrap();
        let p = &memory.procedures[0];
        assert_eq!(p.success_count, 5);
        assert_eq!(p.failure_count, 1);
        assert!(p.confidence < 0.8);
    }

    #[test]
    fn test_update_success_rate_not_found() {
        let mut memory = make_memory(1024 * 1024);
        let result = update_success_rate(&mut memory, ProcedureId(999), true);
        assert!(result.is_err());
    }

    #[test]
    fn test_select_best() {
        let mut memory = make_memory(1024 * 1024);
        store(&mut memory, make_procedure(1, "cook rice", 10, 2, 0.8)).unwrap();
        store(&mut memory, make_procedure(2, "cook rice perfectly", 50, 1, 0.9)).unwrap();
        store(&mut memory, make_procedure(3, "cook pasta", 20, 5, 0.7)).unwrap();

        let best = select_best(&memory, "cook rice").unwrap();
        assert_eq!(best.id, ProcedureId(2));
    }

    #[test]
    fn test_select_best_empty() {
        let memory = make_memory(1024 * 1024);
        assert!(select_best(&memory, "cook rice").is_none());
    }

    #[test]
    fn test_eviction() {
        let mut memory = make_memory(400);
        store(&mut memory, make_procedure(1, "alpha task", 10, 0, 0.9)).unwrap();
        store(&mut memory, make_procedure(2, "beta task", 1, 9, 0.1)).unwrap();
        store(&mut memory, make_procedure(3, "gamma task", 8, 2, 0.7)).unwrap();
        store(&mut memory, make_procedure(4, "delta task", 5, 5, 0.5)).unwrap();

        assert!(memory.procedures.len() < 4);
        let ids: Vec<ProcedureId> = memory.procedures.iter().map(|p| p.id).collect();
        assert!(!ids.contains(&ProcedureId(2)));
    }

    #[test]
    fn test_size_estimation() {
        let p1 = make_procedure(1, "short", 0, 0, 0.5);
        let p2 = make_procedure(2, "a much longer description that takes more space", 0, 0, 0.5);
        assert!(estimate_size(&p2) > estimate_size(&p1));
    }

    #[test]
    fn test_procedure_score() {
        let p = make_procedure(1, "test", 8, 2, 0.7);
        let score = procedure_score(&p);
        let expected = 0.8 * 0.6 + 0.7 * 0.4;
        assert!((score - expected).abs() < 0.001);
    }
}
