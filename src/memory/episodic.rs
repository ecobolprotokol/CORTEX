use serde::{Deserialize, Serialize};

use crate::types::common::Timestamp;
use crate::types::ids::EpisodeId;
use crate::types::observation::Observation;
use crate::types::scalars::Scalar;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    pub id: EpisodeId,
    pub observation: Observation,
    pub timestamp: Timestamp,
    pub importance: Scalar,
    pub confidence: Scalar,
    pub consolidated: bool,
    pub retrieval_count: u64,
}

impl Episode {
    pub fn value_score(&self) -> Scalar {
        let now = Timestamp::now();
        let age_secs = now.elapsed_since(self.timestamp).as_secs() as Scalar;
        let age_factor = 1.0 / (1.0 + age_secs / 3600.0);
        let retrieval_factor = (self.retrieval_count as Scalar).min(10.0) / 10.0;

        self.importance * 0.30
            + self.confidence * 0.20
            + age_factor * 0.25
            + retrieval_factor * 0.15
            + if self.consolidated { 0.10 } else { 0.0 }
    }
}

#[derive(Debug, Clone)]
pub struct EpisodicMemory {
    pub episodes: Vec<Episode>,
    pub capacity: usize,
    pub next_id: u64,
}

impl Default for EpisodicMemory {
    fn default() -> Self {
        Self::new(512)
    }
}

impl EpisodicMemory {
    pub fn new(capacity: usize) -> Self {
        Self {
            episodes: Vec::new(),
            capacity,
            next_id: 1,
        }
    }

    pub fn store(&mut self, observation: Observation) -> Episode {
        let importance = observation.importance;
        let confidence = observation.source.confidence.overall();
        let episode = Episode {
            id: EpisodeId::from(self.next_id),
            observation,
            timestamp: Timestamp::now(),
            importance,
            confidence,
            consolidated: false,
            retrieval_count: 0,
        };
        self.next_id += 1;

        if self.episodes.len() >= self.capacity {
            self.evict_lowest_value();
        }

        self.episodes.push(episode.clone());
        episode
    }

    pub fn store_with_text(&mut self, text: &str, _importance: Scalar) -> Episode {
        let obs = Observation::user_provided(text);
        self.store(obs)
    }

    fn evict_lowest_value(&mut self) {
        if let Some(pos) = self
            .episodes
            .iter()
            .enumerate()
            .min_by(|a, b| {
                a.1.value_score()
                    .partial_cmp(&b.1.value_score())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(i, _)| i)
        {
            self.episodes.remove(pos);
        }
    }

    pub fn recent(&self, n: usize) -> &[Episode] {
        let start = self.episodes.len().saturating_sub(n);
        &self.episodes[start..]
    }

    pub fn get(&self, id: EpisodeId) -> Option<&Episode> {
        self.episodes.iter().find(|e| e.id == id)
    }

    pub fn get_mut(&mut self, id: EpisodeId) -> Option<&mut Episode> {
        self.episodes.iter_mut().find(|e| e.id == id)
    }

    pub fn record_retrieval(&mut self, id: EpisodeId) {
        if let Some(ep) = self.get_mut(id) {
            ep.retrieval_count += 1;
            ep.confidence = (ep.confidence + 0.02).min(1.0);
        }
    }

    pub fn mark_consolidated(&mut self, id: EpisodeId) {
        if let Some(ep) = self.get_mut(id) {
            ep.consolidated = true;
        }
    }

    pub fn by_importance(&self) -> Vec<&Episode> {
        let mut sorted: Vec<&Episode> = self.episodes.iter().collect();
        sorted.sort_by(|a, b| {
            b.importance
                .partial_cmp(&a.importance)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        sorted
    }

    pub fn unconsolidated(&self) -> Vec<&Episode> {
        self.episodes.iter().filter(|e| !e.consolidated).collect()
    }

    pub fn usage_bytes(&self) -> usize {
        self.episodes.len() * std::mem::size_of::<Episode>()
    }

    pub fn is_full(&self) -> bool {
        self.episodes.len() >= self.capacity
    }
}
