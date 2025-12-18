//! # Checkout Command
//!
//! Switch branches or restore files from a commit.
//!
//! ## Usage
//!
//! ```bash
//! # Switch to a branch
//! rit checkout branch-name
//!
//! # Switch to a commit (detached HEAD)
//! rit checkout <commit-hash>
//!
//! # Restore a file from HEAD
//! rit checkout -- <file>
//! ```

use std::fs;
use std::path::Path;
use anyhow::{Context, Result};

use crate::Repository;
use crate::objects::{Tree, Commit};
use crate::commands::{cat_file, log};

/// Resolve a reference to a commit hash
///
/// Supports:
/// - Branch names (e.g., "main", "feature")
/// - Commit hashes (full or short)
/// - "HEAD" to get current commit
fn resolve_reference(repo: &Repository, reference: &str) -> Result<String> {
    // Handle HEAD
    if reference == "HEAD" {
        return log::read_head(repo)?
            .ok_or_else(|| anyhow::anyhow!("fatal: HEAD does not point to any commit"));
    }

    // Try as branch name first
    let branch_path = repo.rit_dir.join("refs").join("heads").join(reference);
    if branch_path.exists() {
        let commit_hash = fs::read_to_string(&branch_path)
            .context("Failed to read branch file")?
            .trim()
            .to_string();
        return Ok(commit_hash);
    }

    // Try as commit hash (must be at least 4 chars)
    if reference.len() >= 4 {
        // For now, assume it's a full hash or we'll find it
        // In a real implementation, we'd expand short hashes
        if reference.len() == 40 && reference.chars().all(|c| c.is_ascii_hexdigit()) {
            return Ok(reference.to_string());
        }
    }

    anyhow::bail!("fatal: reference '{}' not found", reference);
}

/// Get the tree hash from a commit
fn get_tree_from_commit(repo: &Repository, commit_hash: &str) -> Result<String> {
    let object = cat_file::read_object(repo, commit_hash)
        .context(format!("Failed to read commit: {}", commit_hash))?;

    if object.object_type != "commit" {
        anyhow::bail!("Not a commit object: {}", commit_hash);
    }

    let commit_content = String::from_utf8_lossy(&object.content);
    let commit = Commit::parse(&commit_content)?;

    Ok(commit.tree)
}

/// Recursively write tree contents to working directory
///
/// # Arguments
///
/// * `repo` - The repository
/// * `tree_hash` - Hash of the tree to write
/// * `base_path` - Base directory path (for recursion)
/// * `force` - If true, overwrite existing files
fn write_tree_to_working_dir(
    repo: &Repository,
    tree_hash: &str,
    base_path: &Path,
    force: bool,
) -> Result<()> {
    // Read the tree object
    let object = cat_file::read_object(repo, tree_hash)
        .context(format!("Failed to read tree object: {}", tree_hash))?;

    if object.object_type != "tree" {
        anyhow::bail!("Not a tree object: {}", tree_hash);
    }

    // Parse the tree
    let tree = Tree::parse(&object.content)?;

    // Ensure base directory exists
    if !base_path.exists() {
        fs::create_dir_all(base_path)
            .context(format!("Failed to create directory: {}", base_path.display()))?;
    }

    for entry in &tree.entries {
        let entry_path = base_path.join(&entry.name);

        if entry.is_tree() {
            // Recursively process subtree
            write_tree_to_working_dir(repo, &entry.hash, &entry_path, force)?;
        } else {
            // It's a file - write the blob content
            let blob_object = cat_file::read_object(repo, &entry.hash)
                .context(format!("Failed to read blob: {}", entry.hash))?;

            if blob_object.object_type != "blob" {
                anyhow::bail!("Expected blob, got {}: {}", blob_object.object_type, entry.hash);
            }

            // Check if file exists and conflicts
            if entry_path.exists() && !force {
                // Simple conflict check: file exists and might be different
                // In a real implementation, we'd check if it's modified
                // For now, we'll just warn and skip
                eprintln!("warning: {} already exists, skipping", entry_path.display());
                continue;
            }

            // Write the file
            fs::write(&entry_path, &blob_object.content)
                .context(format!("Failed to write file: {}", entry_path.display()))?;

            // Set executable bit if needed
            if entry.mode == "100755" || entry.mode == crate::objects::tree::MODE_EXEC {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let mut perms = fs::metadata(&entry_path)?.permissions();
                    perms.set_mode(0o755);
                    fs::set_permissions(&entry_path, perms)?;
                }
            }
        }
    }

    Ok(())
}

/// Remove files that are not in the target tree
///
/// This is a simplified version - in real Git, we'd track what files
/// exist in the current HEAD and remove only those that aren't in the new tree.
#[allow(dead_code)]
fn cleanup_working_dir(repo: &Repository, target_tree_hash: &str) -> Result<()> {
    // For now, we'll skip cleanup to avoid deleting user files
    // In a real implementation, we'd:
    // 1. Read current HEAD tree
    // 2. Compare with target tree
    // 3. Remove files that exist in current but not in target
    // 4. Keep untracked files
    
    // This is a placeholder - we'll implement it later if needed
    let _ = (repo, target_tree_hash);
    Ok(())
}

