use qr_rs::error::QRError;
use qr_rs::{LocationData, QRBuilder};
use std::fs;

fn main() -> Result<(), QRError> {
    // Generate Location QR code (e.g., Eiffel Tower)
    let location = LocationData {
        latitude: 48.8584,
        longitude: 2.2945,
    };

    let location_png = QRBuilder::new()
        .location(location)
        .build()?
        .to_png(300, None)?;

    fs::write("location.png", location_png)?;
    println!("Generated location.png");

    Ok(())
}
