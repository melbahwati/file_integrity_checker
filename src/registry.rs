use crate::hashing;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

/// one tracked file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: PathBuf,
    pub hash: String,
    pub last_verified: DateTime<Utc>,
}

/// status during verification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Status {
    Unchanged,
    Modified,
    New,
    Missing,
}

/// in-memory registry of files
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Registry {
    pub entries: HashMap<String, FileEntry>,
}

impl Registry {
    /// start with an empty registry
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// load registry from json file
    pub fn load(path: &Path) -> io::Result<Self> {
        let data = fs::read_to_string(path)?;
        let reg: Registry = serde_json::from_str(&data).map_err(io::Error::other)?;
        Ok(reg)
    }

    /// load registry or return an empty one if the file does not exist
    pub fn load_or_default(path: &Path) -> io::Result<Self> {
        match Self::load(path) {
            Ok(r) => Ok(r),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(Self::new()),
            Err(e) => Err(e),
        }
    }

    /// save registry to json file
    pub fn save(&self, path: &Path) -> io::Result<()> {
        let data = serde_json::to_string_pretty(self).map_err(io::Error::other)?;
        fs::write(path, data)
    }

    /// add a file or a directory (recursively) to the registry
    pub fn add_path(&mut self, path: &Path) -> io::Result<usize> {
        let mut count = 0usize;

        if path.is_file() {
            self.add_file(path)?;
            count += 1;
        } else if path.is_dir() {
            for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
                let p = entry.path();
                if p.is_file() {
                    self.add_file(p)?;
                    count += 1;
                }
            }
        }

        Ok(count)
    }

    /// add or update a single file entry
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

    /// drop entries whose files no longer exist
    pub fn prune_missing(&mut self) -> usize {
        let before = self.entries.len();
        self.entries.retain(|_, entry| entry.path.exists());
        before.saturating_sub(self.entries.len())
    }
}

#[cfg(test)]
mod tests {
    use super::Registry;
    use std::{fs, io, path::PathBuf};

    // helper to make a temp file for a test
    fn write_temp_file(name: &str, contents: &str) -> io::Result<PathBuf> {
        let mut path = std::env::temp_dir();
        path.push(format!("fic_registry_test_{name}.txt"));
        fs::write(&path, contents)?;
        Ok(path)
    }

    #[test]
    fn add_path_adds_single_file() {
        let path = write_temp_file("single", "hello").unwrap();
        let mut reg = Registry::new();
        let count = reg.add_path(&path).unwrap();
        assert_eq!(count, 1);
        assert_eq!(reg.entries.len(), 1);
    }

    #[test]
    fn prune_missing_removes_deleted_files() {
        let path = write_temp_file("prune", "hello").unwrap();
        let mut reg = Registry::new();
        reg.add_path(&path).unwrap();

        fs::remove_file(&path).unwrap();

        let removed = reg.prune_missing();
        assert_eq!(removed, 1);
        assert!(reg.entries.is_empty());
    }
}
