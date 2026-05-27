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
    // and exit before creating output_file.
    fs::write(dummy_logo, "not an image").unwrap();

    let mut cmd = Command::cargo_bin("qr-cli").unwrap();
    cmd.arg("url")
        .arg("https://example.com")
        .arg("--logo")
        .arg(dummy_logo)
        .arg("--output")
        .arg(output_file)
        .assert()
        .failure(); // The CLI should exit with a non-zero status code when logo fails to load.

    // Check that output file was not created.
    assert!(
        !Path::new(output_file).exists(),
        "Output file should not be created if logo loading fails"
    );

    fs::remove_file(dummy_logo).unwrap();
    if Path::new(output_file).exists() {
        fs::remove_file(output_file).unwrap();
    }
}
