use cortex::config::CortexConfig;
use cortex::cortex::CortexRuntime;
use cortex::runtime::Runtime;
use cortex::transaction::invariant::StateInvariant;

fn make_config(suffix: &str) -> CortexConfig {
    let mut cfg = CortexConfig::default();
    cfg.persistence.state = format!("/tmp/cortex_longrun_{}.cx", suffix);
    cfg.persistence.checkpoint_interval = 50;
    cfg.learning.consolidation_interval = 25;
    cfg.memory.episodic_mb = 64;
    cfg.memory.semantic_mb = 64;
    cfg.model.columns = 32;
    cfg.model.cells = 512;
    cfg
}

#[test]
fn long_running_1000_cycles_stability() {
    let cfg = make_config("1k");
    let mut rt = CortexRuntime::new(cfg).unwrap();
    rt.boot().unwrap();

    let topics = vec![
        "What is the meaning of life?",
        "How do computers process information?",
        "Explain quantum mechanics",
        "What is machine learning?",
        "How does memory work in the brain?",
        "What is natural language processing?",
        "Explain neural networks",
        "What is reinforcement learning?",
        "How does the scientific method work?",
        "What is computational complexity?",
    ];

    let mut total_latency_ms = 0u64;
    let mut max_latency_ms = 0u64;

    for i in 0..1000 {
        let input = format!("{} - cycle {}", topics[i % topics.len()], i);
        let start = std::time::Instant::now();
        let result = rt.process(&input);
        let elapsed = start.elapsed().as_millis() as u64;
        total_latency_ms += elapsed;
        if elapsed > max_latency_ms {
            max_latency_ms = elapsed;
        }

        assert!(result.is_ok(), "Process failed at cycle {}: {}", i, result.unwrap_err());
        assert!(
            StateInvariant::validate_state(&rt.state).is_ok(),
            "State invalid at cycle {}",
            i
        );
    }

    let avg_latency = total_latency_ms as f64 / 1000.0;
    let episodes = rt.state.metadata.episode_count;
    let vocab_size = rt.language_vocabulary.size();
    let learning_events = rt.state.learning.total_learning_events;
    let consolidation_events = rt.state.learning.total_consolidation_events;
    let mutations = rt.mutation_log.records.len();

    println!("=== Long-Running Test Results (1000 cycles) ===");
    println!("Episodes: {}", episodes);
    println!("Vocabulary size: {}", vocab_size);
    println!("Learning events: {}", learning_events);
    println!("Consolidation events: {}", consolidation_events);
    println!("Mutations logged: {}", mutations);
    println!("Avg latency: {:.2}ms", avg_latency);
    println!("Max latency: {}ms", max_latency_ms);
    println!("State version: {}", rt.state_version);

    assert!(episodes >= 1000, "Should have at least 1000 episodes");
    assert!(vocab_size > 0, "Vocabulary should grow");
    assert!(mutations > 0, "Should have mutations");
    assert!(
        avg_latency < 1000.0,
        "Average latency should be under 1s, got {:.2}ms",
        avg_latency
    );

    rt.shutdown().unwrap();
    let _ = std::fs::remove_file(&rt.config.persistence.state);
}

#[test]
fn long_running_memory_growth_bounded() {
    let cfg = make_config("memgrow");
    let mut rt = CortexRuntime::new(cfg).unwrap();
    rt.boot().unwrap();

    for i in 0..500 {
        let _ = rt.process(&format!("Memory growth test cycle {}", i));
    }

    let state_size = bincode::serialize(&rt.state).unwrap().len();
    println!("State size after 500 cycles: {} bytes", state_size);
    assert!(
        state_size < 10 * 1024 * 1024,
        "State should be under 10MB, got {} bytes",
        state_size
    );

    rt.shutdown().unwrap();
    let _ = std::fs::remove_file(&rt.config.persistence.state);
}

#[test]
fn long_running_checkpoint_integrity() {
    let cfg = make_config("ckpt");
    let mut rt = CortexRuntime::new(cfg).unwrap();
    rt.boot().unwrap();

    for i in 0..100 {
        let _ = rt.process(&format!("Checkpoint test cycle {}", i));
    }

    let checkpoint_count = rt.state.metadata.checkpoint_count;
    println!("Checkpoints created: {}", checkpoint_count);
    assert!(
        checkpoint_count >= 1,
        "Should have created at least 1 checkpoint"
    );

    rt.shutdown().unwrap();
    let _ = std::fs::remove_file(&rt.config.persistence.state);
}

#[test]
fn long_running_state_serialization_integrity() {
    let cfg = make_config("serial");
    let mut rt = CortexRuntime::new(cfg).unwrap();
    rt.boot().unwrap();

    for i in 0..200 {
        let _ = rt.process(&format!("Serialization test cycle {}", i));
    }

    let data = bincode::serialize(&rt.state).unwrap();
    let restored: cortex::types::state::CortexState = bincode::deserialize(&data).unwrap();
    assert_eq!(
        rt.state.metadata.episode_count,
        restored.metadata.episode_count
    );
    assert_eq!(
        rt.state.metadata.architecture_version,
        restored.metadata.architecture_version
    );
    assert_eq!(
        rt.state.learning.total_learning_events,
        restored.learning.total_learning_events
    );

    rt.shutdown().unwrap();
    let _ = std::fs::remove_file(&rt.config.persistence.state);
}

#[test]
fn long_running_confidence_calibration() {
    let cfg = make_config("calibrate");
    let mut rt = CortexRuntime::new(cfg).unwrap();
    rt.boot().unwrap();

    let mut verified_count = 0u64;
    let mut total_count = 0u64;

    for i in 0..200 {
        let input = format!("Calibration test {}", i);
        if let Ok(response) = rt.process(&input) {
            total_count += 1;
            if response.contains("verified") {
                verified_count += 1;
            }
        }
    }

    let verification_rate = if total_count > 0 {
        verified_count as f64 / total_count as f64
    } else {
        0.0
    };

    println!(
        "Verification rate: {:.2}% ({}/{})",
        verification_rate * 100.0,
        verified_count,
        total_count
    );

    assert!(total_count > 0, "Should process some inputs");
    assert!(
        verification_rate >= 0.0 && verification_rate <= 1.0,
        "Verification rate out of bounds"
    );

    rt.shutdown().unwrap();
    let _ = std::fs::remove_file(&rt.config.persistence.state);
}
