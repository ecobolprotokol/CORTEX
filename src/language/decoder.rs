use crate::error::Result;
use crate::types::*;

pub fn generate(verified: &VerifiedResult) -> Result<GeneratedResponse> {
    let mut parts: Vec<String> = Vec::new();

    if let Some(reasoning) = &verified.reasoning_result {
        if let Some(conclusion) = &reasoning.conclusion {
            parts.push(generate_conclusion_response(
                &conclusion.proposition,
                conclusion.confidence,
                verified.verification_status,
            ));
        } else if !reasoning.hypotheses.is_empty() {
            parts.push(generate_hypothesis_response(&reasoning.hypotheses));
        }

        if !reasoning.contradictions.is_empty() {
            parts.push(generate_contradiction_report(&reasoning.contradictions));
        }
    }

    if parts.is_empty() && !verified.claims.is_empty() {
        parts.push(generate_claims_summary(&verified.claims));
    }

    if parts.is_empty() {
        parts.push("No information available to generate a response.".to_string());
    }

    let text = parts.join(" ");

    Ok(GeneratedResponse {
        text,
        confidence: verified.overall_confidence.overall(),
        verification_status: verified.verification_status,
    })
}

fn generate_conclusion_response(
    proposition: &Proposition,
    confidence: Scalar,
    status: VerificationStatus,
) -> String {
    let subject = format_id(&proposition.subject);
    let predicate = &proposition.predicate;
    let negation = if proposition.negated { "not " } else { "" };

    let certainty = match status {
        VerificationStatus::Verified => "Verified",
        VerificationStatus::Supported => "Supported by evidence",
        VerificationStatus::Provisional => "Provisionally",
        VerificationStatus::Observed => "Based on observation",
        VerificationStatus::Inferred => "Inferred",
        VerificationStatus::Contradicted => "Contradicted",
        VerificationStatus::Unknown => "Unknown",
    };

    let confidence_pct = (confidence * 100.0) as u32;

    if let Some(object) = &proposition.object {
        let object_str = format_id(object);
        let mut modifiers = String::new();
        for m in &proposition.modifiers {
            modifiers.push_str(&format!(" {} ", m));
        }
        format!(
            "{} {}{}{}{} {} with {}% confidence.",
            certainty, subject, negation, predicate, modifiers, object_str, confidence_pct
        )
    } else {
        let mut modifiers = String::new();
        for m in &proposition.modifiers {
            modifiers.push_str(&format!(" {} ", m));
        }
        format!(
            "{} {}{}{}{}with {}% confidence.",
            certainty, subject, negation, predicate, modifiers, confidence_pct
        )
    }
}

fn generate_hypothesis_response(hypotheses: &[Hypothesis]) -> String {
    let mut best: Option<&Hypothesis> = None;
    for h in hypotheses {
        match &best {
            None => best = Some(h),
            Some(b) if h.confidence > b.confidence => best = Some(h),
            _ => {}
        }
    }

    match best {
        Some(h) => {
            let subject = format_id(&h.proposition.subject);
            let predicate = &h.proposition.predicate;
            let negation = if h.proposition.negated { "not " } else { "" };
            let confidence_pct = (h.confidence * 100.0) as u32;

            if let Some(object) = &h.proposition.object {
                let object_str = format_id(object);
                format!(
                    "Analysis suggests that {} {}{}{} {} (confidence: {}%).",
                    subject, negation, predicate, object_str, "", confidence_pct
                )
            } else {
                format!(
                    "Analysis suggests that {} {}{} (confidence: {}%).",
                    subject, negation, predicate, confidence_pct
                )
            }
        }
        None => "Multiple hypotheses were considered but none reached sufficient confidence.".to_string(),
    }
}

