---
description: Develop and test features in a template instance
---

# Develop Feature

This workflow guides you through developing a feature in the generated instance
for backporting it into the template.

**CRITICAL**: You must develop and verify features in the **instance** first. Do not modify the template directly until verification is complete.

## Prerequisites

- You must have run the `instantiate-template.md` workflow first.
- Working directory for this workflow is `../wulfpack-instances/<PROJECT_SLUG>`.
- You have to be on a clean `main` branch.

## Steps

1.  **Navigate to Instance**
    Switch your context to the generated project.
    _(Note: As an agent, you can just execute commands in `../wulfpack-instances/<PROJECT_SLUG>`)_

    ```bash
    // turbo
    cd ../wulfpack-instances/<PROJECT_SLUG>
    ```

2.  **Install Dependencies**

    **Frontend**:

    ```bash
    // turbo
    cd frontend
    npm install
    ```

    **Backend**:

    ```bash
    // turbo
    cd backend
    cargo update
    ```

    **Infrastructure**:

    ```bash
    // turbo
    cd infrastructure
    npm install
    ```

3.  **Implement Feature and Verify Feature**

    You **MUST** read and **MUST FOLLOW** the rules in `../wulfpack-instances/<PROJECT_SLUG>/.agent/rules/` and workflows in `../wulfpack-instances/<PROJECT_SLUG>/.agent/workflows/` before making any code changes in the template instance.

    Make your code changes in `../wulfpack-instances/<PROJECT_SLUG>/...`.
    Continuously test the running application.
    Run unit tests and end-to-end tests to verify your changes.
    Depending on the nature of the feature make changes to the `.agent` rules & workflows.

4.  **Run linters**
    Run the final checks from template/%[cookiecutter.project_slug]%/.agent/rules/backend.md
    Run the final checks from template/%[cookiecutter.project_slug]%/.agent/rules/frontend.md

5.  **Prepare for Backport**
    Once the feature works in the instance, you are ready to backport it to the template. Proceed to the `backport-changes.md` workflow.