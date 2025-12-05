use assert_cmd::Command;
use serde_json::Value;
use std::fs;
use tempfile::tempdir;

// hash subcommand should print plain hex, not labels
#[test]
fn hash_command_outputs_plain_64_char_hex() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("data.txt");
    fs::write(&file_path, "hello world").unwrap();

    let mut cmd = Command::cargo_bin("file_integrity_checker").unwrap();
    cmd.arg("hash").arg(&file_path);

    let assert = cmd.assert().success();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);

    assert_eq!(stdout.trim().len(), 64);
    assert!(stdout.trim().chars().all(|c| c.is_ascii_hexdigit()));
}

// add and verify should notice when a tracked file changes
#[test]
fn add_and_verify_detects_modified_file() {
    let dir = tempdir().unwrap();
    let registry_path = dir.path().join("registry.json");
    let file_path = dir.path().join("tracked.txt");

    fs::write(&file_path, "original contents").unwrap();

    // add file to registry
    let mut add_cmd = Command::cargo_bin("file_integrity_checker").unwrap();
    add_cmd
        .arg("--registry")
        .arg(&registry_path)
        .arg("add")
        .arg(&file_path);

    add_cmd.assert().success();

    // change file on disk
    fs::write(&file_path, "modified contents").unwrap();

    // verify should report at least one modified file
    let mut verify_cmd = Command::cargo_bin("file_integrity_checker").unwrap();
    let assert = verify_cmd
        .arg("--registry")
        .arg(&registry_path)
        .arg("verify")
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    assert!(
        stdout.contains("Modified") || stdout.contains("modified"),
        "verify output did not mention modified file: {stdout:?}"
    );
}

// list should show the same entries that were added, without touching the files again
#[test]
fn list_shows_registry_entries_without_rehashing() {
    let dir = tempdir().unwrap();
    let registry_path = dir.path().join("registry.json");
    let file_a = dir.path().join("a.txt");
    let file_b = dir.path().join("b.txt");

    fs::write(&file_a, "aaa").unwrap();
    fs::write(&file_b, "bbb").unwrap();

    // add directory, which should pull in both files
    let mut add_cmd = Command::cargo_bin("file_integrity_checker").unwrap();
    add_cmd
        .arg("--registry")
        .arg(&registry_path)
        .arg("add")
        .arg(dir.path());

    add_cmd.assert().success();

    // plain text list
    let mut list_cmd = Command::cargo_bin("file_integrity_checker").unwrap();
    let assert = list_cmd
        .arg("--registry")
        .arg(&registry_path)
        .arg("list")
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    assert!(stdout.contains("a.txt"), "list output: {stdout:?}");
    assert!(stdout.contains("b.txt"), "list output: {stdout:?}");

    // json list
    let mut list_json_cmd = Command::cargo_bin("file_integrity_checker").unwrap();
    let assert = list_json_cmd
        .arg("--registry")
        .arg(&registry_path)
        .arg("list")
        .arg("--json")
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    let value: Value = serde_json::from_str(&stdout).unwrap();
    assert!(value.get("entries").is_some(), "json output: {value:?}");
}
