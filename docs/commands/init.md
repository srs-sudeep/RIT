# rit init

Initialize a new Rit repository.

## Synopsis

```bash
rit init [path]
```

## Description

This command creates a new `.rit` directory with the necessary subdirectories and files to begin tracking changes.

## What Gets Created

```
.rit/
├── HEAD            # Points to current branch
├── objects/        # Object database (empty)
└── refs/
    ├── heads/      # Branch references (empty)
    └── tags/       # Tag references (empty)
```

### HEAD

The `HEAD` file initially contains:
```
ref: refs/heads/main
```

This means "the current branch is main" - even though no commits exist yet.

## Examples

```bash
# Initialize in current directory
$ rit init
Initialized empty rit repository in .rit/

# Initialize in a specific directory
$ rit init /path/to/project
Initialized empty rit repository in /path/to/project/.rit/

# Re-running init is safe
$ rit init
Reinitialized existing rit repository in .rit/
```

## Implementation Details

```rust
pub fn init(path: &Path) -> Result<Repository> {
    let rit_dir = path.join(".rit");

    if rit_dir.exists() {
        println!("Reinitialized existing rit repository");
    } else {
        fs::create_dir(&rit_dir)?;
        fs::create_dir(rit_dir.join("objects"))?;
        fs::create_dir(rit_dir.join("refs"))?;
        fs::create_dir(rit_dir.join("refs/heads"))?;
        fs::create_dir(rit_dir.join("refs/tags"))?;
        fs::write(rit_dir.join("HEAD"), "ref: refs/heads/main\n")?;
        println!("Initialized empty rit repository");
    }

    Ok(Repository { root: path, rit_dir })
}
```

## See Also

- [Architecture](../architecture.md) - Repository structure overview
- [hash-object](./hash-object.md) - Store files in the object database

