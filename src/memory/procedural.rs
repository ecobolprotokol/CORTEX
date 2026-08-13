use serde::{Deserialize, Serialize};

use crate::types::ids::ProcedureId;
use crate::types::scalars::Scalar;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Procedure {
    pub id: ProcedureId,
    pub name: String,
    pub condition: String,
    pub steps: Vec<String>,
    pub expected_outcome: String,
    pub success_count: u64,
    pub failure_count: u64,
    pub confidence: Scalar,
}

impl Procedure {
    pub fn success_rate(&self) -> Scalar {
        let total = self.success_count + self.failure_count;
        if total == 0 {
            0.5
        } else {
            self.success_count as Scalar / total as Scalar
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProceduralMemory {
    pub procedures: Vec<Procedure>,
    pub next_id: u64,
}

impl Default for ProceduralMemory {
    fn default() -> Self {
        Self::new()
    }
}

impl ProceduralMemory {
    pub fn new() -> Self {
        Self {
            procedures: Vec::new(),
            next_id: 1,
        }
    }

    pub fn store(
        &mut self,
        name: &str,
        condition: &str,
        steps: Vec<String>,
        expected_outcome: &str,
    ) -> Procedure {
        let p = Procedure {
            id: ProcedureId::from(self.next_id),
            name: name.to_string(),
            condition: condition.to_string(),
            steps,
            expected_outcome: expected_outcome.to_string(),
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

    pub fn get(&self, id: ProcedureId) -> Option<&Procedure> {
        self.procedures.iter().find(|p| p.id == id)
    }

    pub fn find_by_name(&self, name: &str) -> Vec<&Procedure> {
        self.procedures
            .iter()
            .filter(|p| p.name == name)
            .collect()
    }

    pub fn by_success_rate(&self) -> Vec<&Procedure> {
        let mut sorted: Vec<&Procedure> = self.procedures.iter().collect();
        sorted.sort_by(|a, b| {
            b.success_rate()
                .partial_cmp(&a.success_rate())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        sorted
    }

    pub fn usage_bytes(&self) -> usize {
        self.procedures.len() * std::mem::size_of::<Procedure>()
    }
}
