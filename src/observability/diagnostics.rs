use std::collections::VecDeque;

const DEFAULT_MAX_ENTRIES: usize = 1000;
const ANOMALY_WINDOW: usize = 60;
const SPIKE_THRESHOLD: f64 = 3.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Severity {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

impl Severity {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "debug" => Severity::Debug,
            "info" => Severity::Info,
            "warning" | "warn" => Severity::Warning,
            "error" => Severity::Error,
            "critical" | "fatal" => Severity::Critical,
            _ => Severity::Info,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Debug => "debug",
            Severity::Info => "info",
            Severity::Warning => "warning",
            Severity::Error => "error",
            Severity::Critical => "critical",
        }
    }
}

#[derive(Debug, Clone)]
pub struct DiagnosticEntry {
    pub kind: String,
    pub message: String,
    pub timestamp: u64,
    pub severity: Severity,
}

#[derive(Debug, Clone)]
pub struct DiagnosticSummary {
    pub total_entries: usize,
    pub by_severity: [(Severity, usize); 5],
    pub by_kind: Vec<(String, usize)>,
    pub error_rate_per_minute: f64,
    pub anomaly_detected: bool,
}

pub struct Diagnostics {
    entries: VecDeque<DiagnosticEntry>,
    max_entries: usize,
    recent_error_timestamps: VecDeque<u64>,
}

