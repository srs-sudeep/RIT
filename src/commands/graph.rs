//! # Graph Command
//!
//! Visualize the commit DAG (Directed Acyclic Graph) in various formats.
//!
//! ## Usage
//!
//! ```bash
//! # ASCII graph (default)
//! rit graph
//!
//! # Mermaid.js format
//! rit graph --format mermaid
//!
//! # DOT format (Graphviz)
//! rit graph --format dot
//!
//! # Output to file
//! rit graph --output graph.mmd
//! ```

use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use anyhow::{Context, Result};

use crate::Repository;
use crate::objects::commit::Commit;
use crate::commands::{cat_file, log};

/// Represents a commit node in the graph
#[derive(Debug, Clone)]
struct CommitNode {
    #[allow(dead_code)]
    hash: String,
    short_hash: String,
    message: String,
    parents: Vec<String>,
    is_merge: bool,
}

/// Collect all commits from all branches
fn collect_all_commits(repo: &Repository) -> Result<HashMap<String, CommitNode>> {
    let mut commits = HashMap::new();
    let mut to_visit = VecDeque::new();
    let mut visited = HashSet::new();
    
    // Start from all branch heads
    let heads_dir = repo.rit_dir.join("refs").join("heads");
    if heads_dir.exists() {
        for entry in fs::read_dir(&heads_dir)
            .context("Failed to read refs/heads")? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                if path.file_name().and_then(|n| n.to_str()).is_some() {
                    if let Ok(commit_hash) = fs::read_to_string(&path) {
                        let commit_hash = commit_hash.trim().to_string();
                        if !commit_hash.is_empty() && !visited.contains(&commit_hash) {
                            to_visit.push_back(commit_hash);
                        }
                    }
                }
            }
        }
    }
    
    // Also check HEAD
    if let Some(head_hash) = log::read_head(repo)? {
        if !visited.contains(&head_hash) {
            to_visit.push_back(head_hash);
        }
    }
    
    // Traverse all commits
    while let Some(commit_hash) = to_visit.pop_front() {
        if visited.contains(&commit_hash) {
            continue;
        }
        visited.insert(commit_hash.clone());
        
        // Read commit
        let object = match cat_file::read_object(repo, &commit_hash) {
            Ok(obj) => obj,
            Err(_) => continue, // Skip if commit doesn't exist
        };
        
        if object.object_type != "commit" {
            continue;
        }
        
        let commit_content = String::from_utf8_lossy(&object.content);
        let commit = match Commit::parse(&commit_content) {
            Ok(c) => c,
            Err(_) => continue,
        };
        
        let short_hash = commit_hash[..7.min(commit_hash.len())].to_string();
        let message = commit.message.lines().next().unwrap_or("").to_string();
        let is_merge = commit.parents.len() > 1;
        
        // Add to graph
        commits.insert(commit_hash.clone(), CommitNode {
            hash: commit_hash.clone(),
            short_hash,
            message,
            parents: commit.parents.clone(),
            is_merge,
        });
        
        // Add parents to visit queue
        for parent in commit.parents {
            if !visited.contains(&parent) {
                to_visit.push_back(parent);
            }
        }
    }
    
    Ok(commits)
}

