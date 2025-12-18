# checkout

Switch branches or restore files from a commit.

## Synopsis

```bash
rit checkout <branch>              # Switch to a branch
rit checkout <commit-hash>          # Switch to a commit (detached HEAD)
rit checkout -- <file>              # Restore a file from HEAD
rit checkout <commit> -- <file>     # Restore a file from a specific commit
```

## Description

The `checkout` command allows you to:

1. **Switch branches** - Move HEAD to point to a different branch and update the working directory
2. **Checkout a commit** - Move to a specific commit (detached HEAD state)
3. **Restore files** - Restore individual files from a commit or branch

### Switching Branches

When you checkout a branch:
- HEAD is updated to point to the branch
- The working directory is updated to match the branch's commit
- Files are written from the branch's tree

### Detached HEAD

When you checkout a commit hash directly (not a branch), you enter "detached HEAD" state:
- HEAD points directly to the commit hash
- You can make commits, but they won't be on any branch
- Useful for inspecting old commits

### Restoring Files

You can restore individual files from any commit or branch:
- `rit checkout HEAD -- <file>` - Restore from current commit
- `rit checkout <branch> -- <file>` - Restore from a branch
- `rit checkout <commit> -- <file>` - Restore from a specific commit

## Options

- `-f`, `--force` - Force checkout, overwriting local changes (use with caution)

## Examples

### Switch to a branch

```bash
$ rit branch
* main
  feature

$ rit checkout feature
Switched to branch 'feature'
```

### Checkout a specific commit

```bash
$ rit checkout abc1234
Note: checking out 'abc1234'.
You are in 'detached HEAD' state.
```

### Restore a file from HEAD

```bash
$ rit checkout -- file.txt
Updated 'file.txt'
```

### Restore a file from a branch

```bash
$ rit checkout feature -- file.txt
Updated 'file.txt'
```

### Force checkout (overwrite local changes)

```bash
$ rit checkout -f main
Switched to branch 'main'
```

## How It Works

### Branch Checkout

1. Resolve the branch name to a commit hash
2. Read the commit to get its tree hash
3. Recursively read the tree and write files to the working directory
4. Update HEAD to point to the branch

### File Checkout

1. Resolve the reference (branch/commit) to a commit hash
2. Navigate through the tree to find the file
3. Read the blob and write it to the working directory

### Tree Writing

When writing a tree to the working directory:
- Directories are created as needed
- Files are written with their blob contents
- Executable permissions are preserved (on Unix systems)

## Current Branch vs Detached HEAD

After checkout:
- **Branch checkout**: HEAD contains `ref: refs/heads/<branch>`
- **Commit checkout**: HEAD contains the commit hash directly

You can check your state with:
```bash
$ cat .rit/HEAD
ref: refs/heads/main    # On a branch
# or
abc1234...              # Detached HEAD
```

## Warnings

- **Local changes**: If files have been modified, checkout will warn and skip them (unless `--force` is used)
- **Untracked files**: Untracked files are not removed during checkout
- **Detached HEAD**: Commits made in detached HEAD state won't be on any branch

## See Also

- [branch](branch.md) - List and manage branches
- [commit](commit.md) - Create commits
- [log](log.md) - View commit history

