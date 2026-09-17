---
name: Antigravity Dependabot
description: Instructions and assets for adding automated Dependabot safety assessment and auto-fix workflows with Antigravity.
---

# Antigravity Dependabot Automation

This skill equips your repository with autonomous Antigravity CI workflows to handle Dependabot pull requests end-to-end:
1. **Safety Assessment (`dependabot-assessment.yml`)**: When a Dependabot PR passes CI checks, the assessment agent evaluates domain safety, breaking changes, and business implications of catch-all branches (`_ => ...`). If safe, it approves and enables auto-merge. If risky, it flags the PR with `requires-human-review` and details specific considerations for human reviewers.
2. **Automated Repair (`dependabot-autofix.yml`)**: When a Dependabot PR fails CI checks, the auto-fix agent inspects filtered failure logs (`filter_ci_log.py`), directly repairs breaking changes in code or configuration within a strict turn budget (2–4 turns), verifies using lightweight checks, and commits fixes with a concise summary.
3. **Usage & Cost Tracking**: Calculates token consumption and estimated cost using the LiteLLM pricing catalog via `calculate_cost.py`, applying compact PR cost labels and posting summary comments.

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

---

## Installing Assets

Copy the assets provided by this skill into your repository:

### 1. Workflows
Copy workflow files into `.github/workflows/`:
- `assets/workflows/dependabot-assessment.yml` -> `.github/workflows/dependabot-assessment.yml`
- `assets/workflows/dependabot-autofix.yml` -> `.github/workflows/dependabot-autofix.yml`

### 2. Scripts & Tests
Copy Python helper scripts and their unit tests into `.github/scripts/`:
- `assets/scripts/calculate_cost.py` -> `.github/scripts/calculate_cost.py`
- `assets/scripts/filter_ci_log.py` -> `.github/scripts/filter_ci_log.py`
- `assets/scripts/test_calculate_cost.py` -> `.github/scripts/test_calculate_cost.py`
- `assets/scripts/test_filter_ci_log.py` -> `.github/scripts/test_filter_ci_log.py`

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

---

## Verification (CRITICAL)

Always verify your implementation:

1. **Verify Python Scripts Locally**:
   Run the test suite in the repository root:
   ```bash
   python3 -m unittest discover -s .github/scripts
   ```
   Ensure all 13 tests pass without errors.

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
