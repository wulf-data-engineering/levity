---
name: Antigravity Dependabot
description: Instructions and assets for adding automated Dependabot safety assessment and auto-fix workflows with Antigravity.
---

# Antigravity Dependabot Automation

This skill equips your repository with autonomous Antigravity CI workflows to handle Dependabot pull requests end-to-end:
1. **Safety Assessment (`dependabot-assessment.yml`)**: When a Dependabot PR passes CI checks, the assessment agent evaluates domain safety, breaking changes, and business implications of catch-all branches (`_ => ...`). If safe, it approves and enables auto-merge. If risky, it flags the PR with `requires-human-review` and details specific considerations for human reviewers.
2. **Automated Repair (`dependabot-autofix.yml`)**: When a Dependabot PR fails CI checks, the auto-fix agent inspects filtered failure logs (`filter_ci_log.py`), directly repairs breaking changes in code or configuration within a strict turn budget (2–4 turns), verifies using lightweight checks, and commits fixes with a concise summary.
3. **Sequential Dispatcher (`dependabot-dispatcher.yml`)**: Automatically chains and rebases queued Dependabot PRs with auto-merge enabled one by one (`gh pr update-branch --rebase`), preventing parallel CI storms.
4. **GitHub App Token Integration (`$project-ci-bot`)**: Uses a dedicated GitHub App to allow auto-merged PRs to trigger downstream `on: push` workflows (like deployment and the sequential dispatcher) without `GITHUB_TOKEN` event suppression.
5. **Usage & Cost Tracking**: Calculates token consumption and estimated cost using the LiteLLM pricing catalog via `calculate_cost.py`, applying compact PR cost labels and posting summary comments.

---

## Prerequisites & GitHub Configuration

Before enabling the workflows, configure the following in your GitHub repository settings:

### 1. Repository Secrets & Variables
- **Secret**: `GEMINI_API_KEY` (Required)
  - Navigate to **Settings > Secrets and variables > Actions**.
  - Add repository secret `GEMINI_API_KEY` containing your Google Gemini API key.
- **Variable**: `GA_GEMINI_MODEL` (Optional)
  - Navigate to **Settings > Secrets and variables > Actions > Variables**.
  - Set `GA_GEMINI_MODEL` to override the default model (defaults to `gemini-3.8-flash`).

### 2. GitHub Actions Permissions
- Navigate to **Settings > Actions > General > Workflow permissions**:
  - Select **Read and write permissions**.
  - Check **Allow GitHub Actions to create and approve pull requests**.
  - Click **Save**.

### 3. Repository Auto-Merge
- Navigate to **Settings > General > Pull Requests**:
  - Check **Allow auto-merge**.
  - (Recommended) Ensure **Automatically delete head branches** is enabled.

### 4. GitHub App Setup (`@@ cookiecutter.project_slug @@-ci-bot`) — Recommended

When GitHub Actions uses the default `GITHUB_TOKEN` to auto-merge a pull request, GitHub intentionally suppresses subsequent `on: push` events to prevent recursion loops. Consequently:
- Downstream deployment pipelines (`continuous-deployment.yml`) are not triggered on `main`.
- The sequential dependabot dispatcher (`dependabot-dispatcher.yml`) is not triggered on `main`.

To enable automatic downstream triggering and rebase chaining, configure a dedicated GitHub App:

1. **Create the GitHub App**:
   - Navigate to GitHub **Settings > Developer Settings > GitHub Apps > New GitHub App** (for your user or organization).
   - **GitHub App name**: `@@ cookiecutter.project_slug @@-ci-bot` (or `$project-ci-bot`).
   - **Homepage URL**: Your repository URL (e.g. `https://github.com/<owner>/@@ cookiecutter.project_slug @@`).
   - **Webhook**: Uncheck **Active** (no webhook URL or secret needed).
   - **Permissions**:
     - Under **Repository permissions**:
       - **Contents**: `Read and write` (to update/rebase branches and push commits).
       - **Pull requests**: `Read and write` (to review, approve, comment, and set auto-merge).
       - **Issues**: `Read and write` (to manage comments).
       - **Workflows**: `Read and write` (optional, needed if Dependabot PRs modify workflow files).
       - **Metadata**: `Read-only` (default).
   - **Where can this GitHub App be installed?**: Select **Only on this account**.
   - Click **Create GitHub App**.

2. **Generate Private Key & Note App ID**:
   - On the app's **General** settings page, copy the numeric **App ID**.
   - Scroll down to **Private keys**, click **Generate a private key**, and save the downloaded `.pem` file.

3. **Install the App**:
   - In the left sidebar of the GitHub App settings, click **Install App**.
   - Click **Install** next to the target account or organization.
   - Choose **Only select repositories** and select your project repository. Click **Install**.

4. **Add Secrets to Repository**:
   - Run the following `gh` commands (or add them via **Settings > Secrets and variables > Actions**):
     ```bash
     gh secret set APP_ID --body "<APP_ID>"
     gh secret set APP_PRIVATE_KEY < path/to/private-key.pem
     ```
   *(Note: The workflows gracefully fall back to `GITHUB_TOKEN` if `APP_ID` is not configured).*

---

## Installing Assets

Copy the assets provided by this skill into your repository:

### 1. Workflows
Copy workflow files into `.github/workflows/`:
- `assets/workflows/dependabot-assessment.yml` -> `.github/workflows/dependabot-assessment.yml`
- `assets/workflows/dependabot-autofix.yml` -> `.github/workflows/dependabot-autofix.yml`
- `assets/workflows/dependabot-dispatcher.yml` -> `.github/workflows/dependabot-dispatcher.yml`