fn generate_contradiction_report(contradictions: &[Contradiction]) -> String {
    let mut reports: Vec<String> = Vec::new();

    let mut unresolved_count = 0;
    let mut resolved_count = 0;

    for c in contradictions {
        if c.resolved {
            resolved_count += 1;
        } else {
            unresolved_count += 1;
            let severity_pct = (c.severity * 100.0) as u32;
            reports.push(format!(
                "Contradiction detected (severity {}%): {}",
                severity_pct, c.description
            ));
        }
    }

    if reports.is_empty() {
        if resolved_count > 0 {
            return format!(
                "{} contradiction(s) were detected and resolved.",
                resolved_count
            );
        }
        return String::new();
    }

    let header = format!(
        "Note: {} contradiction(s) detected{}.",
        unresolved_count,
        if resolved_count > 0 {
            format!(", {} resolved", resolved_count)
        } else {
            String::new()
        }
    );

    reports.insert(0, header);
    reports.join(" ")
}

fn generate_claims_summary(claims: &[Claim]) -> String {
    if claims.is_empty() {
        return "No claims to summarize.".to_string();
    }

    let mut verified_claims: Vec<&Claim> = Vec::new();
    let mut supported_claims: Vec<&Claim> = Vec::new();
    let mut other_claims: Vec<&Claim> = Vec::new();

    for claim in claims {
        match claim.status {
            VerificationStatus::Verified => verified_claims.push(claim),
            VerificationStatus::Supported => supported_claims.push(claim),
            _ => other_claims.push(claim),
        }
    }

    let mut parts: Vec<String> = Vec::new();

    for claim in verified_claims.iter().take(3) {
        let pct = (claim.confidence.belief * 100.0) as u32;
        parts.push(format!("Verified: {} ({}% confidence)", claim.text, pct));
    }

    for claim in supported_claims.iter().take(3) {
        let pct = (claim.confidence.belief * 100.0) as u32;
        parts.push(format!("Supported: {} ({}% confidence)", claim.text, pct));
    }

    for claim in other_claims.iter().take(2) {
        let status_label = match claim.status {
            VerificationStatus::Provisional => "Provisional",
            VerificationStatus::Observed => "Observed",
            VerificationStatus::Inferred => "Inferred",
            VerificationStatus::Contradicted => "Contradicted",
            _ => "Pending",
        };
        let pct = (claim.confidence.belief * 100.0) as u32;
        parts.push(format!("{}: {} ({}% confidence)", status_label, claim.text, pct));
    }

    if parts.is_empty() {
        "Claims were processed but no summary could be generated.".to_string()
    } else {
        parts.join(". ")
    }
}

