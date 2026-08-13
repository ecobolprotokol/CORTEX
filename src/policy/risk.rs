use crate::types::scalars::Scalar;

#[derive(Debug, Clone)]
pub struct RiskEstimate {
    pub score: Scalar,
    pub level: RiskLevel,
    pub factors: Vec<RiskFactor>,
    pub overall_assessment: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskLevel {
    Low,
    Moderate,
    High,
    Critical,
}

impl std::fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RiskLevel::Low => write!(f, "Low"),
            RiskLevel::Moderate => write!(f, "Moderate"),
            RiskLevel::High => write!(f, "High"),
            RiskLevel::Critical => write!(f, "Critical"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RiskFactor {
    pub name: String,
    pub weight: Scalar,
    pub value: Scalar,
}

pub struct RiskEstimator {
    pub factors: Vec<RiskFactor>,
}

impl RiskEstimator {
    pub fn new() -> Self {
        Self {
            factors: vec![
                RiskFactor {
                    name: "Impact".into(),
                    weight: 0.3,
                    value: 0.0,
                },
                RiskFactor {
                    name: "Reversibility".into(),
                    weight: 0.25,
                    value: 0.0,
                },
                RiskFactor {
                    name: "Scope".into(),
                    weight: 0.2,
                    value: 0.0,
                },
                RiskFactor {
                    name: "Duration".into(),
                    weight: 0.15,
                    value: 0.0,
                },
                RiskFactor {
                    name: "Dependencies".into(),
                    weight: 0.1,
                    value: 0.0,
                },
            ],
        }
    }

    pub fn estimate(
        &self,
        operation: &str,
        impact: Scalar,
        reversibility: Scalar,
    ) -> RiskEstimate {
        let scope = self.assess_scope(operation);
        let duration = self.assess_duration(operation);
        let dependencies = self.assess_dependencies(operation);

        let factors = vec![
            RiskFactor {
                name: "Impact".into(),
                weight: 0.3,
                value: impact.min(1.0),
            },
            RiskFactor {
                name: "Reversibility".into(),
                weight: 0.25,
                value: (1.0 - reversibility).clamp(0.0, 1.0),
            },
            RiskFactor {
                name: "Scope".into(),
                weight: 0.2,
                value: scope,
            },
            RiskFactor {
                name: "Duration".into(),
                weight: 0.15,
                value: duration,
            },
            RiskFactor {
                name: "Dependencies".into(),
                weight: 0.1,
                value: dependencies,
            },
        ];

        let score: Scalar = factors.iter().map(|f| f.weight * f.value).sum();
        let score = score.min(1.0);

        let level = if score < 0.25 {
            RiskLevel::Low
        } else if score < 0.5 {
            RiskLevel::Moderate
        } else if score < 0.75 {
            RiskLevel::High
        } else {
            RiskLevel::Critical
        };

        let overall_assessment = format!(
            "Operation '{}' risk: {} ({:.2})",
            operation, level, score
        );

        RiskEstimate {
            score,
            level,
            factors,
            overall_assessment,
        }
    }

    pub fn estimate_five_factor(
        &self,
        impact: Scalar,
        reversibility: Scalar,
        scope: Scalar,
        duration: Scalar,
        dependencies: Scalar,
    ) -> RiskEstimate {
        let factors = vec![
            RiskFactor {
                name: "Impact".into(),
                weight: 0.3,
                value: impact.min(1.0),
            },
            RiskFactor {
                name: "Reversibility".into(),
                weight: 0.25,
                value: (1.0 - reversibility).clamp(0.0, 1.0),
            },
            RiskFactor {
                name: "Scope".into(),
                weight: 0.2,
                value: scope.min(1.0),
            },
            RiskFactor {
                name: "Duration".into(),
                weight: 0.15,
                value: duration.min(1.0),
            },
            RiskFactor {
                name: "Dependencies".into(),
                weight: 0.1,
                value: dependencies.min(1.0),
            },
        ];

        let score: Scalar = factors.iter().map(|f| f.weight * f.value).sum();
        let score = score.min(1.0);

        let level = if score < 0.25 {
            RiskLevel::Low
        } else if score < 0.5 {
            RiskLevel::Moderate
        } else if score < 0.75 {
            RiskLevel::High
        } else {
            RiskLevel::Critical
        };

        RiskEstimate {
            score,
            level,
            factors,
            overall_assessment: format!("Risk: {} ({:.2})", level, score),
        }
    }

    fn assess_scope(&self, operation: &str) -> Scalar {
        let lower = operation.to_lowercase();
        if lower.contains("global") || lower.contains("all") {
            0.9
        } else if lower.contains("system") || lower.contains("state") {
            0.7
        } else if lower.contains("module") || lower.contains("subsystem") {
            0.5
        } else {
            0.3
        }
    }

    fn assess_duration(&self, operation: &str) -> Scalar {
        let lower = operation.to_lowercase();
        if lower.contains("permanent") || lower.contains("delete") || lower.contains("remove") {
            0.9
        } else if lower.contains("persistent") || lower.contains("save") {
            0.6
        } else if lower.contains("temporary") || lower.contains("cache") {
            0.3
        } else {
            0.4
        }
    }

    fn assess_dependencies(&self, operation: &str) -> Scalar {
        let lower = operation.to_lowercase();
        if lower.contains("cascade") || lower.contains("chain") {
            0.8
        } else if lower.contains("linked") || lower.contains("related") {
            0.5
        } else {
            0.2
        }
    }
}

impl Default for RiskEstimator {
    fn default() -> Self {
        Self::new()
    }
}