/// Generate ASCII graph output
fn generate_ascii_graph(commits: &HashMap<String, CommitNode>, repo: &Repository) -> Result<String> {
    let mut output = Vec::new();
    
    // Get all branch heads
    let mut branch_heads = HashMap::new();
    let heads_dir = repo.rit_dir.join("refs").join("heads");
    if heads_dir.exists() {
        for entry in fs::read_dir(&heads_dir).context("Failed to read refs/heads")? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let (Some(branch_name), Ok(commit_hash)) = (
                    path.file_name().and_then(|n| n.to_str()),
                    fs::read_to_string(&path)
                ) {
                    let commit_hash = commit_hash.trim().to_string();
                    if commits.contains_key(&commit_hash) {
                        branch_heads.insert(commit_hash, branch_name.to_string());
                    }
                }
            }
        }
    }
    
    // Topological sort (simple BFS from heads)
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut levels: HashMap<String, usize> = HashMap::new();
    
    // Initialize with branch heads
    for (hash, _) in &branch_heads {
        queue.push_back(hash.clone());
        levels.insert(hash.clone(), 0);
    }
    
    // Also add HEAD if it's a commit
    if let Some(head_hash) = log::read_head(repo)? {
        if commits.contains_key(&head_hash) && !levels.contains_key(&head_hash) {
            queue.push_back(head_hash.clone());
            levels.insert(head_hash.clone(), 0);
        }
    }
    
    // BFS to assign levels
    while let Some(commit_hash) = queue.pop_front() {
        if visited.contains(&commit_hash) {
            continue;
        }
        visited.insert(commit_hash.clone());
        
        if let Some(node) = commits.get(&commit_hash) {
            let current_level = *levels.get(&commit_hash).unwrap_or(&0);
            
            for parent in &node.parents {
                if !visited.contains(parent) {
                    let parent_level = levels.entry(parent.clone()).or_insert(current_level + 1);
                    *parent_level = (*parent_level).max(current_level + 1);
                    queue.push_back(parent.clone());
                }
            }
        }
    }
    
    // Group commits by level
    let mut commits_by_level: HashMap<usize, Vec<String>> = HashMap::new();
    for (hash, level) in &levels {
        commits_by_level.entry(*level).or_default().push(hash.clone());
    }
    
    // Sort levels
    let mut sorted_levels: Vec<usize> = commits_by_level.keys().cloned().collect();
    sorted_levels.sort();
    
    // Generate ASCII visualization
    for level in sorted_levels {
        let level_commits = commits_by_level.get(&level).unwrap();
        for commit_hash in level_commits {
            if let Some(node) = commits.get(commit_hash) {
                let branch_label = branch_heads.get(commit_hash)
                    .map(|b| format!(" ({})", b))
                    .unwrap_or_default();
                
                output.push(format!("{} {}: {}{}", 
                    "─".repeat(level * 2),
                    &node.short_hash,
                    &node.message,
                    branch_label));
                
                // Show parent connections
                if !node.parents.is_empty() {
                    for (idx, parent) in node.parents.iter().enumerate() {
                        let connector = if idx == 0 { "│" } else { "├" };
                        output.push(format!("{} {} ──→ {}", 
                            " ".repeat(level * 2),
                            connector,
                            &parent[..7.min(parent.len())]));
                    }
                }
            }
        }
    }
    
    Ok(output.join("\n"))
}

/// Generate Mermaid.js graph format
fn generate_mermaid_graph(commits: &HashMap<String, CommitNode>, repo: &Repository) -> Result<String> {
    let mut output = Vec::new();
    output.push("graph TD".to_string());
    
    // Get branch heads for labeling
    let mut branch_heads = HashMap::new();
    let heads_dir = repo.rit_dir.join("refs").join("heads");
    if heads_dir.exists() {
        for entry in fs::read_dir(&heads_dir).context("Failed to read refs/heads")? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let (Some(branch_name), Ok(commit_hash)) = (
                    path.file_name().and_then(|n| n.to_str()),
                    fs::read_to_string(&path)
                ) {
                    let commit_hash = commit_hash.trim().to_string();
                    if commits.contains_key(&commit_hash) {
                        branch_heads.insert(commit_hash, branch_name.to_string());
                    }
                }
            }
        }
    }
    
    // Generate nodes and edges
    for (hash, node) in commits {
        let node_id = format!("C{}", &hash[..7]);
        let label = format!("{}: {}", &node.short_hash, 
            node.message.chars().take(30).collect::<String>());
        let branch_label = branch_heads.get(hash)
            .map(|b| format!("<br/>({})", b))
            .unwrap_or_default();
        
        let shape = if node.is_merge { "{{" } else { "[" };
        let shape_end = if node.is_merge { "}}" } else { "]" };
        
        output.push(format!("    {}{}{}{}{}", 
            node_id, shape, label, branch_label, shape_end));
        
        // Add edges to parents
        for parent in &node.parents {
            if commits.contains_key(parent) {
                let parent_id = format!("C{}", &parent[..7]);
                output.push(format!("    {} --> {}", parent_id, node_id));
            }
        }
    }
    
    Ok(output.join("\n"))
}

