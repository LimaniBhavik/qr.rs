use qr_rs::error::QRError;
use qr_rs::{QRBuilder, WifiData, WifiEncryption};
use std::fs;

fn main() -> Result<(), QRError> {
    // Generate WPA Wifi QR code
    let wifi_wpa = WifiData {
        ssid: "MyHomeNetwork".to_string(),
        password: "securepassword123".to_string(),
        encryption: WifiEncryption::WPA,
        hidden: false,
    };

    let wpa_png = QRBuilder::new().wifi(wifi_wpa).build()?.to_png(300, None)?;

    fs::write("wifi_wpa.png", wpa_png)?;
    println!("Generated wifi_wpa.png");

    // Generate Open Wifi QR code
    let wifi_open = WifiData {
        ssid: "FreePublicWifi".to_string(),
        password: "".to_string(),
        encryption: WifiEncryption::Nopass,
        hidden: false,
    };

    let open_png = QRBuilder::new()
        .wifi(wifi_open)
        .build()?
        .to_png(300, None)?;

    fs::write("wifi_open.png", open_png)?;
    println!("Generated wifi_open.png");

    Ok(())
}
