use qr_rs::{error::QRError, QRBuilder};
use std::fs;

fn main() -> Result<(), QRError> {
    let png = QRBuilder::new()
        .url("https://docs.site.com/blueprints/v4")
        .build()?
        .to_png(400, None)?;
    fs::write("site_entrance.png", png)?;
    println!("Generated Construction QR.");
    Ok(())
}
