use assert_cmd::Command;
use std::path::Path;

#[test]
fn test_path_traversal() {
    let dummy_output = "../traversal_test.png";

    let mut cmd = Command::cargo_bin("qr-cli").unwrap();
    cmd.arg("url")
        .arg("https://example.com")
        .arg("--output")
        .arg(dummy_output)
        .assert()
        .failure();

    assert!(!Path::new(dummy_output).exists());
}
