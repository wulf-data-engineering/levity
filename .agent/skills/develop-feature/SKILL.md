---
name: develop-feature
description: Develop and test features in a generated template instance before backporting to the template.
---

# Develop Feature

This skill guides you through developing a feature in a generated instance before backporting it into the template.

**CRITICAL**: You must develop and verify features in the **instance** first. Do not modify `template/` directly until verification is complete.

## Prerequisites

- Run `/instantiate-template` (`@../instantiate-template/SKILL.md`) first.
- Working directory for this workflow is `../levity-instances/<PROJECT_SLUG>`.
- Ensure the template repo is on a clean `main` branch (or your active feature branch).

## Persistent Reminder: Template Development Mode

You are working in a **temporary instance** (`../levity-instances/<PROJECT_SLUG>`).

**CRITICAL: DUAL CONTEXT**

1. **Repo Rules**: You are **still bound** by the `levity` repo rules (`@../../rules/global.md`).
2. **App Context**: You are acting as a developer of the *generated application* (`@../../../template/@@cookiecutter.project_slug@@/.agent/rules/global.md`).
3. **Resolved Template Values**: All variables from `@../../../template/cookiecutter.json` are resolved to concrete values in the test instance. If you add one of those values during implementation, mark it with a comment for later backporting, e.g.:

   ```typescript
   configure('eu-central-1'); /* TODO: @@ cookiecutter.default_region @@ */
   ```

   ```html
   <h1>Welcome to Tool-Set Project!</h1> <!-- TODO: @@ cookiecutter.project_name @@ //-->
   ```

## Steps

1. **Install Dependencies** (inside `../levity-instances/<PROJECT_SLUG>`):
   - Frontend: `cd frontend && npm install`
   - Backend: `cd backend && cargo fetch`
   - Infrastructure: `cd infrastructure && npm install`

2. **Implement and Verify Feature**:
   - Make code changes in `../levity-instances/<PROJECT_SLUG>/...`.
   - Use the available MCP servers.
   - Run unit tests and end-to-end tests (`@../../../template/@@cookiecutter.project_slug@@/.agent/skills/run-locally/SKILL.md` and `@../../../template/@@cookiecutter.project_slug@@/.agent/skills/test-e2e/SKILL.md`).
   - Update `.agent` rules or skills if the feature changes project conventions.

3. **Run Linters and Formatters**:
   - Run the verification checks from `@../../../template/@@cookiecutter.project_slug@@/.agent/skills/backend/SKILL.md` and `@../../../template/@@cookiecutter.project_slug@@/.agent/skills/frontend/SKILL.md`.

4. **Prepare for Backport**:
   Once the feature works in the instance, proceed to `@../backport-changes/SKILL.md` (`/backport-changes`) or `@../derive-skill/SKILL.md` (`/derive-skill`).
