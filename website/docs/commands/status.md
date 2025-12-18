# status

Show the working tree status.

## Synopsis

```bash
rit status
```

## Description

The `status` command shows the state of your working directory compared to the index (staging area) and the last commit (HEAD). It displays:

- **Staged changes** - Files added to the index (ready to commit)
- **Modified files** - Files changed in working directory but not staged
- **Deleted files** - Files removed from working directory
- **Untracked files** - Files not in the index or HEAD

## Output Sections

### On branch

Shows the current branch name, or "HEAD detached" if in detached HEAD state.

### Changes to be committed

Files that are staged (in the index) and ready to be committed. These will be included in the next `rit commit`.

### Changes not staged for commit

Files that have been modified in the working directory but haven't been added to the index. Use `rit add` to stage them.

### Untracked files

Files in the working directory that aren't tracked by Rit. Use `rit add` to start tracking them.

**Note**: Files matching patterns in `.ritignore` are not shown as untracked. See [.ritignore](../ritignore.md) for details.

## Examples

### Clean working tree

```bash
$ rit status
On branch main

nothing to commit, working tree clean
```

### With staged changes

```bash
$ rit status
On branch main

Changes to be committed:
  (use "rit reset HEAD <file>..." to unstage)

        new file:   README.md

```

### With modified files

```bash
$ rit status
On branch main

Changes not staged for commit:
  (use "rit add <file>..." to update what will be committed)
  (use "rit checkout -- <file>..." to discard changes in working directory)

        modified:   src/main.rs

```

### With untracked files

```bash
$ rit status
On branch main

Untracked files:
  (use "rit add <file>..." to include in what will be committed)

        newfile.txt
        temp/
```

### Complete example

```bash
$ rit status
On branch feature

Changes to be committed:
  (use "rit reset HEAD <file>..." to unstage)

        new file:   newfeature.rs

Changes not staged for commit:
  (use "rit add <file>..." to update what will be committed)
  (use "rit checkout -- <file>..." to discard changes in working directory)

        modified:   src/main.rs
        deleted:    oldfile.txt

Untracked files:
  (use "rit add <file>..." to include in what will be committed)

        untracked.txt
```

## How It Works

The status command compares three states:

1. **Working Directory** - Files on your filesystem
2. **Index (Staging Area)** - Files staged for the next commit (`.rit/index`)
3. **HEAD** - The last commit (from `.rit/HEAD`)

It determines:
- **Staged**: Files in index that differ from HEAD (or new files)
- **Modified**: Files in working directory that differ from index
- **Deleted**: Files in index/HEAD that don't exist in working directory
- **Untracked**: Files in working directory not in index or HEAD

## See Also

- [add](add.md) - Stage files for commit
- [commit](commit.md) - Create a commit
- [checkout](checkout.md) - Discard changes

