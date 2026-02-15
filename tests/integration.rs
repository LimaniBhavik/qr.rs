use qr_rs::{QRBuilder, ContactData, QRData};

#[test]
fn test_url_qr_generation() {
    let data = QRData::URL("example.com".to_string());
    // Use Builder to create generator
    let generator = QRBuilder::new()
        .data(data)
        .build()
        .unwrap();

    // Check if we can generate PNG
    let png = generator.to_png(300, None);
    assert!(png.is_ok());
    assert!(!png.unwrap().is_empty());
}

#[test]
fn test_contact_qr_generation() {
    let contact = ContactData {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        ..Default::default()
    };
    let data = QRData::Contact(contact);
    let generator = QRBuilder::new()
        .data(data)
        .build()
        .unwrap();

    let svg = generator.to_svg();
    assert!(svg.is_ok());
    assert!(svg.unwrap().contains("<svg"));
}

#[test]
fn test_custom_colors() {
    let fg = [255, 0, 0, 255]; // Red
    let bg = [0, 0, 255, 255]; // Blue

    let data = QRData::Text("Test Colors".to_string());

    let generator = QRBuilder::new()
        .data(data)
        .colors(fg, bg)
        .build()
        .unwrap();

    // Internal struct fields are private, so we test behavior
    let image = generator.to_image(100, None).unwrap();

    // Simple check: ensure image is RGBA8
    assert!(image.as_rgba8().is_some());
}
