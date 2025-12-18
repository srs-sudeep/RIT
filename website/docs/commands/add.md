# rit add

Add file contents to the staging area (index).

## Synopsis

```bash
rit add <file>...
rit add <directory>
rit add .
```

## Description

This command stages files for the next commit by adding them to the index (staging area). Files are hashed, stored as blobs, and their metadata is recorded in the index.

## How It Works

1. Reads the file content
2. Hashes it as a blob and stores it in the object database
3. Records file metadata (path, hash, size, mtime) in the index
4. Saves the updated index to `.rit/index`

## Index Format

Rit uses a simplified JSON format for the index (Git uses a complex binary format):

```json
{
  "entries": {
    "file.txt": {
      "path": "file.txt",
      "hash": "abc123...",
      "size": 100,
      "mtime": 1234567890
    }
  }
}
```

## Examples

### Stage Single File

```bash
# Stage a specific file
$ rit add README.md
```

### Stage Multiple Files

```bash
# Stage multiple files
$ rit add file1.txt file2.txt file3.txt
```

### Stage Entire Directory

```bash
# Stage all files in current directory
$ rit add .

# Stage specific directory
$ rit add src/
```

### Stage Modified Files

```bash
# Modify a file
$ echo "updated" >> existing.txt

# Stage the modification
$ rit add existing.txt
```

## Use Cases

### Before Committing

```bash
# Stage files
$ rit add file1.txt file2.txt

# Later, create commit from index
$ rit commit -m "Add files"
```

### Update Staged Files

```bash
# Stage a file
$ rit add file.txt

# Modify it
$ echo "more content" >> file.txt

# Stage the update (replaces old entry)
$ rit add file.txt
```

## What Gets Staged

- **Files**: Content is hashed and stored as blobs
- **Directories**: All files in the directory (recursively) are staged
- **Modifications**: Re-staging updates the index entry

## What Doesn't Get Staged

- `.rit/` directory (always excluded)
- `.ritignore` file itself
- Files matching patterns in `.ritignore`
- Files outside the repository

See [.ritignore](../ritignore.md) for details on ignore patterns.

## Index Location

The index is stored at `.rit/index` as a JSON file.

## See Also

- commit - Create commit from staged files (coming in Commit 10)
- status - Show staging status (coming in Commit 14)
- [write-tree](./write-tree.md) - Create tree from working directory directly

