# tag

Create, list, or delete tags.

## Synopsis

```bash
rit tag                    # List all tags
rit tag name               # Create a lightweight tag
rit tag -a name -m "msg"   # Create an annotated tag
rit tag -d name            # Delete a tag
```

## Description

Tags are references to specific commits, useful for marking releases or important milestones. Rit supports two types of tags:

1. Lightweight tags - Simple refs pointing to a commit
2. Annotated tags - Tag objects with metadata (message, tagger, etc.)

**Lightweight tags** are just files in the refs/tags directory containing a commit hash. They are quick to create and do not store extra metadata.

**Annotated tags** are full Git objects stored in the object database.

Annotated tags are full Git objects stored in the object database. They include:
- The commit they point to
- A tagger (author)
- A message
- A timestamp

**Note**: Currently, Rit creates lightweight tags even with `-a` flag. Full annotated tag support is planned for a future version.

## Options

- `-a`, `--annotated` - Create an annotated tag (currently creates lightweight tag)
- `-m`, `--message` - Tag message (for annotated tags)
- `-d` - Delete the tag

## Examples

### List all tags

```bash
$ rit tag
v1.0.0
v1.1.0
v2.0.0
```

### Create a lightweight tag

```bash
$ rit tag v1.0.0
Created tag 'v1.0.0'
```

This creates a tag pointing to the current HEAD commit.

### Create an annotated tag

```bash
$ rit tag -a v1.0.0 -m "Release version 1.0.0"
Created tag 'v1.0.0'
```

**Note**: Currently creates a lightweight tag. Full annotated tag support coming soon.

### Delete a tag

```bash
$ rit tag -d v1.0.0
Deleted tag 'v1.0.0' (was abc1234)
```

## How It Works

Tags are stored as files in `.rit/refs/tags/`:

```
.rit/
└── refs/
    └── tags/
        ├── v1.0.0    # Contains: abc123...
        └── v2.0.0    # Contains: def456...
```

When you create a tag:
1. Rit reads the current HEAD commit hash
2. Creates a file in refs/tags with the tag name
3. Writes the commit hash to that file

## Tag Naming Conventions

Tag names should:
- Not contain spaces
- Not contain forward slashes
- Not contain backslashes
- Be descriptive (e.g., `v1.0.0`, `release-2024-01-01`)

## See Also

- [branch](branch.md) - Manage branches
- [checkout](checkout.md) - Checkout a tag
- [log](log.md) - View commit history
