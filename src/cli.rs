use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "file_integrity_checker")]
#[command(about = "Simple file integrity checker using SHA-256")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Compute SHA-256 for a single file
    Hash {
        /// Path to the file to hash
        path: PathBuf,
    },

    /// Add a file or directory (recursively) to the registry
    Add {
        /// Path to a file or directory
        path: PathBuf,
    },

    /// Verify files listed in the registry
    Verify,

    /// List registry entries without re-hashing
    List,
}
