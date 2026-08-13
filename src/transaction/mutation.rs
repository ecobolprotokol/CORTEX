use serde::{Deserialize, Serialize};
use crate::types::common::Timestamp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MutationId(u64);

impl MutationId {
    pub fn next() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
    pub fn raw(self) -> u64 { self.0 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MutationKind {
    LanguageEncode,
    NeuralProcess,
    MemoryStore,
    MemoryEvict,
    MemoryConsolidate,
    WorldIntegrate,
    ReasoningEvaluate,
    PlanningEvaluate,
    VerificationEvaluate,
    LearningApply,
    CheckpointCreate,
    StateInitialize,
    StateRecover,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationRecord {
    pub id: MutationId,
    pub kind: MutationKind,
    pub timestamp: Timestamp,
    pub description: String,
    pub pre_version: u64,
    pub post_version: u64,
    pub subsystem: String,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RecordParams<'a> {
    pub kind: MutationKind,
    pub description: &'a str,
    pub subsystem: &'a str,
    pub pre_version: u64,
    pub post_version: u64,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MutationLog {
    pub records: Vec<MutationRecord>,
    pub max_size: usize,
}

impl MutationLog {
    pub fn new(max_size: usize) -> Self {
        Self { records: Vec::new(), max_size }
    }

    pub fn record(&mut self, params: RecordParams) -> MutationId {
        let id = MutationId::next();
        let record = MutationRecord {
            id,
            kind: params.kind,
            timestamp: Timestamp::now(),
            description: params.description.to_string(),
            pre_version: params.pre_version,
            post_version: params.post_version,
            subsystem: params.subsystem.to_string(),
            success: params.success,
            error: params.error,
        };
        self.records.push(record);
        if self.records.len() > self.max_size {
            self.records.remove(0);
        }
        id
    }

    pub fn last_n(&self, n: usize) -> &[MutationRecord] {
        let start = self.records.len().saturating_sub(n);
        &self.records[start..]
    }

    pub fn failed_mutations(&self) -> Vec<&MutationRecord> {
        self.records.iter().filter(|r| !r.success).collect()
    }

    pub fn count_by_kind(&self, kind: MutationKind) -> usize {
        self.records.iter().filter(|r| r.kind == kind && r.success).count()
    }
}
