use crate::hashing;
use crate::registry::{Registry, Status};
use chrono::Utc;
use std::io;

#[derive(Debug, Default, Clone)]
pub struct VerifySummary {
    pub unchanged: usize,
    pub modified: usize,
    pub new: usize,
    pub missing: usize,
}

#[derive(Debug)]
pub struct VerifyResult {
    pub path: String,
    pub status: Status,
}

pub fn verify_registry(registry: &mut Registry) -> io::Result<(Vec<VerifyResult>, VerifySummary)> {
    let mut summary = VerifySummary::default();
    let mut results = Vec::new();

    for (key, entry) in registry.entries.iter_mut() {
        let path = &entry.path;

        if path.exists() {
            let current_hash = hashing::hash_file(path)?;
            if current_hash == entry.hash {
                summary.unchanged += 1;
                results.push(VerifyResult {
                    path: key.clone(),
                    status: Status::Unchanged,
                });
            } else {
                summary.modified += 1;
                entry.hash = current_hash;
                results.push(VerifyResult {
                    path: key.clone(),
                    status: Status::Modified,
                });
            }

            entry.last_verified = Utc::now();
        } else {
            summary.missing += 1;
            results.push(VerifyResult {
                path: key.clone(),
                status: Status::Missing,
            });
        }
    }

    Ok((results, summary))
}
