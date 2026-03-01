use qr_rs::{error::QRError, QRBuilder};
use std::fs;

fn main() -> Result<(), QRError> {
    let png = QRBuilder::new()
        .url("https://menu.com/table/4")
        .build()?
        .to_png(300, None)?;
    fs::write("table_4_menu.png", png)?;
    println!("Generated Hospitality QR.");
    Ok(())
}
