use cortex::reasoning::hypothesis::HypothesisGenerator;
use cortex::reasoning::evidence::EvidenceEvaluator;
use cortex::reasoning::contradiction::ContradictionDetector;
use cortex::types::ids::HypothesisId;
use cortex::types::evidence::{Evidence, EvidencePolarity, EvidenceContent};

#[test]
fn test_hypothesis_generator() {
    let mut generator = HypothesisGenerator::new(10);
    let hypotheses = generator.generate("What is gravity?", &["physics".into()]);
    assert!(!hypotheses.is_empty());
}

#[test]
fn test_evidence_evaluator() {
    let evidence = vec![
        Evidence {
            id: cortex::types::ids::EvidenceId::from(1),
            source: cortex::types::ids::ProvenanceId::from(1),
            content: EvidenceContent::Text("Test evidence".into()),
            strength: 0.8,
            polarity: EvidencePolarity::Supports,
            timestamp: cortex::types::common::Timestamp::now(),
            related: vec![],
        },
    ];
    let quality = EvidenceEvaluator::evaluate_evidence_quality(&evidence);
    assert!(quality > 0.0);
}

#[test]
fn test_contradiction_detector() {
    let detector = ContradictionDetector::new();
    let propositions = vec![
        (HypothesisId::from(1), "The sky is always blue".into()),
        (HypothesisId::from(2), "The sky is never blue".into()),
    ];
    let contradictions = detector.detect(&propositions);
    assert!(!contradictions.is_empty());
}
