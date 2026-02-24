#!/bin/bash
set -e

# Build the CLI
cargo build -p qr-cli

# Path to binary
QR_CLI=./target/debug/qr-cli

echo "Generating QR codes with new CLI options..."

# 1. Basic URL (Default settings)
$QR_CLI "https://www.rust-lang.org" -o rust_basic.png
echo "Generated rust_basic.png"

# 2. Text with Custom Colors (Red FG, Black BG)
$QR_CLI "Hello Red World" -o red_text.png --fg "#FF0000" --bg "#000000"
echo "Generated red_text.png"

# 3. High Error Correction and Scale
$QR_CLI "High Security" -o high_ec.png --error-correction-level high --scale 50
echo "Generated high_ec.png"

# 4. Force Overwrite
touch overwrite_me.png
$QR_CLI "Overwrite" -o overwrite_me.png --force
echo "Generated overwrite_me.png (forced)"

echo "Done! Check the generated PNG files."
