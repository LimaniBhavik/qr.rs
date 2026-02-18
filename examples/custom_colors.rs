use qr_rs::error::QRError;
use qr_rs::QRBuilder;

fn main() -> Result<(), QRError> {
    // Custom colors (Red foreground, White background)
    let fg = [255, 0, 0, 255];
    let bg = [255, 255, 255, 255];

    QRBuilder::new()
        .url("https://example.com/custom")
        .colors(fg, bg)
        .error_correction(qr_rs::qrcode::EcLevel::Q)
        .build()?
        .to_png(400, None)?;

    println!("Generated custom colored QR code");
    Ok(())
}
