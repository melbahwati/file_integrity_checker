use crate::hashing;
use crate::registry::{Registry, Status};
use chrono::Utc;
use std::collections::HashSet;
use std::io;
use std::path::Path;
use walkdir::WalkDir;

/// summary of what verify found
#[derive(Debug, Default, Clone)]
pub struct VerifySummary {
    pub unchanged: usize,
    pub modified: usize,
    pub new: usize,
    pub missing: usize,
}

/// one row in the verify output
#[derive(Debug)]
pub struct VerifyResult {
    pub path: String,
    pub status: Status,
}

/// determine which directories need to be scanned for possible new files.
/// this looks at every tracked file and extracts its parent directory.
/// this lets us find new files without storing extra metadata.
fn collect_scan_roots(registry: &Registry) -> HashSet<String> {
    let mut dirs = HashSet::new();

    for entry in registry.entries.values() {
        if let Some(parent) = entry.path.parent() {
            dirs.insert(parent.to_string_lossy().to_string());
        }
    }

    dirs
}

/// verify all existing entries, and detect new files inside previously added directories
pub fn verify_registry(registry: &mut Registry) -> io::Result<(Vec<VerifyResult>, VerifySummary)> {
    let mut summary = VerifySummary::default();
    let mut results = Vec::new();
    let mut seen = HashSet::new();

    // first: verify tracked files
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

        seen.insert(key.clone());
    }

    // determine which directories to scan for new files
    let scan_dirs = collect_scan_roots(registry);

    // now scan those directories for untracked files
    for dir in &scan_dirs {
        let dir_path = Path::new(dir);

        if !dir_path.exists() {
            continue;
        }

        for entry in WalkDir::new(dir_path).into_iter().filter_map(Result::ok) {
            let p = entry.path();

            if !p.is_file() {
                continue;
            }

            let key = p.to_string_lossy().to_string();

            if registry.entries.contains_key(&key) {
                continue;
            }

            if seen.contains(&key) {
                continue;
            }

            // found a new file
            summary.new += 1;
            results.push(VerifyResult {
                path: key.clone(),
                status: Status::New,
            });

            seen.insert(key);
        }
    }

    Ok((results, summary))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::Registry;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn verify_detects_modified_and_missing() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        let a = dir.path().join("a.txt");
        let b = dir.path().join("b.txt");

        File::create(&a)?.write_all(b"one")?;
        File::create(&b)?.write_all(b"one")?;

        let mut reg = Registry::new();
        reg.add_path(dir.path())?;

        // modify a
        File::create(&a)?.write_all(b"two")?;
        // remove b
        fs::remove_file(&b)?;

        let (results, summary) = verify_registry(&mut reg)?;
        assert_eq!(summary.modified, 1);
        assert_eq!(summary.missing, 1);

        let saw_mod = results
            .iter()
            .any(|r| r.path.ends_with("a.txt") && r.status == Status::Modified);
        let saw_missing = results
            .iter()
            .any(|r| r.path.ends_with("b.txt") && r.status == Status::Missing);

        assert!(saw_mod);
        assert!(saw_missing);
        Ok(())
    }

    #[test]
    fn verify_detects_new_files() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        let tracked = dir.path().join("t.txt");
        let newfile = dir.path().join("new.txt");

        File::create(&tracked)?.write_all(b"tracked")?;

        let mut reg = Registry::new();
        reg.add_path(dir.path())?;

        File::create(&newfile)?.write_all(b"new")?;

        let (results, summary) = verify_registry(&mut reg)?;
        assert_eq!(summary.new, 1);

        let saw_new = results
            .iter()
            .any(|r| r.path.ends_with("new.txt") && r.status == Status::New);
        assert!(saw_new);

        Ok(())
    }
}
