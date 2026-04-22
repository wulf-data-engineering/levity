---
description: Analyze project state and suggest the next feature or improvement as an actionable agent prompt.
---

# Workflow: Next Step Suggestion

This workflow analyzes the gap between the project's vision and its current implementation then generates a prompt for another agent to implement the next logical step.

## 1. Context Analysis

First, read the high-level documentation to understand the vision and architecture:

- @../../README.md: The project vision and features.
- `adr/`: The architectural decisions.
- @../../template/@@cookiecutter.project_slug@@/README.md: The current template documentation (if different).

Then, explore the `template/` directory to understand the current implementation state.

## 2. Gap Analysis

Compare the Vision vs. Reality.

- Are there features in the README not yet in the Template?
- Are there ADRs not reflected in the code?

## 3. Suggestion Selection

**If invoked with "low hanging fruits"**: Pick some small/medium, self-contained tasks.

**Otherwise**: Pick the next most important features to move the template closer to the README/ADR vision.

## 4. Output Generation

For each suggestion explain the **reason** and create a **prompt** that can be pasted to a new agent instance to execute this step. It's important to put the prompts into text artifacts (no markdown) for easy copying.

**CRITICAL: The generated prompt MUST follow this exact structure to ensure safety and consistency:**

### Prompt Template

**Title**: [Task Name]

**Reason**: [Why this is the next step]

**Prompt**:
You are tasked with [Task Description].

1.  **Analyze & Clarify (IMPORTANT)**:
    -   **Clarify**: Ask clarifying questions.
    -   **New Technology**: Evaluate if new tools/tech are introduced. Research and suggest MCP servers. Check if `@../../template/@@cookiecutter.project_slug@@/.agent/workflows/onboarding.md` needs updates.
    -   **Context**: Read relevant ADRs and READMEs.
2.  **Plan**:
    -   Create a detailed implementation plan.
    -   Include the following parts as tasks in your implementation plan.
3.  **Execute**:
    -   **Safe Workspace**: Keep the template repository on `main`.
    -   **Test Instance**: ALWAYS work in a test instance first using `/instantiate-template`.
    -   Implement and verify changes in the test instance.
4.  **Confirm & Backport**:
    -   **Approval**: Show the user the result in the test instance and ask for approval.
    -   **Clean State**: After approval, verify the template repo is on a clean `main` branch.
    -   **Branching**: Checkout a new feature branch for the template.
    -   **Backport**: Apply the changes to the template, preserving cookiecutter variables (`@@ ... @@`).
