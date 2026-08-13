use crate::types::scalars::Scalar;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSource {
    Input,
    Memory,
    World,
    Reasoning,
    Procedure,
    Environment,
}

#[derive(Debug, Clone)]
pub struct ErrorAttribution {
    pub source: ErrorSource,
    pub magnitude: Scalar,
    pub confidence: Scalar,
}

pub struct AttributionEngine;

impl AttributionEngine {
    pub fn new() -> Self { Self }

    pub fn attribute(&self, error: Scalar, context: &str) -> ErrorAttribution {
        let source = if context.contains("input") {
            ErrorSource::Input
        } else if context.contains("memory") {
            ErrorSource::Memory
        } else if context.contains("world") {
            ErrorSource::World
        } else if context.contains("reason") {
            ErrorSource::Reasoning
        } else if context.contains("procedure") {
            ErrorSource::Procedure
        } else {
            ErrorSource::Environment
        };

        ErrorAttribution {
            source,
            magnitude: error,
            confidence: 0.5,
        }
    }
}
