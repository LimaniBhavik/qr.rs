use qr_rs::{error::QRError, QRBuilder};
use std::fs;

fn main() -> Result<(), QRError> {
    let png = QRBuilder::new()
        .url("https://city.gov/report?loc=bus_stop_12")
        .build()?
        .to_png(300, None)?;
    fs::write("bus_stop_311.png", png)?;
    println!("Generated Government QR.");
    Ok(())
}
