use crate::types::*;

pub fn evaluate_risk(plan: &Plan, _world: &crate::types::WorldState) -> RiskAssessment {
    let mut factors = Vec::new();
    let reversibility = 0.8;
    factors.push(RiskFactor {
        description: "Reversibility".into(),
        severity: 1.0 - reversibility,
        likelihood: 0.5,
    });
    factors.push(RiskFactor {
        description: "Uncertainty".into(),
        severity: plan.uncertainty,
        likelihood: 1.0,
    });
    let score = (1.0 - reversibility) * 0.3 + plan.uncertainty * 0.5;
    let level = match score {
        x if x > 0.8 => RiskLevel::Critical,
        x if x > 0.6 => RiskLevel::High,
        x if x > 0.3 => RiskLevel::Moderate,
        x if x > 0.0 => RiskLevel::Low,
        _ => RiskLevel::None,
    };
    RiskAssessment {
        score,
        level,
        factors,
        reversibility,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_risk() {
        let plan = Plan {
            id: PlanId(1),
            goal: GoalId(1),
            steps: Vec::new(),
            estimated_cost: 0.1,
            estimated_risk: 0.1,
            uncertainty: 0.3,
            confidence: 0.7,
            predicted_outcomes: Vec::new(),
        };
        let world = crate::types::WorldState {
            entities: Vec::new(),
            relations: Vec::new(),
            active_events: Vec::new(),
            temporal_context: crate::types::TemporalContext::default(),
            uncertainty: crate::types::UncertaintyState::initial(),
            next_entity_id: crate::types::EntityId(1),
            next_relation_id: crate::types::RelationId(1),
            next_event_id: crate::types::EventId(1),
        };
        let risk = evaluate_risk(&plan, &world);
        assert!(risk.score > 0.0);
        assert!(!risk.factors.is_empty());
    }
}
