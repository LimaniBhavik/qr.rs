# QR.RS

<div align="center">

[![Crates.io](https://img.shields.io/crates/v/qr-rs.svg)](https://crates.io/crates/qr-rs)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Build Status](https://github.com/LimaniBhavik/qr.rs/workflows/CI/badge.svg)](https://github.com/LimaniBhavik/qr.rs/actions)

**A comprehensive, multi-target QR code generator for Rust.**

[Features](#features) • [Installation](#installation) • [Usage](#usage) • [Library](#library-integration) • [Contributing](#contributing)

</div>

---

## 🚀 Overview

**QR.RS** is a versatile QR code generator written in Rust. It supports generating QR codes for URLs, text, and vCard contact information across multiple platforms:
*   **CLI**: A powerful command-line interface for scripting and automation.
*   **GUI**: A native desktop application for visual interaction.
*   **Web**: A WebAssembly-powered browser application.
*   **Library**: A flexible Rust crate for integrating QR generation into your own projects.

## ✨ Features

*   **Versatile Data Modes**:
    *   🔗 **URL**: Automatic protocol formatting (e.g., `example.com` -> `https://example.com`).
    *   📝 **Text**: Encode any arbitrary text.
    *   👤 **Contact**: Generate vCard 3.0 compatible QR codes for easy contact sharing.
*   **🎨 Advanced Customization**:
    *   **Colors**: Custom foreground and background colors.
    *   **Error Correction**: Configurable levels (L, M, Q, H) to balance density and resilience.
    *   **Logo Embedding**: Overlay logos on your QR codes (CLI & Library).
*   **Cross-Platform**: Windows, Linux, macOS, and Web (WASM).

## 📦 Installation

### From Crates.io (CLI)

```bash
cargo install qr-cli
```

### From Source

Clone the repository and build:

```bash
git clone https://github.com/LimaniBhavik/qr.rs.git
cd qr.rs
cargo build --release
```

## 🛠 Usage

### Command Line Interface (CLI)

The CLI tool `qr-cli` allows you to generate QR codes directly from your terminal.

#### 1. Generate a URL QR Code
```bash
qr-cli url "https://www.rust-lang.org" -o rust_qr.png
```

#### 2. Generate a Text QR Code
```bash
qr-cli text "Hello, World!" -o text_qr.png
```

#### 3. Generate a Contact (vCard) QR Code
```bash
qr-cli contact \
  --first-name "Jane" \
  --last-name "Doe" \
  --email "jane@example.com" \
  --phone "+1234567890" \
  --organization "Rust Foundation" \
  -o contact_qr.png
```

#### 4. Customization Options
Customize colors, error correction, and add a logo:

```bash
qr-cli url "https://example.com" \
  --foreground "#FF5733" \
  --background "#FFFFFF" \
  --ec-level H \
  --logo ./my-logo.png \
  -o custom_qr.png
```

*   `--ec-level`: `L` (7%), `M` (15%), `Q` (25%), `H` (30%). Higher levels allow for more damage/logos but are denser.
*   `--foreground`/`--background`: Hex color codes (e.g., `#RRGGBB` or `#RRGGBBAA`).

#### 5. Interactive Mode
If you prefer a guided experience:
```bash
qr-cli interactive
```

---

### Desktop GUI

Run the graphical interface for a visual experience:

```bash
cargo run -p qr-gui
```
*   **Select Mode**: Tabs for URL, Text, and Contact.
*   **Customize**: Real-time color pickers and error correction settings.
*   **Preview**: Instant visual feedback.

---

### Web Application (WASM)

To run the web version locally:

1.  Install [Trunk](https://trunkrs.dev/):
    ```bash
    cargo install trunk
    ```
2.  Start the server:
    ```bash
    cd qr-web
    trunk serve
    ```
3.  Open `http://127.0.0.1:8080` in your browser.

---

## 📚 Library Integration

You can use `qr-rs` as a library in your own Rust projects.

Add to `Cargo.toml`:
```toml
[dependencies]
qr-rs = "0.4.0"
```

### Examples

#### Basic URL Generation
```rust
use qr_rs::{QRBuilder, QRError};

fn main() -> Result<(), QRError> {
    QRBuilder::new()
        .url("https://example.com")
        .build()?
        .to_png(300, None)?; // Generates a Vec<u8> of PNG data

    Ok(())
}
```

#### Advanced Customization
```rust
use qr_rs::{QRBuilder, ContactData, qrcode::EcLevel};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Custom colors: Red foreground, White background
    let fg = [255, 0, 0, 255];
    let bg = [255, 255, 255, 255];

    let contact = ContactData {
        first_name: "John".into(),
        last_name: "Doe".into(),
        email: "john@example.com".into(),
        ..Default::default()
    };

    let png_data = QRBuilder::new()
        .data(qr_rs::QRData::Contact(contact))
        .error_correction(EcLevel::Q)
        .colors(fg, bg)
        .build()?
        .to_png(400, None)?; // Pass Some(&dynamic_image) for logo

    std::fs::write("contact.png", png_data)?;
    Ok(())
}
```

See the `examples/` directory for more detailed code samples.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1.  Fork the repository.
2.  Create your feature branch (`git checkout -b feature/AmazingFeature`).
3.  Commit your changes (`git commit -m 'Add some AmazingFeature'`).
4.  Push to the branch (`git push origin feature/AmazingFeature`).
5.  Open a Pull Request.

## 📄 License

Distributed under the MIT License. See `LICENSE` for more information.
