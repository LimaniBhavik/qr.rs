use criterion::{black_box, criterion_group, criterion_main, Criterion};
use qr_rs::{QRBuilder, QRData};

fn bench_image_generation(c: &mut Criterion) {
    let data = QRData::Text("A".repeat(1000));
    let generator = QRBuilder::new().data(data).build().unwrap();
    let size = 1000;

    c.bench_function("to_image_1000x1000", |b| {
        b.iter(|| {
            let _ = generator.to_image(black_box(size), None).unwrap();
        })
    });
}

criterion_group!(benches, bench_image_generation);
criterion_main!(benches);
