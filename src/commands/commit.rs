//! # Commit Command
//!
//! Create a commit from the staging area (index).
//!
//! ## Usage
//!
//! ```bash
//! # Create commit from staged files
//! rit commit -m "Commit message"
//! ```

use std::collections::HashMap;
use std::path::Path;
use anyhow::{Context, Result};

use crate::Repository;
use crate::index::Index;
use crate::objects::{Tree, TreeEntry};
use crate::commands::hash_object;
use crate::commands::commit_tree;
use crate::commands::log;

/// Build a tree object from index entries
///
/// This creates a tree structure from the index, organizing files
/// into directories as needed. Uses a simplified approach that builds
/// trees level by level from the bottom up.
fn build_tree_from_index(repo: &Repository, index: &Index) -> Result<String> {
    // Group files by their directory
    let mut dir_files: HashMap<String, Vec<(String, String)>> = HashMap::new();
    
    for entry in index.entries() {
        let path = Path::new(&entry.path);
        let parent = path.parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();
        
        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .context("Invalid file path")?
            .to_string();
        
        dir_files.entry(parent)
            .or_insert_with(Vec::new)
            .push((file_name, entry.hash.clone()));
    }
    
    // Build trees from deepest directories to root
    let mut dir_levels: Vec<_> = dir_files.keys().collect();
    dir_levels.sort_by_key(|d| d.matches('/').count());
    dir_levels.reverse();
    
    let mut built_trees: HashMap<String, String> = HashMap::new();
    
    for dir_path in dir_levels {
        let mut tree = Tree::new();
        let files = dir_files.get(dir_path).unwrap();
        
        // Add files
        for (name, hash) in files {
            tree.add_entry(TreeEntry::file(name.clone(), hash.clone()));
        }
        
        // Add subdirectories (if any were built)
        for (subdir_path, subdir_hash) in &built_trees {
            if let Some(parent) = Path::new(subdir_path).parent() {
                let parent_str = parent.to_string_lossy().to_string();
                if parent_str == *dir_path {
                    let dir_name = Path::new(subdir_path)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or(subdir_path);
                    tree.add_entry(TreeEntry::directory(dir_name.to_string(), subdir_hash.clone()));
                }
            }
        }
        
        tree.sort();
        let tree_content = tree.serialize()?;
        let tree_hash = hash_object::store_object(repo, "tree", &tree_content)?;
        built_trees.insert(dir_path.clone(), tree_hash);
    }
    
    // Return root tree hash
    built_trees.get("").cloned()
        .context("Failed to build root tree")
}

/// Update HEAD or branch ref to point to a commit
fn update_ref(repo: &Repository, commit_hash: &str) -> Result<()> {
    let head_path = repo.head_path();
    let head_content = std::fs::read_to_string(&head_path)
        .context("Failed to read HEAD")?;
    
    let head_content = head_content.trim();

    if let Some(ref_path) = head_content.strip_prefix("ref: ") {
        // Update branch ref
        let ref_file = repo.rit_dir.join(ref_path.trim());
        std::fs::write(&ref_file, format!("{}\n", commit_hash))
            .context("Failed to update branch ref")?;
    } else {
        // Detached HEAD - update HEAD directly
        std::fs::write(&head_path, format!("{}\n", commit_hash))
            .context("Failed to update HEAD")?;
    }

    Ok(())
}

/// Get parent commit from HEAD
fn get_parent_commit(repo: &Repository) -> Result<Option<String>> {
    match log::read_head(repo)? {
        Some(hash) => Ok(Some(hash)),
        None => Ok(None),
    }
}

/// Execute the commit command
///
/// # Arguments
///
/// * `message` - Commit message
/// * `auto_add` - If true, automatically stage modified files (not yet implemented)
///
/// # Example
///
/// ```no_run
/// use rit::commands::commit::run;
///
/// // Create commit from staged files
/// run("Initial commit", false).unwrap();
/// ```
pub fn run(message: &str, _auto_add: bool) -> Result<()> {
    let repo = Repository::find()?;
    let index_path = repo.index_path();

    // Load index
    let index = Index::load(&index_path)?;

    // Check if index is empty
    if index.entries().next().is_none() {
        println!("nothing to commit, working tree clean");
        return Ok(());
    }

    // Build tree from index
    let tree_hash = build_tree_from_index(&repo, &index)?;

    // Get parent commit (if any)
    let parent = get_parent_commit(&repo)?;

    // Create commit
    let parents = parent.map(|p| vec![p]).unwrap_or_default();
    let commit_hash = commit_tree::run(&tree_hash, parents, message)?;

    // Update HEAD or branch ref
    update_ref(&repo, &commit_hash)?;

    // Show commit info
    let short_hash = &commit_hash[..7.min(commit_hash.len())];
    let entry_count = index.entries().count();
    println!("[{}] {}", short_hash, message);
    println!(" {} file(s) changed", entry_count);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::Repository;
    use crate::index::IndexEntry;

    #[test]
    fn test_build_tree_from_index() {
        let temp = tempdir().unwrap();
        let repo = Repository::init(temp.path()).unwrap();

        let mut index = Index::new();
        index.add_entry(IndexEntry {
            path: "file1.txt".to_string(),
            hash: "a".repeat(40),
            size: 10,
            mtime: 1000,
        });
        index.add_entry(IndexEntry {
            path: "file2.txt".to_string(),
            hash: "b".repeat(40),
            size: 20,
            mtime: 2000,
        });

        let tree_hash = build_tree_from_index(&repo, &index).unwrap();
        assert_eq!(tree_hash.len(), 40);
    }
}
