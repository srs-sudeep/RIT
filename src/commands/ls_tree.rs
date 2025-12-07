//! # Ls-Tree Command
//!
//! List the contents of a tree object in a human-readable format.
//!
//! ## Usage
//!
//! ```bash
//! # List tree contents
//! rit ls-tree <tree-hash>
//!
//! # Recursive listing
//! rit ls-tree -r <tree-hash>
//!
//! # Show only names
//! rit ls-tree --name-only <tree-hash>
//! ```

use anyhow::{Context, Result};

use crate::Repository;
use crate::objects::Tree;
use crate::commands::cat_file;

/// Format a tree entry for display
///
/// Git's ls-tree format:
/// `<mode> <type> <hash>    <name>`
///
/// Example:
/// `100644 blob abc123...    README.md`
fn format_entry(entry: &crate::objects::TreeEntry, name_only: bool) -> String {
    if name_only {
        return entry.name.clone();
    }

    // Determine object type from mode
    let object_type = if entry.is_tree() {
        "tree"
    } else {
        "blob"
    };

    // Format: mode type hash    name
    format!("{} {} {}    {}", entry.mode, object_type, entry.hash, entry.name)
}

/// List tree contents recursively
///
/// # Arguments
///
/// * `repo` - The repository
/// * `tree_hash` - SHA-1 hash of the tree to list
/// * `recursive` - If true, recursively list subtrees
/// * `name_only` - If true, show only names
/// * `prefix` - Path prefix for recursive listing
fn list_tree_recursive(
    repo: &Repository,
    tree_hash: &str,
    recursive: bool,
    name_only: bool,
    prefix: &str,
) -> Result<Vec<String>> {
    // Read the tree object
    let object = cat_file::read_object(repo, tree_hash)
        .context(format!("Failed to read tree object: {}", tree_hash))?;

    if object.object_type != "tree" {
        anyhow::bail!("Not a tree object: {}", tree_hash);
    }

    // Parse the tree
    let tree = Tree::parse(&object.content)?;

    let mut output = Vec::new();

    for entry in &tree.entries {
        let full_path = if prefix.is_empty() {
            entry.name.clone()
        } else {
            format!("{}/{}", prefix, entry.name)
        };

        if entry.is_tree() {
            // It's a subtree
            if recursive {
                // Recursively list the subtree
                let subtree_output = list_tree_recursive(
                    repo,
                    &entry.hash,
                    recursive,
                    name_only,
                    &full_path,
                )?;
                output.extend(subtree_output);
            } else {
                // Just show this entry (non-recursive)
                if name_only {
                    output.push(full_path);
                } else {
                    output.push(format_entry(entry, name_only));
                }
            }
        } else {
            // It's a blob (file)
            // In recursive mode, show full path; otherwise just the name
            let display_name = if recursive {
                full_path.clone()
            } else {
                entry.name.clone()
            };

            if name_only {
                output.push(display_name);
            } else {
                // Format entry with display name
                let object_type = "blob";
                output.push(format!("{} {} {}    {}", entry.mode, object_type, entry.hash, display_name));
            }
        }
    }

    Ok(output)
}

/// Execute the ls-tree command
///
/// # Arguments
///
/// * `tree_hash` - The SHA-1 hash of the tree object to list
/// * `recursive` - If true, recursively list all subtrees
/// * `name_only` - If true, show only file/directory names
///
/// # Example
///
/// ```no_run
/// use rit::commands::ls_tree::run;
///
/// // List tree contents
/// run("abc123...", false, false).unwrap();
///
/// // Recursive listing
/// run("abc123...", true, false).unwrap();
/// ```
pub fn run(tree_hash: &str, recursive: bool, name_only: bool) -> Result<()> {
    let repo = Repository::find()?;

    let output = list_tree_recursive(&repo, tree_hash, recursive, name_only, "")?;

    for line in output {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::Repository;
    use crate::objects::{Tree, TreeEntry};

    #[test]
    fn test_format_entry() {
        let entry = TreeEntry::file("test.txt".to_string(), "a".repeat(40));
        let formatted = format_entry(&entry, false);
        assert!(formatted.contains("100644"));
        assert!(formatted.contains("blob"));
        assert!(formatted.contains("test.txt"));

        let dir_entry = TreeEntry::directory("src".to_string(), "b".repeat(40));
        let formatted = format_entry(&dir_entry, false);
        assert!(formatted.contains("tree"));
        assert!(formatted.contains("src"));
    }

    #[test]
    fn test_format_entry_name_only() {
        let entry = TreeEntry::file("test.txt".to_string(), "a".repeat(40));
        let formatted = format_entry(&entry, true);
        assert_eq!(formatted, "test.txt");
    }

    #[test]
    fn test_list_tree_simple() {
        let temp = tempdir().unwrap();
        let repo = Repository::init(temp.path()).unwrap();

        // Create a simple tree
        let mut tree = Tree::new();
        tree.add_entry(TreeEntry::file("file1.txt".to_string(), "a".repeat(40)));
        tree.add_entry(TreeEntry::file("file2.txt".to_string(), "b".repeat(40)));

        // Store the tree
        let tree_content = tree.serialize().unwrap();
        let tree_hash = crate::commands::hash_object::store_object(&repo, "tree", &tree_content).unwrap();

        // List it
        let output = list_tree_recursive(&repo, &tree_hash, false, false, "").unwrap();
        assert_eq!(output.len(), 2);
        assert!(output[0].contains("file1.txt"));
        assert!(output[1].contains("file2.txt"));
    }
}

