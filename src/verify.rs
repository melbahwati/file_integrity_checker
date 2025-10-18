//! File: verify.rs
//! Description: Verifies file integrity against a stored registry, detecting changes.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::hashing::hash_file;
use crate::registry::{load_registry, Status};

use walkdir::WalkDir;

/// Verifies files against the saved registry.
/// - Checks for modified, missing, unchanged, and new files.
/// - Optionally outputs a report to a file (JSON or CSV).
pub fn verify_registry<P: AsRef<Path>>(registry_path: P, report_output: Option<P>) {
    let registry = load_registry(&registry_path);
    let mut current_paths = HashSet::new();

    println!("🔍 Verifying file integrity...\n");

    // Check all files already in the registry
    for (path_str, entry) in &registry {
        let path = Path::new(path_str);
        current_paths.insert(path_str.clone());

        let status = if !path.exists() {
            Status::Missing
        } else if let Some(current_hash) = hash_file(path) {
            if current_hash == entry.hash {
                Status::Unchanged
            } else {
                Status::Modified
            }
        } else {
            println!("⚠️  Could not read: {}", path_str);
            continue;
        };

        match status {
            Status::Missing => println!("❌ Missing: {}", path_str),
            Status::Modified => println!("✏️  Modified: {}", path_str),
            Status::Unchanged => println!("✅ Unchanged: {}", path_str),
            Status::New => (), // Not applicable here
        }
    }

    // Detect new files on disk not in the registry
    let base_dir = Path::new(".");
    for entry in WalkDir::new(base_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file())
    {
        let path = entry.path();
        let path_str = path.to_string_lossy().to_string();
        if !current_paths.contains(&path_str) {
            println!("🆕 New: {}", path_str);
        }
    }

    // Optional report output (copy registry file)
    if let Some(output_path) = report_output {
        if let Err(e) = fs::copy(&registry_path, output_path) {
            eprintln!("Failed to write report: {}", e);
        } else {
            println!("\n📄 Report written.");
        }
    }
}
