use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::Path;

#[test]
fn test_help() {
    let mut cmd = Command::cargo_bin("qr-cli").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn test_url_command() {
    let output_file = "test_url.png";
    // Clean up if exists
    if Path::new(output_file).exists() {
        fs::remove_file(output_file).unwrap();
    }

    let mut cmd = Command::cargo_bin("qr-cli").unwrap();
    cmd.arg("url")
        .arg("https://example.com")
        .arg("--output")
        .arg(output_file)
        .assert()
        .success();

    assert!(Path::new(output_file).exists());
    fs::remove_file(output_file).unwrap();
}

#[test]
fn test_text_command() {
    let output_file = "test_text.png";
    if Path::new(output_file).exists() {
        fs::remove_file(output_file).unwrap();
    }

    let mut cmd = Command::cargo_bin("qr-cli").unwrap();
    cmd.arg("text")
        .arg("Hello World")
        .arg("--output")
        .arg(output_file)
        .assert()
        .success();

    assert!(Path::new(output_file).exists());
    fs::remove_file(output_file).unwrap();
}

#[test]
fn test_contact_command() {
    let output_file = "test_contact.png";
    if Path::new(output_file).exists() {
        fs::remove_file(output_file).unwrap();
    }

    let mut cmd = Command::cargo_bin("qr-cli").unwrap();
    cmd.arg("contact")
        .arg("--first-name")
        .arg("John")
        .arg("--last-name")
        .arg("Doe")
        .arg("--email")
        .arg("john@example.com")
        .arg("--output")
        .arg(output_file)
        .assert()
        .success();

    assert!(Path::new(output_file).exists());
    fs::remove_file(output_file).unwrap();
}

#[test]
fn test_invalid_args() {
    let mut cmd = Command::cargo_bin("qr-cli").unwrap();
    cmd.arg("url") // Missing URL argument
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}
