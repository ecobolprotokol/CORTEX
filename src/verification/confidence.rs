use crate::types::*;

const EVIDENCE_SATURATION: Scalar = 20.0;
const DECAY_HALF_LIFE_SECS: u64 = 86400;
const CONFIDENCE_UPDATE_RATE: Scalar = 0.3;

pub fn compute_belief(evidence_count: usize, avg_evidence_strength: Scalar) -> Scalar {
    if evidence_count == 0 {
        return 0.0;
    }
    let count_factor = 1.0 - (-(evidence_count as Scalar) / EVIDENCE_SATURATION).exp();
    let quality_factor = avg_evidence_strength.clamp(0.0, 1.0);
    (count_factor * quality_factor).clamp(0.0, 1.0)
}

pub fn compute_evidence_strength(items: &[Evidence]) -> Scalar {
    if items.is_empty() {
        return 0.0;
    }
    let total: Scalar = items.iter().map(|e| e.strength).sum();
    total / items.len() as Scalar
}

pub fn compute_source_quality(evidence: &[Evidence]) -> Scalar {
    if evidence.is_empty() {
        return 0.0;
    }
    let total: Scalar = evidence
        .iter()
        .map(|e| e.source.source_identity.reliability)
        .sum();
    total / evidence.len() as Scalar
}

pub fn compute_consistency(evidence: &[Evidence]) -> Scalar {
    if evidence.is_empty() {
        return 1.0;
    }
    let supporting = evidence
        .iter()
        .filter(|e| e.polarity == EvidencePolarity::Supports)
        .count();
    let contradicting = evidence
        .iter()
        .filter(|e| e.polarity == EvidencePolarity::Contradicts)
        .count();
    let total = evidence.len() as Scalar;
    let agreement = (supporting as Scalar) / total;
    let contradiction_penalty = (contradicting as Scalar) / total * 0.5;
    (agreement - contradiction_penalty).clamp(0.0, 1.0)
}

pub fn compute_uncertainty(evidence: &[Evidence]) -> Scalar {
    if evidence.is_empty() {
        return 1.0;
    }
    let strengths: Vec<Scalar> = evidence.iter().map(|e| e.strength).collect();
    let mean = strengths.iter().sum::<Scalar>() / strengths.len() as Scalar;
    let variance = strengths
        .iter()
        .map(|s| (s - mean).powi(2))
        .sum::<Scalar>()
        / strengths.len() as Scalar;
    variance.sqrt().clamp(0.0, 1.0)
}

pub fn compute_decay_factor(elapsed_secs: u64) -> Scalar {
    let half_lives = elapsed_secs as Scalar / DECAY_HALF_LIFE_SECS as Scalar;
    (-0.693 * half_lives).exp()
}

pub fn compute_overall_from_evidence(evidence: &[Evidence], evidence_timestamp: Timestamp) -> ConfidenceState {
    let belief = compute_belief(evidence.len(), compute_evidence_strength(evidence));
    let evidence_strength = compute_evidence_strength(evidence);
    let source_quality = compute_source_quality(evidence);
    let consistency = compute_consistency(evidence);
    let uncertainty = compute_uncertainty(evidence);
    let elapsed = Timestamp::now().elapsed_since(evidence_timestamp).as_secs();
    let decay = compute_decay_factor(elapsed);

    let adjusted_belief = belief * decay;
    let adjusted_uncertainty = (uncertainty + (1.0 - decay) * 0.5).clamp(0.0, 1.0);

    let verification_status = if adjusted_belief >= 0.8 && consistency >= 0.7 {
        VerificationStatus::Verified
    } else if adjusted_belief >= 0.5 && consistency >= 0.5 {
        VerificationStatus::Supported
    } else if adjusted_belief >= 0.3 {
        VerificationStatus::Provisional
    } else {
        VerificationStatus::Unknown
    };

    ConfidenceState {
        belief: adjusted_belief,
        evidence_strength,
        source_quality,
        consistency,
        uncertainty: adjusted_uncertainty,
        prediction_reliability: 0.0,
        verification_status,
    }
}

