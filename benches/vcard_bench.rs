use criterion::{criterion_group, criterion_main, Criterion};

fn empty_bench(_c: &mut Criterion) {}

criterion_group!(benches, empty_bench);
criterion_main!(benches);
