---
trigger: manual
description: Instructions and constraints for headless Dependabot PR auto-fixes in CI
---

# Headless Dependabot Auto-Fix Context

You are running headless in an automated GitHub Actions CI workflow to repair breaking changes in a failed Dependabot pull request.

**CRITICAL - DIRECT EXECUTION MODE**:
- Run directly without creating a plan or waiting for user approval.
- Immediately inspect `ci_failure.log`, edit source/test/config files directly in the workspace, verify with lightweight checks, and write `summary.txt`.

## Execution Constraints
- **Strict Turn Budget (2–4 Turns Maximum)**:
  - Turn 1: Read errors in `ci_failure.log`, inspect affected code, and apply all necessary edits in batch.
  - Turn 2: Verify with lightweight checks and write `summary.txt`.
- **Automatic Code Formatting**: Pre-commit handles formatting; do not run standalone formatters unless a check failed on formatting.
- **Strict Typing & Linter Compliance**:
  - Ensure `npm run lint` / `cargo clippy` passes.
  - Never introduce `any` in TypeScript (use `unknown`, proper generics, or specific types).
  - **Type Augmentations (`.d.ts`)**: If extending third-party matchers requires empty interfaces, add file-level disable comments:
    ```typescript
    /* eslint-disable @typescript-eslint/no-empty-object-type, @typescript-eslint/no-explicit-any */
    import 'vitest';
    import type { TestingLibraryMatchers } from '@testing-library/jest-dom/matchers';

    declare module 'vitest' {
    	interface Assertion<R = void, T = any> extends TestingLibraryMatchers<T, R> {}
    	interface AsymmetricMatchersContaining extends TestingLibraryMatchers<any, any> {}
    }
    ```
  - **Test Hoisting**: In Vitest 5+, `vi.mock(...)` must be at module top level, outside `describe()` or functions.
  - Avoid unused variables or type parameters.
- **No Subagents or Background Tasks**: Do NOT use `invoke_subagent`, `schedule`, or `manage_task`.
- **Pre-installed Toolchains & No Reinstallations**: Do NOT run `npm install`, full `cargo build`, or `npx playwright install`.
- **Forbidden Operations**: Do NOT run Docker/LocalStack, Playwright/E2E tests, or create a new git branch.

## Allowed Lightweight Verification Commands
Run only checks for the modified subproject:
- **Frontend** (`frontend/`): `npm run lint`, `npm run check`, `npm run test:unit`
- **Infrastructure** (`infrastructure/`): `npm run lint`, `npm test`
- **Backend** (`backend/`): `cargo clippy`, `cargo check`, `cargo nextest run --lib` (or `cargo test --lib`)

## Pattern & Business Logic Adaptation
- Identify failing call sites from `ci_failure.log` and apply the minimal update matching the new library interface.
- For new enum variants in `match` statements: add the required arm if exhaustiveness failed, or verify whether an existing catch-all (`_ => ...`) is benign.

## Workflow Procedure
1. Read `ci_failure.log` and apply minimal fixes in batch.
2. Run the corresponding lightweight verification commands.
3. Write a concise, bullet-pointed summary of fixes into `summary.txt` in the repository root.
