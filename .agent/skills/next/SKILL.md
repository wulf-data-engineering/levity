---
name: next
description: Analyze project state against READMEs and ADRs and generate an actionable prompt for the next logical feature or improvement.
---

# Next Step Suggestion

Analyze the gap between the project's vision and its current implementation, then generate an actionable prompt for another agent to implement the next logical step.

## 1. Context Analysis

Read the high-level documentation to understand the vision and architecture:
- `@../../../README.md`: Project vision and features.
- `adr/`: Architectural decisions.
- `@../../../template/@@cookiecutter.project_slug@@/README.md`: Current template documentation.

Explore `template/` to understand the current implementation state.

## 2. Gap Analysis

Compare Vision vs. Reality:
- Are there features in `README.md` not yet in `template/`?
- Are there ADRs not yet reflected in the code?

## 3. Suggestion Selection

- **If invoked with "low hanging fruits"**: Pick small/medium, self-contained tasks.
- **Otherwise**: Pick the next most important features to move the template closer to the `README.md` / ADR vision.

## 4. Output Generation

For each suggestion, explain the **reason** and write a **prompt** into a plain-text artifact (no markdown formatting inside the prompt) for easy copying, following this structure:

- **Title**: `[Task Name]`
- **Reason**: `[Why this is the next step]`
- **Prompt**:
  1. **Analyze & Clarify**: Ask clarifying questions, evaluate new tools/MCP servers, check if `@../../../template/@@cookiecutter.project_slug@@/.agent/skills/onboarding/SKILL.md` needs updates, and read relevant ADRs and READMEs.
  2. **Plan**: Create a detailed implementation plan covering the steps below.
  3. **Execute**: Keep the template repo on `main`, generate a test instance via `/instantiate-template`, and implement + verify changes in the test instance.
  4. **Confirm & Backport**: Show the user the result in the test instance, ask for approval, switch the template repo to a new feature branch, and backport changes preserving `@@ ... @@` placeholders.
