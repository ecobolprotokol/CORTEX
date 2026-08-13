use crate::types::*;

pub fn compute_overall(claims: &[Claim]) -> ConfidenceState {
    if claims.is_empty() {
        return ConfidenceState::default();
    }
    let avg_belief: f32 = claims.iter().map(|c| c.confidence.belief).sum::<f32>() / claims.len() as f32;
    let avg_evidence: f32 = claims.iter().map(|c| c.confidence.evidence_strength).sum::<f32>() / claims.len() as f32;
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
