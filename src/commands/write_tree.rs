//! # Write-Tree Command
//!
//! Create a tree object from the current working directory.
//!
//! ## How it works
//!
//! 1. Walk the current directory (excluding .rit/)
//! 2. For each file: hash it as a blob, store it
//! 3. For each subdirectory: recursively create a tree
//! 4. Build a tree object with all entries
//! 5. Store the tree object
//! 6. Return the tree hash
//!
//! ## Usage
//!
//! ```bash
//! # Create tree from current directory
//! rit write-tree
//! ```

use std::fs;
use std::path::Path;
use anyhow::{Context, Result};

use crate::Repository;
use crate::objects::{Tree, TreeEntry};
use crate::commands::hash_object;

/// Write a tree object for a directory
///
/// # Arguments
///
/// * `repo` - The repository
/// * `dir_path` - Path to the directory to process
/// * `base_path` - Base path for relative names (for recursion)
///
/// # Returns
///
/// The SHA-1 hash of the created tree object
fn write_tree_recursive(
    repo: &Repository,
    dir_path: &Path,
    base_path: &Path,
) -> Result<String> {
    let mut tree = Tree::new();

    // Read directory entries
    let mut entries: Vec<_> = fs::read_dir(dir_path)?
        .filter_map(|e| e.ok())
        .collect();

    // Sort entries by name (Git requirement)
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let path = entry.path();
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        // Skip .rit directory
        if file_name_str == ".rit" {
            continue;
        }

        let metadata = entry.metadata()?;

        if metadata.is_file() {
            // Hash and store the file as a blob
            let content = fs::read(&path)
                .context(format!("Failed to read file: {}", path.display()))?;

            // Determine if file is executable
            let mode = if is_executable(&metadata) {
                crate::objects::tree::MODE_EXEC
            } else {
                crate::objects::tree::MODE_FILE
            };

            let blob_hash = hash_object::store_object(repo, "blob", &content)?;

            // Add entry to tree
            tree.add_entry(TreeEntry::new(
                mode.to_string(),
                file_name_str.to_string(),
                blob_hash,
            ));
        } else if metadata.is_dir() {
            // Recursively create tree for subdirectory
            let subtree_hash = write_tree_recursive(repo, &path, base_path)?;

            // Add subtree entry
            tree.add_entry(TreeEntry::directory(
                file_name_str.to_string(),
                subtree_hash,
            ));
        }
        // Skip symlinks and other file types for now
    }

    // Sort entries (Git requirement: dirs with trailing /)
    tree.sort();

    // Serialize and store the tree
    let tree_content = tree.serialize()?;
    let tree_hash = hash_object::store_object(repo, "tree", &tree_content)?;

    Ok(tree_hash)
}

/// Check if a file is executable
///
/// On Unix systems, checks the executable bit.
/// On Windows, always returns false (no executable bit).
fn is_executable(metadata: &fs::Metadata) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        metadata.permissions().mode() & 0o111 != 0
    }

    #[cfg(not(unix))]
    {
        false
    }
}

/// Execute the write-tree command
///
/// Creates a tree object from the current working directory.
///
/// # Example
///
/// ```no_run
/// use rit::commands::write_tree::run;
///
/// let hash = run().unwrap();
/// println!("Tree hash: {}", hash);
/// ```
pub fn run() -> Result<String> {
    let repo = Repository::find()?;
    let current_dir = std::env::current_dir()?;

    // Create tree from current directory
    let tree_hash = write_tree_recursive(&repo, &current_dir, &current_dir)?;

    println!("{}", tree_hash);
    Ok(tree_hash)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::Repository;

    #[test]
    fn test_write_tree_simple() {
        let temp = tempdir().unwrap();
        let repo = Repository::init(temp.path()).unwrap();

        // Create test files
        fs::write(temp.path().join("file1.txt"), b"content1").unwrap();
        fs::write(temp.path().join("file2.txt"), b"content2").unwrap();

        // Change to temp directory
        std::env::set_current_dir(temp.path()).unwrap();

        // Write tree
        let hash = write_tree_recursive(&repo, temp.path(), temp.path()).unwrap();

        // Verify hash is valid (40 hex chars)
        assert_eq!(hash.len(), 40);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));

        // Restore directory
        std::env::set_current_dir("/").unwrap();
    }

    #[test]
    fn test_write_tree_with_subdir() {
        let temp = tempdir().unwrap();
        let repo = Repository::init(temp.path()).unwrap();

        // Create files and subdirectory
        fs::write(temp.path().join("root.txt"), b"root").unwrap();
        fs::create_dir(temp.path().join("subdir")).unwrap();
        fs::write(temp.path().join("subdir").join("sub.txt"), b"sub").unwrap();

        let hash = write_tree_recursive(&repo, temp.path(), temp.path()).unwrap();

        // Should have created tree with 2 entries: root.txt and subdir
        assert_eq!(hash.len(), 40);

        // Read the tree back
        let object = crate::commands::cat_file::read_object(&repo, &hash).unwrap();
        assert_eq!(object.object_type, "tree");

        // Restore directory
        std::env::set_current_dir("/").unwrap();
    }
}

