use criterion::{black_box, criterion_group, criterion_main, Criterion};
use qr_rs::{QRBuilder, QRData};

fn bench_image_conversion(c: &mut Criterion) {
    let generator = QRBuilder::new()
        .data(QRData::URL(
            "https://example.com/very/long/url/for/testing/qr/code/generation".to_string(),
        ))
        .build()
        .unwrap();

    let mut group = c.benchmark_group("image_conversion");

    group.bench_function("to_png", |b| {
        b.iter(|| generator.to_png(black_box(300), None))
    });

    group.finish();
}

criterion_group!(benches, bench_image_conversion);
criterion_main!(benches);
