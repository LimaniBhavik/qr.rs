use qr_rs::error::QRError;
use qr_rs::qrcode::EcLevel;
use qr_rs::QRBuilder;
use rayon::prelude::*;
use std::fs;
use std::path::Path;
use std::time::Instant;

struct Product {
    id: String,
    name: String,
    url: String,
}

fn main() -> Result<(), QRError> {
    let output_dir = "retail_output";
    if !Path::new(output_dir).exists() {
        fs::create_dir(output_dir)?;
    }

    println!("Starting Retail/E-commerce QR Code Generation Batch...");

    // Generate a larger catalog to demonstrate performance benefits
    let mut catalog = Vec::new();
    for i in 1..=50 {
        catalog.push(Product {
            id: format!("SKU-100{}", i),
            name: format!("Product {}", i),
            url: format!("https://shop.example.com/p/sku-100{}", i),
        });
    }

    let start_time = Instant::now();

    // Simulating batch processing for a retail catalog using parallel iteration
    catalog
        .into_par_iter()
        .try_for_each(|product| -> Result<(), QRError> {
            println!("Generating QR for {}: {}", product.id, product.name);

            // Retailers often use high error correction so codes scan even if printed small on tags
            let qr_png = QRBuilder::new()
                .url(&product.url)
                .error_correction(EcLevel::Q)
                .build()?
                .to_png(400, None)?;

            let filename = format!("{}/{}.png", output_dir, product.id);
            fs::write(&filename, qr_png)?;
            println!(" -> Saved to {}", filename);

            Ok(())
        })?;

    let duration = start_time.elapsed();

    println!(
        "Batch generation complete in {:.2?}. Files saved in '{}' directory.",
        duration, output_dir
    );

    // Clean up for CI/demo purposes (optional, but good practice for automated examples)
    // Uncomment the next line if you want the example to clean up after itself
    // fs::remove_dir_all(output_dir)?;

    Ok(())
}