/// Generate DOT format (Graphviz)
fn generate_dot_graph(commits: &HashMap<String, CommitNode>, repo: &Repository) -> Result<String> {
    let mut output = Vec::new();
    output.push("digraph GitGraph {".to_string());
    output.push("    rankdir=LR;".to_string());
    output.push("    node [shape=box];".to_string());
    
    // Get branch heads for labeling
    let mut branch_heads = HashMap::new();
    let heads_dir = repo.rit_dir.join("refs").join("heads");
    if heads_dir.exists() {
        for entry in fs::read_dir(&heads_dir).context("Failed to read refs/heads")? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let (Some(branch_name), Ok(commit_hash)) = (
                    path.file_name().and_then(|n| n.to_str()),
                    fs::read_to_string(&path)
                ) {
                    let commit_hash = commit_hash.trim().to_string();
                    if commits.contains_key(&commit_hash) {
                        branch_heads.insert(commit_hash, branch_name.to_string());
                    }
                }
            }
        }
    }
    
    // Generate nodes
    for (hash, node) in commits {
        let node_id = format!("C{}", &hash[..7]);
        let label = format!("{}\\n{}", &node.short_hash,
            node.message.chars().take(40).collect::<String>());
        let branch_label = branch_heads.get(hash)
            .map(|b| format!("\\n({})", b))
            .unwrap_or_default();
        
        let style = if node.is_merge { "rounded" } else { "box" };
        output.push(format!("    {} [label=\"{}{}\", style={}];", 
            node_id, label, branch_label, style));
    }
    
    // Generate edges
    for (hash, node) in commits {
        let node_id = format!("C{}", &hash[..7]);
        for parent in &node.parents {
            if commits.contains_key(parent) {
                let parent_id = format!("C{}", &parent[..7]);
                output.push(format!("    {} -> {};", parent_id, node_id));
            }
        }
    }
    
    output.push("}".to_string());
    Ok(output.join("\n"))
}

/// Execute the graph command
///
/// # Arguments
///
/// * `format` - Output format: "ascii", "mermaid", or "dot"
/// * `output_file` - Optional file to write output to
///
/// # Example
///
/// ```no_run
/// use rit::commands::graph::run;
///
/// // Generate ASCII graph
/// run("ascii", None).unwrap();
///
/// // Generate Mermaid graph to file
/// run("mermaid", Some("graph.mmd".to_string())).unwrap();
/// ```
pub fn run(format: &str, output_file: Option<String>) -> Result<()> {
    let repo = Repository::find()?;
    
    // Collect all commits
    let commits = collect_all_commits(&repo)?;
    
    if commits.is_empty() {
        println!("No commits found in repository");
        return Ok(());
    }
    
    // Generate graph based on format
    let graph_output = match format {
        "mermaid" => generate_mermaid_graph(&commits, &repo)?,
        "dot" => generate_dot_graph(&commits, &repo)?,
        "ascii" | _ => generate_ascii_graph(&commits, &repo)?,
    };
    
    // Output to file or stdout
    if let Some(file_path) = output_file {
        fs::write(&file_path, graph_output)
            .context(format!("Failed to write graph to: {}", file_path))?;
        println!("Graph written to {}", file_path);
    } else {
        println!("{}", graph_output);
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::Repository;

    #[test]
    fn test_collect_all_commits_empty() {
        let temp = tempdir().unwrap();
        let repo = Repository::init(temp.path()).unwrap();
        
        let commits = collect_all_commits(&repo).unwrap();
        assert!(commits.is_empty());
    }

    #[test]
    fn test_collect_all_commits_with_branch() {
        let temp = tempdir().unwrap();
        let repo = Repository::init(temp.path()).unwrap();
        
        // Create a commit hash
        let commit_hash = "a".repeat(40);
        std::fs::write(repo.rit_dir.join("refs/heads/main"), &commit_hash).unwrap();
        
        // Note: This will fail if commit doesn't exist, but that's expected
        // In a real scenario, we'd create actual commit objects
        let _commits = collect_all_commits(&repo).unwrap();
    }
}

