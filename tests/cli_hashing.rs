use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

// hash on a real file should print a 64-char hex digest plus a newline
#[test]
fn hash_command_outputs_64_char_hex() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("data.txt");
    fs::write(&file_path, "hello world").unwrap();

    let mut cmd = Command::cargo_bin("file_integrity_checker").unwrap();
    cmd.arg("hash").arg(&file_path);

    let assert = cmd.assert().success();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);

    assert!(
        predicate::str::is_match(r"^[0-9a-f]{64}\r?\n$")
            .unwrap()
            .eval(&stdout),
        "stdout was: {stdout:?}"
    );
}

// hash should print something non-empty for a valid file
#[test]
fn hash_command_prints_something() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("something.txt");
    fs::write(&file_path, "contents").unwrap();

    let mut cmd = Command::cargo_bin("file_integrity_checker").unwrap();
    cmd.arg("hash").arg(&file_path);

    let assert = cmd.assert().success();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);

    assert!(!stdout.trim().is_empty());
}

// calling hash without a path should fail but not crash
#[test]
fn hash_command_without_path_fails() {
    let mut cmd = Command::cargo_bin("file_integrity_checker").unwrap();
    cmd.arg("hash");

    let assert = cmd.assert().failure();
    let stderr = String::from_utf8_lossy(&assert.get_output().stderr);

    let lower = stderr.to_lowercase();
    assert!(
        lower.contains("usage") || lower.contains("required"),
        "stderr was: {stderr:?}"
    );
}

// hashing a missing file should fail in a reasonable way
#[test]
fn hash_command_nonexistent_file_fails() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("does_not_exist.txt");

    let mut cmd = Command::cargo_bin("file_integrity_checker").unwrap();
    cmd.arg("hash").arg(&file_path);

    let assert = cmd.assert().failure();
    let stderr = String::from_utf8_lossy(&assert.get_output().stderr);

    let lower = stderr.to_lowercase();
    assert!(
        lower.contains("no such file")
            || lower.contains("cannot find the file")
            || lower.contains("os {"),
        "stderr was: {stderr:?}"
    );
}

// running the binary with no subcommand should show clap help
#[test]
fn binary_shows_help_without_subcommand() {
    let mut cmd = Command::cargo_bin("file_integrity_checker").unwrap();

    let assert = cmd.assert().failure();
    let stderr = String::from_utf8_lossy(&assert.get_output().stderr);
    let lower = stderr.to_lowercase();

    assert!(
        lower.contains("usage"),
        "stderr did not contain 'usage': {stderr:?}"
    );
    assert!(
        lower.contains("hash"),
        "stderr did not mention 'hash' subcommand: {stderr:?}"
    );
    assert!(
        lower.contains("add"),
        "stderr did not mention 'add' subcommand: {stderr:?}"
    );
    assert!(
        lower.contains("verify"),
        "stderr did not mention 'verify' subcommand: {stderr:?}"
    );
    assert!(
        lower.contains("list"),
        "stderr did not mention 'list' subcommand: {stderr:?}"
    );
    assert!(
        lower.contains("prune"),
        "stderr did not mention 'prune' subcommand: {stderr:?}"
    );
}