### 2. Scripts & Tests
Copy Python helper scripts and their unit tests into `.github/scripts/`:
- `assets/scripts/calculate_cost.py` -> `.github/scripts/calculate_cost.py`
- `assets/scripts/filter_ci_log.py` -> `.github/scripts/filter_ci_log.py`
- `assets/scripts/dispatch_dependabot.py` -> `.github/scripts/dispatch_dependabot.py`
- `assets/scripts/test_calculate_cost.py` -> `.github/scripts/test_calculate_cost.py`
- `assets/scripts/test_filter_ci_log.py` -> `.github/scripts/test_filter_ci_log.py`
- `assets/scripts/test_dispatch_dependabot.py` -> `.github/scripts/test_dispatch_dependabot.py`

### 3. Agent Rules
Copy headless CI rules into `.agent/rules/`:
- `assets/rules/dependabot-assessment.md` -> `.agent/rules/dependabot-assessment.md`
- `assets/rules/dependabot-autofix.md` -> `.agent/rules/dependabot-autofix.md`

---

## Configuration Updates

Update existing project configuration files as follows:

### 1. Update `.agent/rules/changing-files.md`
To permit the headless CI agent to apply fixes directly to checked-out Dependabot PR branches without attempting to branch off, add the following exception to `.agent/rules/changing-files.md`:

```markdown
**Exception**: In automated headless CI environments (such as repairing a Dependabot PR branch), do NOT create a new branch. Apply changes directly to the checked-out PR branch.
```

### 2. Update `.github/workflows/pull-request.yml`
Add a step to the `check-dependabot` job in `.github/workflows/pull-request.yml` to run the CI script unit tests automatically:

```yaml
  check-dependabot:
    name: Check Dependabot
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v7
      - name: Validate dependabot.yml
        uses: marocchino/validate-dependabot@v3
        with:
          path: .github/dependabot.yml
      - name: Run CI scripts tests
        run: |
          python3 -m unittest discover -s .github/scripts
```

### 3. Update `.gitignore`
Add Python cache patterns to `.gitignore`:

```gitignore
# Python cache
__pycache__/
*.pyc
```

---

## Workflow Details

### Safety Assessment Flow
1. Triggers on `workflow_run` of "Continuous Integration" when `conclusion == 'success'` and actor is `dependabot[bot]` (or via `workflow_dispatch` / push to `test-assessment/**`).
2. Checks if PR was already evaluated or auto-merge is already active.
3. Ingests PR info and git diff, running `agy` with low effort and structured JSON schema (`safe`, `reasoning`, `considerations`).
4. If `safe: true`: Approves PR, removes `requires-human-review`, and enables `--auto --squash` merge.
5. If `safe: false`: Adds `requires-human-review` label and posts reviewer considerations comment.
6. Calculates token consumption and costs via LiteLLM, applying a PR cost label (e.g., `<$0.01` or `~$0.025`).

### Auto-Fix Flow
1. Triggers on `workflow_run` of "Continuous Integration" when `conclusion == 'failure'` and actor is `dependabot[bot]` (or via `workflow_dispatch` / push to `test-autofix/**`).
2. Checks commit author: if already committed by `github-actions[bot]`, skips execution to prevent infinite loops.
3. Detects modified subproject (`frontend/`, `infrastructure/`, `backend/`) and installs only required toolchains and dependencies.
4. Downloads failed CI log, filtering out runner boilerplate via `filter_ci_log.py`.
5. Runs `agy` headless under strict 2–4 turn budget to patch code, types, or configuration.
6. Verifies using lightweight checks (e.g. `npm run lint && npm run check`, `cargo clippy && cargo check`).
7. Commits changes with `summary.txt` and pushes to the PR branch.
8. Posts status comment and attaches debug logs/artifacts.

### Sequential Dispatcher Flow
1. Triggers on `push` to `main` (e.g. after a Dependabot PR merges) and manual `workflow_dispatch`.
2. Concurrency group `dependabot-dispatcher` with `cancel-in-progress: false` ensures PRs are rebased sequentially.
3. Generates a GitHub App installation token if `APP_ID` is configured (or falls back to `GITHUB_TOKEN`).
4. Executes `dispatch_dependabot.py`:
   - Evaluates all open Dependabot PRs with auto-merge enabled.
   - Selects the oldest candidate (FIFO).
   - If `BEHIND` main, rebases it using `gh pr update-branch <pr_number> --rebase`.
   - If already in progress, leaves it untouched.

---

## Verification (CRITICAL)

Always verify your implementation:

1. **Verify Python Scripts Locally**:
   Run the test suite in the repository root:
   ```bash
   python3 -m unittest discover -s .github/scripts
   ```
   Ensure all 24 tests pass without errors.

2. **Verify CI Workflow Syntax**:
   Ensure YAML files are valid and GitHub Actions secret/permission prerequisites are fulfilled.

3. **Verify in GitHub Actions (Optional)**:
   You can manually trigger either workflow using GitHub CLI or web UI:
   ```bash
   # Test assessment manually for PR #123
   gh workflow run "Antigravity Dependabot Safety Assessment" -f pr_number=123

   # Test auto-fix manually for PR #123
   gh workflow run "Antigravity Dependabot Auto-Fix" -f pr_number=123
   ```
