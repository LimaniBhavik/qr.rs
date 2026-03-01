use qr_rs::{error::QRError, ContactData, QRBuilder, QRData};
use std::fs;

fn main() -> Result<(), QRError> {
    let contact = ContactData {
        first_name: "Alice".into(),
        last_name: "Engineer".into(),
        email: "alice@company.com".into(),
        organization: "Tech Corp".into(),
        ..Default::default()
    };
    let png = QRBuilder::new()
        .data(QRData::Contact(contact))
        .build()?
        .to_png(200, None)?;
    fs::write("employee_badge.png", png)?;
    println!("Generated HR QR.");
    Ok(())
}
