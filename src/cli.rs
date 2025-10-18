use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "File Integrity Checker")]
#[command(about = "Check and verify file integrity via SHA-256", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Compute SHA-256 hash of a file
    Hash {
        #[arg(help = "Path to file")]
        path: String,
    },

    /// Add file or directory to the integrity registry
    Add {
        #[arg(help = "Path to file or directory")]
        path: String,
    },

    /// Verify file integrity using stored registry
    Verify {
        #[arg(short, long, help = "Optional report output file")]
        output: Option<String>,
    },
}
