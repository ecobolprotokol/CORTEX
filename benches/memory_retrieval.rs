use criterion::{criterion_group, criterion_main, Criterion};

fn bench_memory_retrieval(c: &mut Criterion) {
    c.bench_function("retrieval_score_relevance", |b| {
        b.iter(|| {
            cortex::memory::retrieval::RetrievalEngine::score_relevance(
                "gravity force mass",
                "gravity is a fundamental force that attracts objects with mass"
            )
        });
    });
}

criterion_group!(benches, bench_memory_retrieval);
criterion_main!(benches);
