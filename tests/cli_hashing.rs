use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn hash_command_prints_something() {
    // Run the compiled binary named "file_integrity_checker"
    let mut cmd = Command::cargo_bin("file_integrity_checker").unwrap();

    // Ask it to hash Cargo.toml (which exists in the project root)
    cmd.arg("hash").arg("Cargo.toml");

    cmd.assert()
        .success()
        // Just check that stdout is not empty (any digest will do)
        .stdout(predicate::str::is_empty().not());
}
