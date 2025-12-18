# .ritignore

Specify files and directories that Rit should ignore.

## Synopsis

Create a `.ritignore` file in your repository root to tell Rit which files to ignore when staging, checking status, or showing diffs.

## Description

The `.ritignore` file uses pattern matching similar to `.gitignore`. Files and directories matching patterns in `.ritignore` will be:

- Excluded from `rit add`
- Hidden from `rit status` (untracked files)
- Excluded from `rit diff`

## Pattern Format

### Basic Patterns

- `*.log` - Ignore all files ending with `.log`
- `target/` - Ignore the `target` directory
- `temp.txt` - Ignore a specific file
- `*.tmp` - Ignore all `.tmp` files

### Wildcards

- `*` - Matches any sequence of characters (except `/`)
- `?` - Matches a single character
- `**` - Matches zero or more directories (planned)

### Directory Patterns

- `dir/` - Matches only directories named `dir`
- `dir/*` - Matches all files in `dir` directory

### Negation

- `!important.log` - Un-ignore a file (even if it matches an earlier pattern)

Patterns are processed in order, so later patterns can override earlier ones.

## Examples

### Rust Project

```gitignore
# Compiled files
target/
*.o
*.so
*.dylib

# Cargo
Cargo.lock

# IDE
.idea/
.vscode/
*.swp

# But keep important files
!Cargo.lock
```

### Node.js Project

```gitignore
node_modules/
*.log
npm-debug.log*
.DS_Store
.env
dist/
build/
```

### Python Project

```gitignore
__pycache__/
*.py[cod]
*$py.class
*.so
.Python
venv/
env/
.venv
```

## Pattern Rules

1. **Empty lines and comments** - Lines starting with `#` are ignored
2. **Order matters** - Patterns are processed top to bottom
3. **Negation** - Patterns starting with `!` un-ignore matching files
4. **Directory matching** - Patterns ending with `/` only match directories
5. **Anchored patterns** - Patterns not starting with `*` are anchored to the start

## How It Works

When Rit processes files:

1. Reads `.ritignore` from the repository root
2. Parses patterns line by line
3. Checks each file/directory against all patterns
4. If a pattern matches and is not negated, the file is ignored

## Integration

The `.ritignore` file is automatically used by:

- `rit add` - Ignores matching files when staging
- `rit status` - Hides ignored files from untracked files list
- `rit diff` - Excludes ignored files from diff output

## Always Ignored

The following are always ignored, regardless of `.ritignore`:

- `.rit/` directory (repository metadata)
- `.ritignore` file itself

## See Also

- [add](commands/add.md) - Stage files
- [status](commands/status.md) - Show working tree status
- [diff](commands/diff.md) - Show changes

