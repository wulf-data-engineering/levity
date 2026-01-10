---
description: Analyze project state and suggest the next feature or improvement as an actionable agent prompt.
---

# Workflow: Next Step Suggestion

This workflow analyzes the gap between the project's vision and its current implementation then generates a prompt for another agent to implement the next logical step.

## 1. Context Analysis

First, read the high-level documentation to understand the vision and architecture:

- @../../README.md: The project vision and features.
- `adr/`: The architectural decisions.
- @../../template/%[cookiecutter.project_slug]%/README.md: The current template documentation (if different).

Then, explore the `template/` directory to understand the current implementation state.

## 2. Gap Analysis

Compare the Vision vs. Reality.

- Are there features in the README not yet in the Template?
- Are there ADRs not reflected in the code?

## 3. Suggestion Selection

Decide on the **Next Step**.

- **If invoked with "low hanging fruits"**: Pick some small/medium, self-contained tasks.
- **Otherwise**: Pick the next most important features to move the template closer to the README/ADR vision.

## 4. Output Generation

For each suggestion explain the **reason** and create a **prompt** that can be pasted to a new agent instance to execute this step.

The agent should start with a test instance according to @../rules/development.md
The agent should keep the template repository on main.
It's crucial that the agent asks for approval before backporting. After approval it should verify being on clean main, checkout a new branch, backport and make changes to the template repository.

That allows to run several of these tasks in parallel with backporting as the only short bottleneck.