fn format_id(id: &InternalId) -> String {
    match id {
        InternalId::Entity(eid) => format!("entity_{}", eid.0),
        InternalId::Concept(cid) => format!("concept_{}", cid.0),
        InternalId::Episode(eid) => format!("episode_{}", eid.0),
        InternalId::Hypothesis(hid) => format!("hypothesis_{}", hid.0),
        InternalId::Procedure(pid) => format!("procedure_{}", pid.0),
        InternalId::Association(aid) => format!("association_{}", aid.0),
        InternalId::Cell(cid) => format!("cell_{}", cid.0),
        InternalId::Column(cid) => format!("column_{}", cid.0),
        InternalId::Symbol(sid) => format!("symbol_{}", sid.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_claim(text: &str, status: VerificationStatus, belief: Scalar) -> Claim {
        Claim {
            id: ClaimId(1),
            text: text.to_string(),
            status,
            confidence: ConfidenceState {
                belief,
                ..ConfidenceState::default()
            },
            evidence: EvidenceSet::new(),
            created_at: Timestamp::now(),
        }
    }

    fn make_proposition(subject: u64, predicate: &str, object: Option<u64>) -> Proposition {
        Proposition {
            subject: InternalId::Entity(EntityId(subject)),
            predicate: predicate.to_string(),
            object: object.map(|o| InternalId::Entity(EntityId(o))),
            modifiers: Vec::new(),
            negated: false,
        }
    }

    fn make_verified_result(
        claims: Vec<Claim>,
        status: VerificationStatus,
        reasoning: Option<ReasoningResult>,
    ) -> VerifiedResult {
        VerifiedResult {
            overall_confidence: ConfidenceState::default(),
            verification_status: status,
            reasoning_result: reasoning,
            claims,
        }
    }

    #[test]
    fn test_empty_verified_result() {
        let vr = make_verified_result(vec![], VerificationStatus::Unknown, None);
        let response = generate(&vr).unwrap();
        assert!(response.text.contains("No information available"));
        assert_eq!(response.verification_status, VerificationStatus::Unknown);
    }

    #[test]
    fn test_conclusion_generation() {
        let prop = make_proposition(1, "is a type of", Some(2));
        let conclusion = Conclusion {
            hypothesis_id: HypothesisId(1),
            proposition: prop,
            confidence: 0.92,
            evidence_strength: 0.85,
            reasoning_steps: 3,
            bounded: false,
        };
        let reasoning = ReasoningResult {
            hypotheses: vec![],
            contradictions: vec![],
            budget_remaining: 50,
            conclusion: Some(conclusion),
        };
        let vr = make_verified_result(vec![], VerificationStatus::Verified, Some(reasoning));
        let response = generate(&vr).unwrap();
        assert!(response.text.contains("Verified"));
        assert!(response.text.contains("is a type of"));
        assert!(response.text.contains("92% confidence"));
    }

    #[test]
    fn test_hypothesis_fallback() {
        let prop = make_proposition(1, "causes", Some(2));
        let hypothesis = Hypothesis {
            id: HypothesisId(1),
            proposition: prop,
            evidence: EvidenceSet::new(),
            counter_evidence: EvidenceSet::new(),
            confidence: 0.75,
            dependencies: vec![],
            contradictions: vec![],
            provenance: vec![],
            reasoning_type: ReasoningType::Inductive,
            created_at: Timestamp::now(),
        };
        let reasoning = ReasoningResult {
            hypotheses: vec![hypothesis],
            contradictions: vec![],
            budget_remaining: 30,
            conclusion: None,
        };
        let vr = make_verified_result(vec![], VerificationStatus::Provisional, Some(reasoning));
        let response = generate(&vr).unwrap();
        assert!(response.text.contains("Analysis suggests"));
        assert!(response.text.contains("confidence: 75%"));
    }

    #[test]
    fn test_contradiction_report() {
        let contradictions = vec![Contradiction {
            claim_a: HypothesisId(1),
            claim_b: HypothesisId(2),
            description: "conflicting evidence on topic X".to_string(),
            severity: 0.8,
            detected_at: Timestamp::now(),
            resolved: false,
        }];
        let reasoning = ReasoningResult {
            hypotheses: vec![],
            contradictions,
            budget_remaining: 20,
            conclusion: None,
        };
        let vr = make_verified_result(vec![], VerificationStatus::Contradicted, Some(reasoning));
        let response = generate(&vr).unwrap();
        assert!(response.text.contains("Contradiction detected"));
        assert!(response.text.contains("80%"));
    }

    #[test]
    fn test_claims_summary() {
        let claims = vec![
            make_claim("Water boils at 100C", VerificationStatus::Verified, 0.95),
            make_claim("Ice floats", VerificationStatus::Supported, 0.88),
            make_claim("Rocks are heavy", VerificationStatus::Provisional, 0.6),
        ];
        let vr = make_verified_result(claims, VerificationStatus::Supported, None);
        let response = generate(&vr).unwrap();
        assert!(response.text.contains("Verified:"));
        assert!(response.text.contains("95% confidence"));
        assert!(response.text.contains("Supported:"));
    }

    #[test]
    fn test_negated_proposition() {
        let mut prop = make_proposition(1, "is soluble in water", None);
        prop.negated = true;
        let conclusion = Conclusion {
            hypothesis_id: HypothesisId(1),
            proposition: prop,
            confidence: 0.8,
            evidence_strength: 0.7,
            reasoning_steps: 2,
            bounded: false,
        };
        let reasoning = ReasoningResult {
            hypotheses: vec![],
            contradictions: vec![],
            budget_remaining: 40,
            conclusion: Some(conclusion),
        };
        let vr = make_verified_result(vec![], VerificationStatus::Inferred, Some(reasoning));
        let response = generate(&vr).unwrap();
        assert!(response.text.contains("not"));
        assert!(response.text.contains("80% confidence"));
    }

    #[test]
    fn test_confidence_propagation() {
        let vr = make_verified_result(vec![], VerificationStatus::Verified, None);
        let response = generate(&vr).unwrap();
        let expected = ConfidenceState::default().overall();
        assert!((response.confidence - expected).abs() < 0.001);
    }
}
