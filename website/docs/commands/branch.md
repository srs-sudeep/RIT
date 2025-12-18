# branch

List, create, or delete branches.

## Synopsis

```bash
rit branch                    # List all branches
rit branch <name>             # Create a new branch
rit branch -d <name>          # Delete a branch
rit branch -D <name>          # Force delete a branch
```

## Description

The `branch` command manages branches in the repository. Without arguments, it lists all branches with the current branch marked with an asterisk (`*`).

### Creating Branches

When you create a branch with `rit branch <name>`, the new branch is created pointing to the current HEAD commit. This means:

- The new branch starts at the same commit as your current branch
- No files are copied or moved
- You can immediately start making commits on the new branch

### Listing Branches

When you run `rit branch` without arguments, it shows:
- All branches in `refs/heads/`
- The current branch marked with `*`
- Branches sorted alphabetically

### Deleting Branches

You can delete branches with:
- `rit branch -d <name>` - Safe delete (only if merged)
- `rit branch -D <name>` - Force delete (even if not merged)

**Note**: You cannot delete the branch you're currently on unless you use the force flag.

## Options

- `-d`, `--delete` - Delete the specified branch. The branch must be merged into the current branch.
- `-D` - Force delete the branch, even if it's not merged.

## Examples

### List all branches

```bash
$ rit branch
* main
  feature
  bugfix
```

The asterisk (`*`) indicates that `main` is the current branch.

### Create a new branch

```bash
$ rit branch feature
Created branch 'feature'
```

This creates a new branch called `feature` pointing to the current HEAD commit.

### Delete a merged branch

```bash
$ rit branch -d feature
Deleted branch 'feature' (was abc1234)
```

This deletes the branch if it's been merged into the current branch.

### Force delete a branch

```bash
$ rit branch -D feature
Deleted branch 'feature' (was abc1234)
```

This deletes the branch even if it hasn't been merged.

## How It Works

Branches in Rit are simply files in `.rit/refs/heads/` that contain commit hashes:

```
.rit/
└── refs/
    └── heads/
        ├── main      # Contains: abc123...
        ├── feature   # Contains: def456...
        └── bugfix    # Contains: 789abc...
```

When you create a branch:
1. Rit reads the current HEAD commit hash
2. Creates a new file in `refs/heads/<branch-name>`
3. Writes the commit hash to that file

When you list branches:
1. Rit reads all files in `refs/heads/`
2. Compares with HEAD to find the current branch
3. Displays them sorted alphabetically

## Current Branch

The current branch is determined by reading `.rit/HEAD`. If HEAD contains:
- `ref: refs/heads/main` - The current branch is `main`
- A commit hash directly - HEAD is detached (no current branch)

## See Also

- [commit](commit.md) - Create commits
- [log](log.md) - View commit history

