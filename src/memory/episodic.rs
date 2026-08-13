use serde::{Deserialize, Serialize};
use crate::types::ids::EpisodeId;
use crate::types::common::Timestamp;
use crate::types::scalars::Scalar;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    pub id: EpisodeId,
    pub observation: String,
    pub timestamp: Timestamp,
    pub importance: Scalar,
    pub confidence: Scalar,
    pub consolidated: bool,
}

#[derive(Debug, Clone)]
pub struct EpisodicMemory {
    pub episodes: Vec<Episode>,
    pub capacity: usize,
    pub next_id: u64,
}

impl EpisodicMemory {
    pub fn new(capacity: usize) -> Self {
        Self {
            episodes: Vec::new(),
            capacity,
            next_id: 1,
        }
    }

    pub fn store(&mut self, observation: &str, importance: Scalar) -> Episode {
        let episode = Episode {
            id: EpisodeId::from(self.next_id),
            observation: observation.to_string(),
            timestamp: Timestamp::now(),
            importance,
            confidence: 0.5,
            consolidated: false,
        };
        self.next_id += 1;

        if self.episodes.len() >= self.capacity {
            self.evict_lowest_value();
        }

        self.episodes.push(episode.clone());
        episode
    }

    fn evict_lowest_value(&mut self) {
        if let Some(pos) = self.episodes.iter()
            .enumerate()
            .min_by(|a, b| {
                let va = a.1.importance * 0.3 + a.1.confidence * 0.2;
                let vb = b.1.importance * 0.3 + b.1.confidence * 0.2;
                va.partial_cmp(&vb).unwrap_or(std::cmp::Ordering::Equal)
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
}
