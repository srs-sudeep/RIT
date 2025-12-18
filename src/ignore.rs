//! # Ignore Pattern Matching
//!
//! Parses and matches `.ritignore` patterns, similar to `.gitignore`.
//!
//! Supports:
//! - Glob patterns (`*.log`, `target/`, etc.)
//! - Negation patterns (`!important.log`)
//! - Directory patterns (`dir/` matches directories)

use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};

/// Represents a single ignore pattern
#[derive(Debug, Clone)]
struct IgnorePattern {
    /// The pattern string
    pattern: String,
    /// Whether this is a negation pattern (starts with !)
    negated: bool,
    /// Whether this matches directories only (ends with /)
    directory_only: bool,
    /// Whether this is anchored to the start (doesn't start with *)
    anchored: bool,
}

impl IgnorePattern {
    /// Parse a pattern line
    fn parse(line: &str) -> Option<Self> {
        let trimmed = line.trim();
        
        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with('#') {
            return None;
        }
        
        let (pattern, negated) = if trimmed.starts_with('!') {
            (trimmed[1..].trim(), true)
        } else {
            (trimmed, false)
        };
        
        if pattern.is_empty() {
            return None;
        }
        
        let directory_only = pattern.ends_with('/');
        let pattern_clean = if directory_only {
            &pattern[..pattern.len() - 1]
        } else {
            pattern
        };
        
        // Check if anchored (doesn't start with * or **)
        let anchored = !pattern_clean.starts_with('*') && !pattern_clean.starts_with("**/");
        
        Some(Self {
            pattern: pattern_clean.to_string(),
            negated,
            directory_only,
            anchored,
        })
    }
    
    /// Check if a path matches this pattern
    fn matches(&self, path: &str, is_dir: bool) -> bool {
        // Directory-only patterns only match directories
        if self.directory_only && !is_dir {
            return false;
        }
        
        // Convert pattern to a simple glob match
        // This is a simplified implementation - a full implementation would
        // use proper glob matching with ** support
        
        let path_normalized = path.replace('\\', "/");
        let pattern_normalized = self.pattern.replace('\\', "/");
        
        // Simple glob matching
        if self.matches_glob(&path_normalized, &pattern_normalized) {
            return true;
        }
        
        // If pattern doesn't start with /, try matching against basename
        if !self.anchored && !pattern_normalized.contains('/') {
            if let Some(basename) = Path::new(&path_normalized).file_name() {
                if self.matches_glob(&basename.to_string_lossy(), &pattern_normalized) {
                    return true;
                }
            }
        }
        
        false
    }
    
    /// Simple glob matching (supports * and ?)
    fn matches_glob(&self, text: &str, pattern: &str) -> bool {
        // Handle exact match
        if pattern == text {
            return true;
        }
        
        // Handle * wildcard
        if pattern.contains('*') {
            let parts: Vec<&str> = pattern.split('*').collect();
            if parts.len() == 2 {
                // Pattern like "prefix*suffix"
                let prefix = parts[0];
                let suffix = parts[1];
                if text.starts_with(prefix) && text.ends_with(suffix) {
                    return prefix.len() + suffix.len() <= text.len();
                }
            } else if parts.len() == 1 {
                // Pattern like "prefix*" or "*suffix"
                if pattern.ends_with('*') {
                    return text.starts_with(&pattern[..pattern.len() - 1]);
                } else if pattern.starts_with('*') {
                    return text.ends_with(&pattern[1..]);
                }
            }
        }
        
        // Handle ? wildcard (single character)
        if pattern.contains('?') {
            // Simple implementation: convert ? to . and do basic matching
            let _regex_pattern = pattern.replace('?', ".");
            // For now, just check if lengths match and do prefix/suffix matching
            if pattern.len() == text.len() {
                let mut matches = true;
                for (p, t) in pattern.chars().zip(text.chars()) {
                    if p != '?' && p != t {
                        matches = false;
                        break;
                    }
                }
                return matches;
            }
        }
        
        false
    }
}

