use criterion::{criterion_group, criterion_main, Criterion};
use cortex::persistence::format::FormatHandler;
use cortex::types::state::CortexState;

fn bench_persistence_roundtrip(c: &mut Criterion) {
    let handler = FormatHandler::new();
    let state = CortexState::default();
    let bincode_data = bincode::serialize(&state).unwrap();

    c.bench_function("persistence_serialize", |b| {
        b.iter(|| handler.serialize(&bincode_data).unwrap());
    });

    let serialized = handler.serialize(&bincode_data).unwrap();
    c.bench_function("persistence_deserialize", |b| {
        b.iter(|| handler.deserialize(&serialized).unwrap());
    });
}

criterion_group!(benches, bench_persistence_roundtrip);
criterion_main!(benches);
