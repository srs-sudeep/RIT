//! # Git Object Types
//!
//! This module defines the core object types in Git's data model.
//!
//! ## Object Types Overview
//!
//! ### Blob
//! A blob stores file contents. It's just raw bytes with no filename or metadata.
//!
//! ```text
//! blob <size>\0<raw file contents>
//! ```
//!
//! ### Tree
//! A tree represents a directory. It maps names to object hashes.
//!
//! ```text
//! tree <size>\0
//! <mode> <name>\0<20-byte hash>
//! <mode> <name>\0<20-byte hash>
//! ...
//! ```
//!
//! Modes:
//! - `100644` - Regular file
//! - `100755` - Executable file
//! - `040000` - Directory (tree)
//! - `120000` - Symbolic link
//!
//! ### Commit
//! A commit is a snapshot with metadata.
//!
//! ```text
//! commit <size>\0
//! tree <tree-hash>
//! parent <parent-hash>     # (optional, 0+ parents)
//! author <name> <email> <timestamp>
//! committer <name> <email> <timestamp>
//!
//! <commit message>
//! ```

pub mod blob;
pub mod tree;
pub mod commit;

pub use blob::Blob;
pub use tree::{Tree, TreeEntry};
pub use commit::Commit;

