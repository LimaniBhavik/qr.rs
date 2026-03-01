use qr_rs::{error::QRError, LocationData, QRBuilder};
use std::fs;

fn main() -> Result<(), QRError> {
    let loc = LocationData {
        latitude: 36.1069,
        longitude: -112.1129, // Grand Canyon
    };
    let png = QRBuilder::new().location(loc).build()?.to_png(300, None)?;
    fs::write("trail_marker.png", png)?;
    println!("Generated Tourism QR.");
    Ok(())
}
