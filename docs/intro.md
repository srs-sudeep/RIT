# Introduction to Rit

Welcome to **Rit** - a Git implementation written from scratch in Rust!

## What is Rit?

Rit is an educational project that reimplements Git's core functionality to understand:

1. **How version control actually works** - Not just using Git, but understanding its internals
2. **Rust systems programming** - File I/O, binary data, hashing, compression
3. **Data structures** - Content-addressable storage, DAGs, merkle trees

## Why Build Your Own Git?

> "I hear and I forget. I see and I remember. I do and I understand." - Confucius

Reading about Git internals is one thing. Building it forces you to truly understand:

- Why Git uses SHA-1 hashes
- How commits form a linked list (DAG)
- Why the staging area exists
- How branching is just moving pointers

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                      Working Directory                       │
│                    (your actual files)                       │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼ rit add
┌─────────────────────────────────────────────────────────────┐
│                      Staging Area (Index)                    │
│                   (.rit/index - binary file)                 │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼ rit commit
┌─────────────────────────────────────────────────────────────┐
│                      Object Database                         │
│                      (.rit/objects/)                         │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐                  │
│  │  Blobs   │  │  Trees   │  │ Commits  │                  │
│  │ (files)  │  │  (dirs)  │  │(snapshots│                  │
│  └──────────┘  └──────────┘  └──────────┘                  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      References                              │
│              (.rit/refs/ and .rit/HEAD)                      │
│  ┌──────────────────┐  ┌──────────────────┐                │
│  │  refs/heads/main │  │   refs/tags/v1   │                │
│  │  (commit hash)   │  │   (commit hash)  │                │
│  └──────────────────┘  └──────────────────┘                │
└─────────────────────────────────────────────────────────────┘
```

## Quick Start

```bash
# Build Rit
cargo build --release

# Initialize a repository
./target/release/rit init

# Hash a file
./target/release/rit hash-object -w README.md

# Read it back
./target/release/rit cat-file -p <hash>
```

## Project Structure

```
src/
├── main.rs          # CLI entry point
├── lib.rs           # Library root, Repository struct
├── commands/        # Command implementations
│   ├── init.rs
│   ├── hash_object.rs
│   └── cat_file.rs
└── objects/         # Git object types
    ├── blob.rs
    ├── tree.rs
    └── commit.rs
```

## Learning Resources

- [Git Internals - Pro Git Book](https://git-scm.com/book/en/v2/Git-Internals-Plumbing-and-Porcelain)
- [Write Yourself a Git](https://wyag.thb.lt/)
- [The Rust Programming Language](https://doc.rust-lang.org/book/)

