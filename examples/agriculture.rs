use qr_rs::{error::QRError, QRBuilder};
use std::fs;

fn main() -> Result<(), QRError> {
    let png = QRBuilder::new()
        .url("https://farm-trace.com/batch/2023-A-99")
        .build()?
        .to_png(200, None)?;
    fs::write("produce_packaging.png", png)?;
    println!("Generated Agriculture QR.");
    Ok(())
}
