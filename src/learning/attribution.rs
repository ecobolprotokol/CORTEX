use crate::types::scalars::Scalar;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorSource {
    Input,
    Memory,
    World,
    Reasoning,
    Procedure,
    Environment,
}

impl std::fmt::Display for ErrorSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorSource::Input => write!(f, "Input"),
            ErrorSource::Memory => write!(f, "Memory"),
            ErrorSource::World => write!(f, "World"),
            ErrorSource::Reasoning => write!(f, "Reasoning"),
            ErrorSource::Procedure => write!(f, "Procedure"),
            ErrorSource::Environment => write!(f, "Environment"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ErrorAttribution {
    pub source: ErrorSource,
    pub magnitude: Scalar,
    pub confidence: Scalar,
    pub contributing_factors: Vec<String>,
}

pub struct AttributionEngine {
    pub keyword_mapping: HashMap<String, ErrorSource>,
    pub source_reliability: HashMap<ErrorSource, Scalar>,
}

impl AttributionEngine {
    pub fn new() -> Self {
        let mut keyword_mapping = HashMap::new();
        keyword_mapping.insert("input".into(), ErrorSource::Input);
        keyword_mapping.insert("user".into(), ErrorSource::Input);
        keyword_mapping.insert("text".into(), ErrorSource::Input);
        keyword_mapping.insert("memory".into(), ErrorSource::Memory);
        keyword_mapping.insert("recall".into(), ErrorSource::Memory);
        keyword_mapping.insert("retrieve".into(), ErrorSource::Memory);
        keyword_mapping.insert("world".into(), ErrorSource::World);
        keyword_mapping.insert("state".into(), ErrorSource::World);
        keyword_mapping.insert("entity".into(), ErrorSource::World);
        keyword_mapping.insert("reason".into(), ErrorSource::Reasoning);
        keyword_mapping.insert("logic".into(), ErrorSource::Reasoning);
        keyword_mapping.insert("inference".into(), ErrorSource::Reasoning);
        keyword_mapping.insert("procedure".into(), ErrorSource::Procedure);
        keyword_mapping.insert("action".into(), ErrorSource::Procedure);
        keyword_mapping.insert("execute".into(), ErrorSource::Procedure);
        keyword_mapping.insert("environment".into(), ErrorSource::Environment);
        keyword_mapping.insert("external".into(), ErrorSource::Environment);
        keyword_mapping.insert("network".into(), ErrorSource::Environment);

        let mut source_reliability = HashMap::new();
        source_reliability.insert(ErrorSource::Input, 0.8);
        source_reliability.insert(ErrorSource::Memory, 0.7);
        source_reliability.insert(ErrorSource::World, 0.6);
        source_reliability.insert(ErrorSource::Reasoning, 0.75);
        source_reliability.insert(ErrorSource::Procedure, 0.65);
        source_reliability.insert(ErrorSource::Environment, 0.5);

        Self {
            keyword_mapping,
            source_reliability,
        }
    }

    pub fn attribute(&self, error: Scalar, context: &str) -> ErrorAttribution {
        let source = self.detect_source(context);
        let reliability = self.source_reliability.get(&source).copied().unwrap_or(0.5);

        let contributing_factors = self.extract_factors(context);

        ErrorAttribution {
            source,
            magnitude: error,
            confidence: reliability * (1.0 - error * 0.3),
            contributing_factors,
        }
    }

    fn detect_source(&self, context: &str) -> ErrorSource {
        let lower = context.to_lowercase();
        let mut scores: HashMap<ErrorSource, Scalar> = HashMap::new();

        for (keyword, source) in &self.keyword_mapping {
            if lower.contains(keyword.as_str()) {
                *scores.entry(*source).or_insert(0.0) += 1.0;
            }
        }

        scores
            .into_iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(source, _)| source)
            .unwrap_or(ErrorSource::Environment)
    }

    fn extract_factors(&self, context: &str) -> Vec<String> {
        let words: Vec<&str> = context.split_whitespace().collect();
        words
            .iter()
            .filter(|w| w.len() > 3)
            .take(5)
            .map(|w| w.to_string())
            .collect()
    }

    pub fn attribute_batch(&self, errors: &[(Scalar, &str)]) -> Vec<ErrorAttribution> {
        errors
            .iter()
            .map(|(magnitude, context)| self.attribute(*magnitude, context))
            .collect()
    }

    pub fn compute_source_distribution(
        &self,
        attributions: &[ErrorAttribution],
    ) -> HashMap<ErrorSource, Scalar> {
        let mut distribution = HashMap::new();
        let total: Scalar = attributions.iter().map(|a| a.magnitude).sum();

        if total < crate::types::scalars::SCALAR_EPSILON {
            return distribution;
        }

        for attr in attributions {
            *distribution.entry(attr.source).or_insert(0.0) += attr.magnitude / total;
        }

        distribution
    }
}

impl Default for AttributionEngine {
    fn default() -> Self {
        Self::new()
    }
}
