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

/// verify all entries in the registry, and optionally look for new files
pub fn verify_registry(
    registry: &mut Registry,
    scan_root: Option<&Path>,
) -> io::Result<(Vec<VerifyResult>, VerifySummary)> {
    let mut summary = VerifySummary::default();
    let mut results = Vec::new();
    let mut seen = HashSet::new();

    // check all files in the registry
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

    // optional: look for new untracked files
    if let Some(root) = scan_root {
        for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }

            let key = path.to_string_lossy().to_string();
            if registry.entries.contains_key(&key)
                || key.contains("\\target\\")
                || key.contains("/target/")
                || key.contains("\\.git\\")
                || key.contains("/.git/")
            {
                continue;
            }

            if seen.contains(&key) {
                continue;
            }

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
        let file_a = dir.path().join("a.txt");
        let file_b = dir.path().join("b.txt");

        {
            let mut f = File::create(&file_a)?;
            writeln!(f, "first version")?;
        }
        {
            let mut f = File::create(&file_b)?;
            writeln!(f, "first version")?;
        }

        let mut registry = Registry::new();
        registry.add_path(dir.path())?;

        {
            let mut f = File::create(&file_a)?;
            writeln!(f, "second version")?;
        }
        fs::remove_file(&file_b)?;

        let (results, summary) = verify_registry(&mut registry, None)?;
        assert_eq!(summary.modified, 1);
        assert_eq!(summary.missing, 1);

        let mut saw_modified = false;
        let mut saw_missing = false;
        for r in results {
            if r.path.ends_with("a.txt") && r.status == Status::Modified {
                saw_modified = true;
            }
            if r.path.ends_with("b.txt") && r.status == Status::Missing {
                saw_missing = true;
            }
        }
        assert!(saw_modified);
        assert!(saw_missing);
        Ok(())
    }

    #[test]
    fn verify_detects_new_files() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        let file_tracked = dir.path().join("tracked.txt");
        let file_new = dir.path().join("new.txt");

        {
            let mut f = File::create(&file_tracked)?;
            writeln!(f, "this one is tracked")?;
        }

        let mut registry = Registry::new();
        registry.add_path(&file_tracked)?;

        {
            let mut f = File::create(&file_new)?;
            writeln!(f, "this one is new")?;
        }

        // verify, scanning the directory for new files
        let (results, summary) = verify_registry(&mut registry, Some(dir.path()))?;
        assert_eq!(summary.new, 1);

        let found_new = results
            .iter()
            .any(|r| r.path.ends_with("new.txt") && r.status == Status::New);

        assert!(
            found_new,
            "expected to find new.txt reported as Status::New"
        );
        Ok(())
    }
}
