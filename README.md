# 🦀 Rit - A Git Implementation in Rust

<div align="center">

```
██████╗ ██╗████████╗
██╔══██╗██║╚══██╔══╝
██████╔╝██║   ██║   
██╔══██╗██║   ██║   
██║  ██║██║   ██║   
╚═╝  ╚═╝╚═╝   ╚═╝   
                    
  A Git Implementation
      in Rust
```

> **"Write Yourself a Git"** - Learning version control internals by building one from scratch.

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Documentation](https://img.shields.io/badge/docs-docusaurus-blue)](https://srs-sudeep.github.io/rit)

**Created by [Sudeep Ranjan Sahoo](https://github.com/srs-sudeep)**

</div>

---

## 📖 About

Rit is an educational implementation of Git's core functionality, built from scratch in Rust. This project demonstrates how version control systems work internally by reimplementing Git's fundamental operations.

### Why Build Your Own Git?

Building Git from scratch teaches you:
- **Content-addressable storage** - How Git stores data efficiently
- **DAG structures** - Understanding commit graphs and branching
- **Binary formats** - Working with compressed data and hashing
- **Systems programming** - File I/O, process management, and more

---

## 🚀 Quick Start

### Prerequisites

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version
```

### Installation

```bash
# Clone the repository
git clone https://github.com/srs-sudeep/rit.git
cd rit

# Build the project
cargo build --release

# Install globally (optional)
cargo install --path .
```

### Basic Usage

```bash
# Initialize a new repository
rit init

# Stage files
rit add file1.txt file2.txt

# Create a commit
rit commit -m "Initial commit"

# View commit history
rit log --oneline

# List tree contents
rit ls-tree <tree-hash>
```

---

## 📚 Documentation

### Online Documentation

📖 **[Full Documentation](https://srs-sudeep.github.io/rit)** - Complete guide with examples, architecture details, and command reference.

### Local Documentation

```bash
# Start local documentation server
cd website
npm install
npm start
# Opens at http://localhost:3000
```

### Documentation Structure

- **[Introduction](website/docs/intro.md)** - Getting started with Rit
- **[Architecture](website/docs/architecture.md)** - How Git/Rit works internally
- **[Commands](website/docs/commands/)** - Complete command reference
  - [init](website/docs/commands/init.md)
  - [hash-object](website/docs/commands/hash-object.md)
  - [cat-file](website/docs/commands/cat-file.md)
  - [write-tree](website/docs/commands/write-tree.md)
  - [ls-tree](website/docs/commands/ls-tree.md)
  - [commit-tree](website/docs/commands/commit-tree.md)
  - [log](website/docs/commands/log.md)
  - [add](website/docs/commands/add.md)
  - [commit](website/docs/commands/commit.md)
  - [branch](website/docs/commands/branch.md)
  - [checkout](website/docs/commands/checkout.md)

---

## 📖 Commands Reference

### Implemented Commands

| Command | Description | Status |
|---------|-------------|--------|
| `rit init` | Initialize a new repository | ✅ |
| `rit hash-object [-w] <file>` | Hash file contents, optionally write to object store | ✅ |
| `rit cat-file -p <hash>` | Pretty-print object contents | ✅ |
| `rit write-tree` | Create tree object from current directory | ✅ |
| `rit ls-tree <hash>` | List contents of a tree object | ✅ |
| `rit commit-tree <tree> -m <msg>` | Create commit object from tree | ✅ |
| `rit log [--oneline] [--graph]` | Show commit history | ✅ |
| `rit add <file>...` | Stage files for commit | ✅ |
| `rit commit -m <msg>` | Create a new commit | ✅ |
| `rit branch [name]` | List or create branches | ✅ |
| `rit checkout <ref>` | Switch branches or restore files | ✅ |

### Planned Commands

| Command | Description | Status |
|---------|-------------|--------|
| `rit status` | Show working tree status | 🔨 In Progress |
| `rit diff` | Show changes between commits | ⏳ Planned |
| `rit graph` | Visualize commit history | ⏳ Planned |

---

## 🏗️ Architecture

### Core Concepts

Rit implements Git's core data model:

1. **Objects** - Content-addressable storage
   - **Blobs**: Raw file contents
   - **Trees**: Directory listings
   - **Commits**: Snapshots with metadata

2. **Index** - Staging area (simplified JSON format)

3. **References** - Branch and tag pointers

### Repository Structure

```
.rit/
├── HEAD            # Points to current branch
├── objects/        # Object database
│   ├── ab/         # First 2 chars of hash
│   │   └── cdef... # Remaining hash (zlib compressed)
│   └── ...
├── refs/
│   ├── heads/      # Branch references
│   └── tags/       # Tag references
└── index           # Staging area (JSON)
```

### How It Works

```
Working Directory → [rit add] → Index → [rit commit] → Objects → [rit log] → History
```

---

## 📁 Project Structure

```
rit/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── lib.rs           # Library root, Repository struct
│   ├── index.rs         # Staging area implementation
│   ├── commands/        # Command implementations
│   │   ├── init.rs
│   │   ├── hash_object.rs
│   │   ├── cat_file.rs
│   │   ├── write_tree.rs
│   │   ├── ls_tree.rs
│   │   ├── commit_tree.rs
│   │   ├── log.rs
│   │   ├── add.rs
│   │   └── commit.rs
│   └── objects/         # Git object types
│       ├── blob.rs
│       ├── tree.rs
│       └── commit.rs
│
├── website/             # Docusaurus documentation
│   ├── docs/            # Documentation markdown files
│   ├── static/img/      # Logo and images
│   └── docusaurus.config.ts
│
├── Cargo.toml           # Rust dependencies
├── README.md            # This file
└── TODO.md              # Development tracking (git-ignored)
```

---

## 🔧 Development

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Check without building
cargo check
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Full check
cargo check && cargo clippy && cargo fmt --check
```

### Documentation

```bash
# Generate Rust docs
cargo doc --open

# Build Docusaurus docs
cd website && npm run build
```

---

## 🧪 Testing Your Implementation

```bash
# Create a test repository
mkdir /tmp/rit-test && cd /tmp/rit-test
rit init

# Create and stage files
echo "Hello, World!" > hello.txt
rit add hello.txt

# Create a commit
rit commit -m "Initial commit"

# View history
rit log

# Inspect objects
rit ls-tree <tree-hash>
rit cat-file -p <commit-hash>
```

---

## 🤝 Contributing

Contributions are welcome! This is an educational project, so feel free to:

- Report bugs
- Suggest features
- Submit pull requests
- Improve documentation

---

## 📄 License

MIT License - feel free to use this for learning!

Copyright (c) 2025 [Sudeep Ranjan Sahoo](https://github.com/srs-sudeep)

---

## 🙏 Acknowledgments

This project is inspired by:

- [Git Internals - Pro Git Book](https://git-scm.com/book/en/v2/Git-Internals-Plumbing-and-Porcelain)
- [Write Yourself a Git](https://wyag.thb.lt/)
- [Git from the Bottom Up](https://jwiegley.github.io/git-from-the-bottom-up/)
- [Building Git](https://shop.jcoglan.com/building-git/) by James Coglan

---

## 🔗 Links

- 📖 [Documentation](https://srs-sudeep.github.io/rit)
- 💻 [GitHub Repository](https://github.com/srs-sudeep/rit)
- 💼 [LinkedIn](https://www.linkedin.com/in/sudeep-ranjan-sahoo-b82355232/)
- 🐦 [Twitter/X](https://x.com/SUDEEPRANJANSA1)

---

<div align="center">

**Built with Rust and ❤️ by [Sudeep Ranjan Sahoo](https://github.com/srs-sudeep)**

[⭐ Star on GitHub](https://github.com/srs-sudeep/rit) | [📖 Read the Docs](https://srs-sudeep.github.io/rit) | [🐛 Report Bug](https://github.com/srs-sudeep/rit/issues)

</div>
