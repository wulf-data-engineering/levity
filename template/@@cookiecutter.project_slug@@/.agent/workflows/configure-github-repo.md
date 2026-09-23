---
description: Configure GitHub repository settings (Issues, Wiki, Merge strategies, etc.)
---

# Configure GitHub Repository

Guide the user through the steps you as an agent together with the user have to do to configure the GitHub repository settings.

// turbo-all

## Review Suggested Settings

Explain to the user the following suggested settings for the repository:

- **Delete branch on merge**: Enabled (Keeps the repository clean by automatically deleting feature branches after they are merged)
- **Squash merging**: Enabled (Maintains a clean, linear commit history by squashing all commits into one upon merge)
- **Rebase merging**: Enabled (Provides flexibility if a linear commit history without a merge commit is desired)
- **Merge commits**: Disabled (Forces squash or rebase merging to maintain linear history)
- **Auto merge PRs**: Enabled (Convenient and safe with the other configuration)

## Configure Settings using GitHub CLI

Offer to the user to configure these repository settings using the `gh` CLI.

1.  Check if `gh` is installed (`gh --version`).
2.  If installed, ask the user if they want you to set them automatically.
3.  If yes, run:

    ```bash
    #  Find out <org/repo>
    git remote get-url origin
    # Login check
    gh auth status || gh auth login
    # Configure repository settings
    gh repo edit <org/repo> \
      --delete-branch-on-merge \
      --enable-squash-merge \
      --enable-rebase-merge \
      --enable-merge-commit=false \
      --enable-auto-merge

    ```

## Branch Protection (main)

First, check if there are existing protections or rulesets applied to the `main` branch. This ensures you do not overwrite existing organization-wide or manual rules unintentionally:

```bash
# Check if branch rulesets apply
gh ruleset check main

# Check classic branch protection
gh api /repos/<org/repo>/branches/main/protection || echo "No classic protection configured"
```

Explain the following suggested branch protection rules for the `main` branch to ensure code quality and linear history:

- **Require a pull request before merging**: Enabled
  - **Require review from Code Owners**: Enabled (no need to check for CODEOWNERS file)
  - **Dismiss stale pull request approvals when new commits are pushed**: Enabled
- **Require status checks to pass before merging**: Enabled
  - **Require branches to be up to date before merging**: Enabled
  - **Status checks**: Add the leaf checks (no other check depends on them) from @../../.github/workflows/pull-request.yml
- **Require linear history**: Enabled
- **Allow force pushes**: Disabled
- **Allow deletions**: Disabled

If there are differences, suggest to the user to configure these via `gh api`:

4.  Set branch protection for `main`:

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

## Antigravity Dependabot Automation (Optional)

Ask the user if they want to add **Antigravity-based Dependabot PR handling**:
- **Successful Dependabot PRs**: Evaluates domain safety and auto-merges safe updates directly.
- **Failed Dependabot PRs**: Headless Antigravity agent attempts to automatically fix breaking changes and compile errors.
- **Sequential Rebase Dispatcher**: Automatically rebase-chains queued Dependabot PRs one at a time to prevent parallel CI storms.

### User Decision

1. **If the user confirms**:
   - Continue with the skill @../skills/antigravity-dependabot/SKILL.md to install the workflows, scripts, rules, and configuration updates.
   - Configure required GitHub Actions permissions:
     - Ensure **Read and write permissions** are granted under **Settings > Actions > General > Workflow permissions**.
     - Ensure **Allow GitHub Actions to create and approve pull requests** is enabled.
   - **CRITICAL**: Request the user to add the `GEMINI_API_KEY` repository secret before merging the branch into `main`:
     ```bash
     gh secret set GEMINI_API_KEY
     ```
     Or manually in GitHub under **Settings > Secrets and variables > Actions**. Explain that the workflows require this key to execute the Antigravity agent.
   - **GitHub App Setup (`@@ cookiecutter.project_slug @@-ci-bot`) (Recommended for Auto-Merge & Rebase Chaining)**:
     When GitHub Actions merges a PR using default `GITHUB_TOKEN`, GitHub suppresses downstream `on: push` workflows (like deployment and the sequential dispatcher). Setting up a dedicated GitHub App allows auto-merges to trigger downstream workflows:
     1. Walk the user through creating a GitHub App named `@@ cookiecutter.project_slug @@-ci-bot` (or `$project-ci-bot`) under **Settings > Developer Settings > GitHub Apps** with:
        - Webhook: Inactive.
        - Repository Permissions: `Contents: Read and write`, `Pull requests: Read and write`, `Issues: Read and write`.
     2. Install the app on the repository under **Install App** -> **Only select repositories**.
     3. Generate a private key (`.pem`) and store secrets in GitHub:
        ```bash
        gh secret set APP_ID --body "<APP_ID>"
        gh secret set APP_PRIVATE_KEY < path/to/private-key.pem
        ```
     *(Detailed step-by-step guidance is documented in @../skills/antigravity-dependabot/SKILL.md).*

2. **If the user declines**:
   - Inform the user that they can opt-in at any time later by asking the agent to apply the @../skills/antigravity-dependabot/SKILL.md skill.