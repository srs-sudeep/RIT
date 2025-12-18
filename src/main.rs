 
//! - `cat-file` - Read object contents
//! - `commit` - Create a new commit

use clap::{Parser, Subcommand};
use anyhow::Result;

use rit::commands;

/// Rit - A Git implementation in Rust
///
/// This is an educational implementation of Git's core functionality,
/// built from scratch to understand version control internals.
///
/// Created by Sudeep Ranjan Sahoo
/// GitHub: https://github.com/srs-sudeep
#[derive(Parser)]
#[command(name = "rit")]
#[command(author = "Sudeep Ranjan Sahoo <sudeep.ranjan.sahoo@example.com>")]
#[command(version = "0.1.0")]
#[command(about = "A Git implementation in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new rit repository
    ///
    /// Creates the .rit directory structure with:
    /// - objects/ (object database)
    /// - refs/heads/ (branch references)
    /// - HEAD (current branch pointer)
    Init,

    /// Compute object ID and optionally store it
    ///
    /// Takes a file, computes its SHA-1 hash using Git's blob format,
    /// and optionally stores it in the object database.
    #[command(name = "hash-object")]
    HashObject {
        /// The file to hash
        file: String,

        /// Actually write the object to the database
        #[arg(short, long)]
        write: bool,
    },

    /// Print the contents of an object
    ///
    /// Reads an object from the database, decompresses it,
    /// and prints its contents.
    #[command(name = "cat-file")]
    CatFile {
        /// Pretty-print the object contents
        #[arg(short = 'p')]
        pretty_print: bool,

        /// The object hash to read
        object_hash: String,
    },

    /// Create a tree object from the current directory
    ///
    /// Walks the working directory, hashes all files as blobs,
    /// and creates a tree object representing the directory structure.
    #[command(name = "write-tree")]
    WriteTree,

    /// List the contents of a tree object
    ///
    /// Displays tree entries in a human-readable format,
    /// similar to Git's ls-tree command.
    #[command(name = "ls-tree")]
    LsTree {
        /// The tree hash to list
        tree_hash: String,

        /// Recursively list all subtrees
        #[arg(short = 'r')]
        recursive: bool,

        /// Show only names (no mode, type, or hash)
        #[arg(long = "name-only")]
        name_only: bool,
    },

    /// Create a commit object from a tree
    ///
    /// This is a plumbing command that creates a commit object
    /// from a tree hash, optionally with parent commits.
    #[command(name = "commit-tree")]
    CommitTree {
        /// The tree hash to commit
        #[arg(value_name = "TREE_HASH")]
        tree_hash: String,

        /// Parent commit hash (can be specified multiple times: -p <hash1> -p <hash2>)
        #[arg(short = 'p', long = "parent", num_args = 0..)]
        parents: Vec<String>,

        /// Commit message
        #[arg(short, long)]
        message: String,
    },

    /// Record changes to the repository
    ///
    /// Creates a new commit from the staging area (index).
    Commit {
        /// The commit message
        #[arg(short, long)]
        message: String,

        /// Automatically stage files that have been modified (not yet implemented)
        #[arg(short = 'a')]
        auto_add: bool,
    },

    /// Show commit logs
    ///
    /// Displays the commit history starting from HEAD.
    Log {
        /// Show one commit per line
        #[arg(long = "oneline")]
        oneline: bool,

        /// Draw ASCII graph of commit history
        #[arg(long = "graph")]
        graph: bool,
    },

    /// Add file contents to the staging area
    ///
    /// Stages files for the next commit by adding them to the index.
    Add {
        /// Files or directories to stage
        paths: Vec<String>,
    },

    /// List, create, or delete branches
    ///
    /// Without arguments, lists all branches. With a branch name,
    /// creates a new branch pointing to the current HEAD.
    Branch {
        /// Branch name to create or delete
        branch_name: Option<String>,

        /// Delete the branch
        #[arg(short = 'd')]
        delete: bool,

        /// Force delete (even if not merged)
        #[arg(short = 'D')]
        force: bool,
    },

    /// Switch branches or restore files
    ///
    /// Checkout a branch, commit, or individual files from a commit.
    Checkout {
        /// Branch name, commit hash, or "HEAD"
        reference: String,

        /// File path to checkout (for file checkout)
        file_path: Option<String>,

        /// Force checkout, overwriting local changes
        #[arg(short, long)]
        force: bool,
    },

    /// Create, list, or delete tags
    ///
    /// Tags are references to specific commits, useful for marking releases.
    Tag {
        /// Tag name to create or delete
        tag_name: Option<String>,

        /// Delete the tag
        #[arg(short = 'd')]
        delete: bool,

        /// Create an annotated tag (with message)
        #[arg(short = 'a')]
        annotated: bool,

        /// Tag message (for annotated tags)
        #[arg(short, long)]
        message: Option<String>,
    },

    /// Show the working tree status
    ///
    /// Displays staged, unstaged, and untracked files.
    Status,

    /// Show changes between commits, index, and working directory
    ///
    /// Displays differences using unified diff format.
    Diff {
        /// Show staged changes (index vs HEAD)
        #[arg(long)]
        cached: bool,

        /// First commit to compare
        commit1: Option<String>,

        /// Second commit to compare
        commit2: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            commands::init::run(None)?;
        }

        Commands::HashObject { file, write } => {
            commands::hash_object::run(file, *write)?;
        }

        Commands::CatFile { pretty_print, object_hash } => {
            commands::cat_file::run(object_hash, *pretty_print)?;
        }

        Commands::WriteTree => {
            commands::write_tree::run()?;
        }

        Commands::LsTree { tree_hash, recursive, name_only } => {
            commands::ls_tree::run(tree_hash, *recursive, *name_only)?;
        }

        Commands::CommitTree { tree_hash, parents, message } => {
            commands::commit_tree::run(tree_hash, parents.clone(), message)?;
        }

        Commands::Commit { message, auto_add } => {
            commands::commit::run(message, *auto_add)?;
        }

        Commands::Log { oneline, graph } => {
            commands::log::run(*oneline, *graph)?;
        }

        Commands::Add { paths } => {
            commands::add::run(paths.clone())?;
        }

        Commands::Branch { branch_name, delete, force } => {
            commands::branch::run(branch_name.clone(), *delete, *force)?;
        }

        Commands::Checkout { reference, file_path, force } => {
            commands::checkout::run(reference, file_path.clone(), *force)?;
        }

        Commands::Tag { tag_name, delete, annotated, message } => {
            commands::tag::run(tag_name.clone(), *delete, *annotated, message.clone())?;
        }

        Commands::Status => {
            commands::status::run()?;
        }

        Commands::Diff { cached, commit1, commit2 } => {
            commands::diff::run(*cached, commit1.clone(), commit2.clone())?;
        }
    }

    Ok(())
}
