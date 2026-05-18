use qr_rs::error::QRError;
use qr_rs::image::{DynamicImage, Rgba, RgbaImage};
use qr_rs::qrcode::EcLevel;
use qr_rs::{ContactData, LocationData, QRBuilder, WifiData, WifiEncryption};
use std::fs;
use std::path::Path;

fn main() -> Result<(), QRError> {
    let output_dir = "demo_output";
    if !Path::new(output_dir).exists() {
        fs::create_dir(output_dir)?;
    }

    println!("Starting comprehensive demo...");

    // 1. URL with High Error Correction
    println!("Generating URL QR code...");
    let url_png = QRBuilder::new()
        .url("https://www.rust-lang.org")
        .error_correction(EcLevel::H)
        .build()?
        .to_png(400, None)?;
    fs::write(format!("{}/url.png", output_dir), url_png)?;

    // 2. Text with Custom Colors
    println!("Generating Text QR code with custom colors...");
    let fg = [0, 0, 128, 255]; // Navy Blue
    let bg = [255, 255, 0, 255]; // Yellow
    let text_png = QRBuilder::new()
        .text("Rust is awesome!")
        .colors(fg, bg)
        .build()?
        .to_png(400, None)?;
    fs::write(format!("{}/text_custom_colors.png", output_dir), text_png)?;

    // 3. Contact (vCard)
    println!("Generating Contact QR code...");
    let contact = ContactData {
        first_name: "Ferris".to_string(),
        last_name: "The Crab".to_string(),
        email: "ferris@rust-lang.org".to_string(),
        phone: "+123456789".to_string(),
        organization: "Rust Foundation".to_string(),
        website: "https://rust-lang.org".to_string(),
    };
    let contact_png = QRBuilder::new()
        .data(qr_rs::QRData::Contact(contact))
        .build()?
        .to_png(400, None)?;
    fs::write(format!("{}/contact.png", output_dir), contact_png)?;

    // 4. Wifi (WPA)
    println!("Generating Wifi QR code...");
    let wifi = WifiData {
        ssid: "RustNetwork".to_string(),
        password: "safeandfast".to_string(),
        encryption: WifiEncryption::WPA,
        hidden: false,
    };
    let wifi_png = QRBuilder::new().wifi(wifi).build()?.to_png(400, None)?;
    fs::write(format!("{}/wifi.png", output_dir), wifi_png)?;

    // 5. Location
    println!("Generating Location QR code...");
    let location = LocationData {
        latitude: 37.7749,
        longitude: -122.4194, // San Francisco
    };
    let location_png = QRBuilder::new()
        .location(location)
        .build()?
        .to_png(400, None)?;
    fs::write(format!("{}/location.png", output_dir), location_png)?;

    // 6. QR Code with Logo
    println!("Generating QR code with logo...");
    let mut logo = RgbaImage::new(50, 50);
    for pixel in logo.pixels_mut() {
        *pixel = Rgba([255, 0, 0, 255]); // Red square
    }
    let logo_dynamic = DynamicImage::ImageRgba8(logo);

    let logo_png = QRBuilder::new()
        .url("https://www.rust-lang.org")
        .error_correction(EcLevel::H)
        .build()?
        .to_png(400, Some(&logo_dynamic))?;
    fs::write(format!("{}/logo.png", output_dir), logo_png)?;

    println!("Demo complete! Check the '{}' directory.", output_dir);
    Ok(())
}
