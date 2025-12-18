//! # Add Command
//!
//! Stage files for the next commit by adding them to the index.
//!
//! ## Usage
//!
//! ```bash
//! # Stage a single file
//! rit add file.txt
//!
//! # Stage multiple files
//! rit add file1.txt file2.txt
//!
//! # Stage all files in current directory
//! rit add .
//! ```

use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use walkdir::WalkDir;

use crate::Repository;
use crate::index::{Index, IndexEntry};
use crate::commands::hash_object;
use crate::ignore;

/// Get file metadata
fn get_file_metadata(file_path: &Path) -> Result<(u64, u64)> {
    let metadata = fs::metadata(file_path)
        .context(format!("Failed to get metadata for: {}", file_path.display()))?;
    
    let size = metadata.len();
    let mtime = metadata
        .modified()
        .or_else(|_| metadata.created())
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .unwrap_or(0);

    Ok((size, mtime))
}

/// Add a file to the index
fn add_file_to_index(
    repo: &Repository,
    index: &mut Index,
    file_path: &Path,
    repo_root: &Path,
    ignore_rules: &ignore::IgnoreRules,
) -> Result<()> {
    // Get relative path from repo root
    let relative_path = file_path.strip_prefix(repo_root)
        .context("File is not in repository")?
        .to_string_lossy()
        .to_string();

    // Skip .rit directory
    if relative_path.starts_with(".rit/") {
        return Ok(());
    }
    
    // Skip .ritignore file itself
    if relative_path == ".ritignore" {
        return Ok(());
    }
    
    // Check ignore rules
    if ignore_rules.is_ignored(&relative_path, false) {
        return Ok(());
    }

    // Hash and store the file
    let content = fs::read(file_path)
        .context(format!("Failed to read file: {}", file_path.display()))?;
    
    let blob_hash = hash_object::store_object(repo, "blob", &content)?;

    // Get file metadata
    let (size, mtime) = get_file_metadata(file_path)?;

    // Create index entry
    let entry = IndexEntry {
        path: relative_path,
        hash: blob_hash,
        size,
        mtime,
    };

    // Add to index
    index.add_entry(entry);

    Ok(())
}

/// Add files matching a pattern to the index
fn add_path(
    repo: &Repository,
    index: &mut Index,
    path: &Path,
    repo_root: &Path,
    ignore_rules: &ignore::IgnoreRules,
) -> Result<()> {
    if path.is_file() {
        // Single file
        add_file_to_index(repo, index, path, repo_root, ignore_rules)?;
    } else if path.is_dir() {
        // Directory - walk recursively
        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let entry_path = entry.path();
            
            // Check if directory should be skipped
            if entry_path.is_dir() {
                let relative = entry_path.strip_prefix(repo_root)
                    .ok()
                    .map(|p| p.to_string_lossy().to_string());
                if let Some(rel) = relative {
                    if ignore_rules.is_ignored(&rel, true) {
                        // Skip this directory and its contents
                        continue;
                    }
                }
            }
            
            if entry_path.is_file() {
                add_file_to_index(repo, index, entry_path, repo_root, ignore_rules)?;
            }
        }
    } else {
        anyhow::bail!("Path does not exist: {}", path.display());
    }

    Ok(())
}

/// Execute the add command
///
/// # Arguments
///
/// * `paths` - Paths to files or directories to stage
///
/// # Example
///
/// ```no_run
/// use rit::commands::add::run;
///
/// // Stage a file
/// run(vec!["file.txt".to_string()]).unwrap();
///
/// // Stage current directory
/// run(vec![".".to_string()]).unwrap();
/// ```
pub fn run(paths: Vec<String>) -> Result<()> {
    let repo = Repository::find()?;
    let repo_root = repo.root.clone();
    let index_path = repo.index_path();

    // Load existing index
    let mut index = Index::load(&index_path)?;
    
    // Load ignore rules
    let ignore_rules = ignore::load_ignore_rules(&repo_root)?;

    // Add each path
    for path_str in paths {
        let path = PathBuf::from(&path_str);
        
        // Resolve relative to current directory
        let full_path = if path.is_absolute() {
            path
        } else {
            std::env::current_dir()?
                .join(&path)
                .canonicalize()
                .context(format!("Path does not exist: {}", path_str))?
        };

        add_path(&repo, &mut index, &full_path, &repo_root, &ignore_rules)?;
    }

    // Save updated index
    index.save(&index_path)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::Repository;

    #[test]
    fn test_add_file() {
        let temp = tempdir().unwrap();
        let repo = Repository::init(temp.path()).unwrap();

        // Create a test file
        let test_file = temp.path().join("test.txt");
        fs::write(&test_file, b"test content").unwrap();

        std::env::set_current_dir(temp.path()).unwrap();

        // Add file
        run(vec!["test.txt".to_string()]).unwrap();

        // Verify index was updated
        let index = Index::load(&repo.index_path()).unwrap();
        assert!(index.contains("test.txt"));
        assert_eq!(index.get_entry("test.txt").unwrap().size, 12);

        std::env::set_current_dir("/").unwrap();
    }
}

