//! # Index (Staging Area)
//!
//! The index tracks files that are staged for the next commit.
//! We use a simplified JSON format for V1 (Git uses a complex binary format).

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Represents a single entry in the index
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IndexEntry {
    /// File path (relative to repository root)
    pub path: String,
    /// SHA-1 hash of the blob
    pub hash: String,
    /// File size in bytes
    pub size: u64,
    /// Modification time (Unix timestamp)
    pub mtime: u64,
}

/// Represents the index (staging area)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index {
    /// Map of file paths to index entries
    pub entries: HashMap<String, IndexEntry>,
}

impl Index {
    /// Create a new empty index
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Load index from file
    pub fn load(index_path: &Path) -> Result<Self> {
        if !index_path.exists() {
            return Ok(Self::new());
        }

        let content = fs::read_to_string(index_path)
            .context("Failed to read index file")?;

        if content.trim().is_empty() {
            return Ok(Self::new());
        }

        let index: Index = serde_json::from_str(&content)
            .context("Failed to parse index file")?;

        Ok(index)
    }

    /// Save index to file
    pub fn save(&self, index_path: &Path) -> Result<()> {
        let content = serde_json::to_string_pretty(self)
            .context("Failed to serialize index")?;
        
        fs::write(index_path, content)
            .context("Failed to write index file")?;

        Ok(())
    }

    /// Add or update an entry in the index
    pub fn add_entry(&mut self, entry: IndexEntry) {
        self.entries.insert(entry.path.clone(), entry);
    }

    /// Remove an entry from the index
    pub fn remove_entry(&mut self, path: &str) {
        self.entries.remove(path);
    }

    /// Get an entry by path
    pub fn get_entry(&self, path: &str) -> Option<&IndexEntry> {
        self.entries.get(path)
    }

    /// Check if a path is in the index
    pub fn contains(&self, path: &str) -> bool {
        self.entries.contains_key(path)
    }

    /// Get all entries
    pub fn entries(&self) -> impl Iterator<Item = &IndexEntry> {
        self.entries.values()
    }
}

impl Default for Index {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_index_new() {
        let index = Index::new();
        assert!(index.entries.is_empty());
    }

    #[test]
    fn test_index_add_entry() {
        let mut index = Index::new();
        let entry = IndexEntry {
            path: "test.txt".to_string(),
            hash: "abc123".to_string(),
            size: 100,
            mtime: 1234567890,
        };
        index.add_entry(entry.clone());
        assert_eq!(index.get_entry("test.txt"), Some(&entry));
    }

    #[test]
    fn test_index_save_and_load() {
        let temp = tempdir().unwrap();
        let index_path = temp.path().join("index");

        let mut index = Index::new();
        index.add_entry(IndexEntry {
            path: "test.txt".to_string(),
            hash: "abc123".to_string(),
            size: 100,
            mtime: 1234567890,
        });

        index.save(&index_path).unwrap();
        let loaded = Index::load(&index_path).unwrap();

        assert_eq!(loaded.entries.len(), 1);
        assert_eq!(loaded.get_entry("test.txt").unwrap().hash, "abc123");
    }
}

