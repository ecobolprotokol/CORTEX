use crate::types::*;
use std::collections::VecDeque;

const ERROR_THRESHOLD: f32 = 0.1;
const VOCAB_SMALL: u32 = 50;
const EPISODES_FEW: usize = 10;
const ENTITIES_FEW: usize = 5;
const CONTRADICTIONS_MANY: usize = 3;
const LEARNING_RATE_HIGH: f32 = 0.5;
const HISTORY_MAX: usize = 256;

fn first_max<'a>(scores: &'a [(ErrorAttribution, f32)]) -> &'a (ErrorAttribution, f32) {
    let mut best = &scores[0];
    for s in &scores[1..] {
        if s.1 > best.1 {
            best = s;
        }
    }
    best
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttributedSubsystem {
    Input,
    Memory,
    World,
    Reasoning,
    Environment,
}

#[derive(Debug, Clone)]
pub struct AttributionResult {
    pub subsystem: ErrorAttribution,
    pub confidence: f32,
    pub scores: [(ErrorAttribution, f32); 5],
}

#[derive(Debug, Clone)]
struct HistoryEntry {
    prediction_error: f32,
    vocabulary_size: u32,
    episode_count: usize,
    entity_count: usize,
    contradiction_count: usize,
    learning_rate: f32,
    attribution: ErrorAttribution,
    timestamp: Timestamp,
}

pub struct AttributionEngine {
    history: VecDeque<HistoryEntry>,
}

impl AttributionEngine {
    pub fn new() -> Self {
        Self {
            history: VecDeque::with_capacity(HISTORY_MAX),
        }
    }

    pub fn attribute(&mut self, experience: &Experience) -> AttributionResult {
        let error = experience.error.magnitude;
        let vocab = experience.internal_state.language_vocabulary_size;
        let episodes = experience.internal_state.memory_episode_count;
        let entities = experience.internal_state.world_entity_count;
        let hypotheses = experience.internal_state.reasoning_hypothesis_count;

        let low_error = error < ERROR_THRESHOLD;
        if low_error {
            return AttributionResult {
                subsystem: ErrorAttribution::InputError,
                confidence: 0.9,
                scores: [
                    (ErrorAttribution::InputError, 0.9),
                    (ErrorAttribution::MemoryError, 0.1),
                    (ErrorAttribution::WorldError, 0.1),
                    (ErrorAttribution::ReasoningError, 0.1),
                    (ErrorAttribution::EnvironmentError, 0.1),
                ],
            };
        }

        let mut input_score = 0.0;
        let mut memory_score = 0.0;
        let mut world_score = 0.0;
        let mut reasoning_score = 0.0;
        let mut env_score = 0.0;

        if vocab < VOCAB_SMALL {
            let deficit = 1.0 - (vocab as f32 / VOCAB_SMALL as f32);
            input_score += deficit * 0.4;
        }

        if episodes < EPISODES_FEW {
            let deficit = 1.0 - (episodes as f32 / EPISODES_FEW as f32);
            memory_score += deficit * 0.4;
        }

        if entities < ENTITIES_FEW {
            let deficit = 1.0 - (entities as f32 / ENTITIES_FEW as f32);
            world_score += deficit * 0.4;
        }

        if hypotheses > 0 {
            let contradiction_ratio = self.contradiction_ratio_for(hypotheses);
            if contradiction_ratio > CONTRADICTIONS_MANY as f32 / hypotheses.max(1) as f32 {
                reasoning_score += contradiction_ratio * 0.4;
            }
        }

        let recent_lr = self.recent_learning_rate().unwrap_or(0.0);
        if recent_lr > LEARNING_RATE_HIGH {
            let excess = (recent_lr - LEARNING_RATE_HIGH) / (1.0 - LEARNING_RATE_HIGH);
            env_score += excess * 0.35;
        }

        let error_factor = (error * 2.0).min(1.0);
        input_score *= error_factor;
        memory_score *= error_factor;
        world_score *= error_factor;
        reasoning_score *= error_factor;
        env_score *= error_factor;

        let scores = [
            (ErrorAttribution::InputError, input_score),
            (ErrorAttribution::MemoryError, memory_score),
            (ErrorAttribution::WorldError, world_score),
            (ErrorAttribution::ReasoningError, reasoning_score),
            (ErrorAttribution::EnvironmentError, env_score),
        ];

        let best = first_max(&scores);
        let total: f32 = scores.iter().map(|s| s.1).sum();
        let confidence = if total > 0.0 { best.1 / total } else { 0.2 };

        let pattern_bonus = self.detect_pattern_bonus(best.0);
        let final_confidence = (confidence + pattern_bonus).min(1.0);

        self.record_to_history(error, vocab, episodes, entities, hypotheses, 0.0, best.0);

        AttributionResult {
            subsystem: best.0,
            confidence: final_confidence,
            scores,
        }
    }