pub fn compute_overall(claims: &[Claim]) -> ConfidenceState {
    if claims.is_empty() {
        return ConfidenceState::default();
    }

    let all_evidence: Vec<&Evidence> = claims.iter().flat_map(|c| c.evidence.items.iter()).collect();
    let evidence_vec: Vec<Evidence> = all_evidence.into_iter().cloned().collect();
    let earliest_timestamp = claims
        .iter()
        .map(|c| c.created_at)
        .min()
        .unwrap_or_else(Timestamp::now);

    compute_overall_from_evidence(&evidence_vec, earliest_timestamp)
}

pub fn update_confidence(existing: &ConfidenceState, new_evidence: &[Evidence]) -> ConfidenceState {
    let new_belief = compute_belief(new_evidence.len(), compute_evidence_strength(new_evidence));
    let new_evidence_strength = compute_evidence_strength(new_evidence);
    let new_source_quality = compute_source_quality(new_evidence);
    let new_consistency = compute_consistency(new_evidence);

    let combined_belief = existing.belief * (1.0 - CONFIDENCE_UPDATE_RATE)
        + new_belief * CONFIDENCE_UPDATE_RATE;
    let combined_evidence = existing.evidence_strength * (1.0 - CONFIDENCE_UPDATE_RATE)
        + new_evidence_strength * CONFIDENCE_UPDATE_RATE;
    let combined_source = existing.source_quality * (1.0 - CONFIDENCE_UPDATE_RATE)
        + new_source_quality * CONFIDENCE_UPDATE_RATE;
    let combined_consistency = existing.consistency * (1.0 - CONFIDENCE_UPDATE_RATE)
        + new_consistency * CONFIDENCE_UPDATE_RATE;
    let combined_uncertainty = (existing.uncertainty * (1.0 - CONFIDENCE_UPDATE_RATE)
        + compute_uncertainty(new_evidence) * CONFIDENCE_UPDATE_RATE)
        .clamp(0.0, 1.0);

    let verification_status = if combined_belief >= 0.8 && combined_consistency >= 0.7 {
        VerificationStatus::Verified
    } else if combined_belief >= 0.5 && combined_consistency >= 0.5 {
        VerificationStatus::Supported
    } else if combined_belief >= 0.3 {
        VerificationStatus::Provisional
    } else {
        VerificationStatus::Unknown
    };

    ConfidenceState {
        belief: combined_belief.clamp(0.0, 1.0),
        evidence_strength: combined_evidence.clamp(0.0, 1.0),
        source_quality: combined_source.clamp(0.0, 1.0),
        consistency: combined_consistency.clamp(0.0, 1.0),
        uncertainty: combined_uncertainty,
        prediction_reliability: existing.prediction_reliability,
        verification_status,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_evidence(id: u64, strength: Scalar, polarity: EvidencePolarity, reliability: Scalar) -> Evidence {
        Evidence {
            id: EvidenceId(id),
            source: Provenance {
                category: ProvenanceCategory::Internet,
                source: Source {
                    id: SourceId(1),
                    name: "test".into(),
                    kind: SourceKind::Internet,
                },
                source_identity: SourceIdentity {
                    identifier: "test".into(),
                    reliability,
                    verification_count: 0,
                },
                timestamp: Timestamp::now(),
                retrieval_context: None,
                content_hash: [0u8; 32],
                evidence: EvidenceSet::new(),
                verification_status: VerificationStatus::Unknown,
                confidence: ConfidenceState::default(),
            },
            content: EvidenceContent::Text("test".into()),
            strength,
            polarity,
            timestamp: Timestamp::now(),
            related: Vec::new(),
        }
    }

    #[test]
    fn test_compute_overall_empty() {
        let result = compute_overall(&[]);
        assert_eq!(result.verification_status, VerificationStatus::Unknown);
    }

    #[test]
    fn test_compute_overall_high_confidence() {
        let claims = vec![Claim {
            id: ClaimId(1),
            text: "test".into(),
            status: VerificationStatus::Verified,
            confidence: ConfidenceState {
                belief: 0.9,
                evidence_strength: 0.8,
                source_quality: 0.9,
                consistency: 0.9,
                uncertainty: 0.1,
                prediction_reliability: 0.0,
                verification_status: VerificationStatus::Verified,
            },
            evidence: EvidenceSet::new(),
            created_at: Timestamp::now(),
        }];
        let result = compute_overall(&claims);
        assert_eq!(result.verification_status, VerificationStatus::Unknown);
        assert!(result.belief < 0.1);
    }

    #[test]
    fn test_compute_belief_zero_evidence() {
        assert!(scalar_eq(compute_belief(0, 0.5), 0.0));
    }

    #[test]
    fn test_compute_belief_increases_with_count() {
        let b1 = compute_belief(1, 0.8);
        let b5 = compute_belief(5, 0.8);
        let b20 = compute_belief(20, 0.8);
        assert!(b5 > b1);
        assert!(b20 > b5);
        assert!(b20 < 1.0);
    }

    #[test]
    fn test_compute_belief_diminishing_returns() {
        let b10 = compute_belief(10, 0.8);
        let b100 = compute_belief(100, 0.8);
        let b1000 = compute_belief(1000, 0.8);
        assert!(b100 > b10);
        assert!(b1000 > b100);
        let gain_1 = b100 - b10;
        let gain_2 = b1000 - b100;
        assert!(gain_2 < gain_1);
    }

    #[test]
    fn test_compute_evidence_strength_empty() {
        assert!(scalar_eq(compute_evidence_strength(&[]), 0.0));
    }

    #[test]
    fn test_compute_evidence_strength() {
        let items = vec![
            make_evidence(1, 0.8, EvidencePolarity::Supports, 0.9),
            make_evidence(2, 0.6, EvidencePolarity::Supports, 0.7),
        ];
        let result = compute_evidence_strength(&items);
        assert!(scalar_eq(result, 0.7));
    }

    #[test]
    fn test_compute_source_quality() {
        let items = vec![
            make_evidence(1, 0.5, EvidencePolarity::Supports, 0.9),
            make_evidence(2, 0.5, EvidencePolarity::Supports, 0.5),
        ];
        let result = compute_source_quality(&items);
        assert!(scalar_eq(result, 0.7));
    }

    #[test]
    fn test_compute_consistency_all_supporting() {
        let items = vec![
            make_evidence(1, 0.8, EvidencePolarity::Supports, 0.9),
            make_evidence(2, 0.7, EvidencePolarity::Supports, 0.8),
        ];
        let result = compute_consistency(&items);
        assert!(scalar_eq(result, 1.0));
    }

    #[test]
    fn test_compute_consistency_with_contradiction() {
        let items = vec![
            make_evidence(1, 0.8, EvidencePolarity::Supports, 0.9),
            make_evidence(2, 0.7, EvidencePolarity::Contradicts, 0.8),
        ];
        let result = compute_consistency(&items);
        assert!(result < 1.0);
        assert!(result >= 0.0);
    }

    #[test]
    fn test_compute_uncertainty_empty() {
        assert!(scalar_eq(compute_uncertainty(&[]), 1.0));
    }

    #[test]
    fn test_compute_uncertainty_uniform() {
        let items = vec![
            make_evidence(1, 0.5, EvidencePolarity::Supports, 0.9),
            make_evidence(2, 0.5, EvidencePolarity::Supports, 0.8),
        ];
        let result = compute_uncertainty(&items);
        assert!(scalar_eq(result, 0.0));
    }

    #[test]
    fn test_compute_uncertainty_variable() {
        let items = vec![
            make_evidence(1, 0.2, EvidencePolarity::Supports, 0.9),
            make_evidence(2, 0.9, EvidencePolarity::Supports, 0.8),
        ];
        let result = compute_uncertainty(&items);
        assert!(result > 0.0);
    }

    #[test]
    fn test_decay_factor_at_zero() {
        assert!(scalar_eq(compute_decay_factor(0), 1.0));
    }

    #[test]
    fn test_decay_factor_at_half_life() {
        let result = compute_decay_factor(DECAY_HALF_LIFE_SECS);
        assert!((result - 0.5).abs() < 0.05);
    }

    #[test]
    fn test_compute_overall_from_evidence_with_timestamp() {
        let evidence = vec![
            make_evidence(1, 0.9, EvidencePolarity::Supports, 0.9),
            make_evidence(2, 0.85, EvidencePolarity::Supports, 0.85),
            make_evidence(3, 0.8, EvidencePolarity::Supports, 0.8),
        ];
        let result = compute_overall_from_evidence(&evidence, Timestamp::now());
        assert!(result.belief > 0.0);
        assert!(result.evidence_strength > 0.0);
        assert!(result.source_quality > 0.0);
    }

    #[test]
    fn test_compute_overall_from_evidence_old_timestamp() {
        let evidence = vec![
            make_evidence(1, 0.9, EvidencePolarity::Supports, 0.9),
            make_evidence(2, 0.85, EvidencePolarity::Supports, 0.85),
        ];
        let old_time = Timestamp(Timestamp::now().0.saturating_sub(DECAY_HALF_LIFE_SECS * 10 * 1000));
        let result = compute_overall_from_evidence(&evidence, old_time);
        assert!(result.belief < 0.01);
    }

    #[test]
    fn test_update_confidence() {
        let existing = ConfidenceState {
            belief: 0.3,
            evidence_strength: 0.4,
            source_quality: 0.6,
            consistency: 0.7,
            uncertainty: 0.3,
            prediction_reliability: 0.0,
            verification_status: VerificationStatus::Provisional,
        };
        let new_evidence: Vec<Evidence> = (0..50)
            .map(|i| make_evidence(i, 0.9, EvidencePolarity::Supports, 0.9))
            .collect();
        let updated = update_confidence(&existing, &new_evidence);
        assert!(updated.belief > existing.belief);
        assert!(updated.evidence_strength > existing.evidence_strength);
        assert!(updated.source_quality > existing.source_quality);
    }

    #[test]
    fn test_update_confidence_with_contradiction() {
        let existing = ConfidenceState {
            belief: 0.8,
            evidence_strength: 0.7,
            source_quality: 0.8,
            consistency: 0.9,
            uncertainty: 0.1,
            prediction_reliability: 0.0,
            verification_status: VerificationStatus::Verified,
        };
        let new_evidence = vec![
            make_evidence(1, 0.9, EvidencePolarity::Contradicts, 0.9),
            make_evidence(2, 0.85, EvidencePolarity::Contradicts, 0.85),
        ];
        let updated = update_confidence(&existing, &new_evidence);
        assert!(updated.consistency < existing.consistency);
        assert!(updated.belief < existing.belief);
    }

    #[test]
    fn test_update_confidence_empty_evidence() {
        let existing = ConfidenceState {
            belief: 0.7,
            evidence_strength: 0.6,
            source_quality: 0.8,
            consistency: 0.9,
            uncertainty: 0.2,
            prediction_reliability: 0.0,
            verification_status: VerificationStatus::Supported,
        };
        let updated = update_confidence(&existing, &[]);
        assert!(updated.belief < existing.belief);
        assert!(updated.evidence_strength < existing.evidence_strength);
        assert!(updated.uncertainty > existing.uncertainty);
    }

    #[test]
    fn test_consistency_all_contradicting() {
        let items = vec![
            make_evidence(1, 0.8, EvidencePolarity::Contradicts, 0.9),
            make_evidence(2, 0.7, EvidencePolarity::Contradicts, 0.8),
        ];
        let result = compute_consistency(&items);
        assert!(result == 0.0);
    }

    #[test]
    fn test_compute_overall_with_evidence() {
        let mut evidence_set = EvidenceSet::new();
        evidence_set.add(make_evidence(1, 0.9, EvidencePolarity::Supports, 0.9));
        evidence_set.add(make_evidence(2, 0.85, EvidencePolarity::Supports, 0.85));
        evidence_set.add(make_evidence(3, 0.8, EvidencePolarity::Supports, 0.8));

        let claims = vec![Claim {
            id: ClaimId(1),
            text: "test claim".into(),
            status: VerificationStatus::Supported,
            confidence: ConfidenceState::default(),
            evidence: evidence_set,
            created_at: Timestamp::now(),
        }];
        let result = compute_overall(&claims);
        assert!(result.belief > 0.0);
        assert!(result.consistency > 0.8);
    }
}
