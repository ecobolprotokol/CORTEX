use cortex::transaction::mutation::{MutationId, MutationKind, MutationLog, RecordParams};
use cortex::transaction::state_tx::{StateTransaction, TransactionState};
use cortex::transaction::invariant::StateInvariant;
use cortex::types::state::CortexState;

#[test]
fn test_mutation_id_uniqueness() {
    let id1 = MutationId::next();
    let id2 = MutationId::next();
    assert_ne!(id1.raw(), id2.raw());
}

#[test]
fn test_mutation_log_record() {
    let mut log = MutationLog::new(100);
    let id = log.record(RecordParams {
        kind: MutationKind::LanguageEncode,
        description: "test mutation",
        subsystem: "language",
        pre_version: 0,
        post_version: 1,
        success: true,
        error: None,
    });
    assert!(id.raw() > 0);
    assert_eq!(log.records.len(), 1);
}

#[test]
fn test_mutation_log_eviction() {
    let mut log = MutationLog::new(5);
    for i in 0..10u64 {
        log.record(RecordParams {
            kind: MutationKind::NeuralProcess,
            description: "",
            subsystem: "neural",
            pre_version: i,
            post_version: i + 1,
            success: true,
            error: None,
        });
    }
    assert_eq!(log.records.len(), 5);
}

#[test]
fn test_mutation_log_failed() {
    let mut log = MutationLog::new(100);
    log.record(RecordParams {
        kind: MutationKind::MemoryStore,
        description: "ok",
        subsystem: "memory",
        pre_version: 0,
        post_version: 1,
        success: true,
        error: None,
    });
    log.record(RecordParams {
        kind: MutationKind::MemoryStore,
        description: "failed",
        subsystem: "memory",
        pre_version: 1,
        post_version: 1,
        success: false,
        error: Some("disk full".into()),
    });
    assert_eq!(log.failed_mutations().len(), 1);
}

#[test]
fn test_mutation_log_count_by_kind() {
    let mut log = MutationLog::new(100);
    log.record(RecordParams { kind: MutationKind::LanguageEncode, description: "a", subsystem: "lang", pre_version: 0, post_version: 1, success: true, error: None });
    log.record(RecordParams { kind: MutationKind::LanguageEncode, description: "b", subsystem: "lang", pre_version: 1, post_version: 2, success: true, error: None });
    log.record(RecordParams { kind: MutationKind::NeuralProcess, description: "c", subsystem: "neural", pre_version: 0, post_version: 1, success: true, error: None });
    assert_eq!(log.count_by_kind(MutationKind::LanguageEncode), 2);
    assert_eq!(log.count_by_kind(MutationKind::NeuralProcess), 1);
}

#[test]
fn test_transaction_begin() {
    let txn = StateTransaction::begin(MutationKind::LanguageEncode, "test", 0);
    assert_eq!(txn.state, TransactionState::Active);
    assert_eq!(txn.kind, MutationKind::LanguageEncode);
}

#[test]
fn test_transaction_apply() {
    let mut txn = StateTransaction::begin(MutationKind::NeuralProcess, "test", 0);
    txn.apply("step 1").unwrap();
    txn.apply("step 2").unwrap();
    assert_eq!(txn.mutations().len(), 2);
}

#[test]
fn test_transaction_commit() {
    let mut log = MutationLog::new(100);
    let txn = StateTransaction::begin(MutationKind::LanguageEncode, "test", 0);
    txn.commit(&mut log, 1);
    assert_eq!(log.records.len(), 1);
    assert!(log.records[0].success);
}

#[test]
fn test_transaction_rollback() {
    let mut log = MutationLog::new(100);
    let mut txn = StateTransaction::begin(MutationKind::MemoryStore, "test", 1);
    txn.apply("partial mutation").unwrap();
    txn.rollback(&mut log, "validation failed");
    assert_eq!(log.records.len(), 1);
    assert!(!log.records[0].success);
    assert!(log.records[0].error.is_some());
    assert_eq!(log.records[0].post_version, 1);
}

#[test]
fn test_state_invariant_valid_state() {
    let state = CortexState::default();
    let result = StateInvariant::validate_state(&state);
    assert!(result.is_ok());
}

#[test]
fn test_state_invariant_bad_version() {
    let mut state = CortexState::default();
    state.metadata.architecture_version = 0;
    let result = StateInvariant::validate_state(&state);
    assert!(result.is_err());
}

#[test]
fn test_state_invariant_bad_confidence_threshold() {
    let mut state = CortexState::default();
    state.verification.confidence_threshold = 1.5;
    let result = StateInvariant::validate_state(&state);
    assert!(result.is_err());
}

#[test]
fn test_pre_mutation_check() {
    let state = CortexState::default();
    let result = StateInvariant::pre_mutation_check(&state, 0);
    assert!(result.is_ok());
}

#[test]
fn test_post_mutation_check() {
    let state = CortexState::default();
    let result = StateInvariant::post_mutation_check(&state, 1);
    assert!(result.is_ok());
}

#[test]
fn test_mutation_log_last_n() {
    let mut log = MutationLog::new(100);
    for i in 0..10u64 {
        log.record(RecordParams {
            kind: MutationKind::LanguageEncode,
            description: "",
            subsystem: "lang",
            pre_version: i,
            post_version: i + 1,
            success: true,
            error: None,
        });
    }
    let last_3 = log.last_n(3);
    assert_eq!(last_3.len(), 3);
}
