//! # Tree Object
//!
//! A tree object represents a directory in Git.
//! It maps filenames to their blob or subtree hashes.
//!
//! ## Binary Format
//!
//! ```text
//! tree <size>\0
//! <mode> <name>\0<20-byte-sha1>
//! <mode> <name>\0<20-byte-sha1>
//! ...
//! ```
//!
//! Note: The SHA-1 is stored as raw bytes (20 bytes), not hex (40 chars).
//!
//! ## Modes
//!
//! - `100644` - Regular file
//! - `100755` - Executable file
//! - `040000` - Directory (subtree)
//! - `120000` - Symbolic link
//! - `160000` - Gitlink (submodule)

use anyhow::Result;

/// File mode for a regular file
pub const MODE_FILE: &str = "100644";
/// File mode for an executable file
pub const MODE_EXEC: &str = "100755";
/// File mode for a directory (tree)
pub const MODE_DIR: &str = "40000"; // Note: no leading 0 in stored format
/// File mode for a symbolic link
pub const MODE_SYMLINK: &str = "120000";

/// A single entry in a tree object
#[derive(Debug, Clone, PartialEq)]
pub struct TreeEntry {
    /// File mode (e.g., "100644" for regular file)
    pub mode: String,
    /// Filename (just the name, not full path)
    pub name: String,
    /// SHA-1 hash of the blob or subtree (40 hex chars)
    pub hash: String,
}

impl TreeEntry {
    /// Create a new tree entry
    ///
    /// # Example
    ///
    /// ```
    /// use rit::objects::TreeEntry;
    ///
    /// let entry = TreeEntry::new(
    ///     "100644".to_string(),
    ///     "README.md".to_string(),
    ///     "abc123...".to_string(),
    /// );
    /// ```
    pub fn new(mode: String, name: String, hash: String) -> Self {
        Self { mode, name, hash }
    }

    /// Create a tree entry for a regular file
    pub fn file(name: String, hash: String) -> Self {
        Self::new(MODE_FILE.to_string(), name, hash)
    }

    /// Create a tree entry for a directory (subtree)
    pub fn directory(name: String, hash: String) -> Self {
        Self::new(MODE_DIR.to_string(), name, hash)
    }

    /// Check if this entry is a directory (subtree)
    pub fn is_tree(&self) -> bool {
        self.mode == MODE_DIR || self.mode == "040000"
    }

    /// Check if this entry is a regular file (blob)
    pub fn is_blob(&self) -> bool {
        self.mode == MODE_FILE || self.mode == MODE_EXEC
    }

    /// Serialize this entry to binary format
    ///
    /// Format: `<mode> <name>\0<20-byte-hash>`
    pub fn serialize(&self) -> Result<Vec<u8>> {
        let mut data = Vec::new();

        // Mode and name
        data.extend_from_slice(self.mode.as_bytes());
        data.push(b' ');
        data.extend_from_slice(self.name.as_bytes());
        data.push(0); // null byte

        // 20-byte hash (convert from hex)
        let hash_bytes = hex::decode(&self.hash)?;
        data.extend_from_slice(&hash_bytes);

        Ok(data)
    }
}

/// Represents a tree object (directory)
#[derive(Debug, Clone)]
pub struct Tree {
    /// The entries in this tree
    pub entries: Vec<TreeEntry>,
}

impl Tree {
    /// Create a new empty tree
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    /// Add an entry to the tree
    pub fn add_entry(&mut self, entry: TreeEntry) {
        self.entries.push(entry);
    }

    /// Sort entries by name (required by Git)
    ///
    /// Git sorts tree entries in a specific way:
    /// - Directories are sorted as if they had a trailing '/'
    pub fn sort(&mut self) {
        self.entries.sort_by(|a, b| {
            let a_name = if a.is_tree() {
                format!("{}/", a.name)
            } else {
                a.name.clone()
            };
            let b_name = if b.is_tree() {
                format!("{}/", b.name)
            } else {
                b.name.clone()
            };
            a_name.cmp(&b_name)
        });
    }

    /// Serialize the tree to binary format
    pub fn serialize(&self) -> Result<Vec<u8>> {
        let mut data = Vec::new();
        for entry in &self.entries {
            data.extend(entry.serialize()?);
        }
        Ok(data)
    }

    /// Parse a tree from raw content bytes
    pub fn parse(content: &[u8]) -> Result<Self> {
        let mut entries = Vec::new();
        let mut pos = 0;

        while pos < content.len() {
            // Find space after mode
            let space_pos = content[pos..].iter()
                .position(|&b| b == b' ')
                .ok_or_else(|| anyhow::anyhow!("Invalid tree format"))?;

            let mode = String::from_utf8_lossy(&content[pos..pos + space_pos]).to_string();
            pos += space_pos + 1;

            // Find null after name
            let null_pos = content[pos..].iter()
                .position(|&b| b == 0)
                .ok_or_else(|| anyhow::anyhow!("Invalid tree format"))?;

            let name = String::from_utf8_lossy(&content[pos..pos + null_pos]).to_string();
            pos += null_pos + 1;

            // Read 20-byte hash
            if pos + 20 > content.len() {
                anyhow::bail!("Truncated tree entry");
            }
            let hash = hex::encode(&content[pos..pos + 20]);
            pos += 20;

            entries.push(TreeEntry { mode, name, hash });
        }

        Ok(Self { entries })
    }
}

impl Default for Tree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_entry_types() {
        let file = TreeEntry::file("test.txt".to_string(), "a".repeat(40));
        assert!(file.is_blob());
        assert!(!file.is_tree());

        let dir = TreeEntry::directory("src".to_string(), "b".repeat(40));
        assert!(dir.is_tree());
        assert!(!dir.is_blob());
    }

    #[test]
    fn test_tree_sorting() {
        let mut tree = Tree::new();
        tree.add_entry(TreeEntry::file("z.txt".to_string(), "a".repeat(40)));
        tree.add_entry(TreeEntry::file("a.txt".to_string(), "b".repeat(40)));
        tree.add_entry(TreeEntry::directory("m".to_string(), "c".repeat(40)));

        tree.sort();

        assert_eq!(tree.entries[0].name, "a.txt");
        assert_eq!(tree.entries[1].name, "m");
        assert_eq!(tree.entries[2].name, "z.txt");
    }
}

