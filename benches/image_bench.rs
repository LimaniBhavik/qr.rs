use criterion::{black_box, criterion_group, criterion_main, Criterion};
use qr_rs::formats::QRData;
use qr_rs::generator::QRBuilder;

fn bench_to_png(c: &mut Criterion) {
    let generator = QRBuilder::new()
        .data(QRData::Text(
            "https://example.com/some/very/long/url/to/make/qr/code/bigger/and/more/complex"
                .to_string(),
        ))
        .build()
        .unwrap();

    c.bench_function("to_png_512", |b| {
        b.iter(|| {
            let result = generator.to_png(black_box(512), black_box(None));
            black_box(result.unwrap());
        })
    });
}

criterion_group!(benches, bench_to_png);
criterion_main!(benches);
