pub mod diagnostics;

pub use diagnostics::{Diagnostics, Metrics, HealthStatus, RuntimeMetrics};

use crate::error::CortexError;

pub trait ObservabilityInterface {
    fn metrics(&self) -> Result<Metrics, CortexError>;
    fn health_check(&self) -> Result<HealthStatus, CortexError>;
}

pub struct ObservabilityManager {
    pub diagnostics: Diagnostics,
    pub start_time: u64,
}

impl ObservabilityManager {
    pub fn new() -> Self {
        Self {
            diagnostics: Diagnostics::new(),
            start_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }

    pub fn uptime_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            .saturating_sub(self.start_time)
    }

    pub fn collect_metrics(&self) -> Metrics {
        let mut metrics = self.diagnostics.collect(None);
        metrics.uptime_seconds = self.uptime_seconds();
        metrics
    }

    pub fn health_check(&self) -> HealthStatus {
        self.diagnostics.health_check()
    }
}

impl Default for ObservabilityManager {
    fn default() -> Self {
        Self::new()
    }
}
