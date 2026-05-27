use assert_cmd::Command;
use std::fs;
use std::path::Path;

#[test]
fn test_logo_limits() {
    let output_file = "test_logo_limit.png";
    let dummy_logo = "dummy_logo.txt";

    // Clean up if exists
    if Path::new(output_file).exists() {
        fs::remove_file(output_file).unwrap();
    }

    // Create a dummy file that is NOT a valid image.
    fs::write(dummy_logo, "not an image").unwrap();

    let mut cmd = Command::cargo_bin("qr-cli").unwrap();
    let assert = cmd
        .arg("url")
        .arg("https://example.com")
        .arg("--logo")
        .arg(dummy_logo)
        .arg("--output")
        .arg(output_file)
        .assert()
        .success()
        .stderr(predicates::prelude::predicate::str::contains(
            "Warning: Failed to load logo image:",
        )); // The CLI prints a warning and still generates a QR code without a logo.

    assert!(
        Path::new(output_file).exists(),
        "Output file should still be created if logo loading fails with a warning"
    );

    fs::remove_file(dummy_logo).unwrap();
    if Path::new(output_file).exists() {
        fs::remove_file(output_file).unwrap();
    }
}
