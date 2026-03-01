use qr_rs::{error::QRError, QRBuilder};
use std::fs;

fn main() -> Result<(), QRError> {
    let png = QRBuilder::new()
        .url("https://promo.campaign.com")
        .colors([255, 100, 0, 255], [20, 20, 20, 255]) // Orange on Dark Grey
        .build()?
        .to_png(500, None)?;
    fs::write("marketing_poster.png", png)?;
    println!("Generated Marketing QR.");
    Ok(())
}
