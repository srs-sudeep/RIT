# Architecture

This document explains how Git (and Rit) works internally.

## The Big Idea: Content-Addressable Storage

Git is fundamentally a **content-addressable filesystem**. This means:

1. Every piece of data is stored as an "object"
2. Each object is identified by its SHA-1 hash
3. The hash is computed from the content itself

This design has powerful implications:

- **Immutability**: Once stored, objects never change (changing content = new hash)
- **Deduplication**: Identical files share the same object
- **Integrity**: Any corruption is immediately detectable
- **Efficient comparison**: Same hash = same content

## Object Types

### Blob

A blob stores raw file contents. It has:
- **No filename** - just the bytes
- **No permissions** - that's stored in the tree
- **No metadata** - pure content

```
blob <size>\0<content>
```

Example:
```
blob 13\0Hello, World!
```

### Tree

A tree represents a directory. It maps names to hashes:

```
tree <size>\0
<mode> <name>\0<20-byte-hash>
<mode> <name>\0<20-byte-hash>
...
```

Example:
```
100644 README.md\0<hash>
100755 build.sh\0<hash>
040000 src\0<hash-of-subtree>
```

File modes:
- `100644` - Regular file
- `100755` - Executable file
- `040000` - Directory (tree)
- `120000` - Symbolic link

### Commit

A commit is a snapshot with metadata:

```
tree <tree-hash>
parent <parent-hash>
author <name> <email> <timestamp> <tz>
committer <name> <email> <timestamp> <tz>

<commit message>
```

## The DAG (Directed Acyclic Graph)

Commits form a DAG where each commit points to its parent(s):

```
    ┌───────┐
    │ C3    │ ◄── HEAD
    └───┬───┘
        │ parent
    ┌───▼───┐
    │ C2    │
    └───┬───┘
        │ parent
    ┌───▼───┐
    │ C1    │ (initial commit, no parent)
    └───────┘
```

For merge commits:
```
    ┌───────┐
    │ Merge │ ◄── HEAD
    └───┬───┘
       /│\
      / │ \
     /  │  \ (two parents)
┌───▼─┐ │ ┌─▼───┐
│ C2  │ │ │ C3  │
└──┬──┘ │ └──┬──┘
   │    │    │
   └────▼────┘
    ┌───────┐
    │ C1    │
    └───────┘
```

## References

References are just files that contain commit hashes:

```
.rit/
├── HEAD                    # "ref: refs/heads/main"
└── refs/
    ├── heads/
    │   ├── main           # "abc123..."
    │   └── feature        # "def456..."
    └── tags/
        └── v1.0           # "789abc..."
```

### HEAD

HEAD is special - it usually points to a branch reference:
```
ref: refs/heads/main
```

In "detached HEAD" state, it points directly to a commit:
```
abc123def456...
```

## The Index (Staging Area)

The index is a binary file (`.rit/index`) that tracks:

1. Files staged for the next commit
2. Their blob hashes
3. File metadata (mtime, size, etc.)

This enables:
- Partial commits (only stage some changes)
- Fast status checks (compare mtimes before hashing)
- Merge conflict tracking

## Storage Layout

Objects are stored at `.rit/objects/<first-2-chars>/<remaining-chars>`:

```
.rit/objects/
├── ab/
│   └── cdef1234567890...  # Object abc def...
├── 12/
│   └── 3456789abcdef...   # Object 123 456...
└── ...
```

Each object file contains zlib-compressed data:
```
zlib_compress(header + content)
```

## Data Flow

```
                    Working Directory
                          │
            ┌─────────────┼─────────────┐
            │             │             │
            ▼             ▼             ▼
     ┌──────────┐  ┌──────────┐  ┌──────────┐
     │ hash-obj │  │   add    │  │  commit  │
     │   -w     │  │          │  │          │
     └────┬─────┘  └────┬─────┘  └────┬─────┘
          │             │             │
          │             ▼             │
          │       ┌──────────┐        │
          │       │  Index   │        │
          │       └────┬─────┘        │
          │            │              │
          └────────────┼──────────────┘
                       ▼
                ┌──────────────┐
                │   Objects    │
                │   Database   │
                └──────────────┘
```

