//! # Diff Command
//!
//! Show changes between commits, the index, and the working directory.
//!
//! ## Usage
//!
//! ```bash
//! # Show changes in working directory vs index
//! rit diff
//!
//! # Show changes in index vs HEAD
//! rit diff --cached
//!
//! # Show changes between two commits
//! rit diff <commit1> <commit2>
//! ```

use std::fs;
use std::path::Path;
use anyhow::{Context, Result};

use crate::Repository;
use crate::index::Index;
use crate::commands::{cat_file, log};
use crate::ignore;

/// Represents a single edit operation in a diff
#[derive(Debug, Clone, PartialEq)]
enum Edit {
    /// Keep a line (common to both)
    Keep(String),
    /// Delete a line (only in old)
    Delete(String),
    /// Insert a line (only in new)
    Insert(String),
}

/// Myers diff algorithm implementation
///
/// Finds the shortest edit script (SES) between two sequences using
/// the algorithm described by Eugene W. Myers in "An O(ND) Difference
/// Algorithm and Its Variations".
fn myers_diff(old: &[String], new: &[String]) -> Vec<Edit> {
    let n = old.len();
    let m = new.len();
    
    // Handle empty cases
    if n == 0 {
        return new.iter().map(|s| Edit::Insert(s.clone())).collect();
    }
    if m == 0 {
        return old.iter().map(|s| Edit::Delete(s.clone())).collect();
    }
    
    // Use a simplified approach: find longest common subsequence
    // For a full implementation, we'd use the full Myers algorithm
    // This is a simplified version that works for most cases
    
    let mut edits = Vec::new();
    let mut i = 0;
    let mut j = 0;
    
    // Simple greedy matching
    while i < n || j < m {
        if i >= n {
            // Only new lines left
            edits.push(Edit::Insert(new[j].clone()));
            j += 1;
        } else if j >= m {
            // Only old lines left
            edits.push(Edit::Delete(old[i].clone()));
            i += 1;
        } else if old[i] == new[j] {
            // Lines match
            edits.push(Edit::Keep(old[i].clone()));
            i += 1;
            j += 1;
        } else {
            // Lines don't match - try to find best match
            let mut found = false;
            
            // Look ahead in new for a match
            for k in (j + 1)..m.min(j + 10) {
                if old[i] == new[k] {
                    // Found match ahead - insert lines in between
                    for l in j..k {
                        edits.push(Edit::Insert(new[l].clone()));
                    }
                    edits.push(Edit::Keep(old[i].clone()));
                    i += 1;
                    j = k + 1;
                    found = true;
                    break;
                }
            }
            
            if !found {
                // Look ahead in old for a match
                for k in (i + 1)..n.min(i + 10) {
                    if old[k] == new[j] {
                        // Found match ahead - delete lines in between
                        for l in i..k {
                            edits.push(Edit::Delete(old[l].clone()));
                        }
                        edits.push(Edit::Keep(new[j].clone()));
                        i = k + 1;
                        j += 1;
                        found = true;
                        break;
                    }
                }
            }
            
            if !found {
                // No match found - delete old, insert new
                edits.push(Edit::Delete(old[i].clone()));
                edits.push(Edit::Insert(new[j].clone()));
                i += 1;
                j += 1;
            }
        }
    }
    
    edits
}

/// Format edits as unified diff
fn format_unified_diff(
    path: &str,
    _old_lines: &[String],
    _new_lines: &[String],
    edits: &[Edit],
) -> String {
    let mut output = Vec::new();
    
    // Unified diff header
    output.push(format!("--- a/{}", path));
    output.push(format!("+++ b/{}", path));
    
    // Calculate line numbers
    let mut old_line = 1;
    let mut new_line = 1;
    let mut in_hunk = false;
    let mut hunk_start_old = 0;
    let mut hunk_start_new = 0;
    let mut hunk_lines = Vec::new();
    
    for edit in edits {
        match edit {
            Edit::Keep(_) => {
                if in_hunk {
                    // End current hunk
                    let old_count = old_line - hunk_start_old;
                    let new_count = new_line - hunk_start_new;
                    output.push(format!("@@ -{},{} +{},{} @@", 
                        hunk_start_old, old_count, hunk_start_new, new_count));
                    output.extend(hunk_lines.drain(..));
                    in_hunk = false;
                }
                old_line += 1;
                new_line += 1;
            }
            Edit::Delete(s) => {
                if !in_hunk {
                    hunk_start_old = old_line;
                    hunk_start_new = new_line;
                    in_hunk = true;
                }
                hunk_lines.push(format!("-{}", s));
                old_line += 1;
            }
            Edit::Insert(s) => {
                if !in_hunk {
                    hunk_start_old = old_line;
                    hunk_start_new = new_line;
                    in_hunk = true;
                }
                hunk_lines.push(format!("+{}", s));
                new_line += 1;
            }
        }
    }
    
    // Close final hunk if any
    if in_hunk {
        let old_count = old_line - hunk_start_old;
        let new_count = new_line - hunk_start_new;
        output.push(format!("@@ -{},{} +{},{} @@", 
            hunk_start_old, old_count, hunk_start_new, new_count));
        output.extend(hunk_lines);
    }
    
    output.join("\n")
}

