---
name: configure-github-repo
description: Configure GitHub repository merge settings, main branch protection, and optional Antigravity Dependabot automation.
---

# Configure GitHub Repository

Guide the user through configuring the GitHub repository settings.

## 1. Review Suggested Settings

Explain the recommended repository settings:
- **Delete branch on merge**: Enabled
- **Squash merging**: Enabled
- **Rebase merging**: Enabled
- **Merge commits**: Disabled (enforces linear history)
- **Auto-merge PRs**: Enabled

## 2. Configure Settings via GitHub CLI

Offer to apply these settings using `gh`:
1. Check `gh --version` and `gh auth status || gh auth login`.
2. Ask the user for confirmation, then run:

```bash
git remote get-url origin
gh repo edit <org/repo> \
  --delete-branch-on-merge \
  --enable-squash-merge \
  --enable-rebase-merge \
  --enable-merge-commit=false \
  --enable-auto-merge
```

## 3. Branch Protection (`main`)

Check existing protections or rulesets on `main` first so you don't overwrite org-wide rules:

```bash
gh ruleset check main
gh api /repos/<org/repo>/branches/main/protection || echo "No classic protection configured"
```

Explain the suggested `main` protection rules:
- **Require pull request before merging** (dismiss stale approvals, require Code Owners review)
- **Require status checks to pass before merging** (require up-to-date branches; add leaf checks from `@../../../.github/workflows/pull-request.yml`)
- **Require linear history**, **no force pushes**, **no deletions**

If the user agrees, apply branch protection:

```bash
gh api --method PUT \
  -H "Accept: application/vnd.github+json" \
  /repos/<org/repo>/branches/main/protection \
  --input - <<< '{
    "required_status_checks": {
      "strict": true,
      "contexts": [
        "Test Backend",
        "Test Frontend",
        "Test Infrastructure",
        "Test end-to-end",
        "Check Dependabot",
        "Check Protocols"
      ]
    },
    "enforce_admins": false,
    "required_pull_request_reviews": {
      "dismiss_stale_reviews": true,
      "require_code_owner_reviews": true,
      "required_approving_review_count": 0
    },
    "restrictions": null,
    "required_linear_history": true,
    "allow_force_pushes": false,
    "allow_deletions": false
  }'
```

## 4. Antigravity Dependabot Automation (Optional)

Ask the user if they want to enable **Antigravity-based Dependabot PR & security handling** (safety assessment + auto-merge, automated CI failure repair, sequential rebase dispatcher, and optional security remediation).

- **If confirmed**:
  1. Apply `@../antigravity-dependabot/SKILL.md`.
  2. Enable **Read and write permissions** and **Allow GitHub Actions to create and approve pull requests** in **Settings > Actions > General > Workflow permissions**.
  3. Set the `GEMINI_API_KEY` secret (`gh secret set GEMINI_API_KEY`).
  4. Walk through creating the optional `@@ cookiecutter.project_slug @@-ci-bot` GitHub App (`APP_ID`, `APP_PRIVATE_KEY`) as documented in `@../antigravity-dependabot/SKILL.md` so auto-merged PRs trigger downstream workflows.
- **If declined**:
  Inform the user they can opt in later by asking to apply `@../antigravity-dependabot/SKILL.md`.
