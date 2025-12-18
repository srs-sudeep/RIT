//! # Command Implementations
//!
//! This module contains all the CLI command implementations for Rit.
//!
//! ## Command Categories
//!
//! ### Plumbing Commands (Low-level)
//! - `hash-object`: Compute object hash, optionally store
//! - `cat-file`: Read and display object contents
//! - `write-tree`: Create tree from working directory
//! - `commit-tree`: Create commit object
//!
//! ### Porcelain Commands (User-facing)
//! - `init`: Initialize repository
//! - `add`: Stage files
//! - `commit`: Create commit with staged changes
//! - `log`: Display commit history
//! - `status`: Show working tree status
//! - `branch`: Manage branches
//! - `checkout`: Switch branches

pub mod init;
pub mod hash_object;
pub mod cat_file;
pub mod write_tree;
pub mod ls_tree;
pub mod commit_tree;
pub mod log;
pub mod add;
pub mod commit;
pub mod branch;

