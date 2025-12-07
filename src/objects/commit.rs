//! # Commit Object
//!
//! A commit represents a snapshot of the repository at a point in time.
//!
//! ## Format
//!
//! ```text
//! tree <tree-sha1>
//! parent <parent-sha1>     # 0 or more parent lines
//! author <name> <email> <timestamp> <timezone>
//! committer <name> <email> <timestamp> <timezone>
//!
//! <commit message>
//! ```
//!
//! ## Example
//!
//! ```text
//! tree 4b825dc642cb6eb9a060e54bf8d69288fbee4904
//! parent a1b2c3d4e5f6789...
//! author John Doe <john@example.com> 1234567890 +0000
//! committer John Doe <john@example.com> 1234567890 +0000
//!
//! Initial commit
//!
//! This is the body of the commit message.
//! It can span multiple lines.
//! ```

use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::Result;

/// Represents an author or committer
#[derive(Debug, Clone)]
pub struct Author {
    /// Full name
    pub name: String,
    /// Email address
    pub email: String,
    /// Unix timestamp
    pub timestamp: u64,
    /// Timezone offset (e.g., "+0000", "-0500")
    pub timezone: String,
}

impl Author {
    /// Create a new author with current timestamp
    ///
    /// # Example
    ///
    /// ```
    /// use rit::objects::commit::Author;
    ///
    /// let author = Author::new("John Doe", "john@example.com");
    /// ```
    pub fn new(name: &str, email: &str) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        Self {
            name: name.to_string(),
            email: email.to_string(),
            timestamp,
            timezone: "+0000".to_string(), // UTC for simplicity
        }
    }

    /// Create an author with a specific timestamp
    pub fn with_timestamp(name: &str, email: &str, timestamp: u64, timezone: &str) -> Self {
        Self {
            name: name.to_string(),
            email: email.to_string(),
            timestamp,
            timezone: timezone.to_string(),
        }
    }

    /// Serialize to Git format
    ///
    /// Format: `Name <email> timestamp timezone`
    pub fn serialize(&self) -> String {
        format!("{} <{}> {} {}", self.name, self.email, self.timestamp, self.timezone)
    }

    /// Parse from Git format
    pub fn parse(s: &str) -> Result<Self> {
        // Format: "Name <email> timestamp timezone"
        let email_start = s.find('<')
            .ok_or_else(|| anyhow::anyhow!("Invalid author format: no email start"))?;
        let email_end = s.find('>')
            .ok_or_else(|| anyhow::anyhow!("Invalid author format: no email end"))?;

        let name = s[..email_start].trim().to_string();
        let email = s[email_start + 1..email_end].to_string();

        let rest: Vec<&str> = s[email_end + 1..].trim().split(' ').collect();
        if rest.len() < 2 {
            anyhow::bail!("Invalid author format: missing timestamp/timezone");
        }

        let timestamp: u64 = rest[0].parse()?;
        let timezone = rest[1].to_string();

        Ok(Self { name, email, timestamp, timezone })
    }
}

/// Represents a commit object
#[derive(Debug, Clone)]
pub struct Commit {
    /// SHA-1 hash of the tree object
    pub tree: String,
    /// SHA-1 hashes of parent commits (0 for initial, 1 for normal, 2+ for merge)
    pub parents: Vec<String>,
    /// The author (who wrote the changes)
    pub author: Author,
    /// The committer (who created this commit)
    pub committer: Author,
    /// The commit message
    pub message: String,
}

impl Commit {
    /// Create a new commit
    ///
    /// # Example
    ///
    /// ```
    /// use rit::objects::commit::{Commit, Author};
    ///
    /// let author = Author::new("John Doe", "john@example.com");
    /// let commit = Commit::new(
    ///     "tree-hash".to_string(),
    ///     vec![],
    ///     author.clone(),
    ///     author,
    ///     "Initial commit".to_string(),
    /// );
    /// ```
    pub fn new(
        tree: String,
        parents: Vec<String>,
        author: Author,
        committer: Author,
        message: String,
    ) -> Self {
        Self { tree, parents, author, committer, message }
    }

    /// Create a simple commit with same author and committer
    pub fn simple(tree: &str, parent: Option<&str>, author: Author, message: &str) -> Self {
        Self {
            tree: tree.to_string(),
            parents: parent.map(|p| vec![p.to_string()]).unwrap_or_default(),
            author: author.clone(),
            committer: author,
            message: message.to_string(),
        }
    }

    /// Serialize the commit to Git format
    pub fn serialize(&self) -> String {
        let mut lines = Vec::new();

        lines.push(format!("tree {}", self.tree));

        for parent in &self.parents {
            lines.push(format!("parent {}", parent));
        }

        lines.push(format!("author {}", self.author.serialize()));
        lines.push(format!("committer {}", self.committer.serialize()));
        lines.push(String::new()); // Empty line before message
        lines.push(self.message.clone());

        lines.join("\n")
    }

    /// Parse a commit from raw content
    pub fn parse(content: &str) -> Result<Self> {
        let mut tree = String::new();
        let mut parents = Vec::new();
        let mut author = None;
        let mut committer = None;

        let mut lines = content.lines();

        // Parse headers
        for line in lines.by_ref() {
            if line.is_empty() {
                break; // End of headers
            }

            if let Some(hash) = line.strip_prefix("tree ") {
                tree = hash.to_string();
            } else if let Some(hash) = line.strip_prefix("parent ") {
                parents.push(hash.to_string());
            } else if let Some(rest) = line.strip_prefix("author ") {
                author = Some(Author::parse(rest)?);
            } else if let Some(rest) = line.strip_prefix("committer ") {
                committer = Some(Author::parse(rest)?);
            }
        }

        // Rest is the message
        let message: String = lines.collect::<Vec<_>>().join("\n");

        Ok(Self {
            tree,
            parents,
            author: author.ok_or_else(|| anyhow::anyhow!("Missing author"))?,
            committer: committer.ok_or_else(|| anyhow::anyhow!("Missing committer"))?,
            message,
        })
    }

    /// Check if this is the initial commit (no parents)
    pub fn is_initial(&self) -> bool {
        self.parents.is_empty()
    }

    /// Check if this is a merge commit (2+ parents)
    pub fn is_merge(&self) -> bool {
        self.parents.len() >= 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_author_serialize() {
        let author = Author::with_timestamp("John Doe", "john@example.com", 1234567890, "+0000");
        assert_eq!(author.serialize(), "John Doe <john@example.com> 1234567890 +0000");
    }

    #[test]
    fn test_author_parse() {
        let s = "John Doe <john@example.com> 1234567890 +0000";
        let author = Author::parse(s).unwrap();
        assert_eq!(author.name, "John Doe");
        assert_eq!(author.email, "john@example.com");
        assert_eq!(author.timestamp, 1234567890);
        assert_eq!(author.timezone, "+0000");
    }

    #[test]
    fn test_commit_roundtrip() {
        let author = Author::with_timestamp("Test", "test@test.com", 1000, "+0000");
        let commit = Commit::simple("abc123", None, author, "Test message");

        let serialized = commit.serialize();
        let parsed = Commit::parse(&serialized).unwrap();

        assert_eq!(parsed.tree, "abc123");
        assert!(parsed.parents.is_empty());
        assert_eq!(parsed.message, "Test message");
    }
}

