//! # Hash-Object Command
//!
//! Compute the SHA-1 hash of a file and optionally store it in the object database.
//!
//! ## Git Object Format
//!
//! Every git object is stored as:
//! ```text
//! <type> <size>\0<content>
//! ```
//!
//! For a blob:
//! ```text
//! blob 13\0Hello, World!
//! ```
//!
//! This is then:
//! 1. SHA-1 hashed to get the object ID
//! 2. Compressed with zlib
//! 3. Stored at `.rit/objects/ab/cdef...` (first 2 chars / rest)
//!
//! ## Usage
//!
//! ```bash
//! # Just compute hash (don't store)
//! rit hash-object README.md
//!
//! # Compute hash and store in object database
//! rit hash-object -w README.md
//! ```

use std::io::Write;
use anyhow::{Context, Result};
use sha1::{Sha1, Digest};
use flate2::write::ZlibEncoder;
use flate2::Compression;

use crate::Repository;

/// Compute the SHA-1 hash of content with the git object header
///
/// # Arguments
///
/// * `object_type` - The type of object ("blob", "tree", "commit")
/// * `content` - The raw content bytes
///
/// # Returns
///
/// The hex-encoded SHA-1 hash
///
/// # Example
///
/// ```
/// use rit::commands::hash_object::hash_content;
///
/// let hash = hash_content("blob", b"Hello, World!");
/// assert_eq!(hash.len(), 40); // SHA-1 produces 40 hex chars
/// ```
pub fn hash_content(object_type: &str, content: &[u8]) -> String {
    // Create header: "blob 13\0"
    let header = format!("{} {}\0", object_type, content.len());

    // Hash header + content
    let mut hasher = Sha1::new();
    hasher.update(header.as_bytes());
    hasher.update(content);

    hex::encode(hasher.finalize())
}

/// Store an object in the repository's object database
///
/// # Arguments
///
/// * `repo` - The repository
/// * `object_type` - The type of object ("blob", "tree", "commit")
/// * `content` - The raw content bytes
///
/// # Returns
///
/// The hex-encoded SHA-1 hash of the stored object
pub fn store_object(repo: &Repository, object_type: &str, content: &[u8]) -> Result<String> {
    let hash = hash_content(object_type, content);

    // Create object directory (first 2 chars of hash)
    let dir = repo.objects_dir().join(&hash[..2]);
    std::fs::create_dir_all(&dir)?;

    // Object file path (remaining chars)
    let object_path = dir.join(&hash[2..]);

    // Don't overwrite if already exists (objects are immutable)
    if !object_path.exists() {
        // Compress with zlib
        let header = format!("{} {}\0", object_type, content.len());
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(header.as_bytes())?;
        encoder.write_all(content)?;
        let compressed = encoder.finish()?;

        // Write to file
        std::fs::write(&object_path, compressed)
            .context("Failed to write object file")?;
    }

    Ok(hash)
}

/// Execute the hash-object command
///
/// # Arguments
///
/// * `file_path` - Path to the file to hash
/// * `write` - If true, store the object in the database
///
/// # Example
///
/// ```no_run
/// use rit::commands::hash_object::run;
///
/// // Just print hash
/// run("README.md", false).unwrap();
///
/// // Store in database
/// run("README.md", true).unwrap();
/// ```
pub fn run(file_path: &str, write: bool) -> Result<String> {
    let content = std::fs::read(file_path)
        .context(format!("Failed to read file: {}", file_path))?;

    let hash = if write {
        let repo = Repository::find()?;
        store_object(&repo, "blob", &content)?
    } else {
        hash_content("blob", &content)
    };

    println!("{}", hash);
    Ok(hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_content() {
        // This should match git's hash for the same content
        let hash = hash_content("blob", b"Hello, World!");
        assert_eq!(hash.len(), 40);
    }

    #[test]
    fn test_known_hash() {
        // "test content\n" hashed as blob should give known result
        // You can verify with: echo "test content" | git hash-object --stdin
        let hash = hash_content("blob", b"test content\n");
        assert_eq!(hash, "d670460b4b4aece5915caf5c68d12f560a9fe3e4");
    }
}

