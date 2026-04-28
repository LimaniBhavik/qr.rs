# Case Study: Retail & E-commerce

## The Challenge
In modern retail, connecting physical products to digital experiences is crucial. A clothing brand wants to add QR codes to all physical product tags. When scanned, these codes should take the customer directly to the product's online page where they can see reviews, check alternative sizes/colors, and easily reorder.

Generating these codes manually for thousands of SKUs is impractical. The company needs a reliable, scriptable solution to generate high-quality QR codes in bulk as part of their automated catalog publishing pipeline.

## The Solution: `qr-scan-rs` CLI

The `qr-scan-rs` tool suite, specifically the `qr-cli` component, is perfect for this automation.

### Why `qr-cli`?
*   **Scriptability**: Easily integrated into bash scripts or CI/CD pipelines.
*   **High Error Correction**: Retail tags get bent, scratched, or partially obscured. Using `-l quartile` or `-l high` ensures the code remains scannable even if damaged.
*   **Color Customization**: The brand can easily match the QR code to their specific Pantone/Hex colors using `-f` and `-b` to maintain brand identity.

### Implementation Example

Imagine a bash script iterating through a CSV of products:

```bash
#!/bin/bash

# Simulated CSV data: SKU, URL
# SKU-1001,https://shop.example.com/p/sku-1001
# SKU-1002,https://shop.example.com/p/sku-1002

while IFS=, read -r sku url
do
  echo "Generating QR for $sku..."

  # Use qr-cli to generate a high-res, high error-correction QR code
  # -l quartile : 25% error correction
  # -o : output file
  # -f : brand primary color (e.g., dark navy)

  qr-cli "$url" \
    --error-correction-level quartile \
    --fg "#0B192C" \
    --output "./tags/${sku}_tag.png"

done < catalog.csv
```

## Library Integration
If the retail backend is written in Rust, they can bypass the CLI and generate codes directly using the `qr-scan-rs` core library.

Check out the executable example:
```bash
cargo run --example retail_ecommerce
```
This example demonstrates generating a batch of product URLs programmatically.
