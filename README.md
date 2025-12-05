// mo elbahwati 
// project 1/rust final project
// file integrity checker


# file_integrity_checker

simple command-line tool to track file integrity using sha-256 hashes.

you point it at files or directories, it records their hashes in a json “registry” file, and later you can verify whether anything has changed, list what is being tracked, or prune entries for files that no longer exist.

the project is written in rust and is structured as a small but realistic cli application with tests.

## structure 

src/
  main.rs      - program entry point and high-level command dispatch
  cli.rs       - cli definitions using clap
  hashing.rs   - sha-256 hashing utilities
  registry.rs  - registry data structures and json loading/saving
  verify.rs    - verification logic and status tracking

tests/
  cli_hashing.rs  - tests focused on the hash subcommand and cli behavior
  integration.rs  - end-to-end tests of add/verify/list/prune and registry handling

Cargo.toml   - crate metadata and dependencies
README.md    - this file


## features
- compute sha-256 hashes for files
- maintain a json registry of tracked files
- recursively add files from directories
- verify current files against the registry
  - unchanged
  - modified
  - missing
  - new 
- list registry entries without re-hashing
- prune registry entries for files that no longer exist
- configurable registry path (default: `registry.json`)
- unit tests for core logic
- cli and integration tests for end-to-end behavior

## building and running

you need rust and cargo installed

## build and commands:

```bash
cargo build

## run test: 
cargo test

## run program 
cargo run -- <command> [args...]

## hash a single file 
cargo run -- hash <PATH>

## add files/directories to registry, creates one if does not exist yet; updates last_verified to "now" for everything it adds 
cargo run -- add <PATH>

## same, but using a custom registry file
cargo run -- --registry my-registry.json add <PATH> 

## verify/check files against registry (loads registry.json as default)
## Unchanged, Modified, Missing, New;  verifications results 
cargo run -- verify

## verify using a custom registry
cargo run -- --registry my-registry.json verify

## list without hashing, reads registry file only. can print outright or in json 
## default list
cargo run -- list

## JSON list
cargo run -- list --json

## listing from a specific registry file
cargo run -- --registry my-registry.json list

## prune missing files from the default registry.json
cargo run -- prune

# same but on a custom registry
cargo run -- --registry my-registry.json prune

# use a separate registry for experiments
cargo run -- --registry test-registry.json add <PATH> 
cargo run -- --registry test-registry.json verify
cargo run -- --registry test-registry.json list
cargo run -- --registry test-registry.json prune

```
 
## final comments
this tool is for educational purposes only and shouldn't be used for any nefarious or illegal purposes 






















// initial outine: 
/*
 Project 1: File Integrity Checker

 Build a tool to hash files and detect unauthorized changes.

 Tech Stack: 

 Language: Java (primary) or Rust (optional)


 Java libs: java.security.MessageDigest (SHA-256), java.nio.file.*


 Rust crates: sha2, clap (CLI), serde + serde_json (registry file), walkdir


 Tools: Git, GitHub, CLI
 Core Concepts:
 Cryptographic hashing (SHA-256), collisions vs. integrity
 File I/O (read streams, large files, buffering)
 CLI argument parsing and flags
 State/registry design (JSON file with path, hash, last-checked time)
 Error handling (permissions, missing files, moved/renamed files)


 User Stories:
 As a user, I can hash a file and see its SHA-256 value so I can record its integrity.
 As a user, I can add a file to a registry so future scans can verify it hasn’t changed.
 As a user, I can run “verify” to compare current hashes to stored ones and see which files changed.
 As a user, I can scan a directory recursively so I don’t have to add files one by one.
 As a user, I can export a report (JSON/CSV) so I can share results or archive them.



*/
