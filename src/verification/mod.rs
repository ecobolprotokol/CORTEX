pub mod confidence;

pub use confidence::{ConfidenceModel, VerificationResult};

use crate::error::CortexError;
use crate::types::evidence::{EvidenceSet, VerificationStatus};
use crate::types::scalars::Scalar;
use crate::types::state::KnowledgeClaim;

pub trait VerificationEngine {
    fn verify(&self, claim: &str) -> Result<VerificationResult, CortexError>;
    fn minimum_confidence(&self) -> f32;
}

pub struct VerificationPipeline {
    confidence_model: ConfidenceModel,
}

impl VerificationPipeline {
    pub fn new(minimum_confidence: Scalar) -> Self {
        Self {
            confidence_model: ConfidenceModel::new(minimum_confidence),
        }
    }

    pub fn verify_claim(&self, claim: &KnowledgeClaim) -> VerificationResult {
        let evidence_quality =
            crate::reasoning::evidence::EvidenceEvaluator::evaluate_evidence_quality(
                &claim.evidence.items,
            );

        let supporting = claim.evidence.supporting().len() as Scalar;
        let contradicting = claim.counter_evidence.contradicting().len() as Scalar;
        let evidence_balance = if supporting + contradicting > 0.0 {
            supporting / (supporting + contradicting)
        } else {
            0.0
        };

        let consistency = claim.confidence.consistency;
        let source_quality = claim.confidence.source_quality;

        let overall = evidence_quality * 0.3
            + evidence_balance * 0.25
            + consistency * 0.25
            + source_quality * 0.2;

        let status = self.classify_status(overall, &claim.counter_evidence);

        VerificationResult {
            claim: format!(
                "{} {} {}",
                claim.proposition.subject,
                claim.proposition.predicate,
                claim.proposition.object.as_deref().unwrap_or("")
            ),
            status,
            confidence: overall,
            evidence_count: (supporting + contradicting) as u32,
        }
    }

    fn classify_status(
        &self,
        confidence: Scalar,
        counter_evidence: &EvidenceSet,
    ) -> VerificationStatus {
        if counter_evidence.len() > 3 {
            return VerificationStatus::Contradicted;
        }

        if confidence >= self.confidence_model.minimum_confidence {
            VerificationStatus::Verified
        } else if confidence >= 0.6 {
            VerificationStatus::Supported
        } else if confidence >= 0.3 {
            VerificationStatus::Provisional
        } else if confidence > 0.0 {
            VerificationStatus::Unknown
        } else {
            VerificationStatus::Observed
        }
    }

    pub fn batch_verify(&self, claims: &[KnowledgeClaim]) -> Vec<VerificationResult> {
        claims.iter().map(|c| self.verify_claim(c)).collect()
    }

    pub fn summary_stats(&self, results: &[VerificationResult]) -> VerificationSummary {
        let total = results.len() as u32;
        let verified = results
            .iter()
            .filter(|r| r.status == VerificationStatus::Verified)
            .count() as u32;
        let supported = results
            .iter()
            .filter(|r| r.status == VerificationStatus::Supported)
            .count() as u32;
        let contradicted = results
            .iter()
            .filter(|r| r.status == VerificationStatus::Contradicted)
            .count() as u32;

        let avg_confidence = if total > 0 {
            results.iter().map(|r| r.confidence).sum::<Scalar>() / total as Scalar
        } else {
            0.0
        };

        VerificationSummary {
            total,
            verified,
            supported,
            contradicted,
            avg_confidence,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VerificationSummary {
    pub total: u32,
    pub verified: u32,
    pub supported: u32,
    pub contradicted: u32,
    pub avg_confidence: Scalar,
}
