//! # Branch Command
//!
//! Manage branches in the repository.
//!
//! ## Usage
//!
//! ```bash
//! # List all branches
//! rit branch
//!
//! # Create a new branch
//! rit branch feature-branch
//!
//! # Delete a branch
//! rit branch -d old-branch
//! ```

use std::fs;
use std::path::PathBuf;
use anyhow::{Context, Result};

use crate::Repository;
use crate::commands::log;

/// Get the current branch name from HEAD
///
/// Returns None if HEAD is detached or doesn't exist
fn get_current_branch(repo: &Repository) -> Result<Option<String>> {
    let head_path = repo.head_path();
    
    if !head_path.exists() {
        return Ok(None);
    }

    let head_content = fs::read_to_string(&head_path)
        .context("Failed to read HEAD file")?
        .trim()
        .to_string();

    // Check if it's a ref (e.g., "ref: refs/heads/main")
    if let Some(ref_path) = head_content.strip_prefix("ref: ") {
        let ref_path = ref_path.trim();
        // Extract branch name from "refs/heads/main"
        if let Some(branch_name) = ref_path.strip_prefix("refs/heads/") {
            return Ok(Some(branch_name.to_string()));
        }
    }

    Ok(None) // Detached HEAD
}

/// Get the path to a branch reference file
fn branch_ref_path(repo: &Repository, branch_name: &str) -> PathBuf {
    repo.rit_dir.join("refs").join("heads").join(branch_name)
}

/// List all branches
fn list_branches(repo: &Repository) -> Result<()> {
    let heads_dir = repo.rit_dir.join("refs").join("heads");
    
    if !heads_dir.exists() {
        // No branches directory, just show current branch if any
        if let Some(current) = get_current_branch(repo)? {
            println!("* {}", current);
        }
        return Ok(());
    }

    let current_branch = get_current_branch(repo)?;
    let mut branches = Vec::new();

    // Read all branch files
    if heads_dir.is_dir() {
        for entry in fs::read_dir(&heads_dir)
            .context("Failed to read refs/heads directory")? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(branch_name) = path.file_name().and_then(|n| n.to_str()) {
                    branches.push(branch_name.to_string());
                }
            }
        }
    }

    // Sort branches
    branches.sort();

    // Print branches, marking current one with *
    for branch in branches {
        if current_branch.as_ref().map(|b| b == &branch).unwrap_or(false) {
            println!("* {}", branch);
        } else {
            println!("  {}", branch);
        }
    }

    Ok(())
}

/// Create a new branch
///
/// The new branch will point to the current HEAD commit
fn create_branch(repo: &Repository, branch_name: &str) -> Result<()> {
    // Validate branch name
    if branch_name.is_empty() {
        anyhow::bail!("fatal: branch name cannot be empty");
    }

    // Check for invalid characters (basic validation)
    if branch_name.contains('/') || branch_name.contains('\\') || branch_name.contains(' ') {
        anyhow::bail!("fatal: '{}' is not a valid branch name", branch_name);
    }

    let branch_path = branch_ref_path(repo, branch_name);

    // Check if branch already exists
    if branch_path.exists() {
        anyhow::bail!("fatal: a branch named '{}' already exists", branch_name);
    }

    // Get current HEAD commit
    let current_commit = match log::read_head(repo)? {
        Some(hash) => hash,
        None => {
            anyhow::bail!("fatal: not a valid object name: 'HEAD'");
        }
    };

    // Create the branch file
    fs::write(&branch_path, format!("{}\n", current_commit))
        .context(format!("Failed to create branch file: {}", branch_path.display()))?;

    println!("Created branch '{}'", branch_name);
    Ok(())
}

/// Delete a branch
fn delete_branch(repo: &Repository, branch_name: &str, force: bool) -> Result<()> {
    let branch_path = branch_ref_path(repo, branch_name);

    if !branch_path.exists() {
        anyhow::bail!("error: branch '{}' not found", branch_name);
    }

    // Check if trying to delete current branch
    if let Some(current) = get_current_branch(repo)? {
        if current == branch_name {
            if !force {
                anyhow::bail!("error: Cannot delete branch '{}' checked out at '{}'", branch_name, repo.rit_dir.join("HEAD").display());
            }
        }
    }

    // Read the branch commit
    let branch_commit = fs::read_to_string(&branch_path)
        .context("Failed to read branch file")?
        .trim()
        .to_string();

    // Get current HEAD commit
    let current_commit = log::read_head(repo)?;

    // Check if branch is merged (points to same commit as HEAD)
    // If it points to the same commit, it IS merged, so we can delete it
    // If it points to a different commit, it's NOT merged
    if let Some(current) = current_commit {
        if branch_commit != current {
            // Branch points to different commit - not merged
            if !force {
                anyhow::bail!("error: The branch '{}' is not fully merged.\nIf you are sure you want to delete it, run 'rit branch -D {}'.", branch_name, branch_name);
            }
        }
        // If branch_commit == current, it's merged, so we can delete it
    }

    // Delete the branch file
    fs::remove_file(&branch_path)
        .context(format!("Failed to delete branch file: {}", branch_path.display()))?;

    println!("Deleted branch '{}' (was {})", branch_name, &branch_commit[..7.min(branch_commit.len())]);
    Ok(())
}

