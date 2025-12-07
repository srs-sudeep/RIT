# rit log

Display commit history by traversing the parent chain.

## Synopsis

```bash
rit log [--oneline] [--graph]
```

## Description

This command displays the commit history starting from HEAD, traversing backwards through parent commits. It's similar to Git's `log` command.

## Options

| Option | Description |
|--------|-------------|
| `--oneline` | Show one commit per line (short hash + message) |
| `--graph` | Draw ASCII graph of commit history |

## Examples

### Basic Log

```bash
# Show full commit log
$ rit log
commit abc123def456...
Author: John Doe <john@example.com>
Date:   1234567890

    Initial commit

commit def456ghi789...
Author: John Doe <john@example.com>
Date:   1234567891

    Second commit
```

### One-Line Format

```bash
$ rit log --oneline
abc123 Initial commit
def456 Second commit
ghi789 Third commit
```

### With Graph

```bash
$ rit log --graph
* abc123 Initial commit
|
* def456 Second commit
|
* ghi789 Third commit
```

## How It Works

1. Reads HEAD to find the current commit
2. If HEAD points to a branch ref, reads that ref file
3. Traverses backwards through parent commits
4. Formats and displays each commit

## HEAD Resolution

The command resolves HEAD in this order:

1. **Branch Reference**: If HEAD contains `ref: refs/heads/main`, reads `.rit/refs/heads/main`
2. **Detached HEAD**: If HEAD contains a direct commit hash, uses that
3. **No Commits**: If no commits exist, shows an error message

## Use Cases

### View Recent Commits

```bash
# See what you've committed
$ rit log --oneline
```

### Check Commit History

```bash
# Full details of all commits
$ rit log
```

### Visualize Branch Structure

```bash
# See commit graph (useful for branches)
$ rit log --graph
```

## Implementation Details

### Commit Traversal

The command follows the first parent of each commit, creating a linear history. For merge commits, it currently only follows the first parent.

### Format

- **Full format**: Shows commit hash, author, date, and full message
- **Oneline format**: Shows short hash (7 chars) and first line of message
- **Graph format**: Adds ASCII visualization with `*` and `|` characters

## See Also

- [commit-tree](./commit-tree.md) - Create commits
- [cat-file](./cat-file.md) - Read commit objects directly
- [Architecture](../architecture.md) - Commit object format

