# rit ls-tree

List the contents of a tree object in a human-readable format.

## Synopsis

```bash
rit ls-tree [options] <tree-hash>
```

## Description

This plumbing command displays the contents of a tree object, showing each entry's mode, type, hash, and name. It's similar to Git's `ls-tree` command.

## Options

| Option | Description |
|--------|-------------|
| `-r, --recursive` | Recursively list all subtrees |
| `--name-only` | Show only file/directory names (no mode, type, or hash) |

## Output Format

### Default Format

```
<mode> <type> <hash>    <name>
```

Example:
```
100644 blob abc123...    README.md
040000 tree def456...    src
```

### File Modes

- `100644` - Regular file
- `100755` - Executable file
- `040000` - Directory (tree)
- `120000` - Symbolic link

### Object Types

- `blob` - File content
- `tree` - Directory (subtree)

## Examples

### Basic Listing

```bash
# Create a tree first
$ rit write-tree
a1b2c3d4e5f6789abcdef0123456789abcdef0123

# List tree contents
$ rit ls-tree a1b2c3d4e5f6789abcdef0123456789abcdef0123
100644 blob e965047a...    hello.txt
100644 blob 216e97ce...    world.txt
```

### With Subdirectories

```bash
# Create directory structure
$ mkdir src
$ echo "code" > src/main.rs
$ echo "readme" > README.md

# Create tree
$ TREE_HASH=$(rit write-tree)

# Non-recursive listing (shows top level)
$ rit ls-tree $TREE_HASH
100644 blob abc123...    README.md
040000 tree def456...    src

# Recursive listing (shows all files)
$ rit ls-tree -r $TREE_HASH
100644 blob abc123...    README.md
100644 blob 789xyz...    src/main.rs
```

### Name-Only Mode

```bash
# Show only names
$ rit ls-tree --name-only $TREE_HASH
README.md
src

# Recursive with name-only
$ rit ls-tree -r --name-only $TREE_HASH
README.md
src/main.rs
```

### Verify Against Real Git

You can verify your implementation matches Git:

```bash
# Using Git
$ git init
$ echo "test" > test.txt
$ git add test.txt
$ TREE_HASH=$(git write-tree)
$ git ls-tree $TREE_HASH
100644 blob abc123...    test.txt

# Using Rit (should match!)
$ rit init
$ echo "test" > test.txt
$ rit hash-object -w test.txt
$ TREE_HASH=$(rit write-tree)
$ rit ls-tree $TREE_HASH
100644 blob abc123...    test.txt
```

## Use Cases

### Inspecting Repository Structure

```bash
# Get the tree hash from a commit (once commits are implemented)
$ COMMIT_HASH=$(rit log --format=%T | head -1)
$ rit ls-tree $COMMIT_HASH
```

### Finding Files in a Tree

```bash
# Recursively list all files
$ rit ls-tree -r $TREE_HASH | grep "\.rs$"
```

### Comparing Trees

```bash
# List two trees and compare
$ rit ls-tree --name-only $TREE1 > tree1.txt
$ rit ls-tree --name-only $TREE2 > tree2.txt
$ diff tree1.txt tree2.txt
```

## Implementation Details

### Tree Parsing

The command reads the tree object, decompresses it, and parses the binary format:

```
<mode> <name>\0<20-byte-hash>
<mode> <name>\0<20-byte-hash>
...
```

### Recursive Listing

When `-r` is used, the command:
1. Lists entries in the current tree
2. For each subtree entry, recursively calls itself
3. Builds full paths (e.g., `src/main.rs`) for files in subdirectories

## See Also

- [write-tree](./write-tree.md) - Create tree objects
- [cat-file](./cat-file.md) - Read raw object contents
- [Architecture](../architecture.md) - Tree object format details

