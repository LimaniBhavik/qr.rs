use qr_rs::{error::QRError, QRBuilder};
use std::fs;

fn main() -> Result<(), QRError> {
    let png = QRBuilder::new()
        .url("https://carfax.com/vin/1HGCM82633A00000")
        .build()?
        .to_png(300, None)?;
    fs::write("car_window_sticker.png", png)?;
    println!("Generated Automotive QR.");
    Ok(())
}
