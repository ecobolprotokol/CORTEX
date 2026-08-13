use super::mutation::{MutationId, MutationKind, MutationLog, RecordParams};
use crate::error::CortexError;
use crate::types::state::CortexState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionState {
    Active,
    Committed,
    RolledBack,
    Failed,
}

pub struct StateTransaction {
    pub id: MutationId,
    pub kind: MutationKind,
    pub state: TransactionState,
    pub description: String,
    pub pre_version: u64,
    mutations_applied: Vec<String>,
    snapshot: Option<CortexState>,
}

impl StateTransaction {
    pub fn begin(kind: MutationKind, description: &str, pre_version: u64) -> Self {
        Self {
            id: MutationId::next(),
            kind,
            state: TransactionState::Active,
            description: description.to_string(),
            pre_version,
            mutations_applied: Vec::new(),
            snapshot: None,
        }
    }

    pub fn begin_with_snapshot(
        kind: MutationKind,
        description: &str,
        pre_version: u64,
        state: &CortexState,
    ) -> Self {
        Self {
            id: MutationId::next(),
            kind,
            state: TransactionState::Active,
            description: description.to_string(),
            pre_version,
            mutations_applied: Vec::new(),
            snapshot: Some(state.clone()),
        }
    }

    pub fn apply(&mut self, mutation: &str) -> Result<(), CortexError> {
        if self.state != TransactionState::Active {
            return Err(CortexError::RuntimeError("Transaction not active".into()));
        }
        self.mutations_applied.push(mutation.to_string());
        Ok(())
    }

    pub fn commit(self, log: &mut MutationLog, post_version: u64) -> MutationId {
        log.record(RecordParams {
            kind: self.kind,
            description: &self.description,
            subsystem: "cortex",
            pre_version: self.pre_version,
            post_version,
            success: true,
            error: None,
        })
    }

    pub fn rollback(self, log: &mut MutationLog, reason: &str) -> (MutationId, Option<CortexState>) {
        let snapshot = self.snapshot;
        let id = log.record(RecordParams {
            kind: self.kind,
            description: &format!("ROLLBACK: {} — {}", self.description, reason),
            subsystem: "cortex",
            pre_version: self.pre_version,
            post_version: self.pre_version,
            success: false,
            error: Some(reason.to_string()),
        });
        (id, snapshot)
    }

    pub fn mutations(&self) -> &[String] {
        &self.mutations_applied
    }

    pub fn has_snapshot(&self) -> bool {
        self.snapshot.is_some()
    }

    pub fn mutation_count(&self) -> usize {
        self.mutations_applied.len()
    }
}
