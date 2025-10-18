use assert_cmd::Command;
use std::fs;
use predicates;

#[test]
fn test_hash_command_outputs_digest() {
    let mut cmd = Command::cargo_bin("file_integrity_checker").unwrap();
    cmd.args(["hash", "Cargo.toml"])
        .assert()
        .success()
        .stdout(predicates::str::contains("SHA-256:"));
}

#[test]
fn test_add_and_verify_detects_change() {
    let test_file = "test_temp_file.txt";
    fs::write(test_file, "original").unwrap();

    // Add to registry
    let mut cmd = Command::cargo_bin("file_integrity_checker").unwrap();
    cmd.args(["add", test_file]).assert().success();

    // Modify the file
    fs::write(test_file, "modified").unwrap();

    // Run verify (should detect modified)
    let mut cmd = Command::cargo_bin("file_integrity_checker").unwrap();
    cmd.args(["verify"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Modified"));

    // Cleanup
    fs::remove_file(test_file).expect("Failed to remove test file");
    fs::remove_file("registry.json").expect("Failed to remove registry.json");
}
