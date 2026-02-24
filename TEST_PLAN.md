# Testing Plan & Schedule

## 1. Test Cases Inventory

### Core Library (`qr-rs`)
**Unit Tests (`src/generator/mod.rs`):**
- `test_generate_text`: Verifies basic text QR generation.
- `test_generate_url`: Verifies URL QR generation (with/without protocol).
- `test_generate_contact`: Verifies contact data QR generation.
- `test_generate_with_error_correction`: Iterates through all EC levels (L, M, Q, H).

**Unit Tests (`src/formats/mod.rs`):**
- `test_format_url`: Verifies URL normalization.
- `test_vcard_generation`: Verifies vCard formatting.
- `test_is_valid_email`: Verifies email regex validation.
- `test_contact_data_validation`: Validates complete `ContactData` struct logic.

**Integration Tests (`tests/integration.rs`):**
- `test_url_qr_generation`: End-to-end URL QR generation to PNG.
- `test_contact_qr_generation`: End-to-end Contact QR generation to SVG.
- `test_custom_colors`: Verifies custom RGBA colors.

### CLI (`qr-cli`)
**Proposed Integration Tests:**
- `test_help`: Verify `--help` output exists.
- `test_url_command`: Verify `qr-cli url <URL>` generates a file.
- `test_text_command`: Verify `qr-cli text <TEXT>` generates a file.
- `test_contact_command`: Verify `qr-cli contact ...` generates a file.
- `test_invalid_args`: Verify error handling for missing arguments.

### GUI (`qr-gui`)
**Build Verification:**
- `cargo build -p qr-gui`: Ensure successful compilation.

### Web (`qr-web`)
**Build Verification:**
- `cargo build -p qr-web --target wasm32-unknown-unknown`: Ensure successful WASM compilation.

## 2. Execution Schedule

1.  **Phase 1: Core Logic Verification**
    - **Action:** Run `cargo test -p qr-rs`
    - **Goal:** Confirm the library's core functionality is robust.

2.  **Phase 2: CLI Integration**
    - **Action:** Add `assert_cmd` dependency.
    - **Action:** Implement `qr-cli/tests/integration.rs`.
    - **Action:** Run `cargo test -p qr-cli`.
    - **Goal:** Verify the CLI correctly parses arguments and calls the library.

3.  **Phase 3: UI Component Verification**
    - **Action:** Run `cargo build -p qr-gui`.
    - **Action:** Run `cargo build -p qr-web --target wasm32-unknown-unknown`.
    - **Goal:** Ensure no build regressions in GUI/Web components.
