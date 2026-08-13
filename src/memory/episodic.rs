use crate::error::Result;
use crate::types::*;

pub fn store(memory: &mut EpisodicMemory, episode: Episode) -> Result<()> {
    while memory.current_usage_bytes >= memory.capacity_bytes && !memory.episodes.is_empty() {
        evict_lowest_value(memory);
    }
    memory.current_usage_bytes += estimate_size(&episode);
    memory.episodes.push(episode);
    Ok(())
}

fn evict_lowest_value(memory: &mut EpisodicMemory) {
    if memory.episodes.is_empty() {
        return;
    }
    let mut min_idx = 0;
    let mut min_value = f32::MAX;
    for (i, ep) in memory.episodes.iter().enumerate() {
        let value = compute_episode_value(ep);
        if value < min_value {
            min_value = value;
            min_idx = i;
        }
    }
    let removed = memory.episodes.remove(min_idx);
    memory.current_usage_bytes = memory.current_usage_bytes.saturating_sub(estimate_size(&removed));
}

fn compute_episode_value(episode: &Episode) -> f32 {
    let age_hours = (Timestamp::now().0.saturating_sub(episode.timestamp.0)) as f32 / 3_600_000.0;
    let recency = 1.0 / (1.0 + age_hours);
    let retrieval_value = (episode.retrieval_count.min(10) as f32 / 10.0);
    let consolidation_value = if episode.consolidated { 0.15 } else { 0.0 };

    episode.importance * 0.30
        + episode.confidence.overall() * 0.20
        + recency * 0.20
        + retrieval_value * 0.15
        + consolidation_value
}

fn estimate_size(episode: &Episode) -> u64 {
    let text_size = episode.observation.text.len() as u64;
    let overhead = 256;
    text_size + overhead
}

pub fn retrieve_by_relevance(episodes: &[Episode], query_text: &str, max_results: usize) -> Vec<(usize, Scalar)> {
    let query_lower = query_text.to_lowercase();
    let query_words: Vec<&str> = query_lower.split_whitespace().collect();

    let mut scored: Vec<(usize, Scalar)> = episodes.iter().enumerate().map(|(i, ep)| {
        let text_lower = ep.observation.text.to_lowercase();
        let episode_words: Vec<&str> = text_lower.split_whitespace().collect();

        let word_overlap = query_words.iter()
            .filter(|qw| episode_words.iter().any(|ew| ew.contains(*qw) || qw.contains(*ew)))
            .count() as Scalar;
        let semantic_score = if query_words.is_empty() { 0.0 } else { word_overlap / query_words.len() as Scalar };

        let recency = 1.0 / (1.0 + (Timestamp::now().0.saturating_sub(ep.timestamp.0)) as f32 / 3_600_000.0);

        let score = semantic_score * 0.5 + ep.importance * 0.2 + ep.confidence.overall() * 0.15 + recency * 0.15;
        (i, score)
    }).collect();

    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    scored.truncate(max_results);
    scored
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_episode(text: &str, importance: f32) -> Episode {
        Episode {
            id: EpisodeId(1),
            observation: Observation::user_provided(text),
            context: ContextState::initial(),
            action: None,
            outcome: None,
            timestamp: Timestamp::now(),
            prediction: None,
            prediction_error: PredictionError::zero(),
            confidence: ConfidenceState::default(),
            source: Provenance::user_provided(),
            importance,
            retrieval_count: 0,
            last_retrieved: None,
            consolidated: false,
        }
    }

    #[test]
    fn test_store_and_retrieve() {
        let mut memory = EpisodicMemory {
            episodes: Vec::new(),
            capacity_bytes: 1024 * 1024,
            current_usage_bytes: 0,
            next_id: EpisodeId(1),
        };
        store(&mut memory, make_episode("gravity is a force", 0.8)).unwrap();
        store(&mut memory, make_episode("water boils at 100C", 0.5)).unwrap();
        assert_eq!(memory.episodes.len(), 2);

        let results = retrieve_by_relevance(&memory.episodes, "gravity force", 10);
        assert!(!results.is_empty());
        assert_eq!(results[0].0, 0);
    }
}
