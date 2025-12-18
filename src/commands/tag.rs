//! # Tag Command
//!
//! Create, list, or delete tags.
//!
//! ## Usage
//!
//! ```bash
//! # List all tags
//! rit tag
//!
//! # Create a lightweight tag
//! rit tag v1.0.0
//!
//! # Create an annotated tag
//! rit tag -a v1.0.0 -m "Release version 1.0.0"
//!
//! # Delete a tag
//! rit tag -d v1.0.0
//! ```

use std::fs;
use std::path::PathBuf;
use anyhow::{Context, Result};

use crate::Repository;
use crate::commands::log;

/// Get the path to a tag reference file
fn tag_ref_path(repo: &Repository, tag_name: &str) -> PathBuf {
    repo.rit_dir.join("refs").join("tags").join(tag_name)
}

/// List all tags
fn list_tags(repo: &Repository) -> Result<()> {
    let tags_dir = repo.rit_dir.join("refs").join("tags");
    
    if !tags_dir.exists() {
        return Ok(()); // No tags yet
    }

    let mut tags = Vec::new();

    // Read all tag files
    if tags_dir.is_dir() {
        for entry in fs::read_dir(&tags_dir)
            .context("Failed to read refs/tags directory")? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(tag_name) = path.file_name().and_then(|n| n.to_str()) {
                    tags.push(tag_name.to_string());
                }
            }
        }
    }

    // Sort tags
    tags.sort();

    // Print tags
    for tag in tags {
        println!("{}", tag);
    }

    Ok(())
}

/// Create a lightweight tag (just a ref pointing to a commit)
fn create_lightweight_tag(repo: &Repository, tag_name: &str) -> Result<()> {
    // Validate tag name
    if tag_name.is_empty() {
        anyhow::bail!("fatal: tag name cannot be empty");
    }

    // Check for invalid characters (basic validation)
    if tag_name.contains('/') || tag_name.contains('\\') || tag_name.contains(' ') {
        anyhow::bail!("fatal: '{}' is not a valid tag name", tag_name);
    }

    let tag_path = tag_ref_path(repo, tag_name);

    // Check if tag already exists
    if tag_path.exists() {
        anyhow::bail!("fatal: tag '{}' already exists", tag_name);
    }

    // Get current HEAD commit
    let current_commit = match log::read_head(repo)? {
        Some(hash) => hash,
        None => {
            anyhow::bail!("fatal: not a valid object name: 'HEAD'");
        }
    };

    // Create the tag file
    fs::write(&tag_path, format!("{}\n", current_commit))
        .context(format!("Failed to create tag file: {}", tag_path.display()))?;

    println!("Created tag '{}'", tag_name);
    Ok(())
}

/// Create an annotated tag (tag object with metadata)
fn create_annotated_tag(
    repo: &Repository,
    tag_name: &str,
    message: &str,
) -> Result<()> {
    // For now, we'll create a lightweight tag
    // In a full implementation, we'd create a tag object with:
    // - object (commit hash)
    // - type (commit)
    // - tag (tag name)
    // - tagger (author info)
    // - message
    // Then store it in objects/ and create a ref pointing to it
    
    // For simplicity, create a lightweight tag for now
    // TODO: Implement full annotated tag objects
    create_lightweight_tag(repo, tag_name)?;
    
    if !message.is_empty() {
        // Note: message is stored but not used in lightweight tags
        // For annotated tags, it would be part of the tag object
    }
    
    Ok(())
}

/// Delete a tag
fn delete_tag(repo: &Repository, tag_name: &str) -> Result<()> {
    let tag_path = tag_ref_path(repo, tag_name);

    if !tag_path.exists() {
        anyhow::bail!("error: tag '{}' not found", tag_name);
    }

    // Read the tag commit (for display)
    let tag_commit = fs::read_to_string(&tag_path)
        .context("Failed to read tag file")?
        .trim()
        .to_string();

    // Delete the tag file
    fs::remove_file(&tag_path)
        .context(format!("Failed to delete tag file: {}", tag_path.display()))?;

    println!("Deleted tag '{}' (was {})", tag_name, &tag_commit[..7.min(tag_commit.len())]);
    Ok(())
}

