use qr_rs::{error::QRError, qrcode::EcLevel, QRBuilder};
use std::fs;

fn main() -> Result<(), QRError> {
    let png = QRBuilder::new()
        .text("PKG-Zebra-102938")
        .error_correction(EcLevel::H)
        .build()?
        .to_png(200, None)?; // Smaller size for shipping label
    fs::write("shipping_label.png", png)?;
    println!("Generated Logistics QR.");
    Ok(())
}
