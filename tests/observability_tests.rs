use cortex::observability::diagnostics::Diagnostics;

#[test]
fn test_diagnostics_health() {
    let diag = Diagnostics::new();
    assert!(diag.is_healthy());
}

#[test]
fn test_diagnostics_metrics() {
    let diag = Diagnostics::new();
    let metrics = diag.collect();
    assert!(metrics.uptime_seconds == 0);
    assert_eq!(metrics.episode_count, 0);
}

#[test]
fn test_diagnostics_default_metrics() {
    let metrics = cortex::observability::diagnostics::Metrics::default();
    assert_eq!(metrics.prediction_error, 0.0);
    assert_eq!(metrics.memory_pressure, 0.0);
}
