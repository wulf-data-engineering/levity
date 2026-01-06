---
trigger: always_on
---

# Terminal Command Rules

This repository uses a template structure with special characters in filenames (e.g., `%[cookiecutter.project_slug]%`). These characters (`%`, `[`, `]`) are interpreted by shells (like zsh) as glob patterns or special variables.

## CRITICAL: Always Quote Paths
When using `run_command`, you **MUST** enclose any file path in **double quotes** (`"`) or **single quotes** (`'`).
### Why?
If you do not quote the path, the shell will try to expand `%[...]%` and fail with errors like `zsh: no matches found`.

### Examples

#### ❌ Incorrect (Will Fail)
```bash
git add template/%[cookiecutter.project_slug]%/file.txt
rm template/%[cookiecutter.project_slug]%/file.txt
ls template/%[cookiecutter.project_slug]%
cp Source template/%[cookiecutter.project_slug]%/Dest

#### ✅ Correct (Will Succeed)

git add "template/%[cookiecutter.project_slug]%/file.txt"
rm "template/%[cookiecutter.project_slug]%/file.txt"
ls "template/%[cookiecutter.project_slug]%"
cp Source "template/%[cookiecutter.project_slug]%/Dest"
git add "template/%[cookiecutter.project_slug]%/file.txt"