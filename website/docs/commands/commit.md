# rit commit

Create a commit from the staging area (index).

## Synopsis

```bash
rit commit -m <message>
```

## Description

This is the high-level commit command that creates a commit from files staged in the index. It:

1. Reads the index (staging area)
2. Builds a tree object from staged files
3. Creates a commit object with the tree
4. Updates HEAD (or branch ref) to point to the new commit

## Options

| Option | Description |
|--------|-------------|
| `-m, --message <message>` | Commit message (required) |
| `-a, --auto-add` | Automatically stage modified files (not yet implemented) |

## Examples

### Basic Commit

```bash
# Stage files
$ rit add file1.txt file2.txt

# Create commit
$ rit commit -m "Add files"
[abc1234] Add files
 2 file(s) changed
```

### Commit Workflow

```bash
# 1. Make changes
$ echo "Hello" > hello.txt

# 2. Stage files
$ rit add hello.txt

# 3. Create commit
$ rit commit -m "Add hello.txt"
[def5678] Add hello.txt
 1 file(s) changed

# 4. View history
$ rit log --oneline
def5678 Add hello.txt
abc1234 Initial commit
```

### Multiple Commits

```bash
# First commit
$ rit add file1.txt
$ rit commit -m "First commit"
[abc1234] First commit
 1 file(s) changed

# Second commit (with parent)
$ rit add file2.txt
$ rit commit -m "Second commit"
[def5678] Second commit
 1 file(s) changed

# View log
$ rit log
commit def5678...
Author: ...
Date: ...

    Second commit

commit abc1234...
Author: ...
Date: ...

    First commit
```

## How It Works

### From Index to Commit

1. **Read Index**: Loads `.rit/index` (JSON file with staged files)
2. **Build Tree**: Creates tree objects from index entries, organizing files into directory structure
3. **Create Commit**: Uses `commit-tree` internally to create commit object
4. **Update Ref**: Updates HEAD or branch ref to point to new commit

### Tree Building

The command organizes staged files into a tree structure:
- Files in root → root tree
- Files in subdirectories → subtree entries
- Builds trees bottom-up (deepest first)

## Empty Index

If you try to commit with nothing staged:

```bash
$ rit commit -m "Empty commit"
nothing to commit, working tree clean
```

## Use Cases

### Typical Workflow

```bash
# Make changes
$ echo "new content" > file.txt

# Stage changes
$ rit add file.txt

# Commit
$ rit commit -m "Update file.txt"
```

### Verify Commit

```bash
# Create commit
$ rit commit -m "Test commit"
[abc1234] Test commit
 1 file(s) changed

# View commit details
$ rit cat-file -p abc1234

# View log
$ rit log
```

## Differences from Git

- **Simpler index format**: Uses JSON instead of binary format
- **No auto-add yet**: `-a` flag is not yet implemented
- **No commit hooks**: No pre-commit/post-commit hooks

## See Also

- [add](./add.md) - Stage files for commit
- [commit-tree](./commit-tree.md) - Low-level commit creation
- [log](./log.md) - View commit history
- status - Show staging status (coming in Commit 14)

