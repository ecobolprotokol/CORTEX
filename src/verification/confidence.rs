use crate::types::*;

pub fn compute_overall(claims: &[Claim]) -> ConfidenceState {
    if claims.is_empty() {
        return ConfidenceState::default();
    }
    let avg_belief: Scalar = claims.iter().map(|c| c.confidence.belief).sum::<Scalar>() / claims.len() as Scalar;
    let avg_evidence: Scalar = claims.iter().map(|c| c.confidence.evidence_strength).sum::<Scalar>() / claims.len() as Scalar;
    ConfidenceState {
        belief: avg_belief,
        evidence_strength: avg_evidence,
        source_quality: 0.5,
        consistency: 0.5,
        uncertainty: 1.0 - avg_belief,
        prediction_reliability: 0.0,
        verification_status: if avg_belief >= 0.8 {
            VerificationStatus::Verified
        } else if avg_belief >= 0.5 {
            VerificationStatus::Supported
        } else {
            VerificationStatus::Provisional
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(result.verification_status, VerificationStatus::Verified);
        assert!(result.belief > 0.8);
    }
}
