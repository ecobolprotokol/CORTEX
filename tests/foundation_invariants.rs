use cortex::types::common::Timestamp;
use cortex::types::evidence::{ConfidenceState, EvidencePolarity, EvidenceSet, VerificationStatus};
use cortex::types::ids::*;
use cortex::types::observation::Observation;
use cortex::types::scalars::{Confidence, ScalarOps, SCALAR_EPSILON};
use cortex::types::state::{ARCHITECTURE_VERSION, SCHEMA_VERSION};

#[test]
fn test_confidence_clamped_to_range() {
    let values = vec![
        -1.0,
        -0.01,
        0.0,
        0.5,
        1.0,
        1.01,
        2.0,
        f32::INFINITY,
        f32::NEG_INFINITY,
    ];
    for v in values {
        let c = Confidence::new(v);
        assert!(c.raw() >= 0.0, "Confidence({}) should be >= 0", v);
        assert!(c.raw() <= 1.0, "Confidence({}) should be <= 1", v);
    }
}

#[test]
fn test_confidence_nan_becomes_zero() {
    let c = Confidence::new(f32::NAN);
    assert_eq!(c.raw(), 0.0);
}

#[test]
fn test_confidence_infinity_becomes_one() {
    let c = Confidence::new(f32::INFINITY);
    assert_eq!(c.raw(), 1.0);
}

#[test]
fn test_confidence_state_overall_always_in_range() {
    let test_cases = vec![
        ConfidenceState::default(),
        ConfidenceState::low(),
        ConfidenceState::high(),
        ConfidenceState {
            belief: 1.0,
            evidence_strength: 1.0,
            source_quality: 1.0,
            consistency: 1.0,
            uncertainty: 0.0,
            prediction_reliability: 1.0,
            verification_status: VerificationStatus::Verified,
        },
        ConfidenceState {
            belief: 0.0,
            evidence_strength: 0.0,
            source_quality: 0.0,
            consistency: 0.0,
            uncertainty: 1.0,
            prediction_reliability: 0.0,
            verification_status: VerificationStatus::Observed,
        },
    ];
    for cs in &test_cases {
        let overall = cs.overall();
        assert!(
            overall >= 0.0,
            "ConfidenceState::overall() should be >= 0, got {}",
            overall
        );
        assert!(
            overall <= 1.0,
            "ConfidenceState::overall() should be <= 1, got {}",
            overall
        );
    }
}

#[test]
fn test_id_uniqueness() {
    let id1 = CellId::next();
    let id2 = CellId::next();
    let id3 = ColumnId::next();
    assert_ne!(id1, id2);
    assert_ne!(id1.raw(), id3.raw() - 1);
}

#[test]
fn test_id_null_sentinel() {
    let null = CellId::NULL;
    assert_eq!(null.raw(), 0);
    let next = CellId::next();
    assert_ne!(next.raw(), 0);
}

#[test]
fn test_id_from_raw() {
    let id = CellId::from(42);
    assert_eq!(id.raw(), 42);
}

#[test]
fn test_timestamp_now_is_positive() {
    let ts = Timestamp::now();
    assert!(ts.as_millis() > 0);
}

#[test]
fn test_timestamp_elapsed_since() {
    let earlier = Timestamp::from_secs(1000);
    let later = Timestamp::from_secs(2000);
    let elapsed = later.elapsed_since(earlier);
    assert_eq!(elapsed.as_millis(), 1_000_000);
}

#[test]
fn test_timestamp_ordering() {
    let t1 = Timestamp::from_secs(100);
    let t2 = Timestamp::from_secs(200);
    assert!(t1 < t2);
    assert!(t2 > t1);
    assert!(!t1.is_after(t2));
}

#[test]
fn test_timestamp_is_before_after() {
    let t1 = Timestamp::from_secs(100);
    let t2 = Timestamp::from_secs(200);
    assert!(t1.is_before(t2));
    assert!(t2.is_after(t1));
    assert!(!t1.is_after(t2));
    assert!(!t2.is_before(t1));
}

