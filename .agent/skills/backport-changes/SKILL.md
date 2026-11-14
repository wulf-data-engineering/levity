---
name: backport-changes
description: Backport verified changes from a template instance back into template/@@cookiecutter.project_slug@@/ while restoring Jinja2 placeholders.
---

# Backport Changes

**CRITICAL**: Copy your working code from `../levity-instances/<PROJECT_SLUG>` back to `template/@@cookiecutter.project_slug@@/`, **restoring all Jinja2 placeholders (`@@ ... @@`)**.

## Steps

1. **Verify Instance State**:
   Ensure you have fully verified your changes inside `../levity-instances/<PROJECT_SLUG>`:
   - Frontend: `npm run check && npm run lint && npm run test`
   - Backend: `cargo check && cargo clippy && cargo nextest run`
   **CRITICAL**: NEVER run these verification commands directly in `template/`. Unresolved placeholders will cause panics and false negatives.

2. **Check Branch**:
   Follow `@../../rules/changing-files.md` in the template repository.

3. **Ask for User Approval (CRITICAL)**:
   State that the feature is complete and all tests, linters, and formatters passed. Explicitly ask the user if they want to proceed with the backport. Wait for confirmation. *(If the user asks for further changes, switch back to `@../develop-feature/SKILL.md`.)*

4. **Identify Changed Files**:
   List modified files in `../levity-instances/<PROJECT_SLUG>`:
   ```bash
   git status --porcelain
   ```

5. **Compare & Apply Changes Carefully**:
   Map each file from `../levity-instances/<PROJECT_SLUG>/<path>` to `template/@@cookiecutter.project_slug@@/<path>`.
   **DO NOT** blindly overwrite template files that contain placeholders like `@@ cookiecutter.project_slug @@`.
   1. Read the template file and the modified instance file.
   2. Apply logic changes to the template file while preserving/restoring Jinja2 tags (`@@ ... @@`), resolving any `TODO` comments left during `/develop-feature`.
   3. Check `@../../../template/cookiecutter.json` if you introduced new files that need placeholders.
   4. Backport updated lockfiles (`Cargo.lock`, `package-lock.json`) if dependencies changed.
   5. Check `git status` in both repos to ensure nothing was missed.

6. **Verify Template Integrity**:
   Run `/instantiate-template` (`@../instantiate-template/SKILL.md`) to a *new* location (e.g., `../levity-instances/<VERIFICATION_SLUG>`) and verify:
   - Cookiecutter runs without error.
   - The new instance contains your changes with all placeholders cleanly substituted (no stray `@@ ... @@`).

7. **Cleanup**:
   Always suggest removing the temporary instances (even if the user asks to proceed with a commit):
   ```bash
   rm -rf ../levity-instances/<PROJECT_SLUG>
   ```

8. **Ready**:
   You can now suggest a commit and PR.
