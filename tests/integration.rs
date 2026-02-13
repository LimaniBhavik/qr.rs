use qr_rs::{ContactData, QRData, QRGenerator};

#[test]
fn test_url_qr_generation() {
    let generator = QRGenerator::new();
    let data = QRData::URL("example.com".to_string());
    let qr = generator.generate(&data).unwrap();

    // Check if we can generate PNG
    let png = generator.to_png(&qr, 300);
    assert!(png.is_ok());
    assert!(!png.unwrap().is_empty());
}

#[test]
fn test_contact_qr_generation() {
    let generator = QRGenerator::new();
    let contact = ContactData {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        ..Default::default()
    };
    let data = QRData::Contact(contact);
    let qr = generator.generate(&data).unwrap();

    let svg = generator.to_svg(&qr);
    assert!(svg.contains("<svg"));
}
