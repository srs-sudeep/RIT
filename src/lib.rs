//! # Rit - A Git Implementation in Rust
//!
//! Rit is an educational implementation of Git's core functionality,
//! built from scratch in Rust to understand version control internals.
//!
//! ## Architecture Overview
//!
//! Git (and Rit) is built on a simple but powerful concept: a content-addressable filesystem.
//! Every piece of data is stored as an "object" identified by its SHA-1 hash.
//!
//! ### Object Types
//!
//! - **Blob**: Raw file contents
//! - **Tree**: Directory listing (maps names to blob/tree hashes)
//! - **Commit**: Snapshot with metadata (author, message, parent commit, tree hash)
//!
//! ### Repository Structure
//!
//! ```text
//! .rit/
//! ├── HEAD            # Points to current branch (e.g., "ref: refs/heads/main")
//! ├── objects/        # Object database
//! │   ├── ab/         # First 2 chars of hash
//! │   │   └── cdef... # Remaining hash chars (zlib compressed)
//! │   └── ...
//! ├── refs/
//! │   ├── heads/      # Branch pointers
//! │   │   └── main    # Contains commit hash
//! │   └── tags/       # Tag pointers
//! └── index           # Staging area (binary format)
//! ```
//!
//! ## Usage Example
//!
//! ```bash
//! # Initialize a new repository
//! rit init
//!
//! # Hash a file and store it
//! rit hash-object -w README.md
//!
//! # Read an object
//! rit cat-file -p <hash>
//! ```

pub mod commands;
pub mod objects;
pub mod index;

use std::path::{Path, PathBuf};
use anyhow::{Context, Result};

/// The name of the rit directory (like .git)
pub const RIT_DIR: &str = ".rit";

/// Represents a Rit repository
#[derive(Debug)]
pub struct Repository {
    /// The root directory of the repository (where .rit lives)
    pub root: PathBuf,
    /// The .rit directory path
    pub rit_dir: PathBuf,
}

impl Repository {
    /// Find the repository root by walking up from the current directory
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rit::Repository;
    ///
    /// let repo = Repository::find().expect("Not in a rit repository");
    /// println!("Repository root: {:?}", repo.root);
    /// ```
    pub fn find() -> Result<Self> {
        let current_dir = std::env::current_dir()?;
        Self::find_from(&current_dir)
    }

    /// Find repository starting from a specific path
    pub fn find_from(start: &Path) -> Result<Self> {
        let mut current = start.to_path_buf();

        loop {
            let rit_dir = current.join(RIT_DIR);
            if rit_dir.is_dir() {
                return Ok(Self {
                    root: current,
                    rit_dir,
                });
            }

            if !current.pop() {
                anyhow::bail!("fatal: not a rit repository (or any of the parent directories): .rit");
            }
        }
    }

    /// Initialize a new repository at the given path
    ///
    /// Creates the `.rit` directory structure:
    /// - `.rit/objects/` - Object database
    /// - `.rit/refs/heads/` - Branch references
    /// - `.rit/HEAD` - Current branch pointer
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rit::Repository;
    /// use std::path::Path;
    ///
    /// let repo = Repository::init(Path::new(".")).expect("Failed to init");
    /// ```
    pub fn init(path: &Path) -> Result<Self> {
        let root = path.to_path_buf();
        let rit_dir = root.join(RIT_DIR);

        if rit_dir.exists() {
            println!("Reinitialized existing rit repository in {}", rit_dir.display());
        } else {
            // Create directory structure
            std::fs::create_dir(&rit_dir)
                .context("Failed to create .rit directory")?;
            std::fs::create_dir(rit_dir.join("objects"))
                .context("Failed to create objects directory")?;
            std::fs::create_dir(rit_dir.join("refs"))
                .context("Failed to create refs directory")?;
            std::fs::create_dir(rit_dir.join("refs").join("heads"))
                .context("Failed to create refs/heads directory")?;
            std::fs::create_dir(rit_dir.join("refs").join("tags"))
                .context("Failed to create refs/tags directory")?;

            // Create HEAD file pointing to main branch
            std::fs::write(rit_dir.join("HEAD"), "ref: refs/heads/main\n")
                .context("Failed to create HEAD file")?;

            println!("Initialized empty rit repository in {}", rit_dir.display());
        }

        Ok(Self { root, rit_dir })
    }

    /// Get the path to the objects directory
    pub fn objects_dir(&self) -> PathBuf {
        self.rit_dir.join("objects")
    }

    /// Get the path to the refs directory
    pub fn refs_dir(&self) -> PathBuf {
        self.rit_dir.join("refs")
    }

    /// Get the path to the HEAD file
    pub fn head_path(&self) -> PathBuf {
        self.rit_dir.join("HEAD")
    }

    /// Get the path to the index file
    pub fn index_path(&self) -> PathBuf {
        self.rit_dir.join("index")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_init_creates_structure() {
        let temp = tempdir().unwrap();
        let repo = Repository::init(temp.path()).unwrap();

        assert!(repo.rit_dir.exists());
        assert!(repo.objects_dir().exists());
        assert!(repo.refs_dir().exists());
        assert!(repo.head_path().exists());
    }
}

