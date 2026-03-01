use qr_rs::{error::QRError, QRBuilder};
use std::fs;

fn main() -> Result<(), QRError> {
    // Note: Use string for long addresses
    let png = QRBuilder::new()
        .text("bitcoin:1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa?amount=0.01")
        .build()?
        .to_png(300, None)?;
    fs::write("crypto_invoice.png", png)?;
    println!("Generated Finance QR.");
    Ok(())
}
