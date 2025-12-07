# rit commit-tree

Create a commit object from a tree hash.

## Synopsis

```bash
rit commit-tree <tree-hash> -m <message> [-p <parent>]...
```

## Description

This plumbing command creates a commit object from a tree hash. It's the low-level command for creating commits - the high-level `commit` command (coming in Commit 10) will use this internally.

## Options

| Option | Description |
|--------|-------------|
| `-m, --message <message>` | Commit message (required) |
| `-p, --parent <hash>` | Parent commit hash (can be specified multiple times for merge commits) |

## Commit Object Format

A commit object contains:

```
tree <tree-sha1>
parent <parent-sha1>     # 0 or more parent lines
author <name> <email> <timestamp> <timezone>
committer <name> <email> <timestamp> <timezone>

<commit message>
```

## Examples

### Initial Commit (No Parent)

```bash
# Create a tree first
$ rit write-tree
a1b2c3d4e5f6789abcdef0123456789abcdef0123

# Create commit from tree
$ rit commit-tree a1b2c3d4e5f6789abcdef0123456789abcdef0123 -m "Initial commit"
7b539874eb3d54acf7f30f2d38e74307bfb3c3d8

# Read the commit back
$ rit cat-file -p 7b539874eb3d54acf7f30f2d38e74307bfb3c3d8
tree a1b2c3d4e5f6789abcdef0123456789abcdef0123
author Your Name <you@example.com> 1234567890 +0000
committer Your Name <you@example.com> 1234567890 +0000

Initial commit
```

### Commit with Parent

```bash
# First commit
$ TREE1=$(rit write-tree)
$ COMMIT1=$(rit commit-tree $TREE1 -m "First commit")

# Second commit (with parent)
$ echo "new file" > new.txt
$ TREE2=$(rit write-tree)
$ COMMIT2=$(rit commit-tree $TREE2 -m "Second commit" -p $COMMIT1)

# Verify parent relationship
$ rit cat-file -p $COMMIT2
tree <tree2-hash>
parent <commit1-hash>
author ...
committer ...

Second commit
```

### Merge Commit (Multiple Parents)

```bash
# Create merge commit with two parents
$ rit commit-tree <tree-hash> -m "Merge branches" -p <parent1> -p <parent2>
```

## Author Information

The command automatically detects author information from:

1. Environment variables:
   - `GIT_AUTHOR_NAME` / `GIT_COMMITTER_NAME`
   - `GIT_AUTHOR_EMAIL` / `GIT_COMMITTER_EMAIL`

2. System defaults:
   - Username from `$USER` or `$USERNAME`
   - Email: `<username>@localhost`

## Use Cases

### Creating Your First Commit

```bash
# 1. Initialize repository
$ rit init

# 2. Create some files
$ echo "Hello" > hello.txt

# 3. Create tree
$ TREE=$(rit write-tree)

# 4. Create commit
$ COMMIT=$(rit commit-tree $TREE -m "Initial commit")
$ echo $COMMIT > .rit/refs/heads/main  # Point branch to commit
```

### Building Commit History

```bash
# Commit 1
$ TREE1=$(rit write-tree)
$ COMMIT1=$(rit commit-tree $TREE1 -m "First")

# Commit 2 (child of commit 1)
$ # ... make changes ...
$ TREE2=$(rit write-tree)
$ COMMIT2=$(rit commit-tree $TREE2 -m "Second" -p $COMMIT1)

# Commit 3 (child of commit 2)
$ # ... make changes ...
$ TREE3=$(rit write-tree)
$ COMMIT3=$(rit commit-tree $TREE3 -m "Third" -p $COMMIT2)
```

## Verify Against Real Git

You can verify your implementation matches Git:

```bash
# Using Git
$ git init
$ echo "test" > test.txt
$ git add test.txt
$ TREE=$(git write-tree)
$ COMMIT=$(git commit-tree $TREE -m "test")
$ git cat-file -p $COMMIT

# Using Rit (should match!)
$ rit init
$ echo "test" > test.txt
$ TREE=$(rit write-tree)
$ COMMIT=$(rit commit-tree $TREE -m "test")
$ rit cat-file -p $COMMIT
```

## Implementation Details

### Commit Creation Process

1. Verify tree object exists
2. Get author/committer info (from env or defaults)
3. Create `Commit` struct
4. Serialize to Git format
5. Store as object (hash + compress)
6. Return commit hash

### Timestamp

The commit uses the current Unix timestamp (seconds since epoch) for both author and committer times.

## See Also

- [write-tree](./write-tree.md) - Create tree objects
- [cat-file](./cat-file.md) - Read commit objects
- log - View commit history (coming in Commit 8)
- [Architecture](../architecture.md) - Commit object format details

