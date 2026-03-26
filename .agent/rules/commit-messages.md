---
trigger: always_on
---

# Committing

## User's request

Do not proactively commit.
Wait for user's request to commit.

## Build & Tests work

Make sure that tests work before committing.
Make sure that the build works before committing.

## Separate diff and add+commit

Don't run `git status && ... && git commit ...` together.

First do `git diff` (part of your allow list).
Then run `git add` and `git commit`.

## Commit Message Rules

You tend to create commit messages based on your last action, not the difference to last commit:
Reflect the changes in all changed files since last commit. Not just your last action.
`git diff` helps you to understand it.

Focus on the functional change in the subject line.

Structure commit messages as follows:

- **Subject Line**: Imperative, starting with upper case, max. 80 characters.
  - **CRITICAL**: Do NOT use prefixes like `feat:`, `fix:`, `chore:`.
- **Body**: Separated by a blank line. Provide details, using bullet points if required.

### Example

```text
Add users table

* DynamoDB table with user id as primary key
* Email address as secondary index
```