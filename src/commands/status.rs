//! # Status Command
//!
//! Show the working tree status - what's staged, modified, or untracked.
//!
//! ## Usage
//!
//! ```bash
//! rit status
//! ```

use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use walkdir::WalkDir;

use crate::Repository;
use crate::index::Index;
use crate::commands::{log, hash_object, cat_file};
use crate::ignore;

/// Compare working directory with index and HEAD
pub fn run() -> Result<()> {
    let repo = Repository::find()?;
    
    // Load index
    let index = Index::load(&repo.index_path())?;
    
    // Load ignore rules
    let ignore_rules = ignore::load_ignore_rules(&repo.root)?;
    
    // Get HEAD commit
    let head_commit = log::read_head(&repo)?;
    let head_tree_hash = if let Some(commit_hash) = head_commit {
        // Read commit to get tree
        let commit_obj = cat_file::read_object(&repo, &commit_hash)?;
        if commit_obj.object_type != "commit" {
            anyhow::bail!("HEAD does not point to a commit");
        }
        let commit_content = String::from_utf8_lossy(&commit_obj.content);
        let commit = crate::objects::commit::Commit::parse(&commit_content)?;
        Some(commit.tree)
    } else {
        None
    };
    
    // Collect status information
    let mut staged = Vec::new();
    let mut modified = Vec::new();
    let mut deleted = Vec::new();
    let mut untracked = Vec::new();
    
    // Get all files in working directory
    let working_files = get_working_files(&repo.root, &ignore_rules)?;
    
    // Compare working directory with index
    for file_path in &working_files {
        let relative_path = file_path.strip_prefix(&repo.root)
            .unwrap_or(file_path)
            .to_string_lossy()
            .to_string();
        
        if let Some(index_entry) = index.get_entry(&relative_path) {
            // File is in index - check if it's modified
            let current_hash = hash_file(&repo, file_path)?;
            if current_hash != index_entry.hash {
                modified.push(relative_path);
            }
        } else {
            // File is not in index - untracked
            untracked.push(relative_path);
        }
    }
    
    // Compare index with HEAD
    if let Some(tree_hash) = head_tree_hash {
        let head_files = get_tree_files(&repo, &tree_hash)?;
        
        for (path, hash) in &head_files {
            if let Some(index_entry) = index.get_entry(path) {
                // File is in both HEAD and index
                if index_entry.hash != *hash {
                    staged.push(path.clone());
                }
            } else {
                // File is in HEAD but not in index - deleted
                deleted.push(path.clone());
            }
        }
        
        // Files in index but not in HEAD - newly staged
        for entry in index.entries() {
            if !head_files.contains_key(&entry.path) {
                staged.push(entry.path.clone());
            }
        }
    } else {
        // No HEAD - all index entries are staged
        for entry in index.entries() {
            staged.push(entry.path.clone());
        }
    }
    
    // Files in index but not in working directory - deleted
    for entry in index.entries() {
        let file_path = repo.root.join(&entry.path);
        if !file_path.exists() {
            if !deleted.contains(&entry.path) {
                deleted.push(entry.path.clone());
            }
        }
    }
    
    // Print status
    print_status(&staged, &modified, &deleted, &untracked)?;
    
    Ok(())
}

/// Get all files in the working directory
fn get_working_files(root: &Path, ignore_rules: &crate::ignore::IgnoreRules) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    
    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        
        // Skip .rit directory
        if path.strip_prefix(root)
            .map(|p| p.starts_with(".rit"))
            .unwrap_or(false)
        {
            continue;
        }
        
        // Check ignore rules
        if let Ok(relative) = path.strip_prefix(root) {
            let relative_str = relative.to_string_lossy().replace('\\', "/");
            
            // Skip if directory is ignored
            if path.is_dir() && ignore_rules.is_ignored(&relative_str, true) {
                continue;
            }
            
            // Skip if file is ignored
            if path.is_file() && ignore_rules.is_ignored(&relative_str, false) {
                continue;
            }
        }
        
        if path.is_file() {
            files.push(path.to_path_buf());
        }
    }
    
    Ok(files)
}

