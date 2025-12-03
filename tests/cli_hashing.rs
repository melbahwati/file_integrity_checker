use assert_cmd::Command;
use predicates::prelude::*;

/// `hash Cargo.toml` should succeed and print *something*.
#[test]
fn hash_command_prints_something() {
    let mut cmd = Command::cargo_bin("file_integrity_checker").unwrap();

    cmd.arg("hash")
        .arg("Cargo.toml")
        .assert()
        .success()
        // Just make sure stdout is not empty
        .stdout(predicate::str::is_empty().not());
}

/// The `hash` command should print a SHA-256 digest, which is 64 hex characters.
/// We don't care *which* digest here, just that the last "word" looks like a hex digest.
#[test]
fn hash_command_outputs_64_char_hex() {
    let mut cmd = Command::cargo_bin("file_integrity_checker").unwrap();

    let assert = cmd.arg("hash").arg("Cargo.toml").assert().success();

    // Capture stdout
    let output = String::from_utf8(assert.get_output().stdout.clone())
        .expect("stdout should be valid UTF-8");

    // Our program prints something like:
    // "SHA-256: <digest>\n"
    // So we grab the last whitespace-separated "word"
    let digest = output
        .split_whitespace()
        .last()
        .expect("expected to find a digest in output");

    // SHA-256 digest should be 64 hex characters
    assert_eq!(
        digest.len(),
        64,
        "digest should be 64 characters long, got {digest}"
    );
    assert!(
        digest.chars().all(|c| c.is_ascii_hexdigit()),
        "digest should contain only hex characters, got {digest}"
    );
}

/// Calling `hash` with no file path should fail (Clap should error about missing arguments).
#[test]
fn hash_command_without_path_fails() {
    let mut cmd = Command::cargo_bin("file_integrity_checker").unwrap();

    cmd.arg("hash")
        .assert()
        .failure()
        // We don't care about exact wording, just that something was printed to stderr.
        .stderr(predicate::str::is_empty().not());
}
