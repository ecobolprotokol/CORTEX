use criterion::{criterion_group, criterion_main, Criterion};
use cortex::types::*;

fn bench_persistence_save_load(c: &mut Criterion) {
    let config = cortex::config::CortexConfig::default();
    let runtime = cortex::cortex::CortexRuntime::boot(config).unwrap();

    let state = runtime.state.clone();
    let path = "/tmp/bench_cortex.cx";

    let mut group = c.benchmark_group("persistence");

    group.bench_function("save", |b| {
        b.iter(|| {
            cortex::persistence::format::save_cx(path, &state).unwrap();
        })
    });

    group.bench_function("load", |b| {
        b.iter(|| {
            let _ = cortex::persistence::format::load_cx(path);
        })
    });

    group.finish();
    let _ = std::fs::remove_file(path);
}

criterion_group!(benches, bench_persistence_save_load);
criterion_main!(benches);
