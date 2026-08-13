use serde::{Deserialize, Serialize};
use crate::types::ids::ProcedureId;
use crate::types::scalars::Scalar;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Procedure {
    pub id: ProcedureId,
    pub name: String,
    pub steps: Vec<String>,
    pub success_count: u64,
    pub failure_count: u64,
    pub confidence: Scalar,
}

#[derive(Debug, Clone)]
pub struct ProceduralMemory {
    pub procedures: Vec<Procedure>,
    pub next_id: u64,
}

impl ProceduralMemory {
    pub fn new() -> Self {
        Self {
            procedures: Vec::new(),
            next_id: 1,
        }
    }

    pub fn store(&mut self, name: &str, steps: Vec<String>) -> Procedure {
        let p = Procedure {
            id: ProcedureId::from(self.next_id),
            name: name.to_string(),
            steps,
            success_count: 0,
            failure_count: 0,
            confidence: 0.5,
        };
        self.next_id += 1;
        self.procedures.push(p.clone());
        p
    }

    pub fn record_success(&mut self, id: ProcedureId) {
        if let Some(p) = self.procedures.iter_mut().find(|p| p.id == id) {
            p.success_count += 1;
            let total = p.success_count + p.failure_count;
            p.confidence = p.success_count as f32 / total as f32;
        }
    }

    pub fn record_failure(&mut self, id: ProcedureId) {
        if let Some(p) = self.procedures.iter_mut().find(|p| p.id == id) {
            p.failure_count += 1;
            let total = p.success_count + p.failure_count;
            p.confidence = p.success_count as f32 / total as f32;
        }
    }
}
