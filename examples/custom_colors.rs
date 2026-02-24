use qr_rs::error::QRError;
use qr_rs::qrcode::EcLevel;
use qr_rs::QRBuilder;
use std::fs;

fn main() -> Result<(), QRError> {
    // Custom colors (Red foreground, White background)
    let fg = [255, 0, 0, 255];
    let bg = [255, 255, 255, 255];

    let png_data = QRBuilder::new()
        .url("https://example.com/custom")
        .colors(fg, bg)
        .error_correction(EcLevel::Q)
        .build()?
        .to_png(400, None)?;

    fs::write("custom_colors.png", png_data)?;

    println!("Generated custom_colors.png");
    Ok(())
}
