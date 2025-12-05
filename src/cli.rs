use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// small command line tool for tracking file hashes in a json registry
#[derive(Parser, Debug)]
#[command(name = "file_integrity_checker")]
#[command(about = "simple file integrity checker using SHA-256")]
pub struct Cli {
    /// path to the registry file (json)
    #[arg(long, default_value = "registry.json")]
    pub registry: PathBuf,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// compute and print the sha-256 hash of a file
    Hash {
        /// path to the file to hash
        path: PathBuf,
    },

    /// add a file or directory (recursively) to the registry
    Add {
        /// path to a file or directory to track
        path: PathBuf,
    },

    /// verify files listed in the registry
    ///
    /// by default this only checks files already in the registry.
    /// with --scan-root you can also ask it to report files on disk
    /// that are not yet in the registry as "new".
    Verify {
        /// optional root directory to scan for files that are not yet
        /// present in the registry. these are reported as `Status::New`.
        #[arg(long)]
        scan_root: Option<PathBuf>,
    },

    /// list entries in the registry without re-hashing
    List {
        /// print the raw registry json instead of a human friendly view
        #[arg(long)]
        json: bool,
    },

    /// remove registry entries whose files no longer exist on disk
    Prune,
}
