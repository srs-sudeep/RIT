# rit cat-file

Display the contents or information about a repository object.

## Synopsis

```bash
rit cat-file -p <object-hash>
```

## Description

This plumbing command reads an object from the database, decompresses it, and displays its contents.

## Options

| Option | Description |
|--------|-------------|
| `-p` | Pretty-print the object contents based on its type |

## How It Works

1. Take the object hash (e.g., `abc123...`)
2. Find the file at `.rit/objects/ab/c123...`
3. Decompress with zlib
4. Parse the header to get type and size
5. Display the content

## Object Format

Objects are stored as:
```
<type> <size>\0<content>
```

After decompression, we parse:
- Type: "blob", "tree", or "commit"
- Size: Content length in bytes
- Content: The actual data after the null byte

## Examples

### Reading a Blob

```bash
# Store a file
$ echo "Hello, World!" > hello.txt
$ rit hash-object -w hello.txt
8ab686eafeb1f44702738c8b0f24f2567c36da6d

# Read it back
$ rit cat-file -p 8ab686eafeb1f44702738c8b0f24f2567c36da6d
Hello, World!
```

### Reading a Tree

```bash
$ rit cat-file -p <tree-hash>
100644 blob abc123...    README.md
100755 blob def456...    build.sh
040000 tree 789abc...    src
```

### Reading a Commit

```bash
$ rit cat-file -p <commit-hash>
tree 4b825dc642cb6eb9a060e54bf8d69288fbee4904
parent a1b2c3d4e5f6789...
author John Doe <john@example.com> 1234567890 +0000
committer John Doe <john@example.com> 1234567890 +0000

Initial commit
```

## Implementation Details

```rust
pub fn read_object(repo: &Repository, hash: &str) -> Result<GitObject> {
    let object_path = repo.objects_dir()
        .join(&hash[..2])
        .join(&hash[2..]);

    // Read and decompress
    let compressed = fs::read(&object_path)?;
    let mut decoder = ZlibDecoder::new(&compressed[..]);
    let mut data = Vec::new();
    decoder.read_to_end(&mut data)?;

    // Parse header (everything before null byte)
    let null_pos = data.iter().position(|&b| b == 0).unwrap();
    let header = String::from_utf8_lossy(&data[..null_pos]);
    let (object_type, size) = parse_header(&header)?;

    // Content is everything after null byte
    let content = data[null_pos + 1..].to_vec();

    Ok(GitObject { object_type, size, content })
}
```

## Error Cases

```bash
# Object doesn't exist
$ rit cat-file -p nonexistent123
fatal: object not found: nonexistent123

# Hash too short
$ rit cat-file -p abc
fatal: hash too short: abc
```

## See Also

- [hash-object](./hash-object.md) - Store objects in the database
- [Architecture](../architecture.md) - Object format details

