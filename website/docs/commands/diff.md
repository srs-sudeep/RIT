# diff

Show changes between commits, the index, and the working directory.

## Synopsis

```bash
rit diff                    # Show changes in working directory vs index
rit diff --cached           # Show changes in index vs HEAD (staged)
rit diff <commit1> <commit2> # Show changes between two commits
```

## Description

The `diff` command shows the differences between various states of your repository:

- **Working directory vs index** - Shows unstaged changes (default)
- **Index vs HEAD** - Shows staged changes (with `--cached`)
- **Commit vs commit** - Shows differences between two commits (planned)

The output uses the unified diff format, similar to Git's diff output.

## Options

- `--cached` - Show staged changes (index vs HEAD) instead of working directory changes

## Examples

### Show unstaged changes

```bash
$ rit diff
--- a/file.txt
+++ b/file.txt
@@ -1,3 +1,4 @@
 line 1
-line 2
+line 2 modified
 line 3
+line 4
```

### Show staged changes

```bash
$ rit diff --cached
--- a/file.txt
+++ b/file.txt
@@ -1,2 +1,3 @@
 line 1
 line 2
+line 3
```

### Compare two commits

```bash
$ rit diff abc123 def456
```

**Note**: Comparing two commits is planned for a future version.

## Unified Diff Format

The unified diff format shows:

- **Header**: `--- a/path` and `+++ b/path` indicate the file being compared
- **Hunk header**: `@@ -old_start,old_count +new_start,new_count @@` shows line ranges
- **Lines**:
  - Lines starting with ` ` (space) are unchanged
  - Lines starting with `-` are deleted (only in old)
  - Lines starting with `+` are inserted (only in new)

### Example Output

```
--- a/README.md
+++ b/README.md
@@ -1,3 +1,4 @@
 # Project Title
 Description
+New line added
```

## How It Works

The diff command uses the **Myers algorithm** to find the shortest edit script between two sequences of lines. The algorithm:

1. Compares line-by-line between old and new content
2. Finds the longest common subsequence
3. Identifies insertions, deletions, and unchanged lines
4. Formats the result as a unified diff

### Working Directory vs Index

When you run `rit diff` (without `--cached`):
1. Reads files from the working directory
2. Compares with their versions in the index
3. Shows what would be staged if you run `rit add`

### Index vs HEAD

When you run `rit diff --cached`:
1. Reads files from the index (staging area)
2. Compares with their versions in the last commit (HEAD)
3. Shows what will be committed if you run `rit commit`

## Myers Algorithm

The Myers diff algorithm is the standard algorithm used by Git and many other diff tools. It finds the optimal (shortest) sequence of edits to transform one file into another.

**Note**: The current implementation uses a simplified version of the Myers algorithm. A full O(ND) implementation is planned for future versions.

## See Also

- [status](status.md) - Show working tree status
- [add](add.md) - Stage files for commit
- [commit](commit.md) - Create a commit

