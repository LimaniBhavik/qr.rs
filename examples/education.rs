use qr_rs::{error::QRError, QRBuilder};
use std::fs;

fn main() -> Result<(), QRError> {
    let png = QRBuilder::new()
        .url("https://classroom.google.com/assignment")
        .build()?
        .to_png(600, None)?; // Large for smartboard
    fs::write("smartboard.png", png)?;
    println!("Generated Education QR.");
    Ok(())
}
