use qr_rs::error::QRError;
use qr_rs::{ContactData, QRBuilder};

fn main() -> Result<(), QRError> {
    // Generate URL QR code
    QRBuilder::new()
        .url("https://example.com")
        .build()?
        .to_png(300, None)?;
    println!("Generated URL QR code");

    // Generate Text QR code
    QRBuilder::new()
        .text("Hello World")
        .build()?
        .to_png(300, None)?;
    println!("Generated Text QR code");

    // Generate Contact QR code
    let contact = ContactData {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        email: "john@example.com".to_string(),
        phone: "+1234567890".to_string(),
        ..Default::default()
    };

    QRBuilder::new()
        .data(qr_rs::QRData::Contact(contact))
        .build()?
        .to_png(300, None)?;
    println!("Generated Contact QR code");

    Ok(())
}
