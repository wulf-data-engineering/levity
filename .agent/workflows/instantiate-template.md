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
    mkdir -p ../levity-instances/
    ```

2.  **Determine Project Slug**
    Choose a context-specific slug for your instance, e.g., `levity-project_dynamodb` or `levity-project_auth`.
    Derive a corresponding domain name, e.g., `levity-project-dynamodb.wulf.technology`.

    
    Ref: @../../template/cookiecutter.json
    
    *Variable*: `PROJECT_SLUG`

3.  **Run Cookiecutter**
    Generate the project using the chosen slug.

    ```bash
    # // turbo
    # Replace <PROJECT_SLUG> with your chosen slug
    cookiecutter template --output-dir ../levity-instances/ --no-input project_slug=<PROJECT_SLUG> domain=<PROJECT_DOMAIN>
    ```

    *Note: If cookiecutter fails because it's missing, install it via `pip install cookiecutter` or `brew install cookiecutter`.*

4.  **Commit Baseline**
  
    Initialize a git repository and make a baseline commit.
    That helps in tracking changes during feature development.

    ```bash
    # // turbo
    cd ../levity-instances/<PROJECT_SLUG>
    git init
    git add .
    git commit -m "Baseline"
    ```
   
5.  **Ready**
    You can now continue with the @develop-feature.md workflow.