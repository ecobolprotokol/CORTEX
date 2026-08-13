use crate::types::scalars::Scalar;

#[derive(Debug, Clone)]
pub struct RiskAssessment {
    pub score: Scalar,
    pub level: RiskLevel,
    pub factors: Vec<RiskFactor>,
    pub mitigation_suggestions: Vec<String>,
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
    pub contribution: Scalar,
}

pub struct RiskEvaluator {
    pub weights: RiskWeights,
}

#[derive(Debug, Clone)]
pub struct RiskWeights {
    pub cost_weight: Scalar,
    pub uncertainty_weight: Scalar,
    pub complexity_weight: Scalar,
    pub reversibility_weight: Scalar,
    pub cascading_weight: Scalar,
}

impl Default for RiskWeights {
    fn default() -> Self {
        Self {
            cost_weight: 0.25,
            uncertainty_weight: 0.25,
            complexity_weight: 0.2,
            reversibility_weight: 0.15,
            cascading_weight: 0.15,
        }
    }
}

impl RiskEvaluator {
    pub fn new() -> Self {
        Self {
            weights: RiskWeights::default(),
        }
    }

    pub fn with_weights(weights: RiskWeights) -> Self {
        Self { weights }
    }

    pub fn evaluate(
        &self,
        plan_cost: Scalar,
        uncertainty: Scalar,
        complexity: Scalar,
        reversibility: Scalar,
        cascading: Scalar,
    ) -> RiskAssessment {
        let factors = vec![
            RiskFactor {
                name: "Cost".into(),
                weight: self.weights.cost_weight,
                value: plan_cost.min(1.0),
                contribution: plan_cost.min(1.0) * self.weights.cost_weight,
            },
            RiskFactor {
                name: "Uncertainty".into(),
                weight: self.weights.uncertainty_weight,
                value: uncertainty.min(1.0),
                contribution: uncertainty.min(1.0) * self.weights.uncertainty_weight,
            },
            RiskFactor {
                name: "Complexity".into(),
                weight: self.weights.complexity_weight,
                value: complexity.min(1.0),
                contribution: complexity.min(1.0) * self.weights.complexity_weight,
            },
            RiskFactor {
                name: "Irreversibility".into(),
                weight: self.weights.reversibility_weight,
                value: (1.0 - reversibility).clamp(0.0, 1.0),
                contribution: (1.0 - reversibility).clamp(0.0, 1.0) * self.weights.reversibility_weight,
            },
            RiskFactor {
                name: "Cascading".into(),
                weight: self.weights.cascading_weight,
                value: cascading.min(1.0),
                contribution: cascading.min(1.0) * self.weights.cascading_weight,
            },
        ];

        let score: Scalar = factors.iter().map(|f| f.contribution).sum();
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

        let mitigation_suggestions = self.generate_mitigations(&factors, level);

        RiskAssessment {
            score,
            level,
            factors,
            mitigation_suggestions,
        }
    }

    pub fn evaluate_simple(&self, plan_cost: Scalar, uncertainty: Scalar) -> RiskAssessment {
        self.evaluate(plan_cost, uncertainty, 0.3, 0.5, 0.2)
    }

    fn generate_mitigations(&self, factors: &[RiskFactor], level: RiskLevel) -> Vec<String> {
        let mut suggestions = Vec::new();

        for factor in factors {
            if factor.contribution > 0.2 {
                match factor.name.as_str() {
                    "Cost" => suggestions.push("Consider breaking into smaller steps".into()),
                    "Uncertainty" => suggestions.push("Gather more evidence before proceeding".into()),
                    "Complexity" => suggestions.push("Simplify the approach".into()),
                    "Irreversibility" => suggestions.push("Add rollback checkpoints".into()),
                    "Cascading" => suggestions.push("Isolate dependent components".into()),
                    _ => {}
                }
            }
        }

        if level == RiskLevel::Critical {
            suggestions.push("Consider alternative approach entirely".into());
        }

        suggestions
    }

    pub fn compare_plans(
        &self,
        costs: &[Scalar],
        uncertainties: &[Scalar],
    ) -> Vec<(usize, RiskAssessment)> {
        let mut results: Vec<(usize, RiskAssessment)> = costs
            .iter()
            .zip(uncertainties.iter())
            .enumerate()
            .map(|(i, (&cost, &unc))| {
                let assessment = self.evaluate_simple(cost, unc);
                (i, assessment)
            })
            .collect();

        results.sort_by(|a, b| {
            a.1.score
                .partial_cmp(&b.1.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        results
    }
}

impl Default for RiskEvaluator {
    fn default() -> Self {
        Self::new()
    }
}
