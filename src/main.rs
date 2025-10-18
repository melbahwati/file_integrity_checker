mod cli;
mod hashing;
mod registry;
mod verify;

use crate::registry::{add_path_to_registry, load_registry, save_registry};
use crate::verify::verify_registry;
use clap::Parser;
use std::path::Path;

fn main() {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Hash { path } => {
            let hash = hashing::hash_file(Path::new(&path));
            match hash {
                Some(digest) => println!("SHA-256: {}", digest),
                None => eprintln!("Failed to read or hash file: {}", path),
            }
        }

        cli::Commands::Add { path } => {
            let mut registry = load_registry("registry.json");
            add_path_to_registry(&path, &mut registry);
            save_registry("registry.json", &registry);
            println!("Added files to registry.");
        }

        cli::Commands::Verify { output } => {
            verify_registry("registry.json", output.as_deref());
        }
    }
}
