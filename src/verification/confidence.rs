use crate::types::scalars::Scalar;
use crate::types::evidence::VerificationStatus;

#[derive(Debug, Clone)]
pub struct VerificationResult {
    pub claim: String,
    pub status: VerificationStatus,
    pub confidence: Scalar,
    pub evidence_count: u32,
}

pub struct ConfidenceModel {
    pub minimum_confidence: Scalar,
}

impl ConfidenceModel {
    pub fn new(minimum_confidence: Scalar) -> Self {
        Self { minimum_confidence }
    }

    pub fn verify(&self, claim: &str, evidence_strength: Scalar) -> VerificationResult {
        let status = if evidence_strength >= self.minimum_confidence {
            VerificationStatus::Verified
        } else if evidence_strength > 0.5 {
            VerificationStatus::Supported
        } else if evidence_strength > 0.0 {
            VerificationStatus::Provisional
        } else {
            VerificationStatus::Unknown
        };

        VerificationResult {
            claim: claim.to_string(),
            status,
            confidence: evidence_strength,
            evidence_count: 0,
        }
    }

    pub fn compute_overall_confidence(direct: Scalar, corroborated: Scalar, staleness: Scalar) -> Scalar {
        let base = direct * 0.6 + corroborated * 0.4;
        base * (1.0 - staleness)
    }
}
