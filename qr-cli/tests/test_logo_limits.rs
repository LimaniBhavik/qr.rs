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
    // The decoder will fail. Our strict limits or corrupted file should trigger the error path
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
        .success(); // The CLI gracefully continues if logo loading fails

    let output_str = std::str::from_utf8(&assert.get_output().stderr).unwrap();
    assert!(output_str.contains("Warning: Failed to load logo image"));

    // Verify it still generated the QR code despite the logo error
    assert!(
        Path::new(output_file).exists(),
        "Output file should be created despite logo loading failure"
    );

    fs::remove_file(dummy_logo).unwrap();
    if Path::new(output_file).exists() {
        fs::remove_file(output_file).unwrap();
    }
}
