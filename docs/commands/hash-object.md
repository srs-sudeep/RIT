# rit hash-object

Compute the SHA-1 hash of a file and optionally store it.

## Synopsis

```bash
rit hash-object [-w] <file>
```

## Description

This is a "plumbing" command that computes the Git object hash for a file. With `-w`, it also stores the object in the database.

## Options

| Option | Description |
|--------|-------------|
| `-w, --write` | Actually store the object in `.rit/objects/` |

## How Hashing Works

Git doesn't just hash the raw file content. It prepends a header:

```
blob <size>\0<content>
```

For example, a file containing "Hello, World!" (13 bytes) becomes:

```
blob 13\0Hello, World!
```

This entire string is then:
1. SHA-1 hashed → 40-character hex string
2. Zlib compressed
3. Stored at `.rit/objects/ab/cdef...` (first 2 chars / rest)

## Examples

```bash
# Just compute hash (don't store)
$ echo "Hello, World!" > hello.txt
$ rit hash-object hello.txt
8ab686eafeb1f44702738c8b0f24f2567c36da6d

# Store in object database
$ rit hash-object -w hello.txt
8ab686eafeb1f44702738c8b0f24f2567c36da6d

# Verify it was stored
$ ls .rit/objects/8a/
b686eafeb1f44702738c8b0f24f2567c36da6d

# Read it back
$ rit cat-file -p 8ab686eafeb1f44702738c8b0f24f2567c36da6d
Hello, World!
```

## Verify Against Real Git

You can verify your implementation matches Git:

```bash
# Using Git
$ echo "Hello, World!" | git hash-object --stdin
8ab686eafeb1f44702738c8b0f24f2567c36da6d

# Using Rit (should match!)
$ echo "Hello, World!" > test.txt
$ rit hash-object test.txt
8ab686eafeb1f44702738c8b0f24f2567c36da6d
```

## Implementation Details

```rust
pub fn hash_content(object_type: &str, content: &[u8]) -> String {
    // Create header
    let header = format!("{} {}\0", object_type, content.len());

    // Hash header + content
    let mut hasher = Sha1::new();
    hasher.update(header.as_bytes());
    hasher.update(content);

    hex::encode(hasher.finalize())
}

pub fn store_object(repo: &Repository, object_type: &str, content: &[u8]) -> Result<String> {
    let hash = hash_content(object_type, content);

    // Create directory
    let dir = repo.objects_dir().join(&hash[..2]);
    fs::create_dir_all(&dir)?;

    // Compress and write
    let object_path = dir.join(&hash[2..]);
    if !object_path.exists() {
        let header = format!("{} {}\0", object_type, content.len());
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(header.as_bytes())?;
        encoder.write_all(content)?;
        fs::write(&object_path, encoder.finish()?)?;
    }

    Ok(hash)
}
```

## See Also

- [cat-file](./cat-file.md) - Read objects back from the database
- [Architecture](../architecture.md) - Object storage details

