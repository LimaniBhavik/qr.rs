use qr_rs::{error::QRError, QRBuilder, WifiData, WifiEncryption};
use std::fs;

fn main() -> Result<(), QRError> {
    let wifi = WifiData {
        ssid: "Corp-Guest".into(),
        password: "Welcome2024!".into(),
        encryption: WifiEncryption::WPA,
        hidden: false,
    };
    let png = QRBuilder::new().wifi(wifi).build()?.to_png(300, None)?;
    fs::write("guest_wifi.png", png)?;
    println!("Generated IT/Networking QR.");
    Ok(())
}
