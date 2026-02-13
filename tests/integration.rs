use qr_rs::{QRGenerator, QRData, ContactData};
use qr_rs::image::Rgba;

#[test]
fn test_url_qr_generation() {
    let generator = QRGenerator::new();
    let data = QRData::URL("example.com".to_string());
    let qr = generator.generate(&data).unwrap();

    // Check if we can generate PNG
    let png = generator.to_png(&qr, 300, None);
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

#[test]
fn test_custom_colors() {
    let fg = [255, 0, 0, 255]; // Red
    let bg = [0, 0, 255, 255]; // Blue

    let generator = QRGenerator::new().with_colors(fg, bg);

    assert_eq!(generator.foreground_color, Rgba(fg));
    assert_eq!(generator.background_color, Rgba(bg));

    let data = QRData::Text("Test Colors".to_string());
    let qr = generator.generate(&data).unwrap();
    let image = generator.to_image(&qr, 100, None).unwrap();

    // Simple check: ensure image is RGBA8
    assert!(image.as_rgba8().is_some());
}
