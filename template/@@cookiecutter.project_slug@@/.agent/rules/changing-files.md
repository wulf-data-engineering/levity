---
trigger: always_on
---

Never edit repository files on `main` (enforced by `.agent/hooks.json` `PreToolUse` hook).

- If the repository is on `main`, stop and ask the user to switch to (or create) a feature branch.
- If the repository is on another branch you did not create, confirm with the user—it may belong to another agent.
- **Exception**: In automated headless CI environments (`CI=true`, e.g., repairing a Dependabot PR branch), apply changes directly to the checked-out PR branch without creating a new branch.
