use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    io,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

use crate::hashing::hash_file;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Status {
    Clean,
    Modified,
    Missing,
    New,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: PathBuf,
    pub hash: String,
    pub last_seen: DateTime<Utc>,
    pub status: Status,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Registry {
    pub entries: HashMap<PathBuf, FileEntry>,
}

impl Registry {
    pub fn load(path: &Path) -> io::Result<Self> {
        if path.exists() {
            let data = fs::read_to_string(path)?;
            let reg: Registry =
                serde_json::from_str(&data).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            Ok(reg)
        } else {
            Ok(Registry::default())
        }
    }

    pub fn save(&self, path: &Path) -> io::Result<()> {
        let data = serde_json::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        fs::write(path, data)
    }

    /// Add a file or recursively add all files in a directory.
    pub fn add_path(&mut self, root: &Path) -> io::Result<()> {
        let now = Utc::now();

        if root.is_file() {
            self.add_single(root, now)?;
        } else {
            for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_file() {
                    self.add_single(path, now)?;
                }
            }
        }

        Ok(())
    }

    fn add_single(&mut self, path: &Path, now: DateTime<Utc>) -> io::Result<()> {
        let hash = hash_file(path)?;
        let entry = FileEntry {
            path: path.to_path_buf(),
            hash,
            last_seen: now,
            status: Status::Clean,
        };
        self.entries.insert(entry.path.clone(), entry);
        Ok(())
    }
}