/// Get all files in a tree (recursively)
fn get_tree_files(repo: &Repository, tree_hash: &str) -> Result<std::collections::HashMap<String, String>> {
    let mut files = std::collections::HashMap::new();
    get_tree_files_recursive(repo, tree_hash, "", &mut files)?;
    Ok(files)
}

/// Recursively get files from a tree
fn get_tree_files_recursive(
    repo: &Repository,
    tree_hash: &str,
    prefix: &str,
    files: &mut std::collections::HashMap<String, String>,
) -> Result<()> {
    let object = cat_file::read_object(repo, tree_hash)?;
    if object.object_type != "tree" {
        anyhow::bail!("Not a tree object: {}", tree_hash);
    }
    
    let tree = crate::objects::Tree::parse(&object.content)?;
    
    for entry in &tree.entries {
        let full_path = if prefix.is_empty() {
            entry.name.clone()
        } else {
            format!("{}/{}", prefix, entry.name)
        };
        
        if entry.is_tree() {
            // Recursively process subtree
            get_tree_files_recursive(repo, &entry.hash, &full_path, files)?;
        } else {
            // It's a file
            files.insert(full_path, entry.hash.clone());
        }
    }
    
    Ok(())
}

/// Hash a file in the working directory
fn hash_file(_repo: &Repository, file_path: &Path) -> Result<String> {
    let content = fs::read(file_path)
        .context(format!("Failed to read file: {}", file_path.display()))?;
    Ok(hash_object::hash_content("blob", &content))
}

/// Print the status output
fn print_status(
    staged: &[String],
    modified: &[String],
    deleted: &[String],
    untracked: &[String],
) -> Result<()> {
    // Get current branch
    let repo = Repository::find()?;
    let head_path = repo.head_path();
    let current_branch = if head_path.exists() {
        let head_content = fs::read_to_string(&head_path)?;
        if let Some(ref_path) = head_content.trim().strip_prefix("ref: ") {
            if let Some(branch) = ref_path.strip_prefix("refs/heads/") {
                Some(branch.to_string())
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };
    
    if let Some(branch) = current_branch {
        println!("On branch {}", branch);
    } else {
        println!("HEAD detached");
    }
    println!();
    
    if staged.is_empty() && modified.is_empty() && deleted.is_empty() && untracked.is_empty() {
        println!("nothing to commit, working tree clean");
        return Ok(());
    }
    
    // Staged changes
    if !staged.is_empty() {
        println!("Changes to be committed:");
        println!("  (use \"rit reset HEAD <file>...\" to unstage)");
        println!();
        for file in staged {
            println!("        new file:   {}", file);
        }
        println!();
    }
    
    // Modified files
    if !modified.is_empty() {
        println!("Changes not staged for commit:");
        println!("  (use \"rit add <file>...\" to update what will be committed)");
        println!("  (use \"rit checkout -- <file>...\" to discard changes in working directory)");
        println!();
        for file in modified {
            println!("        modified:   {}", file);
        }
        println!();
    }
    
    // Deleted files
    if !deleted.is_empty() {
        if modified.is_empty() {
            println!("Changes not staged for commit:");
            println!("  (use \"rit add <file>...\" to update what will be committed)");
            println!("  (use \"rit checkout -- <file>...\" to discard changes in working directory)");
            println!();
        }
        for file in deleted {
            println!("        deleted:    {}", file);
        }
        println!();
    }
    
    // Untracked files
    if !untracked.is_empty() {
        println!("Untracked files:");
        println!("  (use \"rit add <file>...\" to include in what will be committed)");
        println!();
        for file in untracked {
            println!("        {}", file);
        }
        println!();
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::Repository;

    #[test]
    fn test_get_working_files() {
        let temp = tempdir().unwrap();
        let repo = Repository::init(temp.path()).unwrap();
        
        std::fs::write(temp.path().join("file1.txt"), "content1").unwrap();
        std::fs::write(temp.path().join("file2.txt"), "content2").unwrap();
        
        let ignore_rules = ignore::IgnoreRules::new();
        let files = get_working_files(temp.path(), &ignore_rules).unwrap();
        assert!(files.len() >= 2);
    }
}

