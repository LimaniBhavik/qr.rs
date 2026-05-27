use criterion::{black_box, criterion_group, criterion_main, Criterion};
use qr_rs::{QRBuilder, QRData};

fn bench_png_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("qr_png_generation");

    group.bench_function("generate_png_300", |b| {
        b.iter(|| {
            let data = QRData::URL("https://example.com/very/long/url/to/make/qr/complex".to_string());
            let mut builder = QRBuilder::new();
            builder = builder.error_correction(qr_rs::qrcode::EcLevel::H);
            builder = builder.data(data);

            if let Ok(generator) = builder.build() {
                let _ = black_box(generator.to_png(300, None));
            }
        })
    });

    group.finish();
}

criterion_group!(benches, bench_png_generation);
criterion_main!(benches);
