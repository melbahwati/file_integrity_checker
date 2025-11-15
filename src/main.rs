mod cli;
mod hashing;
mod registry;
mod verify;

use clap::Parser;
use std::{io, path::PathBuf, process};

use crate::cli::{Cli, Commands};
use crate::registry::Registry;

const DEFAULT_REGISTRY: &str = "registry.json";

fn main() {
    let cli = Cli::parse();

    let registry_path = PathBuf::from(DEFAULT_REGISTRY);

    let result = match &cli.command {
        Commands::Hash { path } => cmd_hash(path),
        Commands::Add { path } => cmd_add(&registry_path, path),
        Commands::Verify => cmd_verify(&registry_path),
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn cmd_hash(path: &PathBuf) -> io::Result<()> {
    let digest = hashing::hash_file(path)?;
    println!("SHA-256: {digest}");
    Ok(())
}

fn cmd_add(registry_path: &PathBuf, path: &PathBuf) -> io::Result<()> {
    let mut registry = Registry::load(registry_path)?;
    registry.add_path(path)?;
    registry.save(registry_path)?;
    println!("Added entries under {}", path.display());
    Ok(())
}

fn cmd_verify(registry_path: &PathBuf) -> io::Result<()> {
    let mut registry = Registry::load(registry_path)?;
    let summary = verify::verify_registry(&mut registry)?;
    registry.save(registry_path)?; // persist updated statuses
    verify::print_verify_summary(&summary);
    Ok(())
}
