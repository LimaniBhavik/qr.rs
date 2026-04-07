use qr_rs::{QRBuilder, QRData};
use std::time::Instant;

fn main() {
    let data = QRData::Text("A".repeat(1000));
    let generator = QRBuilder::new().data(data).build().unwrap();

    let size = 1000;

    // Warm up
    for _ in 0..5 {
        let _ = generator.to_image(size, None).unwrap();
    }

    let iterations = 50;

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = generator.to_image(size, None).unwrap();
    }
    let duration = start.elapsed();

    println!(
        "Generation (total for {} iterations): {:?}",
        iterations,
        duration
    );
}
