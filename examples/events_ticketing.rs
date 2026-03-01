use qr_rs::{error::QRError, qrcode::EcLevel, QRBuilder};
use std::fs;

fn main() -> Result<(), QRError> {
    let ticket_id = "TKT-998877-SECURE-HASH";
    let png = QRBuilder::new()
        .text(ticket_id)
        .error_correction(EcLevel::H) // High EC for fast scanning at gates
        .build()?
        .to_png(300, None)?;
    fs::write("ticket_qr.png", png)?;
    println!("Generated Event Ticket QR.");
    Ok(())
}