/// Execute the tag command
///
/// # Arguments
///
/// * `tag_name` - Optional tag name to create
/// * `delete` - If true, delete the tag instead of creating
/// * `annotated` - If true, create an annotated tag (not yet fully implemented)
/// * `message` - Optional message for annotated tags
///
/// # Example
///
/// ```no_run
/// use rit::commands::tag::run;
///
/// // List tags
/// run(None, false, false, None).unwrap();
///
/// // Create lightweight tag
/// run(Some("v1.0.0".to_string()), false, false, None).unwrap();
///
/// // Create annotated tag
/// run(Some("v1.0.0".to_string()), false, true, Some("Release".to_string())).unwrap();
///
/// // Delete tag
/// run(Some("v1.0.0".to_string()), true, false, None).unwrap();
/// ```
pub fn run(tag_name: Option<String>, delete: bool, annotated: bool, message: Option<String>) -> Result<()> {
    let repo = Repository::find()?;

    if delete {
        // Delete tag
        let name = tag_name.ok_or_else(|| anyhow::anyhow!("fatal: tag name required for deletion"))?;
        delete_tag(&repo, &name)?;
    } else if let Some(name) = tag_name {
        // Create tag
        if annotated {
            create_annotated_tag(&repo, &name, message.as_deref().unwrap_or(""))?;
        } else {
            create_lightweight_tag(&repo, &name)?;
        }
    } else {
        // List tags
        list_tags(&repo)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::Repository;

    #[test]
    fn test_list_tags_empty() {
        let temp = tempdir().unwrap();
        let repo = Repository::init(temp.path()).unwrap();
        
        // Should not panic
        list_tags(&repo).unwrap();
    }

    #[test]
    fn test_create_lightweight_tag() {
        let temp = tempdir().unwrap();
        let repo = Repository::init(temp.path()).unwrap();
        
        // Create a commit first
        let commit_hash = "a".repeat(40);
        std::fs::write(repo.rit_dir.join("refs/heads/main"), &commit_hash).unwrap();
        std::fs::write(repo.head_path(), "ref: refs/heads/main\n").unwrap();
        
        // Create a tag
        create_lightweight_tag(&repo, "v1.0.0").unwrap();
        
        // Verify tag file exists
        let tag_path = tag_ref_path(&repo, "v1.0.0");
        assert!(tag_path.exists());
        
        // Verify it points to the same commit
        let tag_content = fs::read_to_string(&tag_path).unwrap();
        let tag_commit = tag_content.trim();
        assert_eq!(tag_commit, commit_hash);
    }

    #[test]
    fn test_create_duplicate_tag() {
        let temp = tempdir().unwrap();
        let repo = Repository::init(temp.path()).unwrap();
        
        let commit_hash = "a".repeat(40);
        std::fs::write(repo.rit_dir.join("refs/heads/main"), &commit_hash).unwrap();
        std::fs::write(repo.head_path(), "ref: refs/heads/main\n").unwrap();
        
        create_lightweight_tag(&repo, "v1.0.0").unwrap();
        
        // Try to create again - should fail
        assert!(create_lightweight_tag(&repo, "v1.0.0").is_err());
    }

    #[test]
    fn test_delete_tag() {
        let temp = tempdir().unwrap();
        let repo = Repository::init(temp.path()).unwrap();
        
        let commit_hash = "a".repeat(40);
        std::fs::write(repo.rit_dir.join("refs/heads/main"), &commit_hash).unwrap();
        std::fs::write(repo.head_path(), "ref: refs/heads/main\n").unwrap();
        
        // Create and then delete tag
        create_lightweight_tag(&repo, "v1.0.0").unwrap();
        delete_tag(&repo, "v1.0.0").unwrap();
        
        // Verify tag file is gone
        let tag_path = tag_ref_path(&repo, "v1.0.0");
        assert!(!tag_path.exists());
    }
}

