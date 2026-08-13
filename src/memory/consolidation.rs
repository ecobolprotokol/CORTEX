pub struct ConsolidationEngine {
    pub interval: u64,
    pub episodes_since_consolidation: u64,
}

impl ConsolidationEngine {
    pub fn new(interval: u64) -> Self {
        Self {
            interval,
            episodes_since_consolidation: 0,
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
}