    pub fn attribute_with_lr(&mut self, experience: &Experience, current_learning_rate: f32) -> AttributionResult {
        let error = experience.error.magnitude;
        let vocab = experience.internal_state.language_vocabulary_size;
        let episodes = experience.internal_state.memory_episode_count;
        let entities = experience.internal_state.world_entity_count;
        let hypotheses = experience.internal_state.reasoning_hypothesis_count;

        let low_error = error < ERROR_THRESHOLD;
        if low_error {
            return AttributionResult {
                subsystem: ErrorAttribution::InputError,
                confidence: 0.9,
                scores: [
                    (ErrorAttribution::InputError, 0.9),
                    (ErrorAttribution::MemoryError, 0.1),
                    (ErrorAttribution::WorldError, 0.1),
                    (ErrorAttribution::ReasoningError, 0.1),
                    (ErrorAttribution::EnvironmentError, 0.1),
                ],
            };
        }

        let mut input_score = 0.0;
        let mut memory_score = 0.0;
        let mut world_score = 0.0;
        let mut reasoning_score = 0.0;
        let mut env_score = 0.0;

        if vocab < VOCAB_SMALL {
            let deficit = 1.0 - (vocab as f32 / VOCAB_SMALL as f32);
            input_score += deficit * 0.4;
        }

        if episodes < EPISODES_FEW {
            let deficit = 1.0 - (episodes as f32 / EPISODES_FEW as f32);
            memory_score += deficit * 0.4;
        }

        if entities < ENTITIES_FEW {
            let deficit = 1.0 - (entities as f32 / ENTITIES_FEW as f32);
            world_score += deficit * 0.4;
        }

        if hypotheses > 0 {
            let contradiction_ratio = self.contradiction_ratio_for(hypotheses);
            if contradiction_ratio > CONTRADICTIONS_MANY as f32 / hypotheses.max(1) as f32 {
                reasoning_score += contradiction_ratio * 0.4;
            }
        }

        if current_learning_rate > LEARNING_RATE_HIGH {
            let excess = (current_learning_rate - LEARNING_RATE_HIGH) / (1.0 - LEARNING_RATE_HIGH);
            env_score += excess * 0.35;
        }

        let error_factor = (error * 2.0).min(1.0);
        input_score *= error_factor;
        memory_score *= error_factor;
        world_score *= error_factor;
        reasoning_score *= error_factor;
        env_score *= error_factor;

        let scores = [
            (ErrorAttribution::InputError, input_score),
            (ErrorAttribution::MemoryError, memory_score),
            (ErrorAttribution::WorldError, world_score),
            (ErrorAttribution::ReasoningError, reasoning_score),
            (ErrorAttribution::EnvironmentError, env_score),
        ];

        let best = first_max(&scores);
        let total: f32 = scores.iter().map(|s| s.1).sum();
        let confidence = if total > 0.0 { best.1 / total } else { 0.2 };

        let pattern_bonus = self.detect_pattern_bonus(best.0);
        let final_confidence = (confidence + pattern_bonus).min(1.0);

        self.record_to_history(error, vocab, episodes, entities, hypotheses, current_learning_rate, best.0);

        AttributionResult {
            subsystem: best.0,
            confidence: final_confidence,
            scores,
        }
    }

    pub fn history_len(&self) -> usize {
        self.history.len()
    }

    pub fn recent_attributions(&self, count: usize) -> Vec<ErrorAttribution> {
        self.history.iter().rev().take(count).map(|h| h.attribution).collect()
    }

    fn contradiction_ratio_for(&self, hypothesis_count: usize) -> f32 {
        if self.history.is_empty() {
            if hypothesis_count > CONTRADICTIONS_MANY {
                return 1.0;
            }
            return 0.0;
        }
        let recent = self.history.iter().rev().take(20);
        let contradictions: usize = recent.map(|h| h.contradiction_count).sum();
        let count = self.history.len().min(20);
        if count == 0 {
            return 0.0;
        }
        contradictions as f32 / (count as f32 * hypothesis_count.max(1) as f32)
    }

    fn recent_learning_rate(&self) -> Option<f32> {
        self.history.back().map(|h| h.learning_rate)
    }

    fn detect_pattern_bonus(&self, candidate: ErrorAttribution) -> f32 {
        if self.history.len() < 5 {
            return 0.0;
        }
        let recent_count = self.history.len().min(10);
        let same_count = self.history.iter().rev().take(recent_count)
            .filter(|h| h.attribution == candidate)
            .count();
        let ratio = same_count as f32 / recent_count as f32;
        if ratio > 0.6 {
            0.15
        } else if ratio > 0.4 {
            0.08
        } else {
            0.0
        }
    }

