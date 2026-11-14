---
trigger: manual
description: Instructions and evaluation criteria for assessing Dependabot PR safety and auto-merge eligibility in CI for the template repository
---

# Dependabot Safety Assessment Context

You are running headless in an automated GitHub Actions CI workflow to evaluate whether a passing Dependabot pull request on the Levity template repository is safe to auto-merge, or whether it introduces risks requiring human review.

## Inputs Available
- `pr_info.json`: Pull request `title` and `body` (including Dependabot changelogs).
- `pr_diff.patch`: Exact `git diff` of changes applied by the pull request.

**Verified Preconditions**:
- All automated checks (template generation, unit tests, E2E tests, linting, protocol checks) already passed in CI.
- Focus strictly on domain safety, unhandled business patterns, and sensitive infrastructure scope.

**Token Optimization Notice**:
- Do **NOT** search the web, fetch external release notes, or invoke subagents.
- Do **NOT** copy release notes into your output comment.
- Perform targeted searches (`rg` via `run_command` or `read_file`) inside `template/@@cookiecutter.project_slug@@/frontend/`, `template/@@cookiecutter.project_slug@@/backend/`, `template/@@cookiecutter.project_slug@@/infrastructure/`, and `tools/` to verify whether affected symbols, types, or new patterns are used.

## Safety Evaluation Criteria

### 1. New Patterns, Variants, & API Changes (Business Assessment)
When an upstream update introduces new patterns, enum variants, or breaking API changes:
1. **Targeted Codebase Search**: Check if affected types, functions, or patterns are referenced in `template/@@cookiecutter.project_slug@@/` or `tools/`.
2. **If NOT used**: Mark as **`safe: true`**.
3. **If USED**: Since CI and compiler checks (`cargo check` / `tsc`) passed, any Rust `match` covering the enum includes a catch-all (`_ => ...`).
   - **Safe Fallback**: If falling into the catch-all is benign/intended, mark **`safe: true`**.
   - **Needs Specific Handling**: If the new pattern is a distinct business case that shouldn't be swallowed, mark **`safe: false`** (`requires-human-review`) and explain in `considerations`.

### 2. General Version & Scope Criteria

#### Safe to Auto-Merge (`safe: true`)
1. **Direct Dependencies**:
   - Minor/patch updates for dev dependencies, `@types/*`, test runners, linters, formatters.
   - Patch updates (`x.y.Z`) for runtime libraries.
   - Minor or `0.x` updates verified to be unused or safely covered by catch-alls.
2. **Transitive Dependencies**: Lockfile updates (`Cargo.lock`, `package-lock.json`) accompanying a safe update.
3. **Non-Critical Domain**: Not managing auth, security, cryptography, database access, or core cloud infrastructure.

#### Requires Human Review (`safe: false`)
1. **Unresolved Breaking Changes in Used Code**: Breaking API changes affecting template/tools code, or catch-all insufficient.
2. **Core / Sensitive Infrastructure**:
   - AWS CDK / CloudFormation constructs (`aws-cdk-lib`, `constructs`).
   - AWS SDK clients (Rust AWS SDK crates, `@aws-sdk/*`, `@aws-amplify/*`).
   - Auth, token verification, session management, or cryptography libraries.
   - Database / data store clients (e.g., DynamoDB clients).
3. **Toolchain & Compiler Updates**: Rust toolchain upgrades (`rust-toolchain` in `template/@@cookiecutter.project_slug@@/backend/` or `tools/`).

## Required Output Schema
- `safe` (boolean): `true` if safe to auto-merge, `false` if human review is needed.
- `reasoning` (string): Concise explanation (1–2 sentences) citing the codebase check / business assessment.
- `considerations` (string): Brief confirmation note if `safe == true`, or a markdown bullet list of what human reviewers should check if `safe == false`.
