//! # Init Command
//!
//! Initialize a new Rit repository.
//!
//! ## What it does
//!
//! Creates the `.rit` directory structure:
//!
//! ```text
//! .rit/
//! ├── HEAD           # "ref: refs/heads/main\n"
//! ├── objects/       # Object database (empty)
//! └── refs/
//!     ├── heads/     # Branch references
//!     └── tags/      # Tag references
//! ```
//!
//! ## Usage
//!
//! ```bash
//! # Initialize in current directory
//! rit init
//!
//! # Initialize in specific directory
//! rit init /path/to/repo
//! ```

use std::path::Path;
use anyhow::Result;
use crate::Repository;

/// Execute the init command
///
/// # Arguments
///
/// * `path` - Optional path where to initialize. Defaults to current directory.
///
/// # Example
///
/// ```no_run
/// use rit::commands::init::run;
///
/// run(None).expect("Failed to initialize repository");
/// ```
pub fn run(path: Option<&Path>) -> Result<()> {
    let target = path.unwrap_or(Path::new("."));
    Repository::init(target)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_init_command() {
        let temp = tempdir().unwrap();
        run(Some(temp.path())).unwrap();

        assert!(temp.path().join(".rit").exists());
        assert!(temp.path().join(".rit/objects").exists());
        assert!(temp.path().join(".rit/refs/heads").exists());

        let head = std::fs::read_to_string(temp.path().join(".rit/HEAD")).unwrap();
        assert_eq!(head, "ref: refs/heads/main\n");
    }
}

