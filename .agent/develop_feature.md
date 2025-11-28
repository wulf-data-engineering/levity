---
description: Develop and test features in an instantiated project
---

# Develop Feature

This workflow guides you through developing a feature in the generated instance
for backporting it into the template.

**CRITICAL**: You must develop and verify features in the **instance** first. Do not modify the template directly until verification is complete.

## Prerequisites

- You must have run the `instantiate_template.md` workflow first.
- You must have run the `instantiate_template.md` workflow first.
- Working directory for this workflow is `tmp/<PROJECT_SLUG>`.

## Steps

1.  **Navigate to Instance**
    Switch your context to the generated project.
    _(Note: As an agent, you can just execute commands in `tmp/<PROJECT_SLUG>`)_

2.  **Install Dependencies**

    **Frontend**:

    ```bash
    cd tmp/<PROJECT_SLUG>/frontend
    npm install
    ```

    **Backend**:

    ```bash
    cd tmp/<PROJECT_SLUG>/backend
    cargo update
    ```

    **Infrastructure**:

    ```bash
    cd tmp/<PROJECT_SLUG>/infrastructure
    npm install
    ```

3.  **Implement Feature and Verify Feature**

    You **MUST** read the instructions at `tmp/<PROJECT_SLUG>/.agent/CONTEXT.md`
    before making any code changes in the template instance.

    Make your code changes in `tmp/<PROJECT_SLUG>/...`.
    Continuously test the running application.
    Run unit tests and end-to-end tests to verify your changes.
    Depending on the nature of the feature make changes to the `.agent` workflows.

4.  **Prepare for Backport**
    Once the feature works in the instance, you are ready to backport it to the template. Proceed to the `backport_changes.md` workflow.