#[test]
fn test_verification_status_valid_transitions() {
    let statuses = vec![
        VerificationStatus::Observed,
        VerificationStatus::Inferred,
        VerificationStatus::Supported,
        VerificationStatus::Provisional,
        VerificationStatus::Verified,
        VerificationStatus::Unknown,
        VerificationStatus::Contradicted,
    ];
    for s in &statuses {
        let debug = format!("{:?}", s);
        assert!(!debug.is_empty());
    }
}

#[test]
fn test_evidence_polarity_valid() {
    let polarities = vec![
        EvidencePolarity::Supports,
        EvidencePolarity::Contradicts,
        EvidencePolarity::Neutral,
    ];
    for p in &polarities {
        let debug = format!("{:?}", p);
        assert!(!debug.is_empty());
    }
}

#[test]
fn test_evidence_set_operations() {
    let mut set = EvidenceSet::new();
    assert!(set.is_empty());
    assert_eq!(set.len(), 0);

    set.add(cortex::types::evidence::Evidence {
        id: EvidenceId::next(),
        source: cortex::types::ids::ProvenanceId::next(),
        content: cortex::types::evidence::EvidenceContent::Text("test".into()),
        strength: Confidence::new(0.8).raw(),
        polarity: EvidencePolarity::Supports,
        timestamp: Timestamp::now(),
        related: vec![],
    });
    assert!(!set.is_empty());
    assert_eq!(set.len(), 1);
}

#[test]
fn test_scalar_ops_validate_range() {
    assert!(0.5_f32.is_valid_cognitive_value());
    assert!(!f32::NAN.is_valid_cognitive_value());
    assert!(!f32::INFINITY.is_valid_cognitive_value());
    assert!(!f32::NEG_INFINITY.is_valid_cognitive_value());

    assert!(0.5_f32.validate_range(0.0, 1.0).is_ok());
    assert!(1.5_f32.validate_range(0.0, 1.0).is_err());
    assert!((-0.1_f32).validate_range(0.0, 1.0).is_err());
}

#[test]
fn test_architecture_version_constant() {
    assert_ne!(ARCHITECTURE_VERSION, 0, "ARCHITECTURE_VERSION must be > 0");
    assert_ne!(SCHEMA_VERSION, 0, "SCHEMA_VERSION must be > 0");
}

#[test]
fn test_observation_user_provided() {
    let obs = Observation::user_provided("test input");
    assert_eq!(obs.text, "test input");
    assert!(obs.importance > 0.0);
}

#[test]
fn test_scalar_epsilon_is_positive_and_usable() {
    let epsilon = SCALAR_EPSILON;
    assert!(epsilon > 0.0, "SCALAR_EPSILON must be positive");
    assert!(epsilon < 1.0, "SCALAR_EPSILON must be less than 1.0");
    assert!(
        1.0_f32 - epsilon < 1.0,
        "SCALAR_EPSILON must be small enough for float comparison"
    );
}

#[test]
fn test_confidence_serialization_roundtrip() {
    let c = Confidence::new(0.75);
    let json = serde_json::to_string(&c).unwrap();
    let deserialized: Confidence = serde_json::from_str(&json).unwrap();
    assert_eq!(c, deserialized);
}

#[test]
fn test_confidence_state_serialization_roundtrip() {
    let cs = ConfidenceState::high();
    let json = serde_json::to_string(&cs).unwrap();
    let deserialized: ConfidenceState = serde_json::from_str(&json).unwrap();
    assert_eq!(cs, deserialized);
}

#[test]
fn test_cortex_state_serialization_roundtrip() {
    let state = cortex::types::state::CortexState::default();
    let json = serde_json::to_string(&state).unwrap();
    let deserialized: cortex::types::state::CortexState = serde_json::from_str(&json).unwrap();
    assert_eq!(
        state.metadata.architecture_version,
        deserialized.metadata.architecture_version
    );
    assert_eq!(
        state.metadata.schema_version,
        deserialized.metadata.schema_version
    );
}
