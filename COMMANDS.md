# 📋 Complete Commands Reference

This document contains ALL the commands you'll need throughout development.

---

## 🚀 One-Time Setup

```bash
# Navigate to project
cd /Users/srs/Desktop/Dev/RIT

# Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Verify Rust installation
rustc --version
cargo --version

# Make scripts executable
chmod +x scripts/*.sh
```

---

## 🔨 Daily Development

### Building

```bash
# Quick check (fastest, no binary)
cargo check

# Build debug version
cargo build

# Build release version (optimized)
cargo build --release

# Build and run
cargo run -- init
cargo run -- hash-object -w README.md
cargo run -- cat-file -p <hash>
```

### Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_blob_hash

# Run tests in specific module
cargo test objects::blob
```

### Code Quality

```bash
# Format code
cargo fmt

# Check formatting (CI)
cargo fmt --check

# Lint with Clippy
cargo clippy

# Full check
cargo check && cargo clippy && cargo fmt --check
```

### Documentation

```bash
# Generate Rust docs
cargo doc

# Generate and open in browser
cargo doc --open

# Generate with private items
cargo doc --document-private-items
```

---

## 🧪 Testing Rit

### Manual Testing

```bash
# Create test directory
mkdir /tmp/rit-test && cd /tmp/rit-test

# Initialize
/Users/srs/Desktop/Dev/RIT/target/debug/rit init

# Verify structure
ls -la .rit/
cat .rit/HEAD

# Create test file
echo "Hello, World!" > hello.txt

# Hash it (just compute)
/Users/srs/Desktop/Dev/RIT/target/debug/rit hash-object hello.txt

# Hash and store
/Users/srs/Desktop/Dev/RIT/target/debug/rit hash-object -w hello.txt

# Verify storage
ls .rit/objects/

# Read it back
/Users/srs/Desktop/Dev/RIT/target/debug/rit cat-file -p <hash>
```

### Verify Against Real Git

```bash
# Create test file
echo "test content" > test.txt

# Compare hashes (should match!)
git hash-object test.txt
rit hash-object test.txt
```

---

## 📚 Documentation Site (Docusaurus)

### Setup (One-Time)

```bash
cd /Users/srs/Desktop/Dev/RIT

# Run setup script
./scripts/setup-docs.sh

# OR manually:
npx create-docusaurus@latest website classic --typescript
```

### Development

```bash
cd website

# Start dev server (hot reload)
npm start
# Opens at http://localhost:3000

# Build for production
npm run build

# Preview production build
npm run serve
```

---

## 📝 Git Workflow

### Commit Convention

```bash
# Feature commits
git add .
git commit -m "feat: implement init command"
git commit -m "feat: add hash-object command"
git commit -m "feat: implement cat-file"

# Documentation
git commit -m "docs: add architecture documentation"
git commit -m "docs: update README with examples"

# Fixes
git commit -m "fix: handle missing objects gracefully"

# Refactoring
git commit -m "refactor: extract object module"

# Tests
git commit -m "test: add blob hash tests"
```

### Planned Commits

```bash
# Phase 1: Setup
git commit -m "chore: initial project setup with CLI skeleton"
git commit -m "feat: implement init command"

# Phase 2: Objects
git commit -m "feat: implement hash-object command"
git commit -m "feat: implement cat-file command"
git commit -m "feat: implement tree objects and write-tree"
git commit -m "feat: implement ls-tree command"

# Phase 3: Commits
git commit -m "feat: implement commit-tree command"
git commit -m "feat: implement log command"
git commit -m "feat: implement staging area (index)"
git commit -m "feat: implement add command"
git commit -m "feat: implement commit command"

# Phase 4: Branches
git commit -m "feat: implement branch command"
git commit -m "feat: implement checkout command"
git commit -m "feat: implement tag command"

# Phase 5: Advanced
git commit -m "feat: implement status command"
git commit -m "feat: implement diff command"
git commit -m "feat: implement ritignore support"
git commit -m "feat: implement graph visualization"
```

---

## 🔧 Troubleshooting

### Common Issues

```bash
# "not a rit repository" error
# → Make sure you're in a directory with .rit/ or run init

# Compilation errors after pulling
cargo clean && cargo build

# Missing dependencies
cargo update

# Permission denied on scripts
chmod +x scripts/*.sh
```

### Debug Mode

```bash
# Run with debug logging
RUST_LOG=debug cargo run -- init

# Run with backtrace
RUST_BACKTRACE=1 cargo run -- init
```

---

## 🎯 Quick Reference Card

```bash
# BUILD
cargo build              # Debug build
cargo build --release    # Release build
cargo check              # Check only

# TEST
cargo test               # Run tests
cargo test -- --nocapture

# QUALITY
cargo fmt                # Format
cargo clippy             # Lint

# RUN
cargo run -- init
cargo run -- hash-object -w file.txt
cargo run -- cat-file -p <hash>

# DOCS
cargo doc --open         # Rust docs
cd website && npm start  # Docusaurus
```

---

## 📦 Installation

```bash
# Install globally
cargo install --path .

# Now use from anywhere
rit init
rit hash-object -w myfile.txt

# Uninstall
cargo uninstall rit
```

