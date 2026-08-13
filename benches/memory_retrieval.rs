use criterion::{criterion_group, criterion_main, Criterion};
use cortex::types::*;

fn bench_memory_retrieval(c: &mut Criterion) {
    let config = cortex::config::CortexConfig::default();
    let runtime = cortex::cortex::CortexRuntime::boot(config).unwrap();

    let query = MemoryQuery {
        query_type: MemoryQueryType::All,
        text: "test query".to_string(),
        concept_ids: Vec::new(),
        time_range: None,
        max_results: 10,
        min_confidence: 0.0,
    };

    c.bench_function("memory_retrieval", |b| {
        b.iter(|| {
            cortex::memory::retrieval::retrieve(
                &runtime.memory.state(),
                &query,
                &ContextState::initial(),
            ).unwrap()
        })
    });
}

criterion_group!(benches, bench_memory_retrieval);
criterion_main!(benches);
