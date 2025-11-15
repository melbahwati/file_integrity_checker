use std::{io, path::Path};

use crate::{
    hashing::hash_file,
    registry::{Registry, Status},
};

pub struct VerifyResult {
    pub total: usize,
    pub modified: usize,
    pub missing: usize,
    pub clean: usize,
}

pub fn verify_registry(registry: &mut Registry) -> io::Result<VerifyResult> {
    let mut result = VerifyResult {
        total: 0,
        modified: 0,
        missing: 0,
        clean: 0,
    };

    for entry in registry.entries.values_mut() {
        result.total += 1;
        let path: &Path = &entry.path;

        if !path.exists() {
            entry.status = Status::Missing;
            result.missing += 1;
            continue;
        }

        let current = hash_file(path)?;
        if current == entry.hash {
            entry.status = Status::Clean;
            result.clean += 1;
        } else {
            entry.status = Status::Modified;
            result.modified += 1;
        }
    }

    Ok(result)
}

pub fn print_verify_summary(result: &VerifyResult) {
    println!("Verified {} entries:", result.total);
    println!("  Clean: {}", result.clean);
    println!("  Modified: {}", result.modified);
    println!("  Missing: {}", result.missing);
}
