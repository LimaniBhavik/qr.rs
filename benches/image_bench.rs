use criterion::{black_box, criterion_group, criterion_main, Criterion};
use qr_rs::formats::QRData;
use qr_rs::generator::QRGenerator;

fn bench_to_image(c: &mut Criterion) {
    let generator = QRGenerator::new(QRData::URL(
        "https://example.com/very/long/url/for/testing/qr/code/generation".to_string(),
    ));

    c.bench_function("to_image", |b| {
        b.iter(|| {
            let _ = generator.to_image(black_box(500), None).unwrap();
        });
    });
}

criterion_group!(benches, bench_to_image);
criterion_main!(benches);
