// mo elbahwati 
// project 1 
// init commit, work in progress file integrity checker

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