impl Diagnostics {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: VecDeque::with_capacity(max_entries),
            max_entries,
            recent_error_timestamps: VecDeque::with_capacity(256),
        }
    }

    pub fn record(&mut self, kind: &str, message: &str, severity: &str) {
        let sev = Severity::from_str(severity);
        let entry = DiagnosticEntry {
            kind: kind.to_string(),
            message: message.to_string(),
            timestamp: crate::types::Timestamp::now().0,
            severity: sev,
        };
        if sev == Severity::Error || sev == Severity::Critical {
            self.recent_error_timestamps.push_back(entry.timestamp);
            self.prune_error_timestamps();
        }
        if self.entries.len() >= self.max_entries {
            self.entries.pop_front();
        }
        self.entries.push_back(entry);
    }

    pub fn record_entry(&mut self, entry: DiagnosticEntry) {
        if entry.severity == Severity::Error || entry.severity == Severity::Critical {
            self.recent_error_timestamps.push_back(entry.timestamp);
            self.prune_error_timestamps();
        }
        if self.entries.len() >= self.max_entries {
            self.entries.pop_front();
        }
        self.entries.push_back(entry);
    }

    pub fn recent(&self, count: usize) -> Vec<&DiagnosticEntry> {
        self.entries.iter().rev().take(count).collect()
    }

    pub fn filter_by_severity(&self, severity: Severity) -> Vec<&DiagnosticEntry> {
        self.entries.iter().filter(|e| e.severity == severity).collect()
    }

    pub fn filter_by_min_severity(&self, min_severity: Severity) -> Vec<&DiagnosticEntry> {
        self.entries.iter().filter(|e| e.severity >= min_severity).collect()
    }

    pub fn count(&self) -> usize {
        self.entries.len()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.recent_error_timestamps.clear();
    }

    pub fn summary(&self) -> DiagnosticSummary {
        let mut by_severity = [
            (Severity::Debug, 0),
            (Severity::Info, 0),
            (Severity::Warning, 0),
            (Severity::Error, 0),
            (Severity::Critical, 0),
        ];
        let mut kind_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

        for entry in &self.entries {
            match entry.severity {
                Severity::Debug => by_severity[0].1 += 1,
                Severity::Info => by_severity[1].1 += 1,
                Severity::Warning => by_severity[2].1 += 1,
                Severity::Error => by_severity[3].1 += 1,
                Severity::Critical => by_severity[4].1 += 1,
            }
            *kind_counts.entry(entry.kind.clone()).or_insert(0) += 1;
        }

        let mut by_kind: Vec<(String, usize)> = kind_counts.into_iter().collect();
        by_kind.sort_by(|a, b| b.1.cmp(&a.1));

        let error_rate = self.error_rate_per_minute();
        let anomaly = self.detect_anomaly();

        DiagnosticSummary {
            total_entries: self.entries.len(),
            by_severity,
            by_kind,
            error_rate_per_minute: error_rate,
            anomaly_detected: anomaly,
        }
    }

    pub fn error_rate_per_minute(&self) -> f64 {
        if self.recent_error_timestamps.is_empty() {
            return 0.0;
        }
        let now = crate::types::Timestamp::now().0;
        let one_minute_ago = now.saturating_sub(60_000);
        let recent_count = self.recent_error_timestamps.iter()
            .filter(|&&ts| ts >= one_minute_ago)
            .count();
        recent_count as f64
    }

    pub fn detect_anomaly(&self) -> bool {
        if self.entries.len() < ANOMALY_WINDOW {
            return false;
        }
        let now = crate::types::Timestamp::now().0;
        let window_ms = 60_000;
        let recent = self.entries.iter()
            .filter(|e| e.timestamp >= now.saturating_sub(window_ms))
            .filter(|e| e.severity == Severity::Error || e.severity == Severity::Critical)
            .count();
        let older = self.entries.iter()
            .filter(|e| e.timestamp < now.saturating_sub(window_ms) && e.timestamp >= now.saturating_sub(window_ms * 2))
            .filter(|e| e.severity == Severity::Error || e.severity == Severity::Critical)
            .count();

        if older == 0 {
            return recent > 5;
        }
        (recent as f64) / (older as f64) > SPIKE_THRESHOLD
    }

    fn prune_error_timestamps(&mut self) {
        let now = crate::types::Timestamp::now().0;
        let cutoff = now.saturating_sub(120_000);
        while let Some(&front) = self.recent_error_timestamps.front() {
            if front < cutoff {
                self.recent_error_timestamps.pop_front();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Timestamp;

    #[test]
    fn test_diagnostics_record() {
        let mut diag = Diagnostics::new(10);
        diag.record("TestError", "test message", "warning");
        assert_eq!(diag.count(), 1);
    }

    #[test]
    fn test_diagnostics_max_entries() {
        let mut diag = Diagnostics::new(2);
        diag.record("Error1", "msg1", "error");
        diag.record("Error2", "msg2", "error");
        diag.record("Error3", "msg3", "error");
        assert_eq!(diag.count(), 2);
    }

    #[test]
    fn test_diagnostics_recent() {
        let mut diag = Diagnostics::new(10);
        diag.record("Error1", "msg1", "error");
        diag.record("Error2", "msg2", "error");
        let recent = diag.recent(1);
        assert_eq!(recent.len(), 1);
        assert_eq!(recent[0].kind, "Error2");
    }

    #[test]
    fn test_diagnostics_clear() {
        let mut diag = Diagnostics::new(10);
        diag.record("Error", "msg", "error");
        diag.clear();
        assert_eq!(diag.count(), 0);
    }

    #[test]
    fn test_severity_from_str() {
        assert_eq!(Severity::from_str("error"), Severity::Error);
        assert_eq!(Severity::from_str("WARNING"), Severity::Warning);
        assert_eq!(Severity::from_str("debug"), Severity::Debug);
        assert_eq!(Severity::from_str("CRITICAL"), Severity::Critical);
        assert_eq!(Severity::from_str("warn"), Severity::Warning);
        assert_eq!(Severity::from_str("info"), Severity::Info);
    }

    #[test]
    fn test_severity_ordering() {
        assert!(Severity::Debug < Severity::Info);
        assert!(Severity::Info < Severity::Warning);
        assert!(Severity::Warning < Severity::Error);
        assert!(Severity::Error < Severity::Critical);
    }

    #[test]
    fn test_filter_by_severity() {
        let mut diag = Diagnostics::new(10);
        diag.record("A", "msg", "error");
        diag.record("B", "msg", "warning");
        diag.record("C", "msg", "error");
        let errors = diag.filter_by_severity(Severity::Error);
        assert_eq!(errors.len(), 2);
        let warnings = diag.filter_by_severity(Severity::Warning);
        assert_eq!(warnings.len(), 1);
    }

    #[test]
    fn test_filter_by_min_severity() {
        let mut diag = Diagnostics::new(10);
        diag.record("A", "msg", "debug");
        diag.record("B", "msg", "info");
        diag.record("C", "msg", "warning");
        diag.record("D", "msg", "error");
        let warn_and_above = diag.filter_by_min_severity(Severity::Warning);
        assert_eq!(warn_and_above.len(), 2);
        let info_and_above = diag.filter_by_min_severity(Severity::Info);
        assert_eq!(info_and_above.len(), 3);
    }

    #[test]
    fn test_summary_counts() {
        let mut diag = Diagnostics::new(100);
        diag.record("TypeA", "msg", "error");
        diag.record("TypeA", "msg", "error");
        diag.record("TypeB", "msg", "warning");
        diag.record("TypeB", "msg", "info");
        let summary = diag.summary();
        assert_eq!(summary.total_entries, 4);
        assert_eq!(summary.by_severity[3].1, 2);
        assert_eq!(summary.by_severity[2].1, 1);
        assert_eq!(summary.by_severity[1].1, 1);
    }

    #[test]
    fn test_summary_by_kind_sorted() {
        let mut diag = Diagnostics::new(100);
        diag.record("X", "msg", "error");
        diag.record("X", "msg", "error");
        diag.record("X", "msg", "error");
        diag.record("Y", "msg", "warning");
        let summary = diag.summary();
        assert_eq!(summary.by_kind[0].0, "X");
        assert_eq!(summary.by_kind[0].1, 3);
    }

    #[test]
    fn test_error_rate_per_minute_no_errors() {
        let diag = Diagnostics::new(10);
        assert_eq!(diag.error_rate_per_minute(), 0.0);
    }

    #[test]
    fn test_error_rate_per_minute_with_errors() {
        let mut diag = Diagnostics::new(100);
        for _ in 0..5 {
            diag.record("Err", "msg", "error");
        }
        let rate = diag.error_rate_per_minute();
        assert!(rate >= 5.0);
    }

    #[test]
    fn test_anomaly_detection_insufficient_data() {
        let diag = Diagnostics::new(100);
        assert!(!diag.detect_anomaly());
    }

    #[test]
    fn test_anomaly_detection_spike() {
        let mut diag = Diagnostics::new(200);
        for _ in 0..50 {
            diag.record("Err", "msg", "info");
        }
        for _ in 0..20 {
            diag.record("Err", "msg", "error");
        }
        assert!(diag.detect_anomaly());
    }

    #[test]
    fn test_anomaly_detection_no_spike() {
        let mut diag = Diagnostics::new(200);
        for _ in 0..50 {
            diag.record("Err", "msg", "info");
        }
        for _ in 0..2 {
            diag.record("Err", "msg", "error");
        }
        assert!(!diag.detect_anomaly());
    }

    #[test]
    fn test_record_entry() {
        let mut diag = Diagnostics::new(10);
        let entry = DiagnosticEntry {
            kind: "Custom".to_string(),
            message: "custom msg".to_string(),
            timestamp: Timestamp::now().0,
            severity: Severity::Critical,
        };
        diag.record_entry(entry);
        assert_eq!(diag.count(), 1);
        assert_eq!(diag.entries[0].severity, Severity::Critical);
    }

    #[test]
    fn test_diagnostics_bounded_with_severity_tracking() {
        let mut diag = Diagnostics::new(5);
        for i in 0..10 {
            diag.record("E", "msg", if i % 2 == 0 { "error" } else { "info" });
        }
        assert_eq!(diag.count(), 5);
        let summary = diag.summary();
        assert!(summary.total_entries <= 5);
    }
}
