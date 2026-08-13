use crate::error::Result;
use crate::types::*;

pub fn generate(verified: &VerifiedResult) -> Result<GeneratedResponse> {
    let mut response_text = String::new();

    if let Some(conclusion) = &verified.reasoning_result {
        if let Some(conc) = &conclusion.conclusion {
            let confidence_pct = (conc.confidence * 100.0) as u32;
            match verified.verification_status {
                VerificationStatus::Verified => {
                    response_text = format!(
                        "Based on verified knowledge ({}% confidence): {}",
                        confidence_pct, conc.proposition.predicate
                    );
                }
                VerificationStatus::Supported => {
                    response_text = format!(
                        "Based on supported evidence ({}% confidence): {}",
                        confidence_pct, conc.proposition.predicate
                    );
                }
                VerificationStatus::Provisional => {
                    response_text = format!(
                        "Based on available evidence ({}% confidence): {}",
                        confidence_pct, conc.proposition.predicate
                    );
                }
                _ => {
                    response_text = format!(
                        "Reasoning suggests ({}% confidence): {}",
                        confidence_pct, conc.proposition.predicate
                    );
                }
            }
        } else if !conclusion.hypotheses.is_empty() {
            let top = &conclusion.hypotheses[0];
            response_text = format!(
                "Analysis suggests: {} (confidence: {:.0}%)",
                top.proposition.predicate,
                top.confidence * 100.0
            );
        } else {
            response_text = generate_from_claims(&verified.claims);
        }
    } else {
        response_text = generate_from_claims(&verified.claims);
    }

    if response_text.is_empty() {
        response_text = "I have processed your input and recorded the observation.".to_string();
    }

    Ok(GeneratedResponse {
        text: response_text,
        confidence: verified.overall_confidence.overall(),
        verification_status: verified.verification_status,
    })
}

fn generate_from_claims(claims: &[Claim]) -> String {
    if claims.is_empty() {
        return "I have processed your input and recorded the observation.".to_string();
    }
    let top = &claims[0];
    match top.status {
        VerificationStatus::Verified => {
            format!("Verified: {} (confidence: {:.0}%)", top.text, top.confidence.belief * 100.0)
        }
        VerificationStatus::Supported => {
            format!("Supported: {} (confidence: {:.0}%)", top.text, top.confidence.belief * 100.0)
        }
        VerificationStatus::Provisional => {
            format!("Provisional understanding: {} (confidence: {:.0}%)", top.text, top.confidence.belief * 100.0)
        }
        VerificationStatus::Observed => {
            format!("Observed: {}", top.text)
        }
        _ => {
            format!("Analysis: {}", top.text)
        }
    }
}