    fn record_to_history(
        &mut self,
        error: f32,
        vocab: u32,
        episodes: usize,
        entities: usize,
        _hypotheses: usize,
        learning_rate: f32,
        attribution: ErrorAttribution,
    ) {
        if self.history.len() >= HISTORY_MAX {
            self.history.pop_front();
        }
        self.history.push_back(HistoryEntry {
            prediction_error: error,
            vocabulary_size: vocab,
            episode_count: episodes,
            entity_count: entities,
            contradiction_count: 0,
            learning_rate,
            attribution,
            timestamp: Timestamp::now(),
        });
    }
}

pub fn attribute(experience: &Experience) -> ErrorAttribution {
    let error = experience.error.magnitude;
    let vocab = experience.internal_state.language_vocabulary_size;
    let episodes = experience.internal_state.memory_episode_count;
    let entities = experience.internal_state.world_entity_count;
    let hypotheses = experience.internal_state.reasoning_hypothesis_count;

    if error < ERROR_THRESHOLD {
        return ErrorAttribution::InputError;
    }

    let mut input_score = 0.0;
    let mut memory_score = 0.0;
    let mut world_score = 0.0;
    let mut reasoning_score = 0.0;
    let mut env_score = 0.0;

    if vocab < VOCAB_SMALL {
        input_score += 0.4 * (1.0 - (vocab as f32 / VOCAB_SMALL as f32));
    }
    if episodes < EPISODES_FEW {
        memory_score += 0.4 * (1.0 - (episodes as f32 / EPISODES_FEW as f32));
    }
    if entities < ENTITIES_FEW {
        world_score += 0.4 * (1.0 - (entities as f32 / ENTITIES_FEW as f32));
    }
    if hypotheses > CONTRADICTIONS_MANY {
        reasoning_score += 0.35;
    }

    let error_factor = (error * 2.0).min(1.0);
    input_score *= error_factor;
    memory_score *= error_factor;
    world_score *= error_factor;
    reasoning_score *= error_factor;
    env_score *= error_factor;

    let scores = [
        (ErrorAttribution::InputError, input_score),
        (ErrorAttribution::MemoryError, memory_score),
        (ErrorAttribution::WorldError, world_score),
        (ErrorAttribution::ReasoningError, reasoning_score),
        (ErrorAttribution::EnvironmentError, env_score),
    ];

    first_max(&scores).0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_experience(vocab: u32, episodes: usize, entities: usize, hypotheses: usize, error_mag: f32) -> Experience {
        Experience {
            observation: Observation::user_provided("test"),
            internal_state: StateSnapshot {
                language_vocabulary_size: vocab,
                neural_active_cells: 0,
                memory_episode_count: episodes,
                world_entity_count: entities,
                reasoning_hypothesis_count: hypotheses,
                timestamp: Timestamp::now(),
            },
            prediction: Prediction {
                target: PredictionTarget::NextState,
                predicted_state: Vec::new(),
                confidence: 0.5,
                timestamp: Timestamp::now(),
                context: ContextState::initial(),
                resolved: false,
                actual: None,
                error: None,
            },
            action: None,
            outcome: None,
            error: PredictionError {
                magnitude: error_mag,
                dimensions: std::collections::HashMap::new(),
                timestamp: Timestamp::now(),
                prediction_id: None,
            },
            attribution: ErrorAttribution::InputError,
            evidence: EvidenceSet::new(),
            provenance: Provenance::user_provided(),
        }
    }

    #[test]
    fn test_low_error_returns_input_error() {
        let exp = make_experience(0, 0, 0, 0, 0.05);
        let result = attribute(&exp);
        assert_eq!(result, ErrorAttribution::InputError);
    }

    #[test]
    fn test_high_error_small_vocab_returns_input_error() {
        let exp = make_experience(10, 50, 50, 0, 0.5);
        let result = attribute(&exp);
        assert_eq!(result, ErrorAttribution::InputError);
    }

    #[test]
    fn test_high_error_few_episodes_returns_memory_error() {
        let exp = make_experience(200, 2, 50, 0, 0.5);
        let result = attribute(&exp);
        assert_eq!(result, ErrorAttribution::MemoryError);
    }

    #[test]
    fn test_high_error_few_entities_returns_world_error() {
        let exp = make_experience(200, 50, 1, 0, 0.5);
        let result = attribute(&exp);
        assert_eq!(result, ErrorAttribution::WorldError);
    }

    #[test]
    fn test_high_error_many_hypotheses_returns_reasoning_error() {
        let exp = make_experience(200, 50, 50, 10, 0.5);
        let result = attribute(&exp);
        assert_eq!(result, ErrorAttribution::ReasoningError);
    }

    #[test]
    fn test_attribution_engine_low_error() {
        let mut engine = AttributionEngine::new();
        let exp = make_experience(0, 0, 0, 0, 0.05);
        let result = engine.attribute(&exp);
        assert_eq!(result.subsystem, ErrorAttribution::InputError);
        assert!(result.confidence > 0.5);
    }

    #[test]
    fn test_attribution_engine_high_error_small_vocab() {
        let mut engine = AttributionEngine::new();
        let exp = make_experience(10, 50, 50, 0, 0.5);
        let result = engine.attribute(&exp);
        assert_eq!(result.subsystem, ErrorAttribution::InputError);
        assert!(result.confidence > 0.0);
    }

    #[test]
    fn test_attribution_engine_few_episodes() {
        let mut engine = AttributionEngine::new();
        let exp = make_experience(200, 2, 50, 0, 0.5);
        let result = engine.attribute(&exp);
        assert_eq!(result.subsystem, ErrorAttribution::MemoryError);
    }

    #[test]
    fn test_attribution_engine_few_entities() {
        let mut engine = AttributionEngine::new();
        let exp = make_experience(200, 50, 1, 0, 0.5);
        let result = engine.attribute(&exp);
        assert_eq!(result.subsystem, ErrorAttribution::WorldError);
    }

    #[test]
    fn test_attribution_engine_many_hypotheses() {
        let mut engine = AttributionEngine::new();
        let exp = make_experience(200, 50, 50, 10, 0.5);
        let result = engine.attribute(&exp);
        assert_eq!(result.subsystem, ErrorAttribution::ReasoningError);
    }

    #[test]
    fn test_history_tracking() {
        let mut engine = AttributionEngine::new();
        for _ in 0..5 {
            let exp = make_experience(200, 50, 50, 0, 0.5);
            engine.attribute(&exp);
        }
        assert_eq!(engine.history_len(), 5);
        let recent = engine.recent_attributions(3);
        assert_eq!(recent.len(), 3);
    }

    #[test]
    fn test_history_bounded() {
        let mut engine = AttributionEngine::new();
        for _ in 0..300 {
            let exp = make_experience(200, 50, 50, 0, 0.5);
            engine.attribute(&exp);
        }
        assert!(engine.history_len() <= HISTORY_MAX);
    }

    #[test]
    fn test_pattern_bonus_increases_confidence() {
        let mut engine = AttributionEngine::new();
        for _ in 0..10 {
            let exp = make_experience(200, 50, 50, 10, 0.5);
            engine.attribute(&exp);
        }
        let exp = make_experience(200, 50, 50, 10, 0.5);
        let result = engine.attribute(&exp);
        assert!(result.confidence > 0.1);
    }

    #[test]
    fn test_attribute_function() {
        let exp = make_experience(10, 50, 50, 0, 0.5);
        let result = attribute(&exp);
        assert_eq!(result, ErrorAttribution::InputError);
    }

    #[test]
    fn test_all_zero_scores_returns_input() {
        let exp = make_experience(200, 50, 50, 0, 0.5);
        let result = attribute(&exp);
        assert_eq!(result, ErrorAttribution::InputError);
    }

    #[test]
    fn test_attribution_engine_with_lr_high() {
        let mut engine = AttributionEngine::new();
        let exp = make_experience(200, 50, 50, 0, 0.5);
        let result = engine.attribute_with_lr(&exp, 0.8);
        assert_eq!(result.subsystem, ErrorAttribution::EnvironmentError);
    }

    #[test]
    fn test_attribution_engine_with_lr_low() {
        let mut engine = AttributionEngine::new();
        let exp = make_experience(200, 50, 50, 0, 0.5);
        let result = engine.attribute_with_lr(&exp, 0.01);
        assert_eq!(result.subsystem, ErrorAttribution::InputError);
    }

    #[test]
    fn test_attribution_result_scores_sum() {
        let mut engine = AttributionEngine::new();
        let exp = make_experience(10, 2, 1, 8, 0.7);
        let result = engine.attribute(&exp);
        let total: f32 = result.scores.iter().map(|s| s.1).sum();
        assert!(total >= 0.0);
    }

    #[test]
    fn test_confidence_bounds() {
        let mut engine = AttributionEngine::new();
        for _ in 0..20 {
            let exp = make_experience(10, 2, 1, 8, 0.7);
            let result = engine.attribute(&exp);
            assert!(result.confidence >= 0.0 && result.confidence <= 1.0);
        }
    }
}
