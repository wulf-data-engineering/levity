---
trigger: always_on
---

# Commit Message Rules

Structure commit messages as follows:

- **Subject Line**: Imperative, starting with upper case, max. 80 characters.
  - **CRITICAL**: Do NOT use prefixes like `feat:`, `fix:`, `chore:`.
- **Body**: Separated by a blank line. Provide details, using bullet points if required.

## Example

```text
Add users table

* DynamoDB table with user id as primary key
* Email address as secondary index
```