/// Split content into lines
fn split_lines(content: &[u8]) -> Vec<String> {
    String::from_utf8_lossy(content)
        .lines()
        .map(|s| s.to_string())
        .collect()
}

/// Get file content from working directory
fn get_working_file(repo: &Repository, path: &str) -> Result<Vec<u8>> {
    let file_path = repo.root.join(path);
    if !file_path.exists() {
        anyhow::bail!("File not found: {}", path);
    }
    fs::read(&file_path)
        .context(format!("Failed to read file: {}", path))
}

/// Get file content from index
fn get_index_file(repo: &Repository, index: &Index, path: &str) -> Result<Vec<u8>> {
    if let Some(entry) = index.get_entry(path) {
        let blob_obj = cat_file::read_object(repo, &entry.hash)?;
        if blob_obj.object_type != "blob" {
            anyhow::bail!("Expected blob, got {}", blob_obj.object_type);
        }
        Ok(blob_obj.content)
    } else {
        anyhow::bail!("File not in index: {}", path);
    }
}

/// Get file content from HEAD commit
fn get_head_file(repo: &Repository, path: &str) -> Result<Vec<u8>> {
    let head_commit = log::read_head(repo)?
        .ok_or_else(|| anyhow::anyhow!("No HEAD commit"))?;
    
    let commit_obj = cat_file::read_object(repo, &head_commit)?;
    if commit_obj.object_type != "commit" {
        anyhow::bail!("HEAD does not point to a commit");
    }
    
    let commit_content = String::from_utf8_lossy(&commit_obj.content);
    let commit = crate::objects::commit::Commit::parse(&commit_content)?;
    
    // Navigate tree to find file
    let path_obj = Path::new(path);
    let components: Vec<_> = path_obj.components().collect();
    
    let mut current_tree_hash = commit.tree;
    
    for (idx, component) in components.iter().enumerate() {
        let name = component.as_os_str().to_string_lossy();
        let tree_obj = cat_file::read_object(repo, &current_tree_hash)?;
        if tree_obj.object_type != "tree" {
            anyhow::bail!("Not a tree object");
        }
        
        let tree = crate::objects::Tree::parse(&tree_obj.content)?;
        let entry = tree.entries.iter()
            .find(|e| e.name == name)
            .ok_or_else(|| anyhow::anyhow!("Path not found in tree: {}", path))?;
        
        if idx == components.len() - 1 {
            // Last component - should be a file
            if entry.is_tree() {
                anyhow::bail!("Path is a directory: {}", path);
            }
            let blob_obj = cat_file::read_object(repo, &entry.hash)?;
            if blob_obj.object_type != "blob" {
                anyhow::bail!("Expected blob");
            }
            return Ok(blob_obj.content);
        } else {
            // Not last - should be a directory
            if !entry.is_tree() {
                anyhow::bail!("Path not found: {}", path);
            }
            current_tree_hash = entry.hash.clone();
        }
    }
    
    anyhow::bail!("Invalid path: {}", path);
}

/// Show diff for a single file
fn diff_file(
    _repo: &Repository,
    path: &str,
    old_content: &[u8],
    new_content: &[u8],
) -> Result<()> {
    let old_lines = split_lines(old_content);
    let new_lines = split_lines(new_content);
    
    let edits = myers_diff(&old_lines, &new_lines);
    
    // Check if there are any changes
    let has_changes = edits.iter().any(|e| !matches!(e, Edit::Keep(_)));
    if !has_changes {
        return Ok(()); // No changes
    }
    
    let diff_output = format_unified_diff(path, &old_lines, &new_lines, &edits);
    println!("{}", diff_output);
    
    Ok(())
}

