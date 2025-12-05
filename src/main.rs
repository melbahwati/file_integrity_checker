mod cli;
mod hashing;
mod registry;
mod verify;

use crate::cli::{Cli, Commands};
use crate::registry::{Registry, Status};
use crate::verify::{verify_registry, VerifyResult, VerifySummary};
use clap::Parser;
use std::io;
use std::path::Path;

/// entry point: parse args and hand off to subcommands
fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let registry_path = cli.registry;

    match cli.command {
        Commands::Hash { path } => run_hash(&path),
        Commands::Add { path } => run_add(&registry_path, &path),
        Commands::Verify { scan_root } => run_verify(&registry_path, scan_root.as_deref()),
        Commands::List { json } => run_list(&registry_path, json),
        Commands::Prune => run_prune(&registry_path),
    }
}

/// run the `hash` subcommand
fn run_hash(path: &Path) -> io::Result<()> {
    let digest = hashing::hash_file(path)?;
    println!("{digest}");
    Ok(())
}

/// run the `add` subcommand
fn run_add(registry_path: &Path, path: &Path) -> io::Result<()> {
    let mut registry = Registry::load_or_default(registry_path)?;
    let added = registry.add_path(path)?;
    registry.save(registry_path)?;

    println!("Added {added} entries from {}", path.display());
    println!();
    println!("Current registry entries:");
    for (key, entry) in &registry.entries {
        println!("  {key} -> hash {}", entry.hash);
    }

    Ok(())
}

/// run the `verify` subcommand
fn run_verify(registry_path: &Path, scan_root: Option<&Path>) -> io::Result<()> {
    if !registry_path.exists() {
        eprintln!(
            "registry file {} does not exist, nothing to verify",
            registry_path.display()
        );
        return Ok(());
    }

    let mut registry = Registry::load(registry_path)?;
    let (results, summary) = verify_registry(&mut registry, scan_root)?;
    registry.save(registry_path)?;

    print_verify_results(&results, &summary);
    Ok(())
}

/// run the `list` subcommand
fn run_list(registry_path: &Path, json: bool) -> io::Result<()> {
    if !registry_path.exists() {
        eprintln!(
            "registry file {} does not exist, nothing to list",
            registry_path.display()
        );
        return Ok(());
    }

    let registry = Registry::load(registry_path)?;

    if json {
        let payload = serde_json::to_string_pretty(&registry).map_err(io::Error::other)?;
        println!("{payload}");
    } else {
        println!("Current registry entries:");
        for (key, entry) in &registry.entries {
            println!("  {} -> hash {}", key, entry.hash);
        }
    }

    Ok(())
}

/// run the `prune` subcommand
fn run_prune(registry_path: &Path) -> io::Result<()> {
    if !registry_path.exists() {
        eprintln!(
            "registry file {} does not exist, nothing to prune",
            registry_path.display()
        );
        return Ok(());
    }

    let mut registry = Registry::load(registry_path)?;
    let removed = registry.prune_missing();
    registry.save(registry_path)?;

    println!("Pruned {removed} entries");
    Ok(())
}

/// print detailed verify results plus the summary block
fn print_verify_results(results: &[VerifyResult], summary: &VerifySummary) {
    println!("Verification results:");
    println!();

    for r in results {
        match r.status {
            Status::Unchanged => println!("Unchanged  {}", r.path),
            Status::Modified => println!("Modified   {}", r.path),
            Status::New => println!("New        {}", r.path),
            Status::Missing => println!("Missing    {}", r.path),
        }
    }

    println!();
    println!("Summary:");
    println!("  Unchanged: {}", summary.unchanged);
    println!("  Modified:  {}", summary.modified);
    println!("  New:       {}", summary.new);
    println!("  Missing:   {}", summary.missing);
}
