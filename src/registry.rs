//! File: registry.rs
//! Description: Manages loading, saving, and updating a registry of file hashes.

use crate::hashing::hash_file;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// Represents a single file entry in the registry.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileEntry {
    pub path: String,
    pub hash: String,
    pub last_checked: DateTime<Utc>,
}

/// Alias for the entire registry mapping file paths to their metadata.
pub type Registry = HashMap<String, FileEntry>;

/// Status of a file when comparing the registry to current state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Status {
    Unchanged,
    Modified,
    Missing,
    New,
}

/// Loads the registry from a JSON file.
/// If the file does not exist or cannot be parsed, returns an empty registry.
pub fn load_registry<P: AsRef<Path>>(path: P) -> Registry {
    if let Ok(data) = fs::read_to_string(&path) {
        if let Ok(registry) = serde_json::from_str(&data) {
            return registry;
        }
    }
    HashMap::new()
}

/// Saves the registry as pretty-printed JSON to the specified file.
pub fn save_registry<P: AsRef<Path>>(path: P, registry: &Registry) {
    if let Ok(json) = serde_json::to_string_pretty(registry) {
        let _ = fs::write(path, json);
    }
}

/// Adds a file or all files in a directory to the registry.
pub fn add_path_to_registry<P: AsRef<Path>>(path: P, registry: &mut Registry) {
    let input_path = path.as_ref();

    let entries = if input_path.is_dir() {
        WalkDir::new(input_path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.path().is_file())
            .map(|e| e.path().to_path_buf())
            .collect::<Vec<_>>()
    } else if input_path.is_file() {
        vec![input_path.to_path_buf()]
    } else {
        eprintln!("Invalid path: {:?}", input_path);
        return;
    };

    for path in entries {
        if let Some(hash) = hash_file(&path) {
            let path_str = path.to_string_lossy().to_string();
            let entry = FileEntry {
                path: path_str.clone(),
                hash,
                last_checked: chrono::Utc::now(),
            };
            registry.insert(path_str, entry);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{remove_file, write};

    #[test]
    fn test_save_and_load_registry() {
        let test_path = "test_registry.json";
        let mut reg = Registry::new();
        reg.insert(
            "test".to_string(),
            FileEntry {
                path: "test".to_string(),
                hash: "abc123".to_string(),
                last_checked: chrono::Utc::now(),
            },
        );
        save_registry(test_path, &reg);
        let loaded = load_registry(test_path);
        assert_eq!(loaded["test"].hash, "abc123");
        let _ = remove_file(test_path);
    }

    #[test]
    fn test_add_path_to_registry_single_file() {
        let path = "temp_add_test.txt";
        write(path, "test content").unwrap();

        let mut registry = Registry::new();
        add_path_to_registry(path, &mut registry);

        assert!(registry.contains_key(path));
        assert_eq!(registry[path].hash, hash_file(Path::new(path)).unwrap());

        let _ = remove_file(path);
    }
}
