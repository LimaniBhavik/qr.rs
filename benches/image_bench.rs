use criterion::{black_box, criterion_group, criterion_main, Criterion};
use qr_rs::generator::QRGenerator;
use qr_rs::formats::QRData;
use image::{Luma, RgbaImage, Rgba};
use std::iter;

fn bench_to_image(c: &mut Criterion) {
    let generator = QRGenerator::new(QRData::Text("A reasonably long text for QR code benchmarking purposes. This needs to be long enough to generate a somewhat large QR code matrix.".to_string()));

    let qr = generator.generate().unwrap();
    let qr_image = qr.render::<Luma<u8>>().min_dimensions(2000, 2000).build();
    let width = qr_image.width();
    let height = qr_image.height();

    let mut group = c.benchmark_group("QR Image Mapping");

    group.bench_function("current_pixel_mapping_2000x2000", |b| {
        b.iter(|| {
            let mut image = RgbaImage::new(width, height);
            for (target_pixel, pixel) in image.pixels_mut().zip(qr_image.pixels()) {
                *target_pixel = if pixel.0[0] == 0 {
                    Rgba([0, 0, 0, 255])
                } else {
                    Rgba([255, 255, 255, 255])
                };
            }
            black_box(image)
        })
    });

    group.bench_function("flat_map_from_raw_struct", |b| {
        b.iter(|| {
            let fg = [0u8, 0, 0, 255];
            let bg = [255u8, 255, 255, 255];
            let pixels: Vec<u8> = qr_image.pixels().flat_map(|p| {
                if p.0[0] == 0 { fg } else { bg }
            }).collect();
            let image = RgbaImage::from_raw(width, height, pixels).unwrap();
            black_box(image)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_to_image);
criterion_main!(benches);
