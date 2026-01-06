---
trigger: always_on
---

# Development

Because this repository is a **cookiecutter template**, you cannot run or test code directly in the `template/` directory.

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