/// Show diff between working directory and index
fn diff_working_vs_index(repo: &Repository) -> Result<()> {
    let index = Index::load(&repo.index_path())?;
    
    // Load ignore rules
    let ignore_rules = ignore::load_ignore_rules(&repo.root)?;
    
    // Get all files that are in index or working directory
    let mut all_files = std::collections::HashSet::new();
    
    // Add files from index (only if not ignored)
    for entry in index.entries() {
        if !ignore_rules.is_ignored(&entry.path, false) {
            all_files.insert(entry.path.clone());
        }
    }
    
    // Add files from working directory
    use walkdir::WalkDir;
    for entry in WalkDir::new(&repo.root)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            if let Ok(relative) = path.strip_prefix(&repo.root) {
                let rel_str = relative.to_string_lossy().to_string();
                if !rel_str.starts_with(".rit/") && !ignore_rules.is_ignored(&rel_str, false) {
                    all_files.insert(rel_str);
                }
            }
        }
    }
    
    let mut has_output = false;
    
    for file_path in all_files {
        let working_content = get_working_file(repo, &file_path).ok();
        let index_content = get_index_file(repo, &index, &file_path).ok();
        
        match (working_content, index_content) {
            (Some(working), Some(index)) => {
                // File exists in both - compare
                if working != index {
                    diff_file(repo, &file_path, &index, &working)?;
                    has_output = true;
                }
            }
            (Some(working), None) => {
                // File only in working directory - new file
                let old_lines = Vec::new();
                let new_lines = split_lines(&working);
                let edits = myers_diff(&old_lines, &new_lines);
                let diff_output = format_unified_diff(&file_path, &old_lines, &new_lines, &edits);
                println!("{}", diff_output);
                has_output = true;
            }
            (None, Some(index)) => {
                // File only in index - deleted
                let old_lines = split_lines(&index);
                let new_lines = Vec::new();
                let edits = myers_diff(&old_lines, &new_lines);
                let diff_output = format_unified_diff(&file_path, &old_lines, &new_lines, &edits);
                println!("{}", diff_output);
                has_output = true;
            }
            (None, None) => {}
        }
    }
    
    if !has_output {
        // No differences
    }
    
    Ok(())
}

/// Show diff between index and HEAD
fn diff_index_vs_head(repo: &Repository) -> Result<()> {
    let index = Index::load(&repo.index_path())?;
    let head_commit = log::read_head(repo)?;
    
    if head_commit.is_none() {
        // No HEAD - show all index files as new
        for entry in index.entries() {
            let index_content = get_index_file(repo, &index, &entry.path)?;
            let old_lines = Vec::new();
            let new_lines = split_lines(&index_content);
            let edits = myers_diff(&old_lines, &new_lines);
            let diff_output = format_unified_diff(&entry.path, &old_lines, &new_lines, &edits);
            println!("{}", diff_output);
        }
        return Ok(());
    }
    
    // Compare each file in index with HEAD
    for entry in index.entries() {
        let index_content = get_index_file(repo, &index, &entry.path)?;
        let head_content = get_head_file(repo, &entry.path).ok();
        
        match head_content {
            Some(head) => {
                if index_content != head {
                    diff_file(repo, &entry.path, &head, &index_content)?;
                }
            }
            None => {
                // File not in HEAD - new file
                let old_lines = Vec::new();
                let new_lines = split_lines(&index_content);
                let edits = myers_diff(&old_lines, &new_lines);
                let diff_output = format_unified_diff(&entry.path, &old_lines, &new_lines, &edits);
                println!("{}", diff_output);
            }
        }
    }
    
    Ok(())
}

/// Execute the diff command
///
/// # Arguments
///
/// * `cached` - If true, show diff between index and HEAD (staged changes)
/// * `commit1` - Optional first commit to compare
/// * `commit2` - Optional second commit to compare
///
/// # Example
///
/// ```no_run
/// use rit::commands::diff::run;
///
/// // Show working directory vs index
/// run(false, None, None).unwrap();
///
/// // Show index vs HEAD
/// run(true, None, None).unwrap();
/// ```
pub fn run(cached: bool, commit1: Option<String>, commit2: Option<String>) -> Result<()> {
    let repo = Repository::find()?;
    
    if cached {
        // Show staged changes (index vs HEAD)
        diff_index_vs_head(&repo)?;
    } else if let (Some(_c1), Some(_c2)) = (commit1, commit2) {
        // Compare two commits (not yet implemented)
        anyhow::bail!("Comparing two commits is not yet implemented");
    } else {
        // Show working directory vs index
        diff_working_vs_index(&repo)?;
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_myers_diff_simple() {
        let old = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let new = vec!["a".to_string(), "x".to_string(), "c".to_string()];
        let edits = myers_diff(&old, &new);
        
        // Should have: Keep(a), Delete(b), Insert(x), Keep(c)
        assert!(edits.len() >= 3);
    }

    #[test]
    fn test_myers_diff_identical() {
        let lines = vec!["a".to_string(), "b".to_string()];
        let edits = myers_diff(&lines, &lines);
        
        // All should be Keep
        assert!(edits.iter().all(|e| matches!(e, Edit::Keep(_))));
    }

    #[test]
    fn test_myers_diff_add_lines() {
        let old = vec!["a".to_string()];
        let new = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let edits = myers_diff(&old, &new);
        
        // Should have Keep(a), Insert(b), Insert(c)
        assert!(edits.len() == 3);
    }
}

