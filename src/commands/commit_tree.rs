//! # Commit-Tree Command
//!
//! Create a commit object from a tree hash.
//!
//! ## Usage
//!
//! ```bash
//! # Create initial commit (no parent)
//! rit commit-tree <tree-hash> -m "Initial commit"
//!
//! # Create commit with parent
//! rit commit-tree <tree-hash> -p <parent-hash> -m "Second commit"
//!
//! # Create merge commit (multiple parents)
//! rit commit-tree <tree-hash> -p <parent1> -p <parent2> -m "Merge"
//! ```

use std::env;
use anyhow::Result;

use crate::Repository;
use crate::objects::commit::{Commit, Author};
use crate::commands::hash_object;

/// Get author information from environment or use defaults
///
/// Checks for:
/// - GIT_AUTHOR_NAME / GIT_COMMITTER_NAME
/// - GIT_AUTHOR_EMAIL / GIT_COMMITTER_EMAIL
/// - Falls back to system defaults
fn get_author() -> Author {
    let name = env::var("GIT_AUTHOR_NAME")
        .or_else(|_| env::var("GIT_COMMITTER_NAME"))
        .unwrap_or_else(|_| {
            // Try to get system username
            env::var("USER")
                .or_else(|_| env::var("USERNAME"))
                .unwrap_or_else(|_| "Unknown".to_string())
        });

    let email = env::var("GIT_AUTHOR_EMAIL")
        .or_else(|_| env::var("GIT_COMMITTER_EMAIL"))
        .unwrap_or_else(|_| format!("{}@localhost", name.to_lowercase().replace(' ', ".")));

    Author::new(&name, &email)
}

/// Execute the commit-tree command
///
/// # Arguments
///
/// * `tree_hash` - SHA-1 hash of the tree object
/// * `parents` - Optional parent commit hashes
/// * `message` - Commit message
///
/// # Example
///
/// ```no_run
/// use rit::commands::commit_tree::run;
///
/// // Initial commit
/// run("abc123...", vec![], "Initial commit").unwrap();
///
/// // With parent
/// run("def456...", vec!["abc123..."], "Second commit").unwrap();
/// ```
pub fn run(tree_hash: &str, parents: Vec<String>, message: &str) -> Result<String> {
    let repo = Repository::find()?;

    // Verify tree exists
    let tree_path = repo.objects_dir().join(&tree_hash[..2]).join(&tree_hash[2..]);
    if !tree_path.exists() {
        anyhow::bail!("tree object not found: {}", tree_hash);
    }

    // Get author info
    let author = get_author();
    let committer = get_author(); // For now, same as author

    // Create commit object
    let commit = Commit::new(
        tree_hash.to_string(),
        parents,
        author,
        committer,
        message.to_string(),
    );

    // Serialize commit
    let commit_content = commit.serialize();
    let commit_bytes = commit_content.as_bytes();

    // Store commit object
    let commit_hash = hash_object::store_object(&repo, "commit", commit_bytes)?;

    println!("{}", commit_hash);
    Ok(commit_hash)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::Repository;

    #[test]
    fn test_get_author() {
        // Should not panic
        let author = get_author();
        assert!(!author.name.is_empty());
        assert!(!author.email.is_empty());
    }

    #[test]
    fn test_commit_tree_initial() {
        let temp = tempdir().unwrap();
        let repo = Repository::init(temp.path()).unwrap();

        // Create a tree first
        std::fs::write(temp.path().join("test.txt"), b"test").unwrap();
        std::env::set_current_dir(temp.path()).unwrap();

        let tree_hash = crate::commands::write_tree::run().unwrap();

        // Create commit
        let commit_hash = run(&tree_hash, vec![], "Initial commit").unwrap();

        // Verify commit was stored
        let commit_path = repo.objects_dir().join(&commit_hash[..2]).join(&commit_hash[2..]);
        assert!(commit_path.exists());

        // Read it back
        let object = crate::commands::cat_file::read_object(&repo, &commit_hash).unwrap();
        assert_eq!(object.object_type, "commit");

        std::env::set_current_dir("/").unwrap();
    }

    #[test]
    fn test_commit_tree_with_parent() {
        let temp = tempdir().unwrap();
        let repo = Repository::init(temp.path()).unwrap();

        std::fs::write(temp.path().join("test.txt"), b"test").unwrap();
        std::env::set_current_dir(temp.path()).unwrap();

        let tree_hash = crate::commands::write_tree::run().unwrap();

        // First commit
        let parent_hash = run(&tree_hash, vec![], "First commit").unwrap();

        // Second commit with parent
        let commit_hash = run(&tree_hash, vec![parent_hash.clone()], "Second commit").unwrap();

        // Verify commit has parent
        let object = crate::commands::cat_file::read_object(&repo, &commit_hash).unwrap();
        let commit_content = String::from_utf8_lossy(&object.content);
        assert!(commit_content.contains(&format!("parent {}", parent_hash)));

        std::env::set_current_dir("/").unwrap();
    }
}

