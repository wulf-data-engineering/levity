---
name: Template Development
description: Instructions for developing and testing the template itself.
---

# Development

Because this repository is a **cookiecutter template**, you cannot run or test code directly in the `template/` directory.

## 🛑 FORBIDDEN ACTIONS (CRITICAL)

- **NEVER** run `npm run test`, `npm run lint`, `npm run check`, `cargo test`, `cargo check`, or ANY build commands directly inside the `template/` folder. The raw template source contains unexpanded Jinja placeholders (e.g. `%[cookiecutter...]%`) which will permanently break compilers, panicking Rust, or failing Svelte checks.
- ALL verification commands **MUST** be executed strictly within your generated `../levity-instances/<PROJECT_SLUG>` directory BEFORE initiating any backport process.

## CRITICAL: Test-First Workflow

You **MUST** follow this cycle for every feature or fix:

1.  **Instantiate**
    Generate a temporary instance to work in.
    Ref: @../workflows/instantiate-template.md

2.  **Modify Instance**
    Implement changes and run tests in the temporary instance.
    **DO NOT** modify `template/` directly during this phase.
    Ref: @../workflows/develop-feature.md

3.  **Verify**
    Ensure changes work as expected (run tests, check UI) within the instance.

4.  **Confirm**
    Ask the user if the changes are ready to be backported.

5.  **Backport**
    Manually apply the changes from the instance back to the `template/` directory, ensuring placeholders (`%[ ... ]%`) are preserved.
    Ref: @../workflows/backport-changes.md

6.  **CI/CD**
    Update the template's own GitHub workflows if the changes affect the generation process or CI checks.
