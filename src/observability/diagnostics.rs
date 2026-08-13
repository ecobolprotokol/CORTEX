use crate::types::scalars::Scalar;

#[derive(Debug, Clone)]
pub struct Metrics {
    pub prediction_error: Scalar,
    pub memory_pressure: Scalar,
    pub learning_rate_effective: Scalar,
    pub episode_count: u64,
    pub total_learning_events: u64,
    pub uptime_seconds: u64,
    pub entity_count: u64,
    pub hypothesis_count: u64,
    pub checkpoint_count: u32,
    pub verification_rate: Scalar,
    pub reasoning_consistency: Scalar,
    pub api_requests: u64,
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            prediction_error: 0.0,
            memory_pressure: 0.0,
            learning_rate_effective: 0.0,
            episode_count: 0,
            total_learning_events: 0,
            uptime_seconds: 0,
            entity_count: 0,
            hypothesis_count: 0,
            checkpoint_count: 0,
            verification_rate: 0.0,
            reasoning_consistency: 0.0,
            api_requests: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HealthStatus {
    pub healthy: bool,
    pub components: Vec<ComponentHealth>,
    pub overall_score: Scalar,
}

#[derive(Debug, Clone)]
pub struct ComponentHealth {
    pub name: String,
    pub healthy: bool,
    pub score: Scalar,
    pub message: String,
}

pub struct Diagnostics {
    pub prediction_errors: Vec<Scalar>,
    pub memory_usage: Vec<Scalar>,
    pub learning_signals: Vec<u64>,
    pub max_history: usize,
}

impl Diagnostics {
    pub fn new() -> Self {
        Self {
            prediction_errors: Vec::new(),
            memory_usage: Vec::new(),
            learning_signals: Vec::new(),
            max_history: 1000,
        }
    }

    pub fn collect(&self) -> Metrics {
        let avg_error = if self.prediction_errors.is_empty() {
            0.0
        } else {
            self.prediction_errors.iter().sum::<Scalar>()
                / self.prediction_errors.len() as Scalar
        };

        let memory_pressure = if self.memory_usage.is_empty() {
            0.0
        } else {
            *self.memory_usage.last().unwrap_or(&0.0)
        };

        Metrics {
            prediction_error: avg_error,
            memory_pressure,
            learning_rate_effective: 0.001,
            episode_count: 0,
            total_learning_events: self.learning_signals.iter().sum(),
            uptime_seconds: 0,
            entity_count: 0,
            hypothesis_count: 0,
            checkpoint_count: 0,
            verification_rate: 0.0,
            reasoning_consistency: 0.0,
            api_requests: 0,
        }
    }

    pub fn health_check(&self) -> HealthStatus {
        let mut components = Vec::new();

        let prediction_health = if self.prediction_errors.is_empty() {
            (true, 1.0, "No prediction data".into())
        } else {
            let avg = self.prediction_errors.iter().sum::<Scalar>()
                / self.prediction_errors.len() as Scalar;
            let healthy = avg < 0.5;
            let score = (1.0 - avg).max(0.0);
            (healthy, score, format!("avg_error={:.4}", avg))
        };
        components.push(ComponentHealth {
            name: "prediction".into(),
            healthy: prediction_health.0,
            score: prediction_health.1,
            message: prediction_health.2,
        });

        let memory_health = if self.memory_usage.is_empty() {
            (true, 1.0, "No memory data".into())
        } else {
            let pressure = *self.memory_usage.last().unwrap_or(&0.0);
            let healthy = pressure < 0.8;
            let score = (1.0 - pressure).max(0.0);
            (healthy, score, format!("pressure={:.2}", pressure))
        };
        components.push(ComponentHealth {
            name: "memory".into(),
            healthy: memory_health.0,
            score: memory_health.1,
            message: memory_health.2,
        });

        let learning_health = {
            let total: u64 = self.learning_signals.iter().sum();
            let healthy = total > 0 || self.learning_signals.len() < 100;
            let score = if total > 0 { 0.8 } else { 0.5 };
            (healthy, score, format!("total_signals={}", total))
        };
        components.push(ComponentHealth {
            name: "learning".into(),
            healthy: learning_health.0,
            score: learning_health.1,
            message: learning_health.2,
        });

        let overall_score = components.iter().map(|c| c.score).sum::<Scalar>()
            / components.len() as Scalar;
        let healthy = components.iter().all(|c| c.healthy);

        HealthStatus {
            healthy,
            components,
            overall_score,
        }
    }

    pub fn record_prediction_error(&mut self, error: Scalar) {
        self.prediction_errors.push(error);
        if self.prediction_errors.len() > self.max_history {
            self.prediction_errors.remove(0);
        }
    }

    pub fn record_memory_usage(&mut self, usage: Scalar) {
        self.memory_usage.push(usage);
        if self.memory_usage.len() > self.max_history {
            self.memory_usage.remove(0);
        }
    }

    pub fn record_learning_signal(&mut self, count: u64) {
        self.learning_signals.push(count);
        if self.learning_signals.len() > self.max_history {
            self.learning_signals.remove(0);
        }
    }

    pub fn is_healthy(&self) -> bool {
        self.health_check().healthy
    }

    pub fn average_prediction_error(&self) -> Scalar {
        if self.prediction_errors.is_empty() {
            return 0.0;
        }
        self.prediction_errors.iter().sum::<Scalar>()
            / self.prediction_errors.len() as Scalar
    }

    pub fn current_memory_pressure(&self) -> Scalar {
        self.memory_usage.last().copied().unwrap_or(0.0)
    }
}

impl Default for Diagnostics {
    fn default() -> Self {
        Self::new()
    }
}
