# 🦀 Rit - A Git Implementation in Rust

> **"Write Yourself a Git"** - Learning version control internals by building one from scratch.

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 📋 Table of Contents

- [Quick Start](#-quick-start)
- [Commands Reference](#-commands-reference)
- [Development Roadmap](#-development-roadmap)
- [Project Structure](#-project-structure)
- [Documentation](#-documentation)
- [Contributing](#-contributing)

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

### Build & Run

```bash
# Clone and enter project
cd /Users/srs/Desktop/Dev/RIT

# Build the project
cargo build

# Run rit
cargo run -- --help

# Build release version
cargo build --release

# Install globally (optional)
cargo install --path .
```

### Basic Usage

```bash
# Initialize a new repository
rit init

# Hash a file
rit hash-object README.md

# Store a file in the object database
rit hash-object -w README.md

# Read an object
rit cat-file -p <hash>

# Create a commit
rit commit -m "Initial commit"

# View commit history
rit log
```

---

## 📖 Commands Reference

| Command | Description | Status |
|---------|-------------|--------|
| `rit init` | Initialize a new repository | 🔨 In Progress |
| `rit hash-object [-w] <file>` | Hash file contents, optionally write to object store | ⏳ Pending |
| `rit cat-file -p <hash>` | Pretty-print object contents | ⏳ Pending |
| `rit write-tree` | Create tree object from current directory | ⏳ Pending |
| `rit ls-tree <hash>` | List contents of a tree object | ⏳ Pending |
| `rit commit -m <msg>` | Create a new commit | ⏳ Pending |
| `rit log` | Show commit history | ⏳ Pending |
| `rit add <file>` | Stage files for commit | ⏳ Pending |
| `rit status` | Show working tree status | ⏳ Pending |
| `rit branch [name]` | List or create branches | ⏳ Pending |
| `rit checkout <ref>` | Switch branches or restore files | ⏳ Pending |
| `rit diff` | Show changes between commits | ⏳ Pending |
| `rit graph` | Visualize commit history | ⏳ Pending |

---

## 🗺️ Development Roadmap

### Phase 1: Project Setup ✅
- [x] Initialize Rust project with Cargo
- [x] Set up CLI with clap
- [x] Create project structure
- [ ] Set up Docusaurus documentation

### Phase 2: Object Storage (The Plumbing)
- [ ] Implement blob object format
- [ ] `hash-object` command
- [ ] `cat-file` command
- [ ] Tree object format
- [ ] `write-tree` command
- [ ] `ls-tree` command

### Phase 3: Commits & History
- [ ] Commit object format
- [ ] `commit-tree` low-level command
- [ ] Staging area (index file)
- [ ] `add` command
- [ ] `commit` high-level command
- [ ] `log` command

### Phase 4: Branches & References
- [ ] References system (`refs/heads/`)
- [ ] `branch` command
- [ ] `checkout` command
- [ ] `tag` command

### Phase 5: Advanced Features
- [ ] `status` command
- [ ] `diff` command (Myers algorithm)
- [ ] `.ritignore` support
- [ ] `graph` visualization

---

## 📁 Project Structure

```
RIT/
├── Cargo.toml              # Rust dependencies
├── Cargo.lock              # Locked dependency versions
├── README.md               # This file
├── .gitignore              # Git ignore rules
│
├── src/
│   ├── main.rs             # CLI entry point
│   ├── lib.rs              # Library root
│   ├── commands/           # Command implementations
│   │   ├── mod.rs
│   │   ├── init.rs
│   │   ├── hash_object.rs
│   │   ├── cat_file.rs
│   │   ├── commit.rs
│   │   └── ...
│   └── objects/            # Git object types
│       ├── mod.rs
│       ├── blob.rs
│       ├── tree.rs
│       └── commit.rs
│
├── docs/                   # Markdown documentation
│   ├── intro.md
│   ├── architecture.md
│   └── commands/
│
└── website/                # Docusaurus site (generated)
    ├── docusaurus.config.js
    ├── docs/
    └── src/
```

---

## 📚 Documentation

We use [Docusaurus](https://docusaurus.io/) for documentation. The `docs/` folder contains all markdown files, and Docusaurus reads directly from it (no copying needed).

### Local Development

```bash
# Navigate to project root
cd /Users/srs/Desktop/Dev/RIT

# Setup Docusaurus (one-time)
./scripts/setup-docs.sh

# Start documentation server
cd website
npm start
# Opens at http://localhost:3000
```

### Deploy to Vercel

#### Option 1: Vercel CLI (Recommended)

```bash
# Install Vercel CLI
npm i -g vercel

# Deploy from project root
vercel

# Follow prompts:
# - Set up and deploy? Yes
# - Which scope? (your account)
# - Link to existing project? No
# - Project name? rit-docs (or your choice)
# - Directory? ./website
# - Override settings? No

# For production deployment
vercel --prod
```

#### Option 2: Vercel Dashboard

1. Go to [vercel.com](https://vercel.com) and sign in
2. Click **"Add New Project"**
3. Import your Git repository
4. Configure:
   - **Framework Preset**: Other
   - **Root Directory**: `website`
   - **Build Command**: `npm install && npm run build`
   - **Output Directory**: `build`
5. Click **Deploy**

#### Option 3: GitHub Integration

1. Push your code to GitHub
2. Go to Vercel Dashboard → Add Project
3. Import from GitHub
4. Vercel will auto-detect the `vercel.json` configuration

### Documentation Structure

```
docs/                    # Source markdown files (edit these!)
├── intro.md
├── architecture.md
└── commands/
    ├── init.md
    ├── hash-object.md
    └── cat-file.md

website/                 # Docusaurus site (generated)
├── docusaurus.config.ts # Points to ../docs
└── build/               # Built site (for deployment)
```

**Note**: Edit files in `docs/`, not `website/docs/`. Docusaurus reads directly from the root `docs/` folder.

---

## 🔧 Development Commands

### Daily Development

```bash
# Check code compiles
cargo check

# Run with arguments
cargo run -- init
cargo run -- hash-object -w myfile.txt
cargo run -- cat-file -p abc123

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run -- init

# Format code
cargo fmt

# Lint code
cargo clippy
```

### Git Workflow (for this project)

```bash
# After completing each feature
git add .
git commit -m "feat: implement <feature>"
git push origin main
```

---

## 🧪 Testing Your Implementation

```bash
# Create a test directory
mkdir /tmp/rit-test && cd /tmp/rit-test

# Initialize repo
rit init

# Create some files
echo "Hello World" > hello.txt
echo "Another file" > another.txt

# Hash and store
rit hash-object -w hello.txt

# Verify storage
ls .rit/objects/

# Read it back
rit cat-file -p <hash-from-above>
```

---

## 📝 Commit Message Convention

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add new feature
fix: bug fix
docs: documentation changes
refactor: code refactoring
test: adding tests
chore: maintenance tasks
```

### Planned Commits

| # | Message | Description |
|---|---------|-------------|
| 1 | `chore: initial project setup` | Cargo.toml, CLI skeleton, README |
| 2 | `feat: implement init command` | Create .rit directory structure |
| 3 | `feat: implement hash-object` | Blob hashing and storage |
| 4 | `feat: implement cat-file` | Read objects from store |
| 5 | `feat: implement tree objects` | Directory representation |
| 6 | `feat: implement ls-tree` | List tree contents |
| 7 | `feat: implement commit-tree` | Low-level commit creation |
| 8 | `feat: implement log` | Traverse commit history |
| 9 | `feat: implement staging area` | Index file management |
| 10 | `feat: implement commit` | High-level commit command |
| 11 | `feat: implement branch` | Branch management |
| 12 | `feat: implement checkout` | Switch branches/commits |
| 13 | `feat: implement tag` | Tag management |
| 14 | `feat: implement status` | Working tree status |
| 15 | `feat: implement diff` | Show file differences |
| 16 | `feat: implement ritignore` | Ignore file patterns |
| 17 | `feat: implement graph` | Visualize history |

---

## 🎯 Showcase Ideas

1. **Self-Hosting**: Use `rit` to version control its own source code
2. **Blog Series**: Document your learning journey
3. **Graph Visualization**: Output commit DAG as Mermaid.js
4. **Performance Benchmarks**: Compare with real Git
5. **Interactive Demo**: Create a web playground

---

## 📄 License

MIT License - feel free to use this for learning!

---

## 🙏 Resources

- [Git Internals - Pro Git Book](https://git-scm.com/book/en/v2/Git-Internals-Plumbing-and-Porcelain)
- [Write Yourself a Git](https://wyag.thb.lt/)
- [Git from the Bottom Up](https://jwiegley.github.io/git-from-the-bottom-up/)
