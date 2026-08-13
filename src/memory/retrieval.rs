use crate::types::common::ContextState;
use crate::types::observation::Observation;
use crate::types::scalars::Scalar;

use super::episodic::Episode;
use super::associative::Association;

const WEIGHT_SEMANTIC: Scalar = 0.30;
const WEIGHT_CONTEXT: Scalar = 0.20;
const WEIGHT_TEMPORAL: Scalar = 0.15;
const WEIGHT_ASSOCIATION: Scalar = 0.15;
const WEIGHT_IMPORTANCE: Scalar = 0.10;
const WEIGHT_CONFIDENCE: Scalar = 0.10;

#[derive(Debug, Clone)]
pub struct RetrievalResult {
    pub score: Scalar,
    pub item_type: String,
    pub item_id: u64,
}

pub struct RetrievalEngine;

impl RetrievalEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn score_relevance(query: &str, item: &str) -> Scalar {
        let query_words: Vec<&str> = query.split_whitespace().collect();
        let item_words: Vec<&str> = item.split_whitespace().collect();

        if query_words.is_empty() || item_words.is_empty() {
            return 0.0;
        }

        let overlap = query_words
            .iter()
            .filter(|w| item_words.contains(w))
            .count() as Scalar;

        let max_len = query_words.len().max(item_words.len()) as Scalar;
        overlap / max_len
    }

    pub fn score_episode(
        query: &str,
        episode: &Episode,
        context: &ContextState,
        associations: &[&Association],
    ) -> Scalar {
        let semantic = Self::score_relevance(query, &episode.observation.text);
        let context_score = Self::context_relevance(episode, context);
        let temporal = Self::temporal_relevance(episode);
        let association_score = Self::association_strength(episode, associations);
        let importance = episode.importance;
        let confidence = episode.confidence;

        semantic * WEIGHT_SEMANTIC
            + context_score * WEIGHT_CONTEXT
            + temporal * WEIGHT_TEMPORAL
            + association_score * WEIGHT_ASSOCIATION
            + importance * WEIGHT_IMPORTANCE
            + confidence * WEIGHT_CONFIDENCE
    }

    pub fn score_observation(
        query: &str,
        observation: &Observation,
        context: &ContextState,
    ) -> Scalar {
        let semantic = Self::score_relevance(query, &observation.text);
        let context_score = Self::observation_context_relevance(observation, context);
        let importance = observation.importance;
        let confidence = observation.source.confidence.overall();

        semantic * WEIGHT_SEMANTIC
            + context_score * WEIGHT_CONTEXT
            + importance * WEIGHT_IMPORTANCE
            + confidence * WEIGHT_CONFIDENCE
    }

    fn context_relevance(episode: &Episode, context: &ContextState) -> Scalar {
        let mut score: Scalar = 0.0;

        for concept_id in &context.active_concepts {
            if episode
                .observation
                .context
                .active_concepts
                .contains(concept_id)
            {
                score += 0.3;
            }
        }

        for entity_id in &context.world_assumptions {
            if episode
                .observation
                .context
                .world_assumptions
                .contains(entity_id)
            {
                score += 0.3;
            }
        }

        if context.conversation_id == episode.observation.context.conversation_id {
            score += 0.4;
        }

        score.min(1.0)
    }

    fn observation_context_relevance(observation: &Observation, context: &ContextState) -> Scalar {
        let mut score: Scalar = 0.0;

        for concept_id in &context.active_concepts {
            if observation.context.active_concepts.contains(concept_id) {
                score += 0.4;
            }
        }

        for entity_id in &context.world_assumptions {
            if observation.context.world_assumptions.contains(entity_id) {
                score += 0.3;
            }
        }

        if context.conversation_id == observation.context.conversation_id {
            score += 0.3;
        }

        score.min(1.0)
    }

    fn temporal_relevance(episode: &Episode) -> Scalar {
        let now = crate::types::common::Timestamp::now();
        let age_millis = now.elapsed_since(episode.timestamp).as_millis() as Scalar;
        let age_hours = age_millis / 3_600_000.0;
        1.0 / (1.0 + age_hours)
    }

    fn association_strength(_episode: &Episode, associations: &[&Association]) -> Scalar {
        if associations.is_empty() {
            return 0.0;
        }
        let total: Scalar = associations.iter().map(|a| a.strength).sum();
        (total / associations.len() as Scalar).min(1.0)
    }

    pub fn detect_contradictions<'a>(
        episodes: &'a [Episode],
    ) -> Vec<(&'a Episode, &'a Episode, String)> {
        let mut contradictions = Vec::new();

        for i in 0..episodes.len() {
            for j in (i + 1)..episodes.len() {
                let a = &episodes[i];
                let b = &episodes[j];

                let a_words: Vec<&str> = a.observation.text.split_whitespace().collect();
                let b_words: Vec<&str> = b.observation.text.split_whitespace().collect();

                let negation_markers = ["not", "never", "no", "isn't", "aren't", "won't", "can't"];
                let a_has_negation = a_words
                    .iter()
                    .any(|w| negation_markers.contains(w));
                let b_has_negation = b_words
                    .iter()
                    .any(|w| negation_markers.contains(w));

                let content_overlap = a_words
                    .iter()
                    .filter(|w| b_words.contains(w))
                    .count();

                if content_overlap > 2 && a_has_negation != b_has_negation {
                    contradictions.push((
                        a,
                        b,
                        format!(
                            "Contradiction between '{}' and '{}'",
                            a.observation.text, b.observation.text
                        ),
                    ));
                }
            }
        }

        contradictions
    }

    pub fn filter_by_confidence<'a>(
        episodes: &'a [Episode],
        min_confidence: Scalar,
    ) -> Vec<&'a Episode> {
        episodes
            .iter()
            .filter(|e| e.confidence >= min_confidence)
            .collect()
    }

    pub fn rank_episodes(
        query: &str,
        episodes: &[Episode],
        context: &ContextState,
        associations: &[&Association],
    ) -> Vec<(usize, Scalar)> {
        let mut scored: Vec<(usize, Scalar)> = episodes
            .iter()
            .enumerate()
            .map(|(i, ep)| {
                let score = Self::score_episode(query, ep, context, associations);
                (i, score)
            })
            .collect();

        scored.sort_by(|a, b| {
            b.1.partial_cmp(&a.1)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        scored
    }
}
