use qr_rs::{error::QRError, utils::WHITE, QRBuilder};
use std::fs;

fn main() -> Result<(), QRError> {
    let png = QRBuilder::new()
        .url("https://agency.com/property/123")
        .colors([26, 54, 93, 255], WHITE) // Dark Blue branding
        .build()?
        .to_png(400, None)?;
    fs::write("property_123.png", png)?;
    println!("Generated Real Estate QR.");
    Ok(())
}
