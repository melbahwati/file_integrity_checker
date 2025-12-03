use assert_cmd::prelude::*;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::tempdir;

/// Helper to create a file with given contents in a directory.
fn create_file(
    dir: &Path,
    name: &str,
    contents: &str,
) -> Result<std::path::PathBuf, Box<dyn Error>> {
    let path = dir.join(name);
    fs::write(&path, contents)?;
    Ok(path)
}

#[test]
fn hash_command_outputs_plain_64_char_hex() -> Result<(), Box<dyn Error>> {
    let tmp = tempdir()?;
    let file_path = create_file(tmp.path(), "test.txt", "hello world")?;

    // Run: file_integrity_checker hash <file>
    let output = Command::cargo_bin("file_integrity_checker")?
        .current_dir(tmp.path())
        .arg("hash")
        .arg(&file_path)
        .output()?;

    assert!(
        output.status.success(),
        "hash command should exit successfully"
    );

    let stdout = String::from_utf8(output.stdout)?;
    let trimmed = stdout.trim();

    // Check: 64 hex chars, no prefix, no extra noise
    assert_eq!(
        trimmed.len(),
        64,
        "digest should be exactly 64 hex characters"
    );
    assert!(
        trimmed.chars().all(|c| c.is_ascii_hexdigit()),
        "digest should contain only hex characters"
    );

    Ok(())
}

#[test]
fn add_and_verify_detects_modified_file() -> Result<(), Box<dyn Error>> {
    let tmp = tempdir()?;
    let file_path = create_file(tmp.path(), "data.txt", "original contents")?;

    // Run: file_integrity_checker add <file>
    let output_add = Command::cargo_bin("file_integrity_checker")?
        .current_dir(tmp.path())
        .arg("add")
        .arg(&file_path)
        .output()?;

    assert!(
        output_add.status.success(),
        "add command should exit successfully"
    );

    let stdout_add = String::from_utf8(output_add.stdout)?;
    assert!(
        stdout_add.contains("Added 1 entries"),
        "add output should say it added 1 entry, got: {stdout_add}"
    );

    // Modify the file after it's in the registry
    fs::write(&file_path, "modified contents")?;

    // Run: file_integrity_checker verify
    let output_verify = Command::cargo_bin("file_integrity_checker")?
        .current_dir(tmp.path())
        .arg("verify")
        .output()?;

    assert!(
        output_verify.status.success(),
        "verify command should exit successfully"
    );

    let stdout_verify = String::from_utf8(output_verify.stdout)?;

    // We expect at least one file to be reported as Modified,
    // and the summary should reflect that.
    assert!(
        stdout_verify.contains("Modified"),
        "verify output should contain 'Modified', got: {stdout_verify}"
    );
    assert!(
        stdout_verify.contains("Modified:  1"),
        "summary should report 1 modified file, got: {stdout_verify}"
    );

    Ok(())
}

#[test]
fn list_shows_registry_entries_without_rehashing() -> Result<(), Box<dyn Error>> {
    let tmp = tempdir()?;
    let file_path = create_file(tmp.path(), "listed.txt", "some contents")?;

    // First add a file so the registry has something in it
    let output_add = Command::cargo_bin("file_integrity_checker")?
        .current_dir(tmp.path())
        .arg("add")
        .arg(&file_path)
        .output()?;

    assert!(
        output_add.status.success(),
        "add command should exit successfully"
    );

    // Run: file_integrity_checker list
    let output_list = Command::cargo_bin("file_integrity_checker")?
        .current_dir(tmp.path())
        .arg("list")
        .output()?;

    assert!(
        output_list.status.success(),
        "list command should exit successfully"
    );

    let stdout_list = String::from_utf8(output_list.stdout)?;

    // The list command should print a header and our file name.
    assert!(
        stdout_list.contains("Registry entries"),
        "list output should contain a header, got: {stdout_list}"
    );
    assert!(
        stdout_list.contains("listed.txt"),
        "list output should mention 'listed.txt', got: {stdout_list}"
    );

    Ok(())
}
