---
trigger: model_decision
description: Git commit workflow and commit message formatting rules when staging or committing changes.
---

# Committing

## Build & Tests Work

Make sure tests and the build pass before committing.

## Separate Diff and Add/Commit

Do not chain `git status && ... && git commit ...` together. Run `git diff HEAD`, `git add`, and `git commit` as separate commands so command prefix-matching works cleanly.

## Commit Message Rules

Reflect the changes across **all** modified files since `HEAD` (check `git diff HEAD`), not just your last action. Focus on the functional change in the subject line.

- **Subject Line**: Imperative, starting with upper case, max 80 characters.
  - **CRITICAL**: Do NOT use prefixes like `feat:`, `fix:`, `chore:`.
- **Body**: Separated by a blank line. Provide details, using bullet points if required.

### Example

```text
Add users table

* DynamoDB table with user id as primary key
* Email address as secondary index
```
