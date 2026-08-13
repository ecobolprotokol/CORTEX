use cortex::types::evidence::VerificationStatus;
use cortex::verification::confidence::ConfidenceModel;

#[test]
fn test_confidence_model_verify() {
    let model = ConfidenceModel::new(0.8);
    let result = model.verify("Test claim", 0.9);
    assert_eq!(result.status, VerificationStatus::Verified);
}

#[test]
fn test_confidence_model_provisional() {
    let model = ConfidenceModel::new(0.8);
    let result = model.verify("Test claim", 0.5);
    assert_eq!(result.status, VerificationStatus::Provisional);
}

#[test]
fn test_confidence_model_observed() {
    let model = ConfidenceModel::new(0.8);
    let result = model.verify("Test claim", 0.0);
    assert_eq!(result.status, VerificationStatus::Observed);
}

#[test]
fn test_overall_confidence() {
    let model = ConfidenceModel::new(0.8);
    let confidence = model.compute_overall_confidence(0.8, 0.6, 0.1);
    assert!(confidence > 0.0);
    assert!(confidence < 1.0);
}
