---
description: Generate a fresh template instance for development
---

# Instantiate Template

This workflow generates a fresh instance of the template in a temporary directory. Use this to test the template or to have a working environment for feature development.

## Steps

1.  **Prepare Output Directory**
    Ensure the output directory exists.

    ```bash
    # // turbo
    mkdir -p ../wulfpack-instances/
    ```

2.  **Determine Project Slug**
    Choose a context-specific slug for your instance, e.g., `tool-set-project_dynamodb` or `tool-set-project_auth`.
    
    *Variable*: `PROJECT_SLUG`

3.  **Run Cookiecutter**
    Generate the project using the chosen slug.

    ```bash
    # // turbo
    # Replace <PROJECT_SLUG> with your chosen slug
    cookiecutter template --output-dir ../wulfpack-instances/ --no-input project_slug=<PROJECT_SLUG>
    ```

    *Note: This assumes `cookiecutter` is installed. If not, install it via `pip install cookiecutter` or `brew install cookiecutter`.*

4.  **Verify Generation**
    Check if the project was created.

    ```bash
    # // turbo
    ls -F ../wulfpack-instances/<PROJECT_SLUG>/
    ```

5.  **Commit Baseline**
  
    Initialize a git repository and make a baseline commit.
    That helps in tracking changes during feature development.

    ```bash
    # // turbo
    cd ../wulfpack-instances/<PROJECT_SLUG>
    git init
    git add .
    git commit -m "Baseline"
    ```
    
6.  **Context Switch Warning**
    You are about to switch context to the generated instance.
    **Note**: The instance has its own `.agent/rules/global.md`. Do not confuse them with the repository-level rules.
    When in doubt about backporting or template structure, always refer back to the **repository rules** (`../../.agent/rules/global.md`).

7.  **Ready**
    You can now continue with the `develop-feature.md` workflow.