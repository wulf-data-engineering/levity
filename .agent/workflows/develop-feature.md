---
description: Develop and test features in a template instance
---

# Develop Feature

This workflow guides you through developing a feature in the generated instance
for backporting it into the template.

**CRITICAL**: You must develop and verify features in the **instance** first. Do not modify the template directly until verification is complete.

## Prerequisites

- You must have run the @instantiate-template.md workflow first.
- Working directory for this workflow is `../levity-instances/<PROJECT_SLUG>`.
- You have to be on a clean `main` branch.

## Persistent Reminder: Template Development Mode

You are currently working in a **temporary instance** (`../levity-instances/<PROJECT_SLUG>`).

**CRITICAL: DUAL CONTEXT**

1.  **Repo Rules**: You are **still bound** by the `levity` repo rules.
    Ref: @../rules/global.md

2.  **App Context**: You are acting as a developer of the _generated application_. You must follow the application's internal rules (e.g., Svelte, Rust patterns).
    Ref: @../../template/%[cookiecutter.project_slug]%/.agent/rules/global.md

3. **Resolved Template Values**: All wildcards from @../../template/cookiecutter.json are resolved to values in the test instance. If you add one of those values during implementation, mark them with a comment for later backporting, e.g.

  ```typescript
  configure('eu-central-1'); /* TODO: %[ cookiecutter.default_region ]% */
  ```

  ```html
  <h1>Welcome to Tool-Set Project!</h1> <!-- TODO: %[ cookiecutter.project_name ]% //-->
  ```

**If you are unsure about the tech stack, consult the template rules.**

## Steps

1.  **Navigate to Instance**
    Switch your context to the generated project.
    _(Note: As an agent, you can just execute commands in `../levity-instances/<PROJECT_SLUG>` until backporting)_

    ```bash
    // turbo
    cd ../levity-instances/<PROJECT_SLUG>
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
    cargo fetch
    ```

    **Infrastructure**:

    ```bash
    // turbo
    cd infrastructure
    npm install
    ```

3.  **Implement Feature and Verify Feature**

    Make your code changes in `../levity-instances/<PROJECT_SLUG>/...`.
    Use the available MCP servers.
    Continuously test the running application.
    Run unit tests and end-to-end tests to verify your changes.
    Ref: @../../template/%[cookiecutter.project_slug]%/.agent/workflows/run-locally.md
    Ref: @../../template/%[cookiecutter.project_slug]%/.agent/workflows/test-e2e.md

    Depending on the nature of the feature make changes to the `.agent` rules & workflows.

4.  **Run linters and formatters**
    Run the final checks from @../../template/%[cookiecutter.project_slug]%/.agent/rules/backend.md
    Run the final checks from @../../template/%[cookiecutter.project_slug]%/.agent/rules/frontend.md

5.  **Prepare for Backport**
    Once the feature works in the instance, you are ready to backport it to the template. Proceed to the @backport-changes.md workflow.