---
name: pr
description: Create a GitHub pull request for the current branch, and optionally watch CI checks and merge it once green.
---

# Goal

Your job is to create a PR for the current changes and the previous steps.
Optionally the user asks you to watch or merge the PR.
Merge implies watch.

# Workflow

You do not need to ask for approval to run CLI tools or invoke MCP servers listed in the following steps. The user implied the approval. (Note: networked CLI commands such as `git fetch`, `git push`, and `gh pr checks` may require `BypassSandbox: true`.)

## Testing

Check your conversation history to verify you ran the appropriate checks for the changes:
- **Template changes**: Verified via cookiecutter generation and checks inside `../levity-instances/<project_slug>/` (never run build/lint/test commands inside `template/`), and ensure no temporary `test-instance/` directory is staged.
- **CI script changes**: `python3 -m unittest discover -s .github/scripts`.
- **Default placeholder checks**: `scripts/check_defaults.py` if template variables/defaults were touched.

Focus on related checks (CI runs the full suite). If checks fail, analyze and apply minor fixes directly; report required non-trivial changes to the user.

## Committing

Ensure you are on a feature branch (never commit directly to `main`; create and switch to a feature branch first if needed).
Commit any uncommitted changes following `@../../rules/commit-messages.md` (imperative subject line, max 80 chars, no `feat:`/`fix:`/`chore:` prefixes).

## Rebase Check

Fetch the latest `origin/main` and check if the branch has drifted.
If so, rebase the local branch onto `origin/main`.
Resolve minor conflicts directly; report required non-trivial changes to the user.

## Push

Push the branch to remote (`git push -u origin HEAD`, or `git push --force-with-lease` if history was rebased).

## Create PR

Check if an open PR already exists for the current branch using the GitHub MCP server (`list_pull_requests`); if so, reuse it instead of creating a duplicate.
Otherwise, create the PR using GitHub MCP `create_pull_request` with an imperative title (no `feat:`/`fix:`/`chore:` prefix) and a suitable description depending on the PR size.

## Watch (Optional)

If the user mentions `watch` or `merge`:
- Run `gh pr checks --watch --fail-fast` so it stops immediately on the first failure. Let the background command run until completion without polling.
- If `gh pr checks` exits immediately because checks have not registered yet, wait ~10s using `schedule` (never `sleep`) and retry.
- If a check fails, inspect the failure with `gh run view <run-id> --log-failed` and fix it fast; before pushing the fix, run `gh pr checks` to see if any other jobs failed in the meantime so all failures can be addressed together.
- Apply minor fixes directly, commit, push, and watch again. Report required non-trivial changes to the user.

## Merge (Optional)

If the user mentions `merge`, merge the PR using GitHub MCP `merge_pull_request`:
- **`rebase`** (`merge_method: "rebase"`): if it is a single commit or the PR contains commits with isolated topics.
- **`squash`** (`merge_method: "squash"`): otherwise, deriving an imperative `commit_title` (no `feat:`/`fix:`/`chore:` prefix) and `commit_message` from the branch commits.