/// Represents a collection of ignore patterns
#[derive(Debug, Clone)]
pub struct IgnoreRules {
    patterns: Vec<IgnorePattern>,
}

impl IgnoreRules {
    /// Create empty ignore rules
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
        }
    }
    
    /// Load ignore rules from a file
    pub fn load(ignore_path: &Path) -> Result<Self> {
        if !ignore_path.exists() {
            return Ok(Self::new());
        }
        
        let content = fs::read_to_string(ignore_path)
            .context("Failed to read ignore file")?;
        
        let mut patterns = Vec::new();
        
        for line in content.lines() {
            if let Some(pattern) = IgnorePattern::parse(line) {
                patterns.push(pattern);
            }
        }
        
        Ok(Self { patterns })
    }
    
    /// Check if a path should be ignored
    ///
    /// Returns:
    /// - `true` if the path should be ignored
    /// - `false` if the path should not be ignored
    pub fn is_ignored(&self, path: &str, is_dir: bool) -> bool {
        let mut ignored = false;
        
        // Process patterns in order
        // Later patterns can override earlier ones (negation)
        for pattern in &self.patterns {
            if pattern.matches(path, is_dir) {
                if pattern.negated {
                    // Negation pattern - un-ignore
                    ignored = false;
                } else {
                    // Regular pattern - ignore
                    ignored = true;
                }
            }
        }
        
        ignored
    }
    
    /// Check if a path should be ignored (using PathBuf)
    pub fn is_ignored_path(&self, path: &Path, is_dir: bool) -> bool {
        // Convert to relative path string
        let path_str = path.to_string_lossy().replace('\\', "/");
        self.is_ignored(&path_str, is_dir)
    }
}

impl Default for IgnoreRules {
    fn default() -> Self {
        Self::new()
    }
}

/// Get the path to the .ritignore file
pub fn get_ignore_path(repo_root: &Path) -> PathBuf {
    repo_root.join(".ritignore")
}

/// Load ignore rules for a repository
pub fn load_ignore_rules(repo_root: &Path) -> Result<IgnoreRules> {
    let ignore_path = get_ignore_path(repo_root);
    IgnoreRules::load(&ignore_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_parse_pattern() {
        let pattern = IgnorePattern::parse("*.log").unwrap();
        assert!(!pattern.negated);
        assert!(!pattern.directory_only);
        
        let negated = IgnorePattern::parse("!important.log").unwrap();
        assert!(negated.negated);
        assert_eq!(negated.pattern, "important.log");
        
        let dir_pattern = IgnorePattern::parse("target/").unwrap();
        assert!(dir_pattern.directory_only);
    }

    #[test]
    fn test_matches_glob() {
        let pattern = IgnorePattern::parse("*.log").unwrap();
        assert!(pattern.matches("file.log", false));
        assert!(pattern.matches("test.log", false));
        assert!(!pattern.matches("file.txt", false));
    }

    #[test]
    fn test_ignore_rules() {
        let mut rules = IgnoreRules::new();
        rules.patterns.push(IgnorePattern::parse("*.log").unwrap());
        rules.patterns.push(IgnorePattern::parse("!important.log").unwrap());
        
        assert!(rules.is_ignored("file.log", false));
        assert!(!rules.is_ignored("important.log", false)); // Negated
        assert!(!rules.is_ignored("file.txt", false));
    }

    #[test]
    fn test_load_ignore_file() {
        let temp = tempdir().unwrap();
        let ignore_path = temp.path().join(".ritignore");
        fs::write(&ignore_path, "*.log\ntarget/\n!important.log\n").unwrap();
        
        let rules = IgnoreRules::load(&ignore_path).unwrap();
        assert!(rules.is_ignored("file.log", false));
        assert!(rules.is_ignored("target", true));
        assert!(!rules.is_ignored("important.log", false));
    }
}

