---
name: upgrade
description: Batch, rebase, test, and combine open Dependabot pull requests into a unified upgrades branch and PR.
---

# Upgrades

Batch, rebase, verify, and combine open Dependabot PRs into a unified `upgrades-YYYYMMDD` branch.

## 1. Synchronization & Branch Setup

- Ensure the git working directory is clean (ask the developer to stash or commit any uncommitted changes first).
- Update `main` and fetch all remote branches:
  ```bash
  git checkout main
  git pull origin main
  git fetch --all --prune
  ```
- Create a dated upgrades branch (`YYYYMMDD`, e.g., `upgrades-20260614`):
  ```bash
  git checkout -b upgrades-YYYYMMDD
  ```

## 2. List Dependabot Pull Requests

- List open PRs created by Dependabot using GitHub MCP `list_pull_requests` (author `app/dependabot`).
- Inspect CI check status for each PR and display a clear summary of branches and status.

## 3. Rebase Dependabot Branches

- **Default**: If invoked without arguments, select **only** Dependabot PRs whose checks succeeded.
- **All**: If invoked with `all` (or the user asks for failing/broken PRs too), select **all** open Dependabot PRs.
- For each selected PR, rebase onto `upgrades-YYYYMMDD`:
  ```bash
  git rebase origin/dependabot/<branch-name>
  ```
- **Conflict Resolution**:
  - For lockfile conflicts (`Cargo.lock`, `package-lock.json`), regenerate with `cargo check` / `npm install`, then `git add <conflict-files> && git rebase --continue`.
  - For non-trivial conflicts, resolve if clear or ask the developer how to proceed.

## 4. Verify & Build

Verify the entire monorepo:
- **Backend**: Follow `@../backend/SKILL.md`.
- **Frontend**: Follow `@../frontend/SKILL.md`.
- **E2E Tests**: Follow `@../test-e2e/SKILL.md`.

Fix any compilation or test failures caused by breaking changes or dependency mismatches.

## 5. Rebase Again

Fetch the latest `main` and rebase onto `origin/main`.

## 6. Commit, Push, and PR

- If compatibility fixes were made, commit them.
- Ask the developer for permission before pushing `upgrades-YYYYMMDD` and opening the unified PR via GitHub MCP `create_pull_request`.
