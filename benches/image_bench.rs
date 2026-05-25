use criterion::{black_box, criterion_group, criterion_main, Criterion};
use qr_rs::formats::QRData;
use qr_rs::generator::QRGenerator;

fn bench_to_png(c: &mut Criterion) {
    let generator = QRGenerator::new(QRData::URL("https://example.com/a/very/long/url/to/make/the/qr/code/somewhat/large".to_string()));

    c.bench_function("to_png_300", |b| {
        b.iter(|| {
            let result = generator.to_png(black_box(300), black_box(None)).unwrap();
            black_box(result);
        })
    });
}

criterion_group!(benches, bench_to_png);
criterion_main!(benches);
