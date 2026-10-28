# ADR‑014 — Antigravity Automated Security Remediation

**Status**: Proposed (Supercedes [ADR‑007](./007-antigravity-dependabot-automation.md))  
**Date**: 2026‑10‑28

---

## 1. Recap of ADR‑007

[ADR‑007](./007-antigravity-dependabot-automation.md) introduced the opt‑in `antigravity-dependabot` skill to automate the lifecycle of Dependabot pull requests using headless Antigravity (`agy`) agents in GitHub Actions:
1. **Safety Assessment (`dependabot-assessment.yml`)**: Evaluates passing Dependabot PRs for domain safety and catch‑all branch behavior (`_ => ...`), approving and enabling `--auto --squash` when safe or flagging with `requires-human-review`.
2. **Automated Repair (`dependabot-autofix.yml`)**: Repairs failing Dependabot PRs from filtered CI logs (`filter_ci_log.py`) within a strict 2–4 turn budget.
3. **Sequential Rebase Dispatcher (`dependabot-dispatcher.yml` & `dispatch_dependabot.py`)**: Chains and rebases queued auto‑merge PRs one by one (FIFO) using a dedicated GitHub App token (`$project-ci-bot`) to avoid parallel CI storms and `GITHUB_TOKEN` event suppression.
4. **Usage & Cost Transparency (`calculate_cost.py`)**: Tracks token consumption and tags PRs with real LiteLLM cost labels.

---

## 2. Context & Problem Statement

Standard scheduled Dependabot version updates leave significant security gaps:
1. **Transitive Lockfile Vulnerabilities**: Security advisories (CVEs/GHSAs) in transitive dependencies inside `Cargo.lock` or `package-lock.json` often do not trigger a top‑level manifest bump or require `cargo update -p` / `npm` `overrides`.
2. **Complex Dependency Remediations**: Some advisories cannot be resolved by a version bump alone — they require disabling legacy Cargo default features (e.g., `default-features = false` on AWS SDK crates), replacing an unmaintained dependency, or removing an unused package.
3. **Code Scanning (CodeQL) Alerts**: Static analysis findings in application scripts or GitHub Actions workflows (e.g., shell command injection or missing `permissions: contents: read`) require targeted source or workflow edits.
4. **Unpatched Vulnerabilities**: Advisories without an upstream patch (`first_patched_version: null`) require reachability analysis so human maintainers can decide whether to dismiss them as `tolerable_risk` / `not_used` or replace the dependency.

---

## 3. Focused Changes

We extend the `antigravity-dependabot` skill with an **independent, opt‑in fourth workflow**, **`security-remediation.yml`**, orchestrated by **`.github/scripts/remediate_vulnerabilities.py`** and guided by **`.agent/rules/vulnerability-remediation.md`**, and enable it directly on the open‑source `levity` template repository.

Because GitHub Code Scanning (CodeQL) requires either an open‑source (public) repository or a GitHub Advanced Security license on private repositories, `security-remediation.yml` is designed so its setup is **independent** from `dependabot-assessment.yml`, `dependabot-autofix.yml`, and `dependabot-dispatcher.yml`: teams can install the Dependabot PR workflows alone, `security-remediation.yml` alone, or both together. When installed together, the consolidated version‑upgrade PR automatically chains into the Safety Assessment, Auto‑Fix, and Sequential Dispatcher workflows.

### 3.1 Unified Security Remediation Workflow (`security-remediation.yml`)

Triggered on a **daily cron schedule**, manual **`workflow_dispatch`**, and after the **`CodeQL`** workflow completes on `main`:

1. **Alert Ingestion & Deduplication**:
   - Queries open Dependabot security alerts (`/repos/{owner}/{repo}/dependabot/alerts?state=open`) and open CodeQL code‑scanning alerts (`/repos/{owner}/{repo}/code-scanning/alerts?state=open`).
   - Deduplicates against open pull requests (`dependabot/security-version-upgrades` and `security/fix-*`) and the rolling security tracking issue.

2. **Phase 1 — Consolidated Version‑Upgrade PR (`dependabot/security-version-upgrades`)**:
   - Collects all open Dependabot alerts that have a published `first_patched_version`.
   - Applies upgrades on a single branch (`dependabot/security-version-upgrades`) via `cargo update -p` (updating full `major.minor.patch` pins in `Cargo.toml` when direct) and `npm update` (injecting `"overrides"` in `package.json` when pinned transitively).
   - Verifies each upgraded subproject with lightweight checks (`cargo check`, `npm run lint && npm run check`).
   - Any package whose alert remains unresolved in the lockfile or breaks compilation is automatically reverted from the batch branch and promoted to **Phase 2**.
   - Opens or updates **one consolidated PR** (`dependabot/security-version-upgrades`), which automatically flows into `dependabot-assessment.yml`, `dependabot-autofix.yml`, and `dependabot-dispatcher.yml`.

3. **Phase 2 — Grouped Code & Dependency Replacement PRs (`security/fix-<slug>`)**:
   - Groups open CodeQL alerts by `rule.id` and promoted complex Dependabot alerts by `(ecosystem, package_name)`.
   - Prioritizes groups by severity (`critical` > `high` > `medium` > `low`) and caps execution at **at most 3 individual PRs per workflow run** to bound CI runtime and token costs.
   - For each group, checks out `security/fix-<slug>` and runs `agy` headless (2–4 turns) following `.agent/rules/vulnerability-remediation.md` to apply minimal code fixes, workflow permission fixes, Cargo feature flag adjustments, or unused dependency removals.
   - Verifies with subproject checks, opens **one PR per rule/package**, and labels it **`requires-human-review`** alongside the LiteLLM cost label.

4. **Phase 3 — Unpatched Alerts & Rolling Tracking Issue**:
   - Evaluates unpatched Dependabot alerts (`first_patched_version == null`) and any Phase 2 alert that cannot be fixed without human architectural intervention.
   - Uses `agy` with structured JSON output to assess whether the vulnerable package or code path is reachable in production or restricted to dev/build tooling.
   - Does **not** auto‑dismiss alerts; instead, maintains a single rolling GitHub Issue titled **`Security Alerts Requiring Human Decision`** with one section per alert (`<!-- alert-id: ... -->`) containing the reachability assessment and a ready‑to‑run `gh api` dismissal command.

### 3.2 Downstream Pipeline Integration

- `dependabot-assessment.yml`, `dependabot-autofix.yml`, and `dispatch_dependabot.py` are updated to accept PRs on branch `dependabot/security-version-upgrades` authored by the repository CI bot (`github-actions[bot]` or `*-ci-bot[bot]`) in addition to `dependabot[bot]`.
- Individual code/replacement PRs (`security/fix-*`) are always marked `requires-human-review` so they never auto‑merge without human approval.
