use criterion::{criterion_group, criterion_main, Criterion};

fn bench_cognitive_loop(c: &mut Criterion) {
    let config = cortex::config::CortexConfig::default();
    let mut runtime = cortex::cortex::CortexRuntime::boot(config).unwrap();

    c.bench_function("cognitive_loop_single_input", |b| {
        b.iter(|| {
            runtime.process("What is the meaning of life?").unwrap()
        })
    });
}

criterion_group!(benches, bench_cognitive_loop);
criterion_main!(benches);
