use qr_rs::{error::QRError, QRBuilder};
use std::fs;

fn main() -> Result<(), QRError> {
    let png = QRBuilder::new()
        .url("https://youtu.be/dQw4w9WgXcQ") // Trailer link
        .colors([255, 0, 0, 255], [0, 0, 0, 255])
        .build()?
        .to_png(400, None)?;
    fs::write("movie_poster.png", png)?;
    println!("Generated Entertainment QR.");
    Ok(())
}
