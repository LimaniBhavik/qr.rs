# QR.RS

Project Overview
Create a comprehensive QR code generator in Rust that supports URLs, text, and contact information (vCard). The project should be implemented as a multi-target application supporting CLI, GUI desktop, and web interfaces.

Repository: https://github.com/LimaniBhavik/qr.rs

## Features

### Core Features
- ✅ URL QR codes with automatic protocol formatting (ensure https:// prefix for proper URL scanning, not Google search)
- ✅ Text Mode: Generate QR codes for arbitrary text content
- ✅ Contact Mode: Generate QR codes with vCard 3.0 format for contact information including: First Name / Last Name Phone Number Email Address Organization Website URL
- ✅ Custom colors (foreground/background)
- ✅ Multiple error correction levels (L, M, Q, H)
- ✅ Logo/brand embedding
- ✅ SVG and PNG output formats

### Platform Support
- ✅ **CLI**: Interactive and command-line modes
- ✅ **GUI**: Native desktop application (Linux, Windows, macOS)
- ✅ **Web**: Browser-based WASM application

## Project Structure

- `qr.rs/`: Workspace root
- `src/`: Core library (shared logic)
- `qr-cli/`: CLI application
- `qr-gui/`: GUI desktop application (eframe/egui)
- `qr-web/`: Web application (Yew/WASM)

## Usage

### CLI

```bash
# Install (if published) or run from source
cargo run -p qr-cli -- url "example.com" -o qr.png

# Custom colors and error correction
cargo run -p qr-cli -- url "example.com" --foreground "#FF0000" --background "#FFFFFF" --ec-level Q -o custom.png

# With logo
cargo run -p qr-cli -- url "example.com" --logo logo.png -o branded.png

# Interactive mode
cargo run -p qr-cli -- interactive
```

### GUI

```bash
cargo run -p qr-gui
```

### Web

Requires `trunk`.

```bash
cd qr-web
trunk serve
```

## Library Usage

```rust
use qr_rs::{QRBuilder, ContactData};

// Simple URL
QRBuilder::new()
    .url("https://example.com")
    .colors([255, 0, 0, 255], [255, 255, 255, 255]) // Red on White
    .build()?
    .to_png(300, None)?;
```

## Testing

```bash
cargo test --workspace
```

## License

MIT OR Apache-2.0
