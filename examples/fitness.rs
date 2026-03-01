use qr_rs::{error::QRError, QRBuilder};
use std::fs;

fn main() -> Result<(), QRError> {
    let png = QRBuilder::new()
        .url("https://gym.com/equipment/leg-press-tutorial")
        .build()?
        .to_png(200, None)?;
    fs::write("equipment_sticker.png", png)?;
    println!("Generated Fitness QR.");
    Ok(())
}
