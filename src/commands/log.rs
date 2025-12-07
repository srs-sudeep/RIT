//! # Log Command
//!
//! Display commit history by traversing the parent chain.
//!
//! ## Usage
//!
//! ```bash
//! # Show commit log
//! rit log
//!
//! # One-line format
//! rit log --oneline
//!
//! # With graph visualization
//! rit log --graph
//! ```

use std::collections::HashSet;
use anyhow::{Context, Result};

use crate::Repository;
use crate::objects::commit::Commit;
use crate::commands::cat_file;

/// Read the current HEAD commit hash
///
/// Reads HEAD file which may point to:
/// - A branch ref (e.g., "ref: refs/heads/main")
/// - A direct commit hash (detached HEAD)
fn read_head(repo: &Repository) -> Result<Option<String>> {
    let head_path = repo.head_path();
    
    if !head_path.exists() {
        return Ok(None);
    }

    let head_content = std::fs::read_to_string(&head_path)
        .context("Failed to read HEAD file")?;
    
    let head_content = head_content.trim();

    // Check if it's a ref (e.g., "ref: refs/heads/main")
    if let Some(ref_path) = head_content.strip_prefix("ref: ") {
        let ref_file = repo.rit_dir.join(ref_path.trim());
        if ref_file.exists() {
            let commit_hash = std::fs::read_to_string(&ref_file)
                .context("Failed to read ref file")?
                .trim()
                .to_string();
            return Ok(Some(commit_hash));
        }
        // Ref file doesn't exist yet (no commits)
        return Ok(None);
    }

    // It's a direct commit hash (detached HEAD)
    if !head_content.is_empty() {
        Ok(Some(head_content.to_string()))
    } else {
        Ok(None)
    }
}

/// Format a commit for display
fn format_commit(commit: &Commit, commit_hash: &str, oneline: bool) -> String {
    if oneline {
        // One-line format: <short-hash> <message-first-line>
        let short_hash = &commit_hash[..7.min(commit_hash.len())];
        let message_line = commit.message.lines().next().unwrap_or("");
        format!("{} {}", short_hash, message_line)
    } else {
        // Full format
        let mut output = Vec::new();
        output.push(format!("commit {}", commit_hash));
        
        if commit.is_merge() {
            output.push(format!("Merge: {} {}", 
                &commit.parents[0][..7.min(commit.parents[0].len())],
                &commit.parents[1][..7.min(commit.parents[1].len())]));
        }
        
        output.push(format!("Author: {} <{}>", commit.author.name, commit.author.email));
        output.push(format!("Date:   {}", format_timestamp(commit.author.timestamp)));
        output.push(String::new());
        
        // Message with indentation
        for line in commit.message.lines() {
            output.push(format!("    {}", line));
        }
        
        output.join("\n")
    }
}

/// Format Unix timestamp to readable date
fn format_timestamp(timestamp: u64) -> String {
    // Simple format: just show the timestamp for now
    // Could use chrono crate for better formatting
    format!("{}", timestamp)
}

/// Traverse commit history starting from a commit hash
fn traverse_commits(
    repo: &Repository,
    start_hash: &str,
    visited: &mut HashSet<String>,
    oneline: bool,
) -> Result<Vec<String>> {
    let mut output = Vec::new();
    let mut current = start_hash.to_string();

    while !current.is_empty() && !visited.contains(&current) {
        visited.insert(current.clone());

        // Read commit object
        let object = cat_file::read_object(repo, &current)
            .context(format!("Failed to read commit: {}", current))?;

        if object.object_type != "commit" {
            break;
        }

        // Parse commit
        let commit_content = String::from_utf8_lossy(&object.content);
        let commit = Commit::parse(&commit_content)?;

        // Format and add to output
        output.push(format_commit(&commit, &current, oneline));

        // Move to parent (for now, just take first parent)
        // TODO: Handle merge commits properly with graph
        if let Some(parent) = commit.parents.first() {
            current = parent.clone();
        } else {
            break; // No more parents
        }
    }

    Ok(output)
}

/// Execute the log command
///
/// # Arguments
///
/// * `oneline` - If true, show one-line format
/// * `graph` - If true, show ASCII graph (not yet implemented)
///
/// # Example
///
/// ```no_run
/// use rit::commands::log::run;
///
/// // Show full log
/// run(false, false).unwrap();
///
/// // One-line format
/// run(true, false).unwrap();
/// ```
pub fn run(oneline: bool, graph: bool) -> Result<()> {
    let repo = Repository::find()?;

    // Read HEAD to get starting commit
    let start_hash = match read_head(&repo)? {
        Some(hash) => hash,
        None => {
            println!("fatal: your current branch 'main' does not have any commits yet");
            return Ok(());
        }
    };

    // Traverse commit history
    let mut visited = HashSet::new();
    let commits = traverse_commits(&repo, &start_hash, &mut visited, oneline)?;

    if graph {
        // Simple graph: just show commits with basic visualization
        for (i, commit_line) in commits.iter().enumerate() {
            if i == 0 {
                println!("* {}", commit_line);
            } else {
                println!("|");
                println!("* {}", commit_line);
            }
        }
    } else {
        // Regular log
        for commit_line in commits {
            println!("{}", commit_line);
            if !oneline {
                println!(); // Empty line between commits
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::Repository;

    #[test]
    fn test_read_head_no_commits() {
        let temp = tempdir().unwrap();
        let repo = Repository::init(temp.path()).unwrap();
        
        let head = read_head(&repo).unwrap();
        assert!(head.is_none());
    }

    #[test]
    fn test_read_head_with_ref() {
        let temp = tempdir().unwrap();
        let repo = Repository::init(temp.path()).unwrap();
        
        // Create a ref pointing to a commit
        let commit_hash = "a".repeat(40);
        std::fs::write(repo.rit_dir.join("refs/heads/main"), &commit_hash).unwrap();
        
        let head = read_head(&repo).unwrap();
        assert_eq!(head, Some(commit_hash));
    }
}

