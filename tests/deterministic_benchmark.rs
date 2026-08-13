use cortex::config::CortexConfig;
use cortex::cortex::CortexRuntime;
use cortex::runtime::Runtime;
use cortex::transaction::invariant::StateInvariant;

fn make_config(suffix: &str) -> CortexConfig {
    let mut cfg = CortexConfig::default();
    cfg.persistence.state = format!("/tmp/cortex_bench_{}.cx", suffix);
    cfg.persistence.checkpoint_interval = 100;
    cfg.learning.consolidation_interval = 50;
    cfg
}

#[test]
fn benchmark_cognitive_loop_throughput() {
    let cfg = make_config("throughput");
    let mut rt = CortexRuntime::new(cfg).unwrap();
    rt.boot().unwrap();

    let inputs: Vec<String> = (0..100)
        .map(|i| format!("Benchmark observation number {}", i))
        .collect();

    let start = std::time::Instant::now();
    for input in &inputs {
        let _ = rt.process(input);
    }
    let total_duration = start.elapsed();
    let throughput = 100.0 / total_duration.as_secs_f64();

    println!("=== Throughput Benchmark ===");
    println!("100 iterations in {:?}", total_duration);
    println!("Throughput: {:.2} ops/sec", throughput);
    println!(
        "Avg latency: {:.2}ms",
        total_duration.as_millis() as f64 / 100.0
    );

    assert!(throughput > 1.0, "Throughput should be > 1 ops/sec");

    rt.shutdown().unwrap();
    let _ = std::fs::remove_file(&rt.config.persistence.state);
}

#[test]
fn benchmark_memory_retrieval() {
    let cfg = make_config("retrieval");
    let mut rt = CortexRuntime::new(cfg).unwrap();
    rt.boot().unwrap();

    for i in 0..200 {
        let _ = rt.process(&format!("Memory population cycle {}", i));
    }

    let start = std::time::Instant::now();
    for i in 0..100 {
        let _ = rt.process(&format!("Retrieval query {}", i));
    }
    let retrieval_duration = start.elapsed();

    println!("=== Retrieval Benchmark ===");
    println!(
        "100 retrieval queries after 200 insertions: {:?}",
        retrieval_duration
    );
    println!(
        "Avg retrieval latency: {:.2}ms",
        retrieval_duration.as_millis() as f64 / 100.0
    );

    rt.shutdown().unwrap();
    let _ = std::fs::remove_file(&rt.config.persistence.state);
}

#[test]
fn benchmark_persistence_save_load() {
    let cfg = make_config("persistence_bench");
    let mut rt = CortexRuntime::new(cfg).unwrap();
    rt.boot().unwrap();

    for i in 0..100 {
        let _ = rt.process(&format!("Persistence benchmark cycle {}", i));
    }

    let start = std::time::Instant::now();
    rt.save_state().unwrap();
    let save_duration = start.elapsed();

    let data_size = std::fs::metadata(&rt.config.persistence.state)
        .map(|m| m.len())
        .unwrap_or(0);

    println!("=== Persistence Benchmark ===");
    println!("Save duration: {:?}", save_duration);
    println!("State file size: {} bytes", data_size);
    println!(
        "Save throughput: {:.2} MB/sec",
        (data_size as f64 / 1024.0 / 1024.0) / save_duration.as_secs_f64()
    );

    rt.shutdown().unwrap();
    let _ = std::fs::remove_file(&rt.config.persistence.state);
}

#[test]
fn benchmark_state_serialization() {
    let cfg = make_config("serial_bench");
    let mut rt = CortexRuntime::new(cfg).unwrap();
    rt.boot().unwrap();

    for i in 0..500 {
        let _ = rt.process(&format!("Serialization benchmark cycle {}", i));
    }

    let start = std::time::Instant::now();
    for _ in 0..100 {
        let _ = bincode::serialize(&rt.state).unwrap();
    }
    let serialize_duration = start.elapsed();

    let data = bincode::serialize(&rt.state).unwrap();
    let start = std::time::Instant::now();
    for _ in 0..100 {
        let _: cortex::types::state::CortexState = bincode::deserialize(&data).unwrap();
    }
    let deserialize_duration = start.elapsed();

    println!("=== Serialization Benchmark ===");
    println!("State size: {} bytes", data.len());
    println!(
        "Serialize: {:?} ({}us avg)",
        serialize_duration,
        serialize_duration.as_micros() / 100
    );
    println!(
        "Deserialize: {:?} ({}us avg)",
        deserialize_duration,
        deserialize_duration.as_micros() / 100
    );

    rt.shutdown().unwrap();
    let _ = std::fs::remove_file(&rt.config.persistence.state);
}

#[test]
fn benchmark_learning_stability() {
    let cfg = make_config("learning_bench");
    let mut rt = CortexRuntime::new(cfg).unwrap();
    rt.boot().unwrap();

    let mut learning_rates = Vec::new();
    for i in 0..200 {
        let _ = rt.process(&format!("Learning stability cycle {}", i));
        learning_rates.push(rt.state.learning.learning_rate);
    }

    let max_lr = learning_rates.iter().cloned().fold(0.0f32, f32::max);
    let min_lr = learning_rates.iter().cloned().fold(1.0f32, f32::min);
    let lr_range = max_lr - min_lr;

    println!("=== Learning Stability Benchmark ===");
    println!("Learning rate range: [{:.6}, {:.6}]", min_lr, max_lr);
    println!("Learning rate range width: {:.6}", lr_range);

    assert!(
        lr_range < 0.1,
        "Learning rate should be stable, range: {}",
        lr_range
    );

    rt.shutdown().unwrap();
    let _ = std::fs::remove_file(&rt.config.persistence.state);
}
