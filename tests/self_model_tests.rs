use cortex::self_model::capability::{CapabilityAssessment, SelfModel};

#[test]
fn test_capability_assessment_default() {
    let assessment = CapabilityAssessment::default();
    assert!(assessment.prediction_accuracy > 0.0);
}

#[test]
fn test_capability_assessment_weakest() {
    let assessment = CapabilityAssessment::default();
    let (name, value) = assessment.weakest_capability();
    assert!(!name.is_empty());
    assert!(value > 0.0);
}

#[test]
fn test_capability_assessment_strongest() {
    let assessment = CapabilityAssessment::default();
    let (name, value) = assessment.strongest_capability();
    assert!(!name.is_empty());
    assert!(value > 0.0);
}

#[test]
fn test_self_model() {
    let model = SelfModel::new();
    let assessment = model.assess();
    assert!(!assessment.weakest_capability().0.is_empty());
}

#[test]
fn test_self_model_task_confidence() {
    let model = SelfModel::new();
    let confidence = model.confidence_in_task("observation");
    assert!(confidence >= 0.0);
    assert!(confidence <= 1.0);
}