/// Execute the branch command
///
/// # Arguments
///
/// * `branch_name` - Optional branch name to create
/// * `delete` - If true, delete the branch instead of creating
/// * `force` - If true, force delete even if not merged
///
/// # Example
///
/// ```no_run
/// use rit::commands::branch::run;
///
/// // List branches
/// run(None, false, false).unwrap();
///
/// // Create branch
/// run(Some("feature".to_string()), false, false).unwrap();
///
/// // Delete branch
/// run(Some("old-branch".to_string()), true, false).unwrap();
/// ```
pub fn run(branch_name: Option<String>, delete: bool, force: bool) -> Result<()> {
    let repo = Repository::find()?;

    if delete {
        // Delete branch
        let name = branch_name.ok_or_else(|| anyhow::anyhow!("fatal: branch name required for deletion"))?;
        delete_branch(&repo, &name, force)?;
    } else if let Some(name) = branch_name {
        // Create branch
        create_branch(&repo, &name)?;
    } else {
        // List branches
        list_branches(&repo)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::Repository;

    #[test]
    fn test_list_branches_empty() {
        let temp = tempdir().unwrap();
        let repo = Repository::init(temp.path()).unwrap();
        
        // Should not panic
        list_branches(&repo).unwrap();
    }

    #[test]
    fn test_create_branch() {
        let temp = tempdir().unwrap();
        let repo = Repository::init(temp.path()).unwrap();
        
        // Create a commit first
        let commit_hash = "a".repeat(40);
        std::fs::write(repo.rit_dir.join("refs/heads/main"), &commit_hash).unwrap();
        std::fs::write(repo.head_path(), "ref: refs/heads/main\n").unwrap();
        
        // Create a new branch
        create_branch(&repo, "feature").unwrap();
        
        // Verify branch file exists
        let branch_path = branch_ref_path(&repo, "feature");
        assert!(branch_path.exists());
        
        // Verify it points to the same commit
        let branch_content = fs::read_to_string(&branch_path).unwrap();
        let branch_commit = branch_content.trim();
        assert_eq!(branch_commit, commit_hash);
    }

    #[test]
    fn test_create_duplicate_branch() {
        let temp = tempdir().unwrap();
        let repo = Repository::init(temp.path()).unwrap();
        
        let commit_hash = "a".repeat(40);
        std::fs::write(repo.rit_dir.join("refs/heads/main"), &commit_hash).unwrap();
        std::fs::write(repo.head_path(), "ref: refs/heads/main\n").unwrap();
        
        create_branch(&repo, "feature").unwrap();
        
        // Try to create again - should fail
        assert!(create_branch(&repo, "feature").is_err());
    }

    #[test]
    fn test_delete_branch() {
        let temp = tempdir().unwrap();
        let repo = Repository::init(temp.path()).unwrap();
        
        let commit_hash = "a".repeat(40);
        std::fs::write(repo.rit_dir.join("refs/heads/main"), &commit_hash).unwrap();
        std::fs::write(repo.head_path(), "ref: refs/heads/main\n").unwrap();
        
        // Create and then delete branch
        create_branch(&repo, "feature").unwrap();
        delete_branch(&repo, "feature", false).unwrap();
        
        // Verify branch file is gone
        let branch_path = branch_ref_path(&repo, "feature");
        assert!(!branch_path.exists());
    }

    #[test]
    fn test_get_current_branch() {
        let temp = tempdir().unwrap();
        let repo = Repository::init(temp.path()).unwrap();
        
        std::fs::write(repo.head_path(), "ref: refs/heads/main\n").unwrap();
        
        let current = get_current_branch(&repo).unwrap();
        assert_eq!(current, Some("main".to_string()));
    }
}

