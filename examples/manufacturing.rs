use qr_rs::{error::QRError, qrcode::EcLevel, QRBuilder};
use std::fs;

fn main() -> Result<(), QRError> {
    let png = QRBuilder::new()
        .url("internal://manuals/lathe-mk4")
        .error_correction(EcLevel::H) // High EC for dirty environments
        .build()?
        .to_png(300, None)?;
    fs::write("machine_tag.png", png)?;
    println!("Generated Manufacturing QR.");
    Ok(())
}
