use clap::Parser;
use qrcode::QrCode;
use image::Luma;
use std::error::Error;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The text or URL to encode into the QR code
    #[arg(short, long)]
    text: String,

    /// The output file name (default: qr.png)
    #[arg(short, long, default_value = "qr.png")]
    output: String,
}

fn generate_qr_code(text: &str, output: &str) -> Result<(), Box<dyn Error>> {
    // Generate QR code
    let code = QrCode::new(text.as_bytes())?;

    // Render the QR code into an image
    let image = code.render::<Luma<u8>>().build();

    // Save the image
    image.save(output)?;

    Ok(())
}

fn main() {
    let args = Args::parse();

    match generate_qr_code(&args.text, &args.output) {
        Ok(_) => println!("QR code saved to {}", args.output),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_generate_qr_code() {
        let text = "https://example.com";
        let output = "test_qr.png";

        let result = generate_qr_code(text, output);
        assert!(result.is_ok());

        assert!(fs::metadata(output).is_ok());

        // Clean up
        let _ = fs::remove_file(output);
    }
}
