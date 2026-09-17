---
trigger: model_decision
description: Instructions and constraints for headless Dependabot PR auto-fixes in CI
---

# Headless Dependabot Auto-Fix Context

You are running headless in an automated GitHub Actions CI workflow to repair breaking changes in a failed Dependabot pull request.

**CRITICAL - DIRECT EXECUTION MODE**:
- You are running in automated headless execution mode.
- Do NOT enter planning mode, do NOT create an implementation plan artifact (`implementation_plan.md`), and do NOT wait for user review or approval.
- Immediately inspect `ci_failure.log`, edit the source code / test files / configuration files directly in the repository workspace, verify the changes with lightweight checks, and write `summary.txt`.

## Execution Constraints
- **Strict Turn Budget (2–4 Turns Maximum)**:
  - Do NOT view files and make edits one file at a time across many separate turns.
  - Turn 1: Read the errors in `ci_failure.log`, inspect the affected code, and apply all necessary file modifications in batch.
  - Turn 2: Verify with the allowed lightweight check (e.g. `npm run lint && npm run check && npm run test:unit`) and write `summary.txt`.
- **Automatic Code Formatting**: Code formatting (Prettier / cargo fmt) is automatically handled by the pre-commit hook during commit. Do not waste turns running standalone formatting commands unless a check specifically failed on formatting.
- **Strict Typing & Linter Compliance**:
  - Always ensure `npm run lint` / `cargo clippy` passes.
  - Never introduce `any` types in TypeScript (use `unknown`, proper generics, or specific types).
  - **Type Augmentations / Declaration Merging (`.d.ts`)**: If a module augmentation requires empty interfaces extending third-party matchers (such as augmenting Vitest's `Assertion` or `AsymmetricMatchersContaining` with `@testing-library/jest-dom`), add explicit file-level disable comments:
    ```typescript
    /* eslint-disable @typescript-eslint/no-empty-object-type, @typescript-eslint/no-explicit-any */
    import 'vitest';
    import type { TestingLibraryMatchers } from '@testing-library/jest-dom/matchers';

    declare module 'vitest' {
    	interface Assertion<R = void, T = any> extends TestingLibraryMatchers<T, R> {}
    	interface AsymmetricMatchersContaining extends TestingLibraryMatchers<any, any> {}
    }
    ```
  - **Test Hoisting & Mock Placement**: In modern test frameworks like Vitest 5+, `vi.mock(...)` calls must be declared at the module's top level, outside `describe()` blocks or functions.
  - Avoid unused variables or type parameters that trigger linter warnings.
- **No Subagents or Background Tasks**: Do NOT spawn subagents or background tasks (`invoke_subagent`, `schedule`, `manage_task`). Perform all work directly in your main agent loop.
- **Pre-installed Toolchains**: Node.js, Rust, Protoc, and the relevant project dependencies are already installed.
- **No Reinstallations**: Do NOT execute `npm install`, full `cargo build`, or browser installations (`npx playwright install`).
- **Forbidden Operations**:
  - Do NOT run Docker containers, compose stacks, or LocalStack.
  - Do NOT run End-to-End or Playwright tests (`npm run test:e2e` / `npx playwright test`).
  - Do NOT create a new git branch; operate directly on the current checked-out branch.

## Allowed Lightweight Verification Commands
Run only lightweight checks corresponding to the modified subproject:
- **Frontend** (run in `frontend/`):
  - `npm run lint` (Linter)
  - `npm run check` (TypeScript type check)
  - `npm run test:unit` (Unit tests)
- **Infrastructure** (run in `infrastructure/`):
  - `npm run lint`
  - `npm test`
- **Backend** (run in `backend/`):
  - `cargo clippy`
  - `cargo check`
  - `cargo nextest run --lib` or `cargo test --lib`

## Pattern & Business Logic Adaptation
- When a compile error or test failure is caused by an updated API, changed function signature, or new enum variant:
  - Identify the specific call sites in our codebase from `ci_failure.log`.
  - Apply the minimal correct update matching the new library interface.
  - If adapting a new enum variant in a match statement:
    - If the match statement already has a catch-all (`_ => ...`), verify whether falling into the catch-all is benign or if the business logic requires explicit handling.
    - If exhaustiveness failed (no catch-all), add the required arm following the existing pattern of the match statement.

## Workflow Procedure
1. **Analyze Failure & Apply Fixes in Batch**: Read `ci_failure.log` to pinpoint what broke and apply minimal fixes to all affected code, types, or configuration files in one pass.
2. **Verify**: Run the corresponding lightweight verification command (e.g. `npm run lint && npm run check && npm run test:unit`).
3. **Document Fixes**: Write a concise, bullet-pointed explanation of what was fixed into `summary.txt` in the repository root.
