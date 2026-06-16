---
description: Upgrade dependencies by grouping and rebasing Dependabot branches and running tests
---

# Upgrades Workflow

This workflow automates fetching main, grouping, rebasing open Dependabot PRs, verifying changes, and creating a unified upgrades branch.

## 1. Synchronization & Branch Setup

- Ensure the git working directory is clean. If there are uncommitted changes, ask the developer to stash or commit them before starting.
- Switch to the main branch and pull the latest changes from origin:
  ```bash
  // turbo
  git checkout main
  git pull origin main
  ```
- Fetch all remote branches (specifically including all dependabot branches):
  ```bash
  // turbo
  git fetch --all --prune
  ```
- Get the current date in `YYYYMMDD` format (e.g., `20260614`).
- Create and checkout a new upgrades branch from `main`:
  ```bash
  // turbo
  git checkout -b upgrades-YYYYMMDD
  ```
  *(Replace `YYYYMMDD` with the actual date).*

## 2. List Dependabot Pull Requests

- Retrieve the list of all open pull requests created by Dependabot using the GitHub MCP tool `list_pull_requests` (or search for open PRs with author `app/dependabot`).
- Inspect the CI/CD checks status for each Dependabot PR to see which ones have succeeded (green) and which ones have failed/pending checks.
- Display a clear summary list of the identified Dependabot PRs, their branches, status, and which ones will be selected by default (succeeded only).

## 3. Rebase Dependabot Branches

- **Default Selection**: If the user ran `/upgrade` without arguments, select **only** those Dependabot PRs whose checks succeeded.
- **All Selection**: If the user ran `/upgrade all` (or mentioned "failing", "broken", or similar), select **all** open Dependabot PRs.
- For each selected Dependabot PR:
  - Locate the corresponding remote branch (usually starts with `origin/dependabot/...`).
  - Rebase it onto the current `upgrades-YYYYMMDD` branch:
    ```bash
    // turbo
    git rebase origin/dependabot/branch-name
    ```
  - **Conflict Resolution**: If a rebase conflict occurs:
    - If the conflict is in lockfiles (e.g., `Cargo.lock`, `package-lock.json`), run `cargo check` / `npm install` to regenerate lockfiles, resolve conflicts, and continue the rebase:
      ```bash
      // turbo
      git add <conflict-files>
      git rebase --continue
      ```
    - If there are non-trivial conflicts, try to resolve them automatically if possible, or report the conflict details and ask the developer how to proceed.

## 4. Verify & Build

Once all selected PRs are rebased onto the `upgrades-YYYYMMDD` branch, verify the entire monorepo:

- **Backend Check**: Run the verification and final checks documented in the [Backend Development Skill](@../skills/backend/SKILL.md).
- **Frontend Check**: Run the verification and final checks documented in the [Frontend Development Skill](@../skills/frontend/SKILL.md).
- **E2E Tests**: Run the full end-to-end tests as documented in the [End-to-End Testing Workflow](@test-e2e.md).

- **Fix Problems**: If any check, compilation, or test fails due to breaking changes or dependency mismatch, attempt to fix the compatibility issues in the codebase.

## 5. Rebase again

Fetch latest main and rebase onto origin/main

## 6. Commit, Push, and PR

- If any code changes or fixes were applied to make the upgrades compile or pass tests, commit those fixes to the branch:
  ```bash
  // turbo
  git commit -am "fix: resolve dependency upgrade compatibility issues"
  ```
- Ask the developer for permission to push the upgrades branch to origin and create the unified upgrades PR on GitHub:
  - If approved, push the branch:
    ```bash
    // turbo
    git push origin upgrades-YYYYMMDD
    ```
  - Create the pull request using the GitHub MCP tool `create_pull_request`.