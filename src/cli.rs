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
    /// Hash a single file and print its digest
    Hash {
        /// Path to the file
        path: PathBuf,
    },
    /// Add a file or directory (recursively) to the registry
    Add {
        /// Path to a file or directory
        path: PathBuf,
    },
    /// Verify files in the registry (partial implementation)
    Verify,
}
