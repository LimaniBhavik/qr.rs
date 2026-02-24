use qr_rs::error::QRError;
use qr_rs::{ContactData, QRBuilder};
use std::fs;

fn main() -> Result<(), QRError> {
    // Generate URL QR code
    let url_png = QRBuilder::new()
        .url("https://example.com")
        .build()?
        .to_png(300, None)?;
    fs::write("basic_url.png", url_png)?;
    println!("Generated basic_url.png");

    // Generate Text QR code
    let text_png = QRBuilder::new()
        .text("Hello World")
        .build()?
        .to_png(300, None)?;
    fs::write("basic_text.png", text_png)?;
    println!("Generated basic_text.png");

    // Generate Contact QR code
    let contact = ContactData {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        email: "john@example.com".to_string(),
        phone: "+1234567890".to_string(),
        ..Default::default()
    };

    let contact_png = QRBuilder::new()
        .data(qr_rs::QRData::Contact(contact))
        .build()?
        .to_png(300, None)?;
    fs::write("basic_contact.png", contact_png)?;
    println!("Generated basic_contact.png");

    Ok(())
}
