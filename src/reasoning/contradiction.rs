use crate::types::ids::HypothesisId;
use crate::types::scalars::Scalar;
use crate::types::common::Timestamp;

#[derive(Debug, Clone)]
pub struct Contradiction {
    pub claim_a: HypothesisId,
    pub claim_b: HypothesisId,
    pub description: String,
    pub severity: Scalar,
    pub detected_at: Timestamp,
    pub resolved: bool,
}

pub struct ContradictionDetector {
    pub negation_patterns: Vec<String>,
    pub mutual_exclusion_pairs: Vec<(String, String)>,
}

impl ContradictionDetector {
    pub fn new() -> Self {
        Self {
            negation_patterns: vec![
                "not".into(),
                "no".into(),
                "never".into(),
                "neither".into(),
                "nor".into(),
                "cannot".into(),
                "impossible".into(),
            ],
            mutual_exclusion_pairs: vec![
                ("always".into(), "never".into()),
                ("true".into(), "false".into()),
                ("yes".into(), "no".into()),
                ("possible".into(), "impossible".into()),
                ("certain".into(), "uncertain".into()),
                ("increase".into(), "decrease".into()),
            ],
        }
    }

    pub fn detect(&self, propositions: &[(HypothesisId, String)]) -> Vec<Contradiction> {
        let mut contradictions = Vec::new();
        let now = Timestamp::now();

        for i in 0..propositions.len() {
            for j in (i + 1)..propositions.len() {
                let (id_a, prop_a) = &propositions[i];
                let (id_b, prop_b) = &propositions[j];

                if let Some(severity) = self.check_contradiction(prop_a, prop_b) {
                    contradictions.push(Contradiction {
                        claim_a: *id_a,
                        claim_b: *id_b,
                        description: format!("'{}' contradicts '{}'", prop_a, prop_b),
                        severity,
                        detected_at: now,
                        resolved: false,
                    });
                }
            }
        }

        contradictions
    }

    fn check_contradiction(&self, a: &str, b: &str) -> Option<Scalar> {
        let a_lower = a.to_lowercase();
        let b_lower = b.to_lowercase();

        if self.check_negation_pattern(&a_lower, &b_lower) {
            return Some(0.8);
        }

        if self.check_mutual_exclusion(&a_lower, &b_lower) {
            return Some(0.7);
        }

        if self.check_semantic_opposition(&a_lower, &b_lower) {
            return Some(0.6);
        }

        None
    }

    fn check_negation_pattern(&self, a: &str, b: &str) -> bool {
        for pattern in &self.negation_patterns {
            let negated_a = format!("{} {}", pattern, a);
            let negated_b = format!("{} {}", pattern, b);
            if (a.contains(&negated_b)) || (b.contains(&negated_a)) {
                return true;
            }
            if a.starts_with(pattern) && b.contains(&a[pattern.len()..].trim()) {
                return true;
            }
            if b.starts_with(pattern) && a.contains(&b[pattern.len()..].trim()) {
                return true;
            }
        }
        false
    }

    fn check_mutual_exclusion(&self, a: &str, b: &str) -> bool {
        for (term_a, term_b) in &self.mutual_exclusion_pairs {
            let a_has_first = a.contains(term_a.as_str());
            let a_has_second = a.contains(term_b.as_str());
            let b_has_first = b.contains(term_a.as_str());
            let b_has_second = b.contains(term_b.as_str());

            if (a_has_first && b_has_second) || (a_has_second && b_has_first) {
                return true;
            }
        }
        false
    }

    fn check_semantic_opposition(&self, a: &str, b: &str) -> bool {
        let opposite_pairs = [
            ("increase", "decrease"),
            ("rise", "fall"),
            ("grow", "shrink"),
            ("more", "less"),
            ("higher", "lower"),
            ("better", "worse"),
            ("positive", "negative"),
            ("active", "inactive"),
            ("enable", "disable"),
            ("add", "remove"),
        ];

        for (term_a, term_b) in &opposite_pairs {
            if (a.contains(term_a) && b.contains(term_b))
                || (a.contains(term_b) && b.contains(term_a))
            {
                return true;
            }
        }
        false
    }

    pub fn resolve_contradiction(&self, _contradiction: &mut Contradiction) {
        _contradiction.resolved = true;
    }

    pub fn get_severity_level(severity: Scalar) -> &'static str {
        if severity >= 0.8 {
            "Critical"
        } else if severity >= 0.6 {
            "High"
        } else if severity >= 0.4 {
            "Moderate"
        } else {
            "Low"
        }
    }
}

impl Default for ContradictionDetector {
    fn default() -> Self {
        Self::new()
    }
}
