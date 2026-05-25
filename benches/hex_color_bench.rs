use criterion::{black_box, criterion_group, criterion_main, Criterion};
use qr_rs::utils::parse_hex_color;

fn bench_parse_hex_color(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_hex_color");

    group.bench_function("valid_6_char", |b| {
        b.iter(|| parse_hex_color(black_box("#FFFFFF")))
    });

    group.bench_function("valid_8_char", |b| {
        b.iter(|| parse_hex_color(black_box("#FF5733AA")))
    });

    group.bench_function("invalid_length", |b| {
        b.iter(|| parse_hex_color(black_box("#FFF")))
    });

    group.finish();
}

criterion_group!(benches, bench_parse_hex_color);
criterion_main!(benches);
