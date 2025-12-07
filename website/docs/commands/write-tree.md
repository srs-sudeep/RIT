# rit write-tree

Create a tree object from the current working directory.

## Synopsis

```bash
rit write-tree
```

## Description

This plumbing command creates a tree object representing the current directory structure. It:

1. Walks the working directory (excluding `.rit/`)
2. Hashes each file as a blob and stores it
3. Recursively creates trees for subdirectories
4. Builds a tree object with all entries
5. Stores the tree and returns its hash

## How It Works

### Tree Structure

A tree object maps filenames to their object hashes:

```
tree <size>\0
<mode> <name>\0<20-byte-hash>
<mode> <name>\0<20-byte-hash>
...
```

### File Modes

- `100644` - Regular file
- `100755` - Executable file
- `040000` - Directory (subtree)

### Entry Sorting

Git requires tree entries to be sorted:
- Directories are sorted as if they had a trailing `/`
- Example: `a.txt`, `dir/`, `z.txt` (not `a.txt`, `z.txt`, `dir/`)

## Examples

### Basic Usage

```bash
# Initialize repository
$ rit init

# Create some files
$ echo "Hello" > hello.txt
$ echo "World" > world.txt

# Create tree from current directory
$ rit write-tree
a1b2c3d4e5f6789abcdef0123456789abcdef0123

# Verify the tree was stored
$ ls .rit/objects/a1/
b2c3d4e5f6789abcdef0123456789abcdef0123

# Read the tree back
$ rit cat-file -p a1b2c3d4e5f6789abcdef0123456789abcdef0123
100644 blob abc123...    hello.txt
100644 blob def456...    world.txt
```

### With Subdirectories

```bash
# Create directory structure
$ mkdir src
$ echo "code" > src/main.rs
$ echo "readme" > README.md

# Create tree (recursively processes subdirectories)
$ rit write-tree
789abc...def012...

# The tree includes both files and subdirectories
$ rit cat-file -p 789abc...def012...
100644 blob abc123...    README.md
040000 tree def456...    src
```

### Verify Against Real Git

You can verify your implementation matches Git:

```bash
# Using Git
$ git init
$ echo "test" > test.txt
$ git add test.txt
$ git write-tree
a1b2c3d4e5f6789abcdef0123456789abcdef0123

# Using Rit (should match if same files!)
$ rit init
$ echo "test" > test.txt
$ rit hash-object -w test.txt
# (same hash as above)
$ rit write-tree
# (should match Git's tree hash)
```

## Implementation Details

### Recursive Tree Building

```rust
fn write_tree_recursive(
    repo: &Repository,
    dir_path: &Path,
    base_path: &Path,
) -> Result<String> {
    let mut tree = Tree::new();

    // Process each entry in directory
    for entry in read_dir(dir_path)? {
        if entry.is_file() {
            // Hash file as blob
            let hash = hash_and_store_blob(&entry)?;
            tree.add_entry(TreeEntry::file(name, hash));
        } else if entry.is_dir() {
            // Recursively create subtree
            let subtree_hash = write_tree_recursive(&subdir)?;
            tree.add_entry(TreeEntry::directory(name, subtree_hash));
        }
    }

    // Sort and store tree
    tree.sort();
    store_tree(repo, &tree)
}
```

### What Gets Excluded

- `.rit/` directory (always excluded)
- Symlinks (not yet supported)
- Other special files

## Use Cases

### Before Committing

```bash
# Create tree from working directory
$ rit write-tree
abc123...

# Later, use this tree hash in a commit
$ rit commit-tree abc123... -m "Initial commit"
```

### Understanding Directory Structure

```bash
# See how Git represents your directory
$ rit write-tree > tree_hash.txt
$ rit cat-file -p $(cat tree_hash.txt)
```

## See Also

- [hash-object](./hash-object.md) - Store individual files
- [cat-file](./cat-file.md) - Read tree objects back
- ls-tree - List tree contents (coming in Commit 6)
- [Architecture](../architecture.md) - Tree object format details

