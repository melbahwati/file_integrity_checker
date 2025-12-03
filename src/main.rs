mod cli;
mod hashing;
mod registry;
mod verify;

use crate::cli::{Cli, Commands};
use crate::registry::{Registry, Status};
use crate::verify::verify_registry;
use clap::Parser;
use std::path::Path;
use std::process;

const REGISTRY_FILE: &str = "registry.json";

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn run() -> std::io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hash { path } => {
            let digest = hashing::hash_file(&path)?;
            println!("{digest}");
        }

        Commands::Add { path } => {
            let registry_path = Path::new(REGISTRY_FILE);

            let mut registry = if registry_path.exists() {
                Registry::load(registry_path)?
            } else {
                Registry::new()
            };

            let added = registry.add_path(&path)?;
            registry.save(registry_path)?;

            println!("Added {added} entries from {}", path.display());

            if added > 0 {
                println!();
                println!("Current registry entries:");

                let mut keys: Vec<_> = registry.entries.keys().cloned().collect();
                keys.sort();

                for key in keys {
                    if let Some(entry) = registry.entries.get(&key) {
                        println!("  {} -> hash {}", entry.path.display(), entry.hash);
                    }
                }
            }
        }

        Commands::Verify => {
            let registry_path = Path::new(REGISTRY_FILE);

            if !registry_path.exists() {
                eprintln!(
                    "No registry found at {}. Run `add` first to create it.",
                    registry_path.display()
                );
                return Ok(());
            }

            let mut registry = Registry::load(registry_path)?;
            let (results, summary) = verify_registry(&mut registry)?;
            registry.save(registry_path)?;

            println!("Verification results:");
            println!();

            for result in results {
                match result.status {
                    Status::Unchanged => {
                        println!("Unchanged  {}", result.path);
                    }
                    Status::Modified => {
                        println!("Modified  {}", result.path);
                    }
                    Status::Missing => {
                        println!("Missing   {}", result.path);
                    }
                    Status::New => {
                        println!("New       {}", result.path);
                    }
                }
            }

            println!();
            println!("Summary:");
            println!("  Unchanged: {}", summary.unchanged);
            println!("  Modified:  {}", summary.modified);
            println!("  New:       {}", summary.new);
            println!("  Missing:   {}", summary.missing);
        }

        Commands::List => {
            let registry_path = Path::new(REGISTRY_FILE);

            if !registry_path.exists() {
                eprintln!(
                    "No registry found at {}. Run `add` first to create it.",
                    registry_path.display()
                );
                return Ok(());
            }

            let registry = Registry::load(registry_path)?;

            println!("Registry entries ({} total):", registry.entries.len());
            println!();

            let mut keys: Vec<_> = registry.entries.keys().cloned().collect();
            keys.sort();

            for key in keys {
                if let Some(entry) = registry.entries.get(&key) {
                    println!("  {} -> hash {}", entry.path.display(), entry.hash);
                }
            }
        }
    }

    Ok(())
}
