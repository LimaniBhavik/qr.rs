use qr_rs::error::QRError;
use qr_rs::qrcode::EcLevel;
use qr_rs::QRBuilder;
use std::fs;
use std::path::Path;

struct Product {
    id: &'static str,
    name: &'static str,
    url: &'static str,
}

fn main() -> Result<(), QRError> {
    let output_dir = "retail_output";
    if !Path::new(output_dir).exists() {
        fs::create_dir(output_dir)?;
    }

    println!("Starting Retail/E-commerce QR Code Generation Batch...");

    let catalog = vec![
        Product {
            id: "SKU-1001",
            name: "Classic T-Shirt",
            url: "https://shop.example.com/p/sku-1001",
        },
        Product {
            id: "SKU-1002",
            name: "Denim Jeans",
            url: "https://shop.example.com/p/sku-1002",
        },
        Product {
            id: "SKU-1003",
            name: "Running Shoes",
            url: "https://shop.example.com/p/sku-1003",
        },
    ];

    // Simulating batch processing for a retail catalog
    for product in catalog {
        println!("Generating QR for {}: {}", product.id, product.name);

        // Retailers often use high error correction so codes scan even if printed small on tags
        let qr_png = QRBuilder::new()
            .url(product.url)
            .error_correction(EcLevel::Q)
            .build()?
            .to_png(400, None)?;

        let filename = format!("{}/{}.png", output_dir, product.id);
        fs::write(&filename, qr_png)?;
        println!(" -> Saved to {}", filename);
    }

    println!(
        "Batch generation complete. Files saved in '{}' directory.",
        output_dir
    );

    // Clean up for CI/demo purposes (optional, but good practice for automated examples)
    // Uncomment the next line if you want the example to clean up after itself
    // fs::remove_dir_all(output_dir)?;

    Ok(())
}
