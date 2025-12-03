use crate::hashing;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: PathBuf,
    pub hash: String,
    pub last_verified: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Status {
    Unchanged,
    Modified,
    New,
    Missing,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Registry {
    pub entries: HashMap<String, FileEntry>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn load(path: &Path) -> io::Result<Self> {
        let data = fs::read_to_string(path)?;
        let reg: Registry = serde_json::from_str(&data).map_err(io::Error::other)?;
        Ok(reg)
    }

    pub fn save(&self, path: &Path) -> io::Result<()> {
        let data = serde_json::to_string_pretty(self).map_err(io::Error::other)?;
        fs::write(path, data)
    }

    /// Add a file or directory (recursively) to the registry.
    pub fn add_path(&mut self, path: &Path) -> io::Result<usize> {
        let mut count = 0usize;

        if path.is_file() {
            self.add_file(path)?;
            count += 1;
        } else if path.is_dir() {
            // Walk the directory, skipping ignored entries like `target/` and `.git/`
            for entry in WalkDir::new(path)
                .into_iter()
                .filter_entry(|e| !is_ignored(e))
                .filter_map(Result::ok)
            {
                let p = entry.path();
                if p.is_file() {
                    self.add_file(p)?;
                    count += 1;
                }
            }
        }

        Ok(count)
    }

    fn add_file(&mut self, path: &Path) -> io::Result<()> {
        let hash = hashing::hash_file(path)?;
        let now = Utc::now();
        let key = path.to_string_lossy().to_string();

        let entry = FileEntry {
            path: path.to_path_buf(),
            hash,
            last_verified: now,
        };

        self.entries.insert(key, entry);
        Ok(())
    }
}

/// Return true if this directory entry should be skipped when scanning.
///
/// This prevents the registry from being flooded with build artifacts, VCS
/// metadata, and editor config directories.
fn is_ignored(entry: &DirEntry) -> bool {
    if let Some(name) = entry.file_name().to_str() {
        matches!(
            name,
            "target" | ".git" | ".idea" | ".vscode" | "node_modules"
        )
    } else {
        false
    }
}
