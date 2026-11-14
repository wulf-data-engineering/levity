---
name: template-development
description: Instructions for developing and testing the template itself.
---

# Development

Because this repository is a **cookiecutter template**, you cannot run or test code directly in the `template/` directory.

## FORBIDDEN ACTIONS (CRITICAL)

- **NEVER** run `npm run test`, `npm run lint`, `npm run check`, `cargo test`, `cargo check`, or ANY build commands directly inside `template/`. Unexpanded Jinja placeholders (`@@ cookiecutter... @@`) will break compilers and checks.
- ALL verification commands **MUST** be executed inside `../levity-instances/<PROJECT_SLUG>` before backporting.

## Test-First Workflow

Follow this cycle for every feature or fix:

1. **Instantiate**: Generate a temporary instance (`/instantiate-template`, `@../instantiate-template/SKILL.md`).
2. **Modify Instance**: Implement changes and run tests in the instance. Do **not** modify `template/` directly during this phase (`/develop-feature`, `@../develop-feature/SKILL.md`).
3. **Verify**: Ensure tests and UI checks pass within the instance.
4. **Confirm**: Ask the user if the changes are ready to be backported.
5. **Backport**: Apply changes from the instance back to `template/` while preserving `@@ ... @@` placeholders (`/backport-changes`, `@../backport-changes/SKILL.md`).
6. **CI/CD**: Update `.github/workflows/` if the changes affect template generation or CI checks.
