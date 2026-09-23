# ADR‑007 — Antigravity Dependabot Automation Skill

**Status**: Proposed  
**Date**: 2026‑09‑17

---

## 1. Context & Problem Statement

Dependabot regularly creates pull requests to update project dependencies across frontend, backend, protocols, and infrastructure. Managing these PRs manually creates cognitive overhead:
1. **Passing PRs**: Developers must inspect changelogs and diffs to determine if minor or patch updates introduce subtle behavioral shifts, new enum variants that silently fall into catch-all arms, or changes in sensitive areas (like authentication or cloud infrastructure).
2. **Failing PRs**: Upstream breaking changes or compilation regressions require developer intervention to adjust types, call sites, or configurations.

We want an automated mechanism using autonomous Antigravity agents (`agy`) in CI to:
- Automatically assess passing Dependabot PRs for domain safety and enable auto-merge when safe.
- Automatically attempt headless repairs on failing Dependabot PRs within a strict turn budget.

---

## 2. Decision

We provide this capability as an **opt-in skill** (`antigravity-dependabot`) within `template/@@cookiecutter.project_slug@@/.agent/skills/` rather than bundling it into the default template workflows.

### Workflow Behavior

1. **Safety Assessment (`dependabot-assessment.yml`)**:
   - Triggers on successful completion of CI on Dependabot PRs (or manual `workflow_dispatch` / test branch push).
   - Ingests PR description (`pr_info.json`) and diff (`pr_diff.patch`).
   - Uses `agy` with structured JSON output (`safe`, `reasoning`, `considerations`) to evaluate whether breaking changes or new patterns affect code used in the project.
   - Performs a business assessment on catch-all branches (e.g., Rust `_ => ...`).
   - If safe: approves the PR, enables GitHub auto-merge (`--auto --squash`), and posts an assessment comment.
   - If risky: marks the PR with the `requires-human-review` label and outlines specific reviewer considerations.

2. **Automated Repair (`dependabot-autofix.yml`)**:
   - Triggers on failed CI on Dependabot PRs.
   - Selectively provisions toolchains (Node.js, Rust, Protoc) only for the subprojects affected by the PR.
   - Fetches failure logs and runs `filter_ci_log.py` to extract actionable compilation and test errors.
   - Executes `agy` in headless direct execution mode under a strict turn budget (2–4 turns) to apply minimal fixes to code, types, and configs.
   - Verifies changes using lightweight checks (e.g., `npm run lint && npm run check`, `cargo clippy && cargo check`).
   - Commits changes using `summary.txt` and pushes back to the PR branch.

3. **Sequential Rebase Dispatcher (`dependabot-dispatcher.yml` & `dispatch_dependabot.py`)**:
   - Triggers on `push: branches: [main]` (e.g. after a PR merges) and manual `workflow_dispatch`.
   - Uses concurrency grouping (`group: dependabot-dispatcher`, `cancel-in-progress: false`) to process queues sequentially.
   - Finds the oldest open Dependabot PR with auto-merge enabled. If it is `BEHIND` main, rebases it via native `gh pr update-branch <pr_number> --rebase`.
   - Avoids duplicate rebase commands and leaves actively running PRs untouched.
   - Prevents parallel CI storms and merge conflicts across concurrent Dependabot PRs.

### GitHub App Token Integration (`$project-ci-bot`)

When pull requests are merged or pushed using the default `GITHUB_TOKEN`, GitHub Actions intentionally suppresses subsequent `on: push` workflow triggers to prevent recursive execution loops. Consequently:
- Automatic PR merges do not trigger downstream deployment pipelines (`continuous-deployment.yml`).
- Automatic PR merges do not trigger the sequential dispatcher on `main`.

To overcome this, repositories can configure a dedicated GitHub App (`$project-ci-bot`):
- Uses `actions/create-github-app-token@v1` with `APP_ID` and `APP_PRIVATE_KEY` repository secrets.
- Generates ephemeral installation tokens with `Contents: write` and `Pull requests: write` permissions.
- Automatically falls back to `GITHUB_TOKEN` if GitHub App secrets are not configured.

### Loop Prevention

To eliminate runaway agent cascades:
- **Author Guard**: Before executing repairs, the workflow checks if the latest commit was made by `github-actions[bot]`. If so, it immediately exits to prevent infinite repair loops.
- **Deduplication**: Assessment checks whether the PR already has auto-merge enabled or has already been flagged with `requires-human-review`.
- **Branch Rule Exception**: Headless CI is exempted from the standard "create a new branch" rule, applying fixes directly to the checked-out PR branch.

### Token Optimization & Cost Tracking

- **Log Filtering**: `filter_ci_log.py` strips runner boilerplate (timestamps, setup, teardown) and truncates intermediate logs to keep prompts lean.
- **Constrained Prompts**: Agents are instructed not to invoke subagents, browse external sites, or reproduce full changelogs.
- **Usage Transparency**: Both workflows run `calculate_cost.py`, querying the LiteLLM pricing catalog to calculate real estimated running costs based on actual input/output tokens, tagging PRs with compact cost labels (`<$0.01` or `~$X.XXX`) and posting step summaries.
- **Standardized Model**: Both assessment and autofix workflows standardize on `GA_GEMINI_MODEL` (defaulting to `gemini-3.8-flash`).

---

## 3. Tool-Set Support

- `template/@@cookiecutter.project_slug@@/.agent/skills/antigravity-dependabot/`:
  - `SKILL.md`: Instructions for installing and enabling the workflows, scripts, and GitHub App credentials on demand.
  - `assets/workflows/`: `dependabot-assessment.yml`, `dependabot-autofix.yml`, and `dependabot-dispatcher.yml`.
  - `assets/scripts/`: `calculate_cost.py`, `filter_ci_log.py`, `dispatch_dependabot.py`, and their unit test suites.
  - `assets/rules/`: `dependabot-assessment.md` and `dependabot-autofix.md`.

---

## 4. Consequences

### Positive
- Routine dependency bumps are merged automatically with safety validation.
- Breaking changes in dependencies are frequently resolved without human intervention.
- Predictable and transparent costs via token usage reporting.
- Base template remains lean and unopinionated for projects that do not require autonomous CI bots.

### Negative / Trade-offs
- Requires provisioning a `GEMINI_API_KEY` secret in the repository settings.
- Requires GitHub Actions permissions to approve and auto-merge pull requests.

---

## 5. Alternatives Considered

1. **Include workflows directly in the default template**:
   *Rejected*: Not every project developer wants autonomous AI agents running automatically on their repository or wants to configure Gemini API secrets. Offering it as an on-demand skill follows the established pattern of `wasm-integration` and `websockets-integration`.
2. **Run the agent on Gemini Agent Platform instead of GitHub Actions**:
   *Rejected*: The runtime of the agent in headless mode is brief (2–4 turns, typically ~1–3 minutes), which is significantly shorter than the full CI test suite. Running within GitHub Actions keeps compute ephemeral, co-located with the repository checkout, and simple to debug via GitHub artifact uploads.
