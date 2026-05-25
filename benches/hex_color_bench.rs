use criterion::{black_box, criterion_group, criterion_main, Criterion};
use qr_rs::utils::parse_hex_color;

fn bench_parse_hex_color(c: &mut Criterion) {
    c.bench_function("parse_hex_color_valid", |b| {
        b.iter(|| {
            black_box(parse_hex_color("#FFFFFF"));
            black_box(parse_hex_color("#1a2b3c4d"));
            black_box(parse_hex_color("000000FF"));
        })
    });

    c.bench_function("parse_hex_color_invalid", |b| {
        b.iter(|| {
            black_box(parse_hex_color("#FFF"));
            black_box(parse_hex_color("GGGGGG"));
            black_box(parse_hex_color("#FF🚀000"));
        })
    });
}

criterion_group!(benches, bench_parse_hex_color);
criterion_main!(benches);
