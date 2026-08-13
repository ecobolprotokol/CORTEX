use crate::types::common::Timestamp;
use crate::types::scalars::Scalar;

use super::episodic::EpisodicMemory;
use super::semantic::SemanticMemory;
use super::associative::{AssociativeMemory, AssociationKind};

pub struct ConsolidationEngine {
    pub interval: u64,
    pub episodes_since_consolidation: u64,
    pub decay_rate: Scalar,
    pub merge_threshold: Scalar,
}

impl Default for ConsolidationEngine {
    fn default() -> Self {
        Self::new(1000)
    }
}

impl ConsolidationEngine {
    pub fn new(interval: u64) -> Self {
        Self {
            interval,
            episodes_since_consolidation: 0,
            decay_rate: 0.01,
            merge_threshold: 0.8,
        }
    }

    pub fn should_consolidate(&self) -> bool {
        self.episodes_since_consolidation >= self.interval
    }

    pub fn record_episode(&mut self) {
        self.episodes_since_consolidation += 1;
    }

    pub fn reset_counter(&mut self) {
        self.episodes_since_consolidation = 0;
    }

    pub fn consolidate(
        &mut self,
        episodic: &mut EpisodicMemory,
        semantic: &mut SemanticMemory,
        associative: &mut AssociativeMemory,
    ) -> ConsolidationReport {
        self.reset_counter();

        let mut report = ConsolidationReport::default();

        let merged = self.merge_similar_episodes(episodic);
        report.episodes_merged = merged;

        let generalized = self.generalize(episodic, semantic);
        report.knowledge_extracted = generalized;

        let strengthened = self.strengthen_frequent_patterns(episodic, associative);
        report.patterns_strengthened = strengthened;

        let decayed = self.decay_old_memories(episodic);
        report.memories_decayed = decayed;

        report.consolidated_at = Timestamp::now();
        report
    }

    fn merge_similar_episodes(&self, episodic: &mut EpisodicMemory) -> u64 {
        let mut merged_count = 0;
        let mut to_mark = Vec::new();

        let len = episodic.episodes.len();
        for i in 0..len {
            for j in (i + 1)..len {
                let similarity = crate::memory::retrieval::RetrievalEngine::score_relevance(
                    &episodic.episodes[i].observation.text,
                    &episodic.episodes[j].observation.text,
                );

                if similarity >= self.merge_threshold {
                    let higher_importance = episodic.episodes[i]
                        .importance
                        .max(episodic.episodes[j].importance);
                    let combined_retrievals = episodic.episodes[i].retrieval_count
                        + episodic.episodes[j].retrieval_count;

                    if !to_mark.contains(&i) {
                        episodic.episodes[i].importance = higher_importance;
                        episodic.episodes[i].retrieval_count = combined_retrievals;
                        episodic.episodes[i].confidence =
                            (episodic.episodes[i].confidence + 0.1).min(1.0);
                    }

                    to_mark.push(j);
                    merged_count += 1;
                }
            }
        }

        to_mark.sort_unstable();
        to_mark.dedup();
        for idx in to_mark.into_iter().rev() {
            episodic.episodes.remove(idx);
        }

        merged_count
    }

    fn generalize(
        &self,
        episodic: &EpisodicMemory,
        semantic: &mut SemanticMemory,
    ) -> u64 {
        let mut extracted = 0;

        let mut word_frequency: std::collections::HashMap<String, u64> =
            std::collections::HashMap::new();

        for episode in &episodic.episodes {
            for word in episode.observation.text.split_whitespace() {
                let lower = word.to_lowercase();
                if lower.len() > 3 {
                    *word_frequency.entry(lower).or_insert(0) += 1;
                }
            }
        }

        let frequent_words: Vec<String> = word_frequency
            .into_iter()
            .filter(|(_, count)| *count >= 3)
            .map(|(word, _)| word)
            .collect();

        for word in frequent_words {
            let exists = semantic
                .knowledge
                .iter()
                .any(|k| k.concept == word);

            if !exists {
                semantic.store(
                    &word,
                    vec![
                        ("source".into(), "generalized".into()),
                        ("frequency".into(), "high".into()),
                    ],
                );
                extracted += 1;
            }
        }

        extracted
    }

    fn strengthen_frequent_patterns(
        &self,
        episodic: &EpisodicMemory,
        associative: &mut AssociativeMemory,
    ) -> u64 {
        let mut strengthened = 0;

        let mut bigram_counts: std::collections::HashMap<(String, String), u64> =
            std::collections::HashMap::new();

        for episode in &episodic.episodes {
            let words: Vec<&str> = episode.observation.text.split_whitespace().collect();
            for window in words.windows(2) {
                let key = (window[0].to_lowercase(), window[1].to_lowercase());
                *bigram_counts.entry(key).or_insert(0) += 1;
            }
        }

        for ((w1, w2), count) in &bigram_counts {
            if *count >= 3 {
                let source_id = hash_string(w1);
                let target_id = hash_string(w2);

                let exists = associative
                    .get_associations(source_id)
                    .iter()
                    .any(|a| a.target == target_id);

                if !exists {
                    let delta = (*count as Scalar / 10.0).min(0.3);
                    let new_a = associative.create(
                        source_id,
                        target_id,
                        AssociationKind::Semantic,
                    );
                    associative.strengthen(new_a.id, delta);
                    strengthened += 1;
                } else {
                    let assocs: Vec<_> = associative
                        .between(source_id, target_id)
                        .into_iter()
                        .map(|a| a.id)
                        .collect();
                    for id in assocs {
                        let delta = (*count as Scalar / 20.0).min(0.1);
                        associative.strengthen(id, delta);
                        strengthened += 1;
                    }
                }
            }
        }

        strengthened
    }

    fn decay_old_memories(&self, episodic: &mut EpisodicMemory) -> u64 {
        let mut decayed = 0;
        let now = Timestamp::now();

        for episode in &mut episodic.episodes {
            let age_millis = now.elapsed_since(episode.timestamp).as_millis() as Scalar;
            let age_hours = age_millis / 3_600_000.0;

            if age_hours > 24.0 && episode.retrieval_count == 0 {
                let decay = self.decay_rate * (age_hours / 24.0);
                episode.importance = (episode.importance - decay).max(0.01);
                episode.confidence = (episode.confidence - decay * 0.5).max(0.01);
                decayed += 1;
            }
        }

        decayed
    }
}

#[derive(Debug, Clone, Default)]
pub struct ConsolidationReport {
    pub episodes_merged: u64,
    pub knowledge_extracted: u64,
    pub patterns_strengthened: u64,
    pub memories_decayed: u64,
    pub consolidated_at: Timestamp,
}

fn hash_string(s: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}
