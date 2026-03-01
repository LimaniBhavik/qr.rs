use qr_rs::{error::QRError, qrcode::EcLevel, QRBuilder};
use std::fs;

fn main() -> Result<(), QRError> {
    let hash = "hash://sha256/e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
    let png = QRBuilder::new()
        .text(hash)
        .error_correction(EcLevel::H)
        .build()?
        .to_png(200, None)?;
    fs::write("contract_verification.png", png)?;
    println!("Generated Legal QR.");
    Ok(())
}
