# QR.RS

Project Overview
Create a comprehensive QR code generator in Rust that supports URLs, text, and contact information (vCard). The project should be implemented as a multi-target application supporting CLI, GUI desktop, and web interfaces.

Repository: https://github.com/LimaniBhavik/qr.rs

## Features

- **URL Mode**: Generate QR codes for URLs with automatic protocol formatting (ensure https:// prefix for proper URL scanning, not Google search)
- **Text Mode**: Generate QR codes for arbitrary text content
- **Contact Mode**: Generate QR codes with vCard 3.0 format for contact information including: First Name / Last Name Phone Number Email Address Organization Website URL

## Project Structure

- `qr.rs/`: Workspace root
- `src/`: Core library (shared logic)
- `qr-cli/`: CLI application
- `qr-gui/`: GUI desktop application (eframe/egui)
- `qr-web/`: Web application (Yew/WASM)

## Usage

### CLI

```bash
cargo run -p qr-cli -- url "example.com" -o qr.png
cargo run -p qr-cli -- text "Hello World" -o text.png
cargo run -p qr-cli -- contact --first-name "John" --last-name "Doe" -o contact.png
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

## Testing

```bash
cargo test
```

## License

MIT OR Apache-2.0
