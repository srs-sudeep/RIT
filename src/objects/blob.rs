//! # Blob Object
//!
//! A blob (binary large object) stores file contents.
//! It contains no filename or permissions - just raw bytes.
//!
//! ## Format
//!
//! ```text
//! blob <size>\0<content>
//! ```
//!
//! ## Example
//!
//! For a file containing "Hello, World!":
//! ```text
//! blob 13\0Hello, World!
//! ```

use anyhow::Result;
use crate::Repository;
use crate::commands::hash_object;

/// Represents a blob object
#[derive(Debug, Clone)]
pub struct Blob {
    /// The raw content of the blob
    pub content: Vec<u8>,
}

impl Blob {
    /// Create a new blob from raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use rit::objects::Blob;
    ///
    /// let blob = Blob::new(b"Hello, World!".to_vec());
    /// assert_eq!(blob.content, b"Hello, World!");
    /// ```
    pub fn new(content: Vec<u8>) -> Self {
        Self { content }
    }

    /// Create a blob from a file
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rit::objects::Blob;
    ///
    /// let blob = Blob::from_file("README.md").unwrap();
    /// ```
    pub fn from_file(path: &str) -> Result<Self> {
        let content = std::fs::read(path)?;
        Ok(Self::new(content))
    }

    /// Compute the SHA-1 hash of this blob
    ///
    /// # Example
    ///
    /// ```
    /// use rit::objects::Blob;
    ///
    /// let blob = Blob::new(b"test content\n".to_vec());
    /// let hash = blob.hash();
    /// assert_eq!(hash.len(), 40);
    /// ```
    pub fn hash(&self) -> String {
        hash_object::hash_content("blob", &self.content)
    }

    /// Store this blob in the repository's object database
    ///
    /// # Returns
    ///
    /// The SHA-1 hash of the stored object
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rit::{Repository, objects::Blob};
    ///
    /// let repo = Repository::find().unwrap();
    /// let blob = Blob::new(b"Hello!".to_vec());
    /// let hash = blob.store(&repo).unwrap();
    /// println!("Stored blob: {}", hash);
    /// ```
    pub fn store(&self, repo: &Repository) -> Result<String> {
        hash_object::store_object(repo, "blob", &self.content)
    }

    /// Get the size of the blob content in bytes
    pub fn size(&self) -> usize {
        self.content.len()
    }

    /// Try to interpret the content as UTF-8 text
    ///
    /// # Returns
    ///
    /// `Some(String)` if valid UTF-8, `None` otherwise
    pub fn as_text(&self) -> Option<String> {
        String::from_utf8(self.content.clone()).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blob_hash() {
        let blob = Blob::new(b"test content\n".to_vec());
        // This hash can be verified with: echo "test content" | git hash-object --stdin
        assert_eq!(blob.hash(), "d670460b4b4aece5915caf5c68d12f560a9fe3e4");
    }

    #[test]
    fn test_blob_as_text() {
        let text_blob = Blob::new(b"Hello, World!".to_vec());
        assert_eq!(text_blob.as_text(), Some("Hello, World!".to_string()));

        let binary_blob = Blob::new(vec![0xFF, 0xFE, 0x00]);
        assert!(binary_blob.as_text().is_none());
    }
}

