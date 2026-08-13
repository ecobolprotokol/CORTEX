use cortex::learning::attribution::AttributionEngine;
use cortex::learning::replay::ReplayBuffer;
use cortex::learning::signal::SignalGenerator;
use cortex::learning::stability::StabilityGuard;

#[test]
fn test_signal_generator() {
    let generator = SignalGenerator::new();
    let signal = generator.generate(0.5, 0.3);
    assert!(signal.magnitude > 0.0);
}

#[test]
fn test_attribution_engine() {
    let engine = AttributionEngine::new();
    let attribution = engine.attribute(0.3, "input error");
    assert!(attribution.magnitude > 0.0);
}

#[test]
fn test_replay_buffer() {
    let mut buffer = ReplayBuffer::new(10);
    buffer.add("experience 1", 0.8);
    buffer.add("experience 2", 0.5);
    buffer.add("experience 3", 0.9);
    let sampled = buffer.sample(2);
    assert_eq!(sampled.len(), 2);
}

#[test]
fn test_stability_guard() {
    let guard = StabilityGuard::new(0.01, 0.1);
    assert!(guard.check_stability(0.005));
    assert!(!guard.check_stability(0.02));
    let clamped = guard.clamp_update(0.5);
    assert!(clamped.abs() <= 0.1);
}
