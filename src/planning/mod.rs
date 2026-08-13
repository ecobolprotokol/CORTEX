pub mod plan;
pub mod risk;

use crate::config::PlanningConfig;
use crate::error::Result;
use crate::types::*;

pub trait PlanningEngine {
    fn evaluate(&mut self, reasoning: &ReasoningResult, world: &WorldState) -> Result<Option<Plan>>;
    fn state(&self) -> &PlanningState;
}

pub struct PlanningEngineImpl {
    config: PlanningConfig,
    state: PlanningState,
}

impl PlanningEngineImpl {
    pub fn new(config: &PlanningConfig) -> Result<Self> {
        Ok(Self {
            config: config.clone(),
            state: PlanningState {
                active_goals: Vec::new(),
                candidate_plans: Vec::new(),
                selected_plan: None,
                budget_remaining: config.max_depth,
                simulation_count: 0,
                next_plan_id: PlanId(1),
                next_goal_id: GoalId(1),
            },
        })
    }
}

impl PlanningEngine for PlanningEngineImpl {
    fn evaluate(&mut self, reasoning: &ReasoningResult, _world: &WorldState) -> Result<Option<Plan>> {
        if !self.config.enabled {
            return Ok(None);
        }
        if reasoning.conclusion.is_none() {
            return Ok(None);
        }
        let goal_id = self.state.next_goal_id;
        self.state.next_goal_id = goal_id.next();
        let plan_id = self.state.next_plan_id;
        self.state.next_plan_id = plan_id.next();
        let plan = Plan {
            id: plan_id,
            goal: goal_id,
            steps: Vec::new(),
            estimated_cost: 0.1,
            estimated_risk: 0.1,
            uncertainty: 0.3,
            confidence: reasoning.conclusion.as_ref().map(|c| c.confidence).unwrap_or(0.0) * 0.8,
            predicted_outcomes: Vec::new(),
        };
        self.state.selected_plan = Some(plan.clone());
        Ok(Some(plan))
    }

    fn state(&self) -> &PlanningState {
        &self.state
    }
}