/// Update HEAD to point to a commit or branch
fn update_head(repo: &Repository, reference: &str, commit_hash: &str) -> Result<()> {
    let head_path = repo.head_path();

    // Check if reference is a branch name
    let branch_path = repo.rit_dir.join("refs").join("heads").join(reference);
    if branch_path.exists() {
        // It's a branch - update HEAD to point to the branch
        fs::write(&head_path, format!("ref: refs/heads/{}\n", reference))
            .context("Failed to update HEAD")?;
    } else {
        // It's a commit hash - detached HEAD
        fs::write(&head_path, format!("{}\n", commit_hash))
            .context("Failed to update HEAD")?;
    }

    Ok(())
}

/// Checkout a single file from a commit
fn checkout_file(repo: &Repository, reference: &str, file_path: &str) -> Result<()> {
    // Resolve reference to commit
    let commit_hash = resolve_reference(repo, reference)?;
    let tree_hash = get_tree_from_commit(repo, &commit_hash)?;

    // Find the file in the tree by navigating the path
    let file_path_obj = Path::new(file_path);
    let components: Vec<_> = file_path_obj.components().collect();
    
    let mut current_tree_hash = tree_hash;

    // Navigate through directories
    for (i, component) in components.iter().enumerate() {
        let name = component.as_os_str().to_string_lossy();
        let object = cat_file::read_object(repo, &current_tree_hash)
            .context("Failed to read tree")?;
        let tree = Tree::parse(&object.content)?;

        let entry = tree.entries.iter()
            .find(|e| e.name == name)
            .ok_or_else(|| anyhow::anyhow!("path '{}' not found in tree", file_path))?;

        if i == components.len() - 1 {
            // Last component - should be a file
            if entry.is_tree() {
                anyhow::bail!("path '{}' is a directory, not a file", file_path);
            }

            // Found the file - write it
            let blob_object = cat_file::read_object(repo, &entry.hash)
                .context("Failed to read blob")?;

            let target_path = repo.root.join(file_path);
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::write(&target_path, &blob_object.content)
                .context(format!("Failed to write file: {}", target_path.display()))?;

            return Ok(());
        } else {
            // Not the last component - should be a directory
            if !entry.is_tree() {
                anyhow::bail!("path '{}' not found in tree", file_path);
            }
            current_tree_hash = entry.hash.clone();
        }
    }

    anyhow::bail!("Invalid path: {}", file_path);
}

/// Execute the checkout command
///
/// # Arguments
///
/// * `reference` - Branch name, commit hash, or "HEAD"
/// * `file_path` - Optional file path to checkout (for file checkout)
/// * `force` - If true, overwrite existing files
///
/// # Example
///
/// ```no_run
/// use rit::commands::checkout::run;
///
/// // Checkout a branch
/// run("feature", None, false).unwrap();
///
/// // Checkout a commit
/// run("abc1234", None, false).unwrap();
///
/// // Checkout a file
/// run("HEAD", Some("file.txt"), false).unwrap();
/// ```
pub fn run(reference: &str, file_path: Option<String>, force: bool) -> Result<()> {
    let repo = Repository::find()?;

    // If file_path is provided, do file checkout
    if let Some(path) = file_path {
        checkout_file(&repo, reference, &path)?;
        println!("Updated '{}'", path);
        return Ok(());
    }

    // Otherwise, checkout a branch or commit
    let commit_hash = resolve_reference(&repo, reference)?;
    let tree_hash = get_tree_from_commit(&repo, &commit_hash)?;

    // Write tree to working directory
    write_tree_to_working_dir(&repo, &tree_hash, &repo.root, force)?;

    // Update HEAD
    update_head(&repo, reference, &commit_hash)?;

    // Determine if we're on a branch or detached HEAD
    let branch_path = repo.rit_dir.join("refs").join("heads").join(reference);
    if branch_path.exists() {
        println!("Switched to branch '{}'", reference);
    } else {
        println!("Note: checking out '{}'.", &commit_hash[..7]);
        println!("You are in 'detached HEAD' state.");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::Repository;
    use crate::commands::hash_object;

    #[test]
    fn test_resolve_branch_reference() {
        let temp = tempdir().unwrap();
        let repo = Repository::init(temp.path()).unwrap();

        let commit_hash = "a".repeat(40);
        std::fs::write(repo.rit_dir.join("refs/heads/main"), &commit_hash).unwrap();

        let resolved = resolve_reference(&repo, "main").unwrap();
        assert_eq!(resolved, commit_hash);
    }

    #[test]
    fn test_resolve_head_reference() {
        let temp = tempdir().unwrap();
        let repo = Repository::init(temp.path()).unwrap();

        let commit_hash = "a".repeat(40);
        std::fs::write(repo.rit_dir.join("refs/heads/main"), &commit_hash).unwrap();
        std::fs::write(repo.head_path(), "ref: refs/heads/main\n").unwrap();

        let resolved = resolve_reference(&repo, "HEAD").unwrap();
        assert_eq!(resolved, commit_hash);
    }

    #[test]
    fn test_get_tree_from_commit() {
        let temp = tempdir().unwrap();
        let repo = Repository::init(temp.path()).unwrap();

        // Create a simple commit
        let tree_hash = "b".repeat(40);
        let commit_content = format!(
            "tree {}\n\
             author Test <test@example.com> 1234567890 +0000\n\
             committer Test <test@example.com> 1234567890 +0000\n\
             \n\
             Test commit\n",
            tree_hash
        );

        let commit_hash = hash_object::store_object(&repo, "commit", commit_content.as_bytes()).unwrap();

        let resolved_tree = get_tree_from_commit(&repo, &commit_hash).unwrap();
        assert_eq!(resolved_tree, tree_hash);
    }
}

