use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

/// helpers for hashing files
pub fn hash_file(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 8192];

    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }

    let digest = hasher.finalize();
    Ok(hex::encode(digest))
}

#[cfg(test)]
mod tests {
    use super::hash_file;
    use std::{fs, path::PathBuf};

    // make a temp file in the system temp dir
    fn write_temp_file(name: &str, contents: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!("fic_hash_test_{name}.txt"));
        fs::write(&path, contents).unwrap();
        path
    }

    #[test]
    fn hash_file_produces_64_hex_chars() {
        let path = write_temp_file("len", "hello");
        let digest = hash_file(&path).unwrap();
        assert_eq!(digest.len(), 64);
        assert!(digest.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
