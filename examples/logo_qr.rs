use qr_rs::image::{DynamicImage, RgbaImage};
use qr_rs::{QRBuilder, QRError};
use std::fs;

fn main() -> Result<(), QRError> {
    // 1. Create a dummy logo (50x50 red square)
    let mut logo = RgbaImage::new(50, 50);
    for pixel in logo.chunks_exact_mut(4) {
        pixel.copy_from_slice(&[255, 0, 0, 255]);
    }
    let logo_dynamic = DynamicImage::ImageRgba8(logo);

    // 2. Generate QR code with logo
    println!("Generating QR code with logo...");
    let png_data = QRBuilder::new()
        .url("https://www.rust-lang.org")
        .error_correction(qr_rs::qrcode::EcLevel::H) // High error correction for logos
        .build()?
        .to_png(400, Some(&logo_dynamic))?;

    fs::write("logo_qr.png", png_data)?;
    println!("Generated logo_qr.png");

    Ok(())
}
