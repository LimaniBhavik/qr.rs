use qr_rs::{error::QRError, ContactData, QRBuilder, QRData};
use std::fs;

fn main() -> Result<(), QRError> {
    let patient = ContactData {
        first_name: "Jane".into(),
        last_name: "Doe".into(),
        organization: "Patient ID: 887766".into(),
        ..Default::default()
    };
    let png = QRBuilder::new()
        .data(QRData::Contact(patient))
        .build()?
        .to_png(300, None)?;
    fs::write("patient_bracelet.png", png)?;
    println!("Generated Healthcare QR.");
    Ok(())
}
