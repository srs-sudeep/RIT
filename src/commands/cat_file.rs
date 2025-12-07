//! # Cat-File Command
//!
//! Read and display the contents of a git object.
//!
//! ## How it works
//!
//! 1. Take the object hash (e.g., `abc123...`)
//! 2. Find the file at `.rit/objects/ab/c123...`
//! 3. Decompress with zlib
//! 4. Parse header to get type and size
//! 5. Return the content
//!
//! ## Usage
//!
//! ```bash
//! # Pretty-print object contents
//! rit cat-file -p abc123def456...
//!
//! # Show object type
//! rit cat-file -t abc123def456...
//!
//! # Show object size
//! rit cat-file -s abc123def456...
//! ```

use std::io::Read;
use anyhow::{Context, Result};
use flate2::read::ZlibDecoder;

use crate::Repository;

/// Represents a parsed git object
#[derive(Debug)]
pub struct GitObject {
    /// The type of object ("blob", "tree", "commit", "tag")
    pub object_type: String,
    /// The size of the content in bytes
    pub size: usize,
    /// The raw content (after header)
    pub content: Vec<u8>,
}

/// Read and parse an object from the repository
///
/// # Arguments
///
/// * `repo` - The repository
/// * `hash` - The full or partial object hash
///
/// # Returns
///
/// The parsed GitObject
///
/// # Example
///
/// ```no_run
/// use rit::Repository;
/// use rit::commands::cat_file::read_object;
///
/// let repo = Repository::find().unwrap();
/// let obj = read_object(&repo, "abc123...").unwrap();
/// println!("Type: {}, Size: {}", obj.object_type, obj.size);
/// ```
pub fn read_object(repo: &Repository, hash: &str) -> Result<GitObject> {
    // Validate hash length
    if hash.len() < 4 {
        anyhow::bail!("hash too short: {}", hash);
    }

    // Find object file
    let object_path = repo.objects_dir().join(&hash[..2]).join(&hash[2..]);

    if !object_path.exists() {
        anyhow::bail!("object not found: {}", hash);
    }

    // Read and decompress
    let compressed = std::fs::read(&object_path)
        .context("Failed to read object file")?;

    let mut decoder = ZlibDecoder::new(&compressed[..]);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)
        .context("Failed to decompress object")?;

    // Parse header
    let null_pos = decompressed.iter()
        .position(|&b| b == 0)
        .context("Invalid object format: no null byte found")?;

    let header = String::from_utf8_lossy(&decompressed[..null_pos]);
    let mut parts = header.split(' ');

    let object_type = parts.next()
        .context("Invalid object format: no type")?
        .to_string();

    let size: usize = parts.next()
        .context("Invalid object format: no size")?
        .parse()
        .context("Invalid object format: size not a number")?;

    let content = decompressed[null_pos + 1..].to_vec();

    // Verify size
    if content.len() != size {
        anyhow::bail!("Size mismatch: header says {} but content is {} bytes",
            size, content.len());
    }

    Ok(GitObject {
        object_type,
        size,
        content,
    })
}

/// Execute the cat-file command
///
/// # Arguments
///
/// * `hash` - The object hash to read
/// * `pretty_print` - If true, format output nicely
///
/// # Example
///
/// ```no_run
/// use rit::commands::cat_file::run;
///
/// run("abc123...", true).unwrap();
/// ```
pub fn run(hash: &str, pretty_print: bool) -> Result<()> {
    let repo = Repository::find()?;
    let object = read_object(&repo, hash)?;

    if pretty_print {
        match object.object_type.as_str() {
            "blob" => {
                // Print blob content as-is
                print!("{}", String::from_utf8_lossy(&object.content));
            }
            "tree" => {
                // TODO: Parse and format tree entries
                println!("(tree formatting not yet implemented)");
                println!("Raw bytes: {:?}", object.content);
            }
            "commit" => {
                // Commit content is text, print as-is
                print!("{}", String::from_utf8_lossy(&object.content));
            }
            _ => {
                println!("Unknown object type: {}", object.object_type);
            }
        }
    } else {
        print!("{}", String::from_utf8_lossy(&object.content));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::commands::hash_object;

    #[test]
    fn test_roundtrip() {
        let temp = tempdir().unwrap();
        let repo = Repository::init(temp.path()).unwrap();

        // Store an object
        let content = b"Hello, World!";
        let hash = hash_object::store_object(&repo, "blob", content).unwrap();

        // Read it back
        let object = read_object(&repo, &hash).unwrap();
        assert_eq!(object.object_type, "blob");
        assert_eq!(object.size, content.len());
        assert_eq!(object.content, content);
    }
